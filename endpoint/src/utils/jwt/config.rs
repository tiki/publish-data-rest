/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub struct Config {
    pub clock_skew: i64,
    pub issuer: Option<String>,
    pub audience: Option<String>,
}
