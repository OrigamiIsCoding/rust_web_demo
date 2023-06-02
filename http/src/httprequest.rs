use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => Self::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Self::V1_1,
            "HTTP/2.0" => Self::V2_0,
            _ => Self::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub message_body: String,
}

fn process_req_line(line: &str) -> (Method, Resource, Version) {
    let mut words = line.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(line: &str) -> (String, String) {
    let mut header_items = line.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path("".into());
        let mut parsed_headers = HashMap::new();
        let mut parsed_message_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
            } else {
                parsed_message_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            message_body: parsed_message_body.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let get: Method = "GET".into();
        assert_eq!(get, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let version1_1: Version = "HTTP/1.1".into();
        assert_eq!(version1_1, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let http = String::from(
            "
GET /greeting HTTP/1.1
Host: localhost
Accept: */*
User-Agent: curl/7.71.1
        ",
        );

        let request: HttpRequest = http.into();
        assert_eq!(request.method, Method::Get);
        assert_eq!(request.version, Version::V1_1);
        assert_eq!(request.resource, Resource::Path("/greeting".to_string()));
        assert_eq!(
            request.headers,
            HashMap::from([
                ("Host".into(), " localhost".into()),
                ("Accept".into(), " */*".into()),
                ("User-Agent".into(), " curl/7.71.1".into()),
            ])
        )
    }
}
