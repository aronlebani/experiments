use std::str;

// TODO
// - Docs
// - Error struct

#[derive(Debug, Clone)]
pub struct Header {
    key: String,
    value: String,
}

impl Header {
    fn new(key: &str, value: &str) -> Self {
        Header {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl ToString for Header {
    fn to_string(&self) -> String {
        format!("{}: {}", self.key, self.value)
    }
}

#[derive(Debug)]
pub enum Method {
    HEAD,
    GET,
    POST,
    PUT,
    DELETE,
}

impl Method {
    fn new(from: &str) -> Result<Self, String> {
        match from {
            "HEAD" => Ok(Method::HEAD),
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            _ => Err("Invalid or unsupported http method".to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Status {
    Ok,
    SeeOther,
    NotFound,
    InternalServerError,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotAllowed,
}

impl Status {
    fn from_code(code: u16) -> Result<Self, String> {
        match code {
            200 => Ok(Self::Ok),
            303 => Ok(Self::SeeOther),
            400 => Ok(Self::BadRequest),
            401 => Ok(Self::Unauthorized),
            403 => Ok(Self::Forbidden),
            404 => Ok(Self::NotFound),
            405 => Ok(Self::NotAllowed),
            500 => Ok(Self::InternalServerError),
            _ => Err("Invalid code".to_string()),
        }
    }

    fn code(&self) -> u16 {
        match self {
            Status::Ok => 200,
            Status::SeeOther => 303,
            Status::BadRequest => 400,
            Status::Unauthorized => 401,
            Status::Forbidden => 403,
            Status::NotFound => 404,
            Status::NotAllowed => 405,
            Status::InternalServerError => 500,
        }
    }

    fn message(&self) -> &str {
        match self {
            Status::Ok => "OK",
            Status::SeeOther => "SEE OTHER",
            Status::BadRequest => "BAD REQUEST",
            Status::Unauthorized => "UNAUTHORIZED",
            Status::Forbidden => "FORBIDDEN",
            Status::NotFound => "NOT FOUND",
            Status::NotAllowed => "NOT ALLOWED",
            Status::InternalServerError => "INTERNAL SERVER ERROR",
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        format!("{} {}", self.code(), self.message())
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub scheme: String,
    pub version: String,
    pub headers: Vec<Header>,
    pub body: String,
}

impl Request {
    pub fn from_string(buffer: &str) -> Self {
        Self::parse(buffer)
    }

    fn parse(buffer: &str) -> Request {
        let mut parts = buffer.split("\r\n");

        let start_line = parts.next().unwrap();

        let (method, path, scheme, version) = Self::parse_start_line(start_line);

        let headers: Vec<Header> = parts
            .clone()
            .take_while(|x| x.to_owned() != "")
            .map(|x| Self::parse_header(x))
            .collect();

        let body: String = parts.clone().skip_while(|x| x.to_owned() != "").collect();

        Request {
            method,
            path: path.to_string(),
            scheme: scheme.to_string(),
            version: version.to_string(),
            headers,
            body,
        }
    }

    fn parse_header(line: &str) -> Header {
        let mut parts = line.split(": ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();

        Header::new(key, value)
    }

    fn parse_protocol(line: &str) -> (&str, &str) {
        let mut parts = line.split("/");

        let scheme = parts.next().unwrap();
        let version = parts.next().unwrap();

        (scheme, version)
    }

    fn parse_start_line(line: &str) -> (Method, &str, &str, &str) {
        let mut parts = line.split(" ");

        let method = parts.next().unwrap();
        let path = parts.next().unwrap();
        let protocol = parts.next().unwrap();

        let (scheme, version) = Self::parse_protocol(protocol);

        (Method::new(method).unwrap(), path, scheme, version)
    }
}

#[derive(Debug)]
pub struct Response {
    scheme: String,
    version: String,
    status: Status,
    headers: Vec<Header>,
    content: String,
}

impl Response {
    pub fn empty() -> Self {
        Response {
            scheme: "HTTP".to_string(),
            version: "1.1".to_string(),
            status: Status::Ok,
            headers: Vec::new(),
            content: String::new(),
        }
    }

    pub fn html(content: String) -> Self {
        let content_length = content.len();

        Response {
            scheme: "HTTP".to_string(),
            version: "1.1".to_string(),
            status: Status::Ok,
            headers: Vec::new(),
            content,
        }
        .header(Header::new("Content-Type", "text/html"))
        .header(Header::new("Content-Length", &content_length.to_string()))
    }

    pub fn json(content: String) -> Self {
        let content_length = content.len();

        Response {
            scheme: "HTTP".to_string(),
            version: "1.1".to_string(),
            status: Status::Ok,
            headers: Vec::new(),
            content,
        }
        .header(Header::new("Content-Type", "application/json"))
        .header(Header::new("Content-Length", &content_length.to_string()))
    }

    pub fn status(self, status: u16) -> Self {
        Response {
            status: Status::from_code(status).unwrap(),
            ..self
        }
    }

    pub fn header(self, header: Header) -> Self {
        let mut headers = self.headers;
        headers.push(header);

        Response { headers, ..self }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let headers = self
            .headers
            .iter()
            .fold(String::new(), |a, b| a + &b.to_string() + "\r\n");

        format!(
            "{}/{} {}\r\n{}\r\n{}",
            self.scheme,
            self.version,
            self.status.to_string(),
            headers,
            self.content
        )
    }
}
