/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use crate::utils::api::ErrorBuilder;
use worker::{Response, Result};

pub fn apply(res: Response) -> Result<Response> {
    let catch = || -> Result<Response> {
        let mut headers = res.headers().clone();
        headers.set("Access-Control-Allow-Origin", "*")?;
        headers.set("Access-Control-Allow-Methods", "POST")?;
        headers.set("Access-Control-Allow-Credentials", "true")?;
        headers.set(
            "Access-Control-Allow-Headers",
            "Authorization, Content-Type, Accept",
        )?;
        Ok(res.with_headers(headers))
    };
    match catch() {
        Ok(res) => Ok(res),
        Err(error) => Ok(ErrorBuilder::new()
            .status(500)
            .message(&error.to_string())
            .to_response()),
    }
}
