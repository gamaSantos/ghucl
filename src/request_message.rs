use std::collections::HashMap;

use serde::Deserialize;

use crate::request_error::RequestError;

pub enum HttpVerb {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    PATCH,
}

pub struct RequestMessage {
    pub method: HttpVerb,
    pub url: String,
    pub body: String,
    pub headers: HashMap<String, String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct RequestMessageBuilder {
    method: Option<String>,
    host: Option<String>,
    path: Option<String>,
    body: Option<String>,
    headers: Option<HashMap<String, String>>,
}

impl RequestMessage {
    pub fn from_text(file_text: &str) -> Result<RequestMessageBuilder, RequestError> {
        let parsed = toml::from_str::<RequestMessageBuilder>(&file_text);

        return parsed.map_err(|e| -> RequestError {
            return RequestError::TomlParserError {
                message: e.message().to_string(),
            };
        });
    }
}

impl RequestMessageBuilder {
    pub fn merge_with(&self, new_message: &RequestMessageBuilder) -> RequestMessageBuilder {
        fn increment_header(
            opt_headers: Option<HashMap<String, String>>,
            mut target_headers: HashMap<String, String>,
        ) -> HashMap<String, String> {
            if let Some(cur_headers) = opt_headers {
                for (k, v) in cur_headers {
                    target_headers.insert(k, v);
                }
            }
            return target_headers;
        }

        let method = new_message.method.as_ref().or(self.method.as_ref()).and_then(|v| Some(v.clone()));
        let host = new_message.host.as_ref().or(self.host.as_ref()).and_then(|v| Some(v.clone()));
        let path = new_message.path.as_ref().or(self.path.as_ref()).and_then(|v| Some(v.clone()));
        let body = new_message.body.as_ref().or(self.body.as_ref()).and_then(|v| Some(v.clone()));

        let copied_values = increment_header(self.headers.clone(), HashMap::new());
        let incremented = increment_header(new_message.headers.clone(), copied_values);

        let headers = Some(incremented);
        return RequestMessageBuilder {
            method,
            host,
            path,
            body,
            headers,
        };
    }

    pub fn to_message(&self) -> Result<RequestMessage, RequestError> {
        let host = match &self.host {
            Some(x) => x,
            None => {
                return Err(RequestError::BuildError {
                    property_name: "host".to_owned(),
                })
            }
        };
        let path = match &self.path {
            Some(x) => x,
            None => {
                return Err(RequestError::BuildError {
                    property_name: "path".to_owned(),
                })
            }
        };

        let method_candidate = match &self.method {
            Some(x) => x,
            None => {
                return Err(RequestError::BuildError {
                    property_name: "method".to_owned(),
                })
            }
        };
        let body = match &self.body {
            Some(s) => s.as_str(),
            None => "",
        };
        
        return Ok(RequestMessage {
            method: parse_method(method_candidate),
            url: String::from(host) + path,
            body: body.to_string(),
            headers: self.headers.clone().unwrap_or(HashMap::new()),
        });

        fn parse_method(candidate: &str) -> HttpVerb {
            match candidate.to_uppercase().as_str() {
                "GET" => HttpVerb::GET,
                "HEAD" => HttpVerb::HEAD,
                "POST" => HttpVerb::POST,
                "PUT" => HttpVerb::PUT,
                "DELETE" => HttpVerb::DELETE,
                "OPTIONS" => HttpVerb::OPTIONS,
                "PATCH" => HttpVerb::PATCH,
                _ => HttpVerb::HEAD,
            }
        }
    }
}
