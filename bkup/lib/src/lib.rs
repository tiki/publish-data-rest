/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use apache_avro;

pub mod api;
pub mod aws;
pub mod jwt;
pub mod misc;

pub trait Ingest {
    fn id(&self) -> String;
    fn schema() -> apache_avro::Schema;
}
