use std::{
    collections::HashMap,
    io::{Result, Write},
};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: Default::default(),
            body: Default::default(),
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(resp: HttpResponse<'a>) -> Self {
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &resp.version(),
            &resp.status_code(),
            &resp.status_text(),
            &resp.headers(),
            &resp.body.clone().unwrap().len(),
            &resp.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();

        if status_code != "200" {
            response.status_code = status_code.into();
        }

        response.headers = match headers {
            Some(_) => headers,
            None => Some(HashMap::from([("Content-Type", "text/html")])),
        };

        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Interval Server Error",
            _ => "Not Found",
        }
        .into();

        response.body = body;

        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let response = self.clone();
        let response = String::from(response);

        let _ = write!(write_stream, "{}", response);

        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let mut header_string = String::from("");
        for (k, v) in self.headers.clone().unwrap().iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: Some(HashMap::from([("Content-Type", "text/html")])),
            body: Some("xxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new("404", None, Some("xxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: Some(HashMap::from([("Content-Type", "text/html")])),
            body: Some("xxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: Some(HashMap::from([("Content-Type", "text/html")])),
            body: Some("xxx".into()),
        };
        let response_expected: String = response_expected.into();

        let actual_string = "\
HTTP/1.1 404 Not Found\r
Content-Type:text/html\r
Content-Length: 3\r
\r
xxx"
        .to_string();

        assert_eq!(actual_string, response_expected);
    }
}
