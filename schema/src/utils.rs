/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[allow(unused)]
pub fn avro_schema(s: &str) -> apache_avro::Schema {
    let res = apache_avro::Schema::parse_str(s);
    match res {
        Ok(schema) => schema,
        Err(_) => panic!("Invalid Avro Schema")
    }
}
