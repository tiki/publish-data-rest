/*
* Copyright (c) TIKI Inc.
* MIT license. See LICENSE file in root directory.
*/

mod shopify_order;

use apache_avro::{Codec, Writer};
use ingest_lib::{
    api::{cors, ErrorBuilder},
    aws::s3::S3,
    jwt, Ingest,
};
use shopify_order::ShopifyOrder;
use worker::{event, Context, Env, Request, Response, Result};

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let res: Result<Response> = match wrapper(req, env, ctx).await {
        Ok(res) => Ok(res),
        Err(error) => Ok(match error.downcast_ref::<ErrorBuilder>() {
            Some(error) => error.to_response(),
            None => ErrorBuilder::new()
                .status(500)
                .message(&error.to_string())
                .to_response(),
        }),
    };
    cors::apply(res.unwrap())
}

async fn wrapper(
    mut req: Request,
    env: Env,
    _: Context,
) -> std::result::Result<Response, Box<dyn std::error::Error>> {
    if let Err(res) = guard_method(&req) {
        return Ok(res);
    }
    let public_key = env.var("PUBLIC_KEY")?.to_string();
    let token = jwt::validate(
        &req,
        &public_key,
        jwt::Config {
            issuer: Some(String::from("com.mytiki.l0_auth")),
            audience: Some(String::from("storage.l0.mytiki.com")),
            clock_skew: 60,
        },
    )?;
    let s3 = S3::new(&env.var("BUCKET")?.to_string())
        .with_region(&env.var("BUCKET_REGION")?.to_string())
        .with_key(
            &env.var("BUCKET_KEY_ID")?.to_string(),
            &env.var("BUCKET_KEY_SECRET")?.to_string(),
        );
    match req.json::<ShopifyOrder>().await {
        Ok(json) => {
            let key = format!("/{}/{}.avro", token.claims().custom.subject, json.id());
            let schema = ShopifyOrder::schema();
            let mut writer = Writer::with_codec(&schema, Vec::new(), Codec::Snappy);
            writer.append_ser(json).unwrap();
            let encoded = writer.into_inner()?;
            s3.put(&key, encoded).await?;
            Ok(Response::empty().unwrap().with_status(201))
        }
        Err(e) => Err(ErrorBuilder::new()
            .status(400)
            .message("JSON deserialization failed")
            .detail(&e.to_string())
            .help("Double-check the request body")
            .to_error()),
    }
}

fn guard_method(req: &Request) -> std::result::Result<(), Response> {
    if let Err(res) = cors::guard(&req) {
        return Err(res);
    } else if !req.method().eq(&worker::Method::Post) {
        Err(ErrorBuilder::new()
            .status(405)
            .help("Try POST")
            .to_response())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::shopify_order::ShopifyOrder;
    use apache_avro::{Codec, Writer};
    use ingest_lib::Ingest;
    use serde_json;

    const INPUT: &str = "{\"id\":820982911946154508,\"admin_graphql_api_id\":\"gid://shopify/Order/820982911946154508\",\"app_id\":null,\"browser_ip\":null,\"buyer_accepts_marketing\":true,\"cancel_reason\":\"customer\",\"cancelled_at\":\"2021-12-31T19:00:00-05:00\",\"cart_token\":null,\"checkout_id\":null,\"checkout_token\":null,\"client_details\":null,\"closed_at\":null,\"confirmed\":false,\"contact_email\":\"jon@example.com\",\"created_at\":\"2021-12-31T19:00:00-05:00\",\"currency\":\"USD\",\"current_subtotal_price\":\"398.00\",\"current_subtotal_price_set\":{\"shop_money\":{\"amount\":\"398.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"398.00\",\"currency_code\":\"USD\"}},\"current_total_additional_fees_set\":null,\"current_total_discounts\":\"0.00\",\"current_total_discounts_set\":{\"shop_money\":{\"amount\":\"0.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"0.00\",\"currency_code\":\"USD\"}},\"current_total_duties_set\":null,\"current_total_price\":\"398.00\",\"current_total_price_set\":{\"shop_money\":{\"amount\":\"398.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"398.00\",\"currency_code\":\"USD\"}},\"current_total_tax\":\"0.00\",\"current_total_tax_set\":{\"shop_money\":{\"amount\":\"0.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"0.00\",\"currency_code\":\"USD\"}},\"customer_locale\":\"en\",\"device_id\":null,\"discount_codes\":[],\"email\":\"jon@example.com\",\"estimated_taxes\":false,\"financial_status\":\"voided\",\"fulfillment_status\":\"pending\",\"landing_site\":null,\"landing_site_ref\":null,\"location_id\":null,\"merchant_of_record_app_id\":null,\"name\":\"#9999\",\"note\":null,\"note_attributes\":[],\"number\":234,\"order_number\":1234,\"order_status_url\":\"https://jsmith.myshopify.com/548380009/orders/123456abcd/authenticate?key=abcdefg\",\"original_total_additional_fees_set\":null,\"original_total_duties_set\":null,\"payment_gateway_names\":[\"visa\",\"bogus\"],\"phone\":null,\"po_number\":null,\"presentment_currency\":\"USD\",\"processed_at\":null,\"reference\":null,\"referring_site\":null,\"source_identifier\":null,\"source_name\":\"web\",\"source_url\":null,\"subtotal_price\":\"393.00\",\"subtotal_price_set\":{\"shop_money\":{\"amount\":\"393.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"393.00\",\"currency_code\":\"USD\"}},\"tags\":\"\",\"tax_lines\":[],\"taxes_included\":false,\"test\":true,\"token\":\"123456abcd\",\"total_discounts\":\"5.00\",\"total_discounts_set\":{\"shop_money\":{\"amount\":\"5.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"5.00\",\"currency_code\":\"USD\"}},\"total_line_items_price\":\"398.00\",\"total_line_items_price_set\":{\"shop_money\":{\"amount\":\"398.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"398.00\",\"currency_code\":\"USD\"}},\"total_outstanding\":\"398.00\",\"total_price\":\"403.00\",\"total_price_set\":{\"shop_money\":{\"amount\":\"403.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"403.00\",\"currency_code\":\"USD\"}},\"total_shipping_price_set\":{\"shop_money\":{\"amount\":\"10.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"10.00\",\"currency_code\":\"USD\"}},\"total_tax\":\"0.00\",\"total_tax_set\":{\"shop_money\":{\"amount\":\"0.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"0.00\",\"currency_code\":\"USD\"}},\"total_tip_received\":\"0.00\",\"total_weight\":0,\"updated_at\":\"2021-12-31T19:00:00-05:00\",\"user_id\":null,\"billing_address\":{\"first_name\":\"Steve\",\"address1\":\"123ShippingStreet\",\"phone\":\"555-555-SHIP\",\"city\":\"Shippington\",\"zip\":\"40003\",\"province\":\"Kentucky\",\"country\":\"UnitedStates\",\"last_name\":\"Shipper\",\"address2\":null,\"company\":\"ShippingCompany\",\"latitude\":null,\"longitude\":null,\"name\":\"SteveShipper\",\"country_code\":\"US\",\"province_code\":\"KY\"},\"customer\":{\"id\":115310627314723954,\"email\":\"john@example.com\",\"accepts_marketing\":false,\"created_at\":null,\"updated_at\":null,\"first_name\":\"John\",\"last_name\":\"Smith\",\"state\":\"disabled\",\"note\":null,\"verified_email\":true,\"multipass_identifier\":null,\"tax_exempt\":false,\"phone\":null,\"email_marketing_consent\":{\"state\":\"not_subscribed\",\"opt_in_level\":null,\"consent_updated_at\":null},\"sms_marketing_consent\":null,\"tags\":\"\",\"currency\":\"USD\",\"accepts_marketing_updated_at\":null,\"marketing_opt_in_level\":null,\"tax_exemptions\":[],\"admin_graphql_api_id\":\"gid://shopify/Customer/115310627314723954\",\"default_address\":{\"id\":715243470612851245,\"customer_id\":115310627314723954,\"first_name\":null,\"last_name\":null,\"company\":null,\"address1\":\"123ElmSt.\",\"address2\":null,\"city\":\"Ottawa\",\"province\":\"Ontario\",\"country\":\"Canada\",\"zip\":\"K2H7A8\",\"phone\":\"123-123-1234\",\"name\":\"\",\"province_code\":\"ON\",\"country_code\":\"CA\",\"country_name\":\"Canada\",\"default\":true}},\"discount_applications\":[],\"fulfillments\":[],\"line_items\":[{\"id\":866550311766439020,\"admin_graphql_api_id\":\"gid://shopify/LineItem/866550311766439020\",\"attributed_staffs\":[{\"id\":\"gid://shopify/StaffMember/902541635\",\"quantity\":1}],\"fulfillable_quantity\":1,\"fulfillment_service\":\"manual\",\"fulfillment_status\":null,\"gift_card\":false,\"grams\":567,\"name\":\"IPodNano-8GB\",\"price\":\"199.00\",\"price_set\":{\"shop_money\":{\"amount\":\"199.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"199.00\",\"currency_code\":\"USD\"}},\"product_exists\":true,\"product_id\":632910392,\"properties\":[],\"quantity\":1,\"requires_shipping\":true,\"sku\":\"IPOD2008PINK\",\"taxable\":true,\"title\":\"IPodNano-8GB\",\"total_discount\":\"0.00\",\"total_discount_set\":{\"shop_money\":{\"amount\":\"0.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"0.00\",\"currency_code\":\"USD\"}},\"variant_id\":808950810,\"variant_inventory_management\":\"shopify\",\"variant_title\":null,\"vendor\":null,\"tax_lines\":[],\"duties\":[],\"discount_allocations\":[]},{\"id\":141249953214522974,\"admin_graphql_api_id\":\"gid://shopify/LineItem/141249953214522974\",\"attributed_staffs\":[],\"fulfillable_quantity\":1,\"fulfillment_service\":\"manual\",\"fulfillment_status\":null,\"gift_card\":false,\"grams\":567,\"name\":\"IPodNano-8GB\",\"price\":\"199.00\",\"price_set\":{\"shop_money\":{\"amount\":\"199.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"199.00\",\"currency_code\":\"USD\"}},\"product_exists\":true,\"product_id\":632910392,\"properties\":[],\"quantity\":1,\"requires_shipping\":true,\"sku\":\"IPOD2008PINK\",\"taxable\":true,\"title\":\"IPodNano-8GB\",\"total_discount\":\"0.00\",\"total_discount_set\":{\"shop_money\":{\"amount\":\"0.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"0.00\",\"currency_code\":\"USD\"}},\"variant_id\":808950810,\"variant_inventory_management\":\"shopify\",\"variant_title\":null,\"vendor\":null,\"tax_lines\":[],\"duties\":[],\"discount_allocations\":[]}],\"payment_terms\":null,\"refunds\":[],\"shipping_address\":{\"first_name\":\"Steve\",\"address1\":\"123ShippingStreet\",\"phone\":\"555-555-SHIP\",\"city\":\"Shippington\",\"zip\":\"40003\",\"province\":\"Kentucky\",\"country\":\"UnitedStates\",\"last_name\":\"Shipper\",\"address2\":null,\"company\":\"ShippingCompany\",\"latitude\":null,\"longitude\":null,\"name\":\"SteveShipper\",\"country_code\":\"US\",\"province_code\":\"KY\"},\"shipping_lines\":[{\"id\":271878346596884015,\"carrier_identifier\":null,\"code\":null,\"discounted_price\":\"10.00\",\"discounted_price_set\":{\"shop_money\":{\"amount\":\"10.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"10.00\",\"currency_code\":\"USD\"}},\"phone\":null,\"price\":\"10.00\",\"price_set\":{\"shop_money\":{\"amount\":\"10.00\",\"currency_code\":\"USD\"},\"presentment_money\":{\"amount\":\"10.00\",\"currency_code\":\"USD\"}},\"requested_fulfillment_service_id\":null,\"source\":\"shopify\",\"title\":\"GenericShipping\",\"tax_lines\":[],\"discount_allocations\":[]}]}";

    #[test]
    fn deserialize() {
        let json = serde_json::from_str::<ShopifyOrder>(INPUT);
        assert_eq!(json.is_ok(), true);
    }

    #[test]
    fn serialize() {
        let schema = ShopifyOrder::schema();
        let mut writer = Writer::with_codec(&schema, Vec::new(), Codec::Snappy);
        let json = serde_json::from_str::<ShopifyOrder>(INPUT).unwrap();
        writer.append_ser(json).unwrap();
        assert_eq!(writer.into_inner().is_ok(), true);
    }
}
