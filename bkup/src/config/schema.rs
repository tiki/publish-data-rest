/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Schema {
    pub clazz: String,
    pub in_avro: String,
    pub in_rust: String,
    pub in_json: String,
    pub out: String,
}
