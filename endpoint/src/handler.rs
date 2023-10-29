/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use crate::utils::{guard, jwt, aws::s3::S3, api::ErrorBuilder};
use worker::{Context, Env, Request, Response};
use apache_avro::{Codec, Writer};
use publish_data_rest_schema::generic;

pub async fn handle(
    mut req: Request,
    env: Env,
    _: Context,
) -> Result<Response, Box<dyn std::error::Error>> {
    if let Err(res) = guard::method(&req) { return Ok(res); }
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
    match req.json::<generic::Input>().await {
        Ok(json) => {
            let key = format!("/{}/{}.avro", token.claims().custom.subject, "UUID HERE.");
            let schema = generic::output();
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
