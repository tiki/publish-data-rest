/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use worker::{Request, Response};
use crate::utils::api::{ErrorBuilder};

pub fn method(req: &Request) -> Result<(), Response> {
    if let Err(res) = cors(&req) {
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

pub fn cors(req: &Request) -> Result<(), Response> {
    if req.method().eq(&worker::Method::Options) {
        Err(Response::empty().unwrap())
    } else {
        Ok(())
    }
}
