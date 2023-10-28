/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod utils;
mod handler;

use worker::{event, Context, Env, Request, Response, Result};
use utils::{api::cors, api::ErrorBuilder};

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let res: Result<Response> = match handler::handle(req, env, ctx).await {
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
