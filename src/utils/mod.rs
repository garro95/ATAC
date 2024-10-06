pub mod centered_rect;
pub mod choice_popup;
pub mod colors;
pub mod cookie_table;
pub mod cookies_popup;
pub mod help_popup;
pub mod script_console;
pub mod settings_popup;
pub mod stateful_custom_table;
pub mod stateful_list;
pub mod stateful_scrollbar;
pub mod stateful_tree;
pub mod syntax_highlighting;
pub mod text_input;
pub mod text_input_selection;
pub mod validation_popup;
pub mod vim_emulation;


use crate::request::auth::Auth;
use crate::request::auth::Auth::*;


pub fn next_auth(auth: &Auth) -> Auth {
    match auth {
        NoAuth => BasicAuth(String::new(), String::new()),
        BasicAuth(_, _) => BearerToken(String::new()),
        BearerToken(_) => NoAuth,
    }
}

use crate::request::body::ContentType;
use crate::request::body::ContentType::*;
pub fn next_content_type(content_type: &ContentType) -> ContentType {
    match content_type {
        NoBody => Multipart(Vec::new()),
        Multipart(_) => Form(Vec::new()),
        Form(_) => File(String::new()),
        File(_) => Raw(String::new()),
        Raw(body) => Json(body.to_string()),
        Json(body) => Xml(body.to_string()),
        Xml(body) => Html(body.to_string()),
        Html(body) => Javascript(body.to_string()),
        Javascript(_) => NoBody,
    }
}

use regex::Regex;
/// Iter through the headers and tries to catch a file format like `application/<file_format>`
pub fn find_file_format_in_content_type(headers: &[(String, String)]) -> Option<String> {
    if let Some((_, content_type)) = headers.iter().find(|(header, _)| *header == "content-type") {
        // Regex that likely catches the file format
        let regex = Regex::new(r"\w+/(?<file_format>\w+)").unwrap();

        return regex
            .captures(content_type)
            .map(|capture| capture["file_format"].to_string());
    } else {
        None
    }
}
