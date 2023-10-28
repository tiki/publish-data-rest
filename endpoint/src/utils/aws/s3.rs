/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use crate::utils::aws::sign;
use crate::utils::crypto;
use chrono::{DateTime, Utc};
use reqwest;

const SERVICE: &str = "s3";
const REGION: &str = "us-east-2";

struct Key {
    id: String,
    secret: String,
}

pub struct S3 {
    pub bucket: String,
    pub region: String,
    key: Option<Key>,
}

#[allow(dead_code)]
impl S3 {
    pub fn new(bucket: &str) -> S3 {
        S3 {
            bucket: String::from(bucket),
            region: String::from(REGION),
            key: None,
        }
    }

    pub fn with_region(mut self, region: &str) -> S3 {
        self.region = String::from(region);
        self
    }

    pub fn with_bucket(mut self, bucket: &str) -> S3 {
        self.bucket = String::from(bucket);
        self
    }

    pub fn with_key(mut self, id: &str, secret: &str) -> S3 {
        self.key = Some(Key {
            id: String::from(id),
            secret: String::from(secret),
        });
        self
    }

    fn authorize(&self, key: &str, date: &DateTime<Utc>, hashed_body: &str) -> String {
        let signed_headers = "host;x-amz-content-sha256;x-amz-date";
        let canonical_headers = format!(
            "host:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n",
            format!("{}.s3.{}.amazonaws.com", self.bucket, self.region),
            hashed_body,
            sign::to_timestamp(&date)
        );
        let canonical_request = sign::canonical_request(
            "PUT",
            key,
            &canonical_headers,
            signed_headers,
            &hashed_body,
            &None,
        );
        let string_to_sign = sign::string_to_sign(&date, &self.region, SERVICE, &canonical_request);

        let signing_key = sign::signing_key(
            &self.key.as_ref().expect("Requires Access Key").secret,
            &date,
            &self.region,
            SERVICE,
        );
        let signature = sign::signature(&signing_key, &string_to_sign);
        sign::authorization(
            &self.key.as_ref().expect("Requires Access Key").id,
            &date,
            &self.region,
            SERVICE,
            &signed_headers,
            &signature,
        )
    }

    pub async fn put(&self, key: &str, body: Vec<u8>) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("https://{}.s3.{}.amazonaws.com{}", self.bucket, REGION, key);
        let date = Utc::now();
        let hashed_body = crypto::to_hex(&crypto::sha256(&body));
        let authorization = self.authorize(key, &date, &hashed_body);
        let client = reqwest::Client::new();
        let resp = client
            .put(url)
            .header(reqwest::header::AUTHORIZATION, authorization)
            .header("x-amz-date", sign::to_timestamp(&date))
            .header("x-amz-content-sha256", hashed_body)
            .body(body)
            .send()
            .await?;
        resp.error_for_status()
    }
}
