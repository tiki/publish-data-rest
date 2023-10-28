/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use apache_avro::Schema;
use ingest_lib::Ingest;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShopifyOrder {
    pub app_id: Option<u64>,
    pub billing_address: Option<Address>,
    pub browser_ip: Option<String>,
    pub buyer_accepts_marketing: Option<bool>,
    pub cancel_reason: Option<String>,
    pub cancelled_at: Option<String>,
    pub cart_token: Option<String>,
    pub checkout_token: Option<String>,
    pub client_details: Option<ClientDetails>,
    pub closed_at: Option<String>,
    pub company: Option<String>,
    pub created_at: Option<String>,
    pub currency: Option<String>,
    pub current_total_additional_fees_set: Option<PriceSet>,
    pub current_total_discounts: Option<String>,
    pub current_total_discounts_set: Option<PriceSet>,
    pub current_total_duties_set: Option<PriceSet>,
    pub current_total_price: Option<String>,
    pub current_total_price_set: Option<PriceSet>,
    pub current_subtotal_price: Option<String>,
    pub current_subtotal_price_set: Option<PriceSet>,
    pub current_total_tax: Option<String>,
    pub current_total_tax_set: Option<PriceSet>,
    pub customer: Option<Customer>,
    pub customer_locale: Option<String>,
    pub discount_applications: Option<Vec<DiscountApplication>>,
    pub discount_codes: Option<Vec<DiscountCode>>,
    pub email: Option<String>,
    pub estimated_taxes: Option<bool>,
    pub financial_status: Option<String>,
    pub fulfillments: Option<Vec<Fulfillment>>,
    pub fulfillment_status: Option<String>,
    pub gateway: Option<String>,
    pub id: u64,
    pub landing_site: Option<String>,
    pub line_items: Option<Vec<LineItem>>,
    pub location_id: Option<u64>,
    pub merchant_of_record_app_id: Option<u64>,
    pub name: Option<String>,
    pub note: Option<String>,
    pub note_attributes: Option<Vec<NoteAttribute>>,
    pub number: Option<u64>,
    pub order_number: Option<u64>,
    pub original_total_additional_fees_set: Option<PriceSet>,
    pub original_total_duties_set: Option<PriceSet>,
    pub payment_terms: Option<PaymentTerm>,
    pub payment_gateway_names: Option<Vec<String>>,
    pub phone: Option<String>,
    pub presentment_currency: Option<String>,
    pub processed_at: Option<String>,
    pub referring_site: Option<String>,
    pub refunds: Option<Vec<Refund>>,
    pub shipping_address: Option<Address>,
    pub shipping_lines: Option<Vec<ShippingLine>>,
    pub source_name: Option<String>,
    pub source_identifier: Option<String>,
    pub source_url: Option<String>,
    pub subtotal_price: Option<String>,
    pub subtotal_price_set: Option<PriceSet>,
    pub tags: Option<String>,
    pub tax_lines: Option<Vec<TaxLine>>,
    pub taxes_included: Option<bool>,
    pub test: Option<bool>,
    pub token: Option<String>,
    pub total_discounts: Option<String>,
    pub total_discounts_set: Option<PriceSet>,
    pub total_line_items_price: Option<String>,
    pub total_line_items_price_set: Option<PriceSet>,
    pub total_outstanding: Option<String>,
    pub total_price: Option<String>,
    pub total_price_set: Option<PriceSet>,
    pub total_shipping_price_set: Option<PriceSet>,
    pub total_tax: Option<String>,
    pub total_tax_set: Option<PriceSet>,
    pub total_tip_received: Option<String>,
    pub total_weight: Option<u64>,
    pub updated_at: Option<String>,
    pub user_id: Option<u64>,
    pub order_status_url: Option<String>,
}

impl Ingest for ShopifyOrder {
    fn id(&self) -> String {
        self.id.to_string()
    }
    fn schema() -> Schema {
        let raw_schema = r#"
{{{avro}}}
"#;
        Schema::parse_str(raw_schema).unwrap()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub company: Option<String>,
    pub country: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub province: Option<String>,
    pub zip: Option<String>,
    pub name: Option<String>,
    pub province_code: Option<String>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub default: Option<bool>,
    pub id: Option<u64>,
    pub customer_id: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientDetails {
    pub accept_language: Option<String>,
    pub browser_height: Option<u16>,
    pub browser_ip: Option<String>,
    pub browser_width: Option<u16>,
    pub session_hash: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceSet {
    pub shop_money: Option<PriceSetMoney>,
    pub presentment_money: Option<PriceSetMoney>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceSetMoney {
    pub amount: Option<String>,
    pub currency_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Customer {
    pub accepts_marketing: Option<bool>,
    pub accepts_marketing_updated_at: Option<String>,
    pub addresses: Option<Vec<Address>>,
    pub currency: Option<String>,
    pub created_at: Option<String>,
    pub default_address: Option<Address>,
    pub email: Option<String>,
    pub email_marketing_consent: Option<MarketingConsent>,
    pub first_name: Option<String>,
    pub id: Option<u64>,
    pub last_name: Option<String>,
    pub last_order_id: Option<u64>,
    pub last_order_name: Option<String>,
    pub metafield: Option<Metafield>,
    pub marketing_opt_in_level: Option<String>,
    // pub multipass_identifier: Option<String>,
    pub note: Option<String>,
    pub orders_count: Option<u64>,
    // pub password: Option<String>,
    // pub password_confirmation: Option<String>,
    pub phone: Option<String>,
    pub sms_marketing_consent: Option<MarketingConsent>,
    pub state: Option<String>,
    pub tags: Option<String>,
    pub tax_exempt: Option<bool>,
    pub tax_exemptions: Option<Vec<String>>,
    pub total_spent: Option<String>,
    pub updated_at: Option<String>,
    pub verified_email: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketingConsent {
    pub state: Option<String>,
    pub opt_in_level: Option<String>,
    pub consent_updated_at: Option<String>,
    pub consent_collected_from: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metafield {
    pub key: Option<String>,
    pub namespace: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscountApplication {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub value: Option<String>,
    pub value_type: Option<String>,
    pub allocation_method: Option<String>,
    pub target_selection: Option<String>,
    pub target_type: Option<String>,
    pub code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscountCode {
    pub code: Option<String>,
    pub amount: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fulfillment {
    pub created_at: Option<String>,
    pub id: Option<u64>,
    pub line_items: Option<Vec<LineItem>>,
    pub location_id: Option<u64>,
    pub name: Option<String>,
    pub notify_customer: Option<bool>,
    pub order_id: Option<u64>,
    pub origin_address: Option<Address>,
    pub receipt: Option<Receipt>,
    pub service: Option<String>,
    pub shipment_status: Option<String>,
    pub status: Option<String>,
    pub tracking_company: Option<String>,
    pub tracking_numbers: Option<Vec<String>>,
    pub tracking_number: Option<String>,
    pub tracking_urls: Option<Vec<String>>,
    pub tracking_url: Option<String>,
    pub updated_at: Option<String>,
    pub variant_inventory_management: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Receipt {
    pub testcase: Option<bool>,
    pub authorization: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineItem {
    pub id: Option<u64>,
    pub variant_id: Option<u64>,
    pub title: Option<String>,
    pub quantity: Option<u64>,
    pub price: Option<String>,
    pub grams: Option<u64>,
    pub sku: Option<String>,
    pub variant_title: Option<String>,
    pub vendor: Option<String>,
    pub fulfillment_service: Option<String>,
    pub product_id: Option<u64>,
    pub requires_shipping: Option<bool>,
    pub taxable: Option<bool>,
    pub gift_card: Option<bool>,
    pub name: Option<String>,
    pub variant_inventory_management: Option<String>,
    pub properties: Option<Vec<Property>>,
    pub product_exists: Option<bool>,
    pub fulfillable_quantity: Option<u64>,
    pub total_discount: Option<String>,
    pub fulfillment_status: Option<String>,
    pub fulfillment_line_item_id: Option<u64>,
    pub tax_lines: Option<Vec<TaxLine>>,
    pub duties: Option<Vec<Duty>>,
    pub attributed_staffs: Option<Vec<Staff>>,
    pub price_set: Option<PriceSet>,
    pub total_discount_set: Option<PriceSet>,
    pub discount_allocations: Option<Vec<DiscountAllocation>>,
    pub origin_location: Option<Address>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxLine {
    pub title: Option<String>,
    pub price: Option<String>,
    pub rate: Option<String>,
    pub price_set: Option<PriceSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Duty {
    pub id: Option<String>,
    pub harmonized_system_code: Option<String>,
    pub country_code_of_origin: Option<String>,
    pub shop_money: Option<PriceSetMoney>,
    pub presentment_money: Option<PriceSetMoney>,
    pub tax_lines: Option<Vec<TaxLine>>,
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Staff {
    pub id: Option<String>,
    pub quantity: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscountAllocation {
    pub amount: Option<String>,
    pub discount_application_index: Option<u64>,
    pub amount_set: Option<PriceSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteAttribute {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentTerm {
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub payment_terms_name: Option<String>,
    pub payment_terms_type: Option<String>,
    pub due_in_days: Option<u64>,
    pub payment_schedules: Option<Vec<PaymentSchedule>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentSchedule {
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub issued_at: Option<String>,
    pub due_at: Option<String>,
    pub completed_at: Option<String>,
    pub expected_payment_method: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Refund {
    pub created_at: Option<String>,
    pub duties: Option<Vec<Duty>>,
    pub id: Option<u64>,
    pub note: Option<String>,
    pub order_adjustments: Option<Vec<OrderAdjustment>>,
    pub processed_at: Option<String>,
    pub refund_duties: Option<Vec<RefundDuty>>,
    pub refund_line_items: Option<Vec<RefundLineItem>>,
    pub restock: Option<bool>,
    pub transactions: Option<Vec<Transaction>>,
    pub user_id: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderAdjustment {
    pub id: Option<u64>,
    pub order_id: Option<u64>,
    pub refund_id: Option<u64>,
    pub amount: Option<String>,
    pub tax_amount: Option<String>,
    pub kind: Option<String>,
    pub reason: Option<String>,
    pub amount_set: Option<PriceSet>,
    pub tax_amount_set: Option<PriceSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundDuty {
    pub duty_id: Option<u64>,
    pub refund_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundLineItem {
    pub id: Option<u64>,
    pub line_item: Option<LineItem>,
    pub line_item_id: Option<u64>,
    pub quantity: Option<u64>,
    pub location_id: Option<u64>,
    pub restock_type: Option<String>,
    pub subtotal: Option<String>,
    pub total_tax: Option<String>,
    pub subtotal_set: Option<PriceSet>,
    pub total_tax_set: Option<PriceSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<u64>,
    pub order_id: Option<u64>,
    pub amount: Option<String>,
    pub kind: Option<String>,
    pub gateway: Option<String>,
    pub status: Option<String>,
    pub message: Option<String>,
    pub created_at: Option<String>,
    pub test: Option<bool>,
    pub authorization: Option<String>,
    pub currency: Option<String>,
    pub location_id: Option<u64>,
    pub user_id: Option<u64>,
    pub parent_id: Option<u64>,
    pub device_id: Option<u64>,
    pub receipt: Option<Receipt>,
    pub error_code: Option<String>,
    pub source_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingLine {
    pub code: Option<String>,
    pub price: Option<String>,
    pub price_set: Option<PriceSet>,
    pub discounted_price: Option<String>,
    pub discounted_price_set: Option<PriceSet>,
    pub source: Option<String>,
    pub title: Option<String>,
    pub tax_lines: Option<Vec<TaxLine>>,
    pub carrier_identifier: Option<String>,
    pub requested_fulfillment_service_id: Option<String>,
}
