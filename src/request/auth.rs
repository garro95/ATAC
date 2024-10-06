use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Clone, Default, Debug, Display, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Auth {
    #[default]
    #[strum(to_string = "No Auth")]
    NoAuth,
    #[strum(to_string = "Basic")]
    BasicAuth(String, String),
    #[strum(to_string = "Bearer")]
    BearerToken(String),
}
