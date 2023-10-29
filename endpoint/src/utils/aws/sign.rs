/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#![allow(dead_code)]
use crate::utils::crypto::*;
use chrono::prelude::*;

pub fn to_timestamp(date: &DateTime<Utc>) -> String {
    format!(
        "{}{:0>2}{:0>2}T{:0>2}{:0>2}{:0>2}Z",
        date.year(),
        date.month(),
        date.day(),
        date.hour(),
        date.minute(),
        date.second(),
    )
}

pub fn to_datestamp(date: &DateTime<Utc>) -> String {
    format!("{}{:0>2}{:0>2}", date.year(), date.month(), date.day())
}

pub fn canonical_request(
    http_method: &str,
    canonical_uri: &str,
    canonical_headers: &str,
    signed_headers: &str,
    hashed_payload: &str,
    canonical_query_string: &Option<String>,
) -> String {
    let query_string: String = match canonical_query_string {
        Some(s) => String::from(s),
        None => String::new(),
    };
    [
        http_method,
        canonical_uri,
        &query_string,
        canonical_headers,
        signed_headers,
        hashed_payload,
    ]
    .join("\n")
}

pub fn string_to_sign(
    date: &DateTime<Utc>,
    region: &str,
    service: &str,
    canonical_request: &str,
) -> String {
    let scope = [
        to_datestamp(date),
        String::from(region),
        String::from(service),
        String::from("aws4_request"),
    ]
    .join("/");

    let hashed_canonical_request = sha256(&canonical_request.as_bytes().to_vec());
    format!(
        "AWS4-HMAC-SHA256\n{}\n{}\n{}",
        to_timestamp(date),
        scope,
        to_hex(&hashed_canonical_request)
    )
}

pub fn signing_key(key_secret: &str, date: &DateTime<Utc>, region: &str, service: &str) -> String {
    let k_date = hmac_sha256(
        &format!("AWS4{}", key_secret).into_bytes(),
        &to_datestamp(date).into_bytes(),
    );
    let k_region = hmac_sha256(&k_date, &region.as_bytes().to_vec());
    let k_service = hmac_sha256(&k_region, &service.as_bytes().to_vec());
    to_hex(&hmac_sha256(
        &k_service,
        &"aws4_request".as_bytes().to_vec(),
    ))
}

pub fn signature(signing_key: &str, string_to_sign: &str) -> String {
    to_hex(&hmac_sha256(
        &from_hex(signing_key),
        &string_to_sign.as_bytes().to_vec(),
    ))
}

pub fn authorization(
    key_id: &str,
    date: &DateTime<Utc>,
    region: &str,
    service: &str,
    signed_headers: &str,
    signature: &str,
) -> String {
    let credential = [key_id, &to_datestamp(date), region, service, "aws4_request"].join("/");
    [
        format!("AWS4-HMAC-SHA256 Credential={}", credential),
        format!("SignedHeaders={}", signed_headers),
        format!("Signature={}", signature),
    ]
    .join(",")
}

#[cfg(test)]
mod tests {
    use crate::utils::aws::sign::*;

    const TEST_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
    const TEST_ID: &str = "AKIAIOSFODNN7EXAMPLE";
    const TEST_REGION: &str = "us-east-1";
    const TEST_SERVICE: &str = "s3";
    const TEST_CANONICAL_REQUEST: &str = "GET\n\
            /test.txt\n\n\
            host:examplebucket.s3.amazonaws.com\n\
            range:bytes=0-9\n\
            x-amz-content-sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n\
            x-amz-date:20130524T000000Z\n\n\
            host;range;x-amz-content-sha256;x-amz-date\n\
            e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    const TEST_DATESTAMP: &str = "20130524";
    const TEST_TIMESTAMP: &str = "20130524T000000Z";
    const TEST_METHOD: &str = "GET";
    const TEST_CANONICAL_URI: &str = "/test.txt";
    const TEST_SIGNED_HEADERS: &str = "host;range;x-amz-content-sha256;x-amz-date";
    const TEST_STRING_TO_SIGN: &str = "AWS4-HMAC-SHA256\n20130524T000000Z\n20130524/us-east-1/s3/aws4_request\n7344ae5b7ee6c3e7e6b0fe0640412a37625d1fbfff95c48bbb2dc43964946972";
    const TEST_SIGNATURE: &str = "f0e8bdb87c964420e857bd35b5d6ed310bd44f0170aba48dd91039c6036bdb41";
    const TEST_AUTHORIZATION: &str = "AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request,SignedHeaders=host;range;x-amz-content-sha256;x-amz-date,Signature=f0e8bdb87c964420e857bd35b5d6ed310bd44f0170aba48dd91039c6036bdb41";

    fn test_date() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2013, 5, 24, 0, 0, 0).unwrap()
    }

    #[test]
    fn to_datestamp_success() {
        let datestamp = to_datestamp(&test_date());
        assert_eq!(TEST_DATESTAMP, datestamp);
    }

    #[test]
    fn to_timestamp_success() {
        let timestamp = to_timestamp(&test_date());
        assert_eq!(TEST_TIMESTAMP, timestamp);
    }

    #[test]
    fn canonical_request_success() {
        let hashed_payload = sha256(&"".as_bytes().to_vec());
        let request = canonical_request(
            TEST_METHOD,
            TEST_CANONICAL_URI,
            &format!(
                "host:examplebucket.s3.amazonaws.com\nrange:bytes=0-9\nx-amz-content-sha256:{}\nx-amz-date:{}\n",
                to_hex(&hashed_payload),
                &to_timestamp(&test_date())
            ),
            TEST_SIGNED_HEADERS,
            &to_hex(&hashed_payload),
            &None,
        );
        assert_eq!(TEST_CANONICAL_REQUEST, request);
    }

    #[test]
    fn string_to_sign_success() {
        let s2s = string_to_sign(
            &test_date(),
            TEST_REGION,
            TEST_SERVICE,
            TEST_CANONICAL_REQUEST,
        );
        assert_eq!(TEST_STRING_TO_SIGN, s2s);
    }

    #[test]
    fn signing_success() {
        let sign_key = signing_key(TEST_KEY, &test_date(), TEST_REGION, TEST_SERVICE);
        let signature = signature(&sign_key, TEST_STRING_TO_SIGN);
        assert_eq!(TEST_SIGNATURE, signature);
    }

    #[test]
    fn authorization_success() {
        let auth = authorization(
            TEST_ID,
            &test_date(),
            TEST_REGION,
            TEST_SERVICE,
            TEST_SIGNED_HEADERS,
            TEST_SIGNATURE,
        );
        assert_eq!(TEST_AUTHORIZATION, auth);
    }
}
