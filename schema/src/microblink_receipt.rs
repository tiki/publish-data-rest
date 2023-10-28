/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod input;
use crate::utils::avro_schema;

pub use input::MicroblinkReceipt as Input;

pub fn sample() -> String { String::from(include_str!("microblink_receipt/sample.json")) }

pub fn output() -> apache_avro::Schema { avro_schema(include_str!("microblink_receipt/output.avsc")) }
