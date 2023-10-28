/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod input;

use crate::utils::avro_schema;

pub use input::ShopifyOrder as Input;

pub fn sample() -> String { String::from(include_str!("shopify_order/sample.json")) }

pub fn output() -> apache_avro::Schema { avro_schema(include_str!("shopify_order/output.avsc")) }
