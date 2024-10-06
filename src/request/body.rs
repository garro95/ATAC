use serde::{Deserialize, Serialize};
use strum::Display;

use crate::request::body::ContentType::*;
use crate::request::request::KeyValue;

#[derive(Default, Debug, Clone, Display, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    #[default]
    #[strum(to_string = "No Body")]
    NoBody,
    #[strum(to_string = "File")]
    File(String),
    #[strum(to_string = "Multipart")]
    Multipart(Vec<KeyValue>),
    #[strum(to_string = "Form")]
    Form(Vec<KeyValue>),
    #[strum(to_string = "Text")]
    Raw(String),
    #[strum(to_string = "JSON")]
    Json(String),
    #[strum(to_string = "XML")]
    Xml(String),
    #[strum(to_string = "HTML")]
    Html(String),
    #[strum(to_string = "Javascript")]
    Javascript(String),
}

impl ContentType {
    pub fn to_content_type(&self) -> String {
        match &self {
            NoBody => String::new(),
            Multipart(_) => String::from("multipart/form-data"),
            Form(_) => String::from("application/x-www-form-urlencoded"),
            Raw(_) => String::from("text/plain"),
            File(_) => String::from("application/octet-stream"),
            Json(_) | Xml(_) | Html(_) | Javascript(_) => {
                format!("application/{}", self.to_string().to_lowercase())
            }
        }
    }

    pub fn from_content_type(content_type: &str, body: String) -> ContentType {
        match content_type {
            //"multipart/form-data" => Multipart(body),
            //"application/x-www-form-urlencoded" => Form(body),
            "application/octet-stream" => File(body),
            "text/plain" => Raw(body),
            "application/json" => Json(body),
            "application/xml" => Json(body),
            "application/html" => Json(body),
            "application/javascript" => Json(body),
            _ => NoBody,
        }
    }

    pub fn get_form(&self) -> Option<&Vec<KeyValue>> {
        match self {
            Multipart(form) | Form(form) => Some(form),
            _ => None,
        }
    }

    pub fn get_form_mut(&mut self) -> Option<&mut Vec<KeyValue>> {
        match self {
            Multipart(form) | Form(form) => Some(form),
            _ => None,
        }
    }
}
