/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod config;

use crate::config::Config;
use mustache::{Data, MapBuilder};
use std::fs;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const ENDPOINT_DIR: &str = "../endpoint";
const TEMPLATE_DIR: &str = "template";

fn main() {
    let _ = fs::create_dir_all(ENDPOINT_DIR);
    let config = Config::load();

    for schema in config.schema {
        let output_dir = format!("{}/{}", ENDPOINT_DIR, schema.out);
        fs::create_dir_all(format!("{}/src", output_dir))
            .expect(&format!("Failed to create dir {}", output_dir));
        let avro = fs::read_to_string(format!("schema/{}", &schema.in_avro))
            .expect(&format!("Failed to load {}", &schema.in_avro));
        let mut json = fs::read_to_string(format!("schema/{}", &schema.in_json))
            .expect(&format!("Failed to load {}", &schema.in_json));
        json.retain(|c| !c.is_whitespace());

        let properties = MapBuilder::new()
            .insert_str("name", &schema.out.replace("_", "-"))
            .insert_str("version", VERSION)
            .insert_str("filename", &schema.out)
            .insert_str("avro", avro)
            .insert_str("json", json.escape_debug().to_string())
            .insert_str("clazz", schema.clazz)
            .build();

        resolve_template(
            &format!("{}/package.mustache", TEMPLATE_DIR),
            &properties,
            &format!("{}/package.json", output_dir),
        );

        resolve_template(
            &format!("{}/cargo.mustache", TEMPLATE_DIR),
            &properties,
            &format!("{}/Cargo.toml", output_dir),
        );

        resolve_template(
            &format!("{}/wrangler.mustache", TEMPLATE_DIR),
            &properties,
            &format!("{}/wrangler.toml", output_dir),
        );

        resolve_template(
            &format!("{}/lib.mustache", TEMPLATE_DIR),
            &properties,
            &format!("{}/src/lib.rs", output_dir),
        );

        resolve_template(
            &format!("schema/{}", &schema.in_rust),
            &properties,
            &format!(
                "{}/src/{}",
                output_dir,
                schema.in_rust.split("/").last().unwrap()
            ),
        );
    }
}

fn resolve_template(fin: &str, properties: &Data, fout: &str) {
    println!(
        "---Resolving---\n\
         input: {}\n\
         properties: {:#?}\n\
         output: {}",
        fin, properties, fout
    );

    let res = || -> Result<(), Box<dyn std::error::Error>> {
        let mut output = fs::File::create(fout)?;
        let template = fs::read_to_string(fin)?;
        let template = mustache::compile_str(&template)?;
        template.render_data(&mut output, properties)?;
        Ok(())
    };
    match res() {
        Ok(_) => {
            println!("---Complete---\n");
        }
        Err(e) => {
            println!("---Failed---\n{}\n", e);
        }
    }
}
