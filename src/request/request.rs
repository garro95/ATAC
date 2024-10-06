use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::request::auth::Auth;
use crate::request::body::ContentType;
use crate::request::method::Method;
use crate::request::response::RequestResponse;
use crate::request::scripts::RequestScripts;
use crate::request::settings::RequestSettings;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub url: String,
    pub method: Method,
    pub params: Vec<KeyValue>,
    pub headers: Vec<KeyValue>,
    pub body: ContentType,
    pub auth: Auth,
    pub scripts: RequestScripts,
    pub settings: RequestSettings,

    #[serde(skip)]
    pub response: RequestResponse,

    #[serde(skip)]
    pub is_pending: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KeyValue {
    pub data: (String, String),
    pub enabled: bool,
}

lazy_static! {
    pub static ref DEFAULT_HEADERS: Vec<KeyValue> = vec![
        KeyValue {
            enabled: true,
            data: (String::from("cache-control"), String::from("no-cache")),
        },
        KeyValue {
            enabled: true,
            data: (
                String::from("user-agent"),
                format!("ATAC/v{}", env!("CARGO_PKG_VERSION"))
            ),
        },
        KeyValue {
            enabled: true,
            data: (String::from("accept"), String::from("*/*")),
        },
        KeyValue {
            enabled: true,
            data: (
                String::from("accept-encoding"),
                String::from("gzip, deflate, br")
            ),
        },
        KeyValue {
            enabled: true,
            data: (String::from("connection"), String::from("keep-alive")),
        },
    ];
}

impl Request {
    pub fn url_with_params_to_string(&self) -> String {
        let mut base_url = self.url.to_string();

        if !self.params.is_empty() {
            let mut enabled_params: Vec<String> = vec![];

            for (index, param) in self.params.iter().enumerate() {
                if !param.enabled {
                    continue;
                }

                let mut new_param = format!("{}={}", param.data.0, param.data.1);

                if index != self.params.len() - 1 {
                    new_param += "&";
                }

                enabled_params.push(new_param);
            }

            if !enabled_params.is_empty() {
                base_url += "?";

                for enabled_param in enabled_params {
                    base_url += &enabled_param;
                }
            }
        }

        base_url
    }

    pub fn find_and_delete_header(&mut self, input_header: &str) {
        let index = self
            .headers
            .iter()
            .position(|header| header.data.0 == input_header);

        match index {
            None => {}
            Some(index) => {
                self.headers.remove(index);
            }
        }
    }

    pub fn modify_or_create_header(&mut self, input_header: &str, value: &str) {
        let mut was_header_found = false;

        for header in &mut self.headers {
            if header.data.0.to_lowercase() == input_header.to_lowercase() {
                header.data.1 = value.to_string();
                was_header_found = true;
            }
        }

        if !was_header_found {
            self.headers.push(KeyValue {
                enabled: true,
                data: (input_header.to_string(), value.to_string()),
            })
        }
    }
}
