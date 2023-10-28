/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error, fmt};
use worker::Response;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrorBuilder {
    #[serde(skip_serializing)]
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
}

#[allow(dead_code)]
impl ErrorBuilder {
    pub fn new() -> ErrorBuilder {
        ErrorBuilder {
            status: 500,
            id: None,
            message: None,
            detail: None,
            help: None,
            properties: HashMap::new(),
        }
    }

    pub fn id(mut self, id: &str) -> ErrorBuilder {
        self.id = Some(String::from(id));
        self
    }

    pub fn message(mut self, message: &str) -> ErrorBuilder {
        self.message = Some(String::from(message));
        self
    }

    pub fn detail(mut self, detail: &str) -> ErrorBuilder {
        self.detail = Some(String::from(detail));
        self
    }

    pub fn help(mut self, help: &str) -> ErrorBuilder {
        self.help = Some(String::from(help));
        self
    }

    pub fn properties(mut self, properties: HashMap<String, String>) -> ErrorBuilder {
        self.properties = properties;
        self
    }

    pub fn status(mut self, status: u16) -> ErrorBuilder {
        self.status = status;
        self
    }

    pub fn to_response(&self) -> Response {
        Response::from_json(self)
            .expect("Serialization failure")
            .with_status(self.status)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Serialization failure")
    }

    pub fn to_error(&self) -> Box<ErrorBuilder> {
        Box::new(self.clone())
    }
}

impl error::Error for ErrorBuilder {}

impl fmt::Display for ErrorBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ApiError {{ status: {:?}, id: {:?}, message: {:?}, detail: {:?}, help: {:?}, properties: {:?} }}",
            self.status,
            self.id,
            self.message,
            self.detail,
            self.help,
            self.properties
        )
    }
}
