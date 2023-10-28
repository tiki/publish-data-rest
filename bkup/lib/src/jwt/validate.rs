/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use crate::api::ErrorBuilder;
use crate::jwt::{Config, Payload};
use chrono::Duration;
use jwt_compact::{alg::Es256, jwk::JsonWebKey, prelude::*, Algorithm};
use std::collections::HashMap;
use std::error::Error;
use worker::Request;

type PublicKey = <Es256 as Algorithm>::VerifyingKey;

pub fn validate(
    req: &Request,
    key: &str,
    config: Config,
) -> Result<Token<Payload>, Box<dyn Error>> {
    decode(req, key, config).map_err(|error| -> Box<dyn Error> {
        match error.downcast_ref::<ErrorBuilder>() {
            Some(e) => e.to_error(),
            None => ErrorBuilder::new()
                .status(403)
                .message("Invalid JWT")
                .detail(&error.to_string())
                .to_error(),
        }
    })
}

fn decode(req: &Request, key: &str, config: Config) -> Result<Token<Payload>, Box<dyn Error>> {
    let token = req.headers().get("Authorization")?;
    if token.is_none() {
        return Err(ErrorBuilder::new()
            .status(403)
            .help("Missing token in header")
            .to_error());
    }
    let token = token.unwrap().replace("Bearer ", "");
    let jwk: JsonWebKey = serde_json::from_str(key)?;
    let public_key = PublicKey::try_from(&jwk)?;
    let token = UntrustedToken::new(&token)?;
    let token: Token<Payload> = Es256.validator(&public_key).validate(&token)?;
    match validate_claims(&token, config) {
        Ok(_) => Ok(token),
        Err(error) => Err(error),
    }
}

fn validate_claims(token: &Token<Payload>, config: Config) -> Result<(), Box<dyn Error>> {
    let time_options = TimeOptions::from_leeway(Duration::seconds(config.clock_skew));
    token.claims().validate_expiration(&time_options)?;

    if token.claims().not_before.is_some() {
        token.claims().validate_maturity(&time_options)?;
    }

    if config.audience.is_some()
        && !token
            .claims()
            .custom
            .audience
            .contains(&config.audience.unwrap())
    {
        return Err(ErrorBuilder::new()
            .status(403)
            .message("Invalid JWT")
            .detail("Invalid AUD claim")
            .to_error());
    }

    if config.issuer.is_some() && token.claims().custom.issuer != config.issuer.unwrap() {
        return Err(ErrorBuilder::new()
            .status(403)
            .message("Invalid JWT")
            .detail("Invalid ISS claim")
            .properties(HashMap::from([(
                String::from("iss"),
                token.claims().custom.issuer.clone(),
            )]))
            .to_error());
    }
    Ok(())
}
