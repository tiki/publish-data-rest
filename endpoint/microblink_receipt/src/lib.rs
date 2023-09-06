/*
* Copyright (c) TIKI Inc.
* MIT license. See LICENSE file in root directory.
*/

mod microblink_receipt;

use apache_avro::{Codec, Writer};
use ingest_lib::{
    api::{cors, ErrorBuilder},
    aws::s3::S3,
    jwt, Ingest,
};
use microblink_receipt::MicroblinkReceipt;
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
    match req.json::<MicroblinkReceipt>().await {
        Ok(json) => {
            let key = format!("/{}/{}.avro", token.claims().custom.subject, json.id());
            let schema = MicroblinkReceipt::schema();
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
    use crate::microblink_receipt::MicroblinkReceipt;
    use apache_avro::{Codec, Writer};
    use ingest_lib::Ingest;
    use serde_json;

    const INPUT: &str = "{\"receiptDate\":{\"confidence\":99.99691,\"value\":\"07/21/2023\"},\"receiptTime\":{\"confidence\":99.98615,\"value\":\"19:42\"},\"retailerId\":{\"id\":0,\"bannerId\":8598},\"products\":[{\"productNumber\":{\"confidence\":-1,\"value\":\"\"},\"description\":{\"confidence\":99.95183,\"value\":\"CHKQUESCOMBO\"},\"quantity\":{\"confidence\":0,\"value\":1},\"unitPrice\":{\"confidence\":99.95183,\"value\":8.99},\"unitOfMeasure\":{\"confidence\":0},\"totalPrice\":{\"confidence\":100,\"value\":8.99},\"fullPrice\":-1,\"line\":1012,\"productName\":\"ChickenQuesadillaCombo\",\"category\":\"Food|Appetizers&Sides|Quesadillas\",\"imageUrl\":\"https:\\\\/\\\\/d1ralsognjng37.cloudfront.net\\\\/6fa84792-8822-49b6-932a-1a3ae4334d7a.jpeg\",\"additionalLines\":[{\"type\":{\"confidence\":100},\"text\":{\"confidence\":0,\"value\":\"ChickenQuesadilla0.00\"},\"lineNumber\":1013},{\"type\":{\"confidence\":100},\"text\":{\"confidence\":0,\"value\":\"SeasonedFries0.00\"},\"lineNumber\":1014},{\"type\":{\"confidence\":100},\"text\":{\"confidence\":0,\"value\":\"NachoChzDip0.00\"},\"lineNumber\":1015},{\"type\":{\"confidence\":100},\"text\":{\"confidence\":0,\"value\":\"LrgMtDew0.00\"},\"lineNumber\":1016}],\"priceAfterCoupons\":{\"confidence\":0,\"value\":0},\"voided\":false,\"probability\":-1,\"sensitive\":false,\"subProducts\":[],\"added\":false,\"attributes\":[],\"sector\":\"Food\",\"department\":\"Appetizers&Sides\",\"majorCategory\":\"Quesadillas\"},{\"productNumber\":{\"confidence\":-1,\"value\":\"\"},\"description\":{\"confidence\":99.992294,\"value\":\"ChickenQuesadilla\"},\"quantity\":{\"confidence\":0,\"value\":1},\"unitPrice\":{\"confidence\":99.98729,\"value\":0},\"unitOfMeasure\":{\"confidence\":0},\"totalPrice\":{\"confidence\":100,\"value\":0},\"fullPrice\":0,\"line\":1013,\"priceAfterCoupons\":{\"confidence\":0,\"value\":0},\"voided\":false,\"probability\":-1,\"sensitive\":false,\"added\":false,\"attributes\":[]},{\"productNumber\":{\"confidence\":-1,\"value\":\"\"},\"description\":{\"confidence\":99.982285,\"value\":\"SeasonedFries\"},\"quantity\":{\"confidence\":0,\"value\":1},\"unitPrice\":{\"confidence\":99.95366,\"value\":0},\"unitOfMeasure\":{\"confidence\":0},\"totalPrice\":{\"confidence\":100,\"value\":0},\"fullPrice\":0,\"line\":1014,\"priceAfterCoupons\":{\"confidence\":0,\"value\":0},\"voided\":false,\"probability\":-1,\"sensitive\":false,\"added\":false,\"attributes\":[]},{\"productNumber\":{\"confidence\":-1,\"value\":\"\"},\"description\":{\"confidence\":99.99717,\"value\":\"NachoChzDip\"},\"quantity\":{\"confidence\":0,\"value\":1},\"unitPrice\":{\"confidence\":99.99562,\"value\":0},\"unitOfMeasure\":{\"confidence\":0},\"totalPrice\":{\"confidence\":100,\"value\":0},\"fullPrice\":0,\"line\":1015,\"priceAfterCoupons\":{\"confidence\":0,\"value\":0},\"voided\":false,\"probability\":-1,\"sensitive\":false,\"added\":false,\"attributes\":[]},{\"productNumber\":{\"confidence\":-1,\"value\":\"\"},\"description\":{\"confidence\":99.997765,\"value\":\"LrgMtDew\"},\"quantity\":{\"confidence\":0,\"value\":1},\"unitPrice\":{\"confidence\":99.99189,\"value\":0},\"unitOfMeasure\":{\"confidence\":0},\"totalPrice\":{\"confidence\":100,\"value\":0},\"fullPrice\":0,\"line\":1016,\"priceAfterCoupons\":{\"confidence\":0,\"value\":0},\"voided\":false,\"probability\":-1,\"sensitive\":false,\"added\":false,\"attributes\":[]},{\"productNumber\":{\"confidence\":-1,\"value\":\"\"},\"description\":{\"confidence\":99.99393,\"value\":\"BEEFSOFTTACO\"},\"quantity\":{\"confidence\":0,\"value\":2},\"unitPrice\":{\"confidence\":99.99393,\"value\":1.79},\"unitOfMeasure\":{\"confidence\":0},\"totalPrice\":{\"confidence\":100,\"value\":3.58},\"fullPrice\":-1,\"line\":1017,\"productName\":\"SoftTaco\",\"category\":\"Food|Tacos&Burritos|MeatTacos\",\"imageUrl\":\"https:\\\\/\\\\/d1ralsognjng37.cloudfront.net\\\\/ba026203-2472-4b5f-a426-c89d0cf1d599.jpeg\",\"priceAfterCoupons\":{\"confidence\":0,\"value\":0},\"voided\":false,\"probability\":-1,\"sensitive\":false,\"added\":false,\"attributes\":[],\"sector\":\"Food\",\"department\":\"Tacos&Burritos\",\"majorCategory\":\"MeatTacos\"},{\"productNumber\":{\"confidence\":-1,\"value\":\"\"},\"description\":{\"confidence\":99.98706,\"value\":\"CRUNCHWRAPSUPREME\"},\"quantity\":{\"confidence\":0,\"value\":1},\"unitPrice\":{\"confidence\":99.98706,\"value\":5.19},\"unitOfMeasure\":{\"confidence\":0},\"totalPrice\":{\"confidence\":100,\"value\":5.19},\"fullPrice\":-1,\"line\":1018,\"productName\":\"CrunchwrapSupreme\",\"category\":\"Food|Sandwiches&Wraps|MeatWrap\",\"imageUrl\":\"https:\\\\/\\\\/d1ralsognjng37.cloudfront.net\\\\/e4bae42b-aeaf-44f9-b18d-63df0c4a7358.jpeg\",\"priceAfterCoupons\":{\"confidence\":0,\"value\":0},\"voided\":false,\"probability\":-1,\"sensitive\":false,\"added\":false,\"attributes\":[],\"sector\":\"Food\",\"department\":\"Sandwiches&Wraps\",\"majorCategory\":\"MeatWrap\"}],\"total\":{\"confidence\":100,\"value\":19.4},\"subtotal\":{\"confidence\":88.76194,\"value\":17.76},\"taxes\":{\"confidence\":99.995056,\"value\":1.64},\"merchantName\":{\"confidence\":100,\"value\":\"TacoBell\"},\"storeAddress\":{\"confidence\":0,\"value\":\"2101RosaL.ParksBlvd.\"},\"storeCity\":{\"confidence\":0,\"value\":\"Nashville\"},\"blinkReceiptId\":\"4eb3a6ee-ace5-45ef-90d7-702f57e935ec\",\"storeState\":{\"confidence\":0,\"value\":\"TN\"},\"storeZip\":{\"confidence\":99.97463,\"value\":\"37228\"},\"storePhone\":{\"confidence\":0,\"value\":\"615-256-3382\"},\"paymentMethods\":[{\"paymentMethod\":{\"confidence\":99.98854,\"value\":\"Credit\"},\"cardType\":{\"confidence\":100,\"value\":\"Mastercard\"},\"cardIssuer\":{\"confidence\":0},\"amount\":{\"confidence\":0,\"value\":19.4}}],\"last4cc\":{\"confidence\":99.9952,\"value\":\"3210\"},\"ocrConfidence\":99.165535,\"merchantSource\":\"PHONE\",\"foundTopEdge\":false,\"foundBottomEdge\":true,\"subtotalMatches\":true,\"eReceiptAuthenticated\":false,\"instacartShopper\":false,\"eReceipt\":false,\"duplicate\":false,\"fraudulent\":false,\"receiptDateTime\":1689986520000,\"productsPendingLookup\":0,\"loyaltyForBanner\":false,\"channel\":{\"confidence\":1,\"value\":\"QuickService\"},\"eReceiptShippingCosts\":0,\"loyaltyProgram\":false,\"merchantSources\":[],\"combinedRawText\":{\"confidence\":100,\"value\":\"d\\nTellthebell.com\\n\\nSurveyCode:99672032-11911427\\n\\nTacoBell016292\\n2101RosaL.ParksBlvd.\\nNashville,TN37228\\n(615)256-3382\\n7/21/20237:42:35PM\\nOrder365499Cashier:ESAUL\\n\\nCHKQUESCOMBO8.99\\nChickenQuesadilla0.00\\nSeasonedFries0.00\\nNachoChzDip0.00\\nLrgMtDew0.00\\nBeefSoftTaco3.58\\nCrunchwrapSupreme5.19\\nSubTotal17.762\\nTax1.64\\nTotal19.40\\nMasterCard19.40\\nAcct:XXXXXXXX3210\\nApproval:06807B\\n\\nEntryMode:Contactless\\nAPL:MASTERCARD\\nAID:A0000000041010\\nTVR:0000008001\\nTSI:E800\\nIAD:0110A04001220000000000000000000099\\nFF\\nCVM:NONE\\n\\nOrderNumber499\\nTotalItemsinOrder:7\\n\\n\\nStartearningFREErewardsnow.\\n\\nDownloadtheTacoBellApp&\\nScanthebarcodebelowtogetyourpointg.\\nTermsandexclusionsapply\\n\\n\\n9\\n1859804278226\\n18598042782260\\n\"}}";

    #[test]
    fn deserialize() {
        let json = serde_json::from_str::<MicroblinkReceipt>(INPUT);
        assert_eq!(json.is_ok(), true);
    }

    #[test]
    fn serialize() {
        let schema = MicroblinkReceipt::schema();
        let mut writer = Writer::with_codec(&schema, Vec::new(), Codec::Snappy);
        let json = serde_json::from_str::<MicroblinkReceipt>(INPUT).unwrap();
        writer.append_ser(json).unwrap();
        assert_eq!(writer.into_inner().is_ok(), true);
    }
}
