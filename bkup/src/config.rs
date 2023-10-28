/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod schema;

use config::Config as rsConfig;
pub use schema::Schema;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub schema: Vec<Schema>,
}

impl Config {
    pub fn load() -> Config {
        let settings = rsConfig::builder()
            .add_source(config::File::with_name("config.toml"))
            .build()
            .expect("Failed to load config");
        settings
            .try_deserialize::<Config>()
            .expect("Failed to deserialize config")
    }
}
