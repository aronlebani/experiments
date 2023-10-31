use std::str;

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

    fn to_str(&self) -> String {
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
    fn new(from: &str) -> Result<Self, &str> {
        match from {
            "HEAD" => Ok(Method::HEAD),
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            _ => Err("Invalid http method"),
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
    fn new<'a>(code: u16) -> Result<Self, &'a str> {
        match code {
            200 => Ok(Self::Ok),
            303 => Ok(Self::SeeOther),
            400 => Ok(Self::BadRequest),
            401 => Ok(Self::Unauthorized),
            403 => Ok(Self::Forbidden),
            404 => Ok(Self::NotFound),
            405 => Ok(Self::NotAllowed),
            500 => Ok(Self::InternalServerError),
            _ => Err("Invalid code"),
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
    pub fn from_buffer(from: &[u8; 1024]) -> Self {
        Self::parse(from)
    }

    fn parse(buffer: &[u8; 1024]) -> Request {
        let text = str::from_utf8(buffer).unwrap().trim_end_matches("\0");
        let mut parts = text.split("\r\n");

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

        Header {
            key: key.to_string(),
            value: value.to_string(),
        }
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
    pub fn new() -> Self {
        Response {
            scheme: "HTTP".to_string(),
            version: "1.1".to_string(),
            status: Status::new(200).unwrap(),
            headers: Vec::new(),
            content: String::new(),
        }
    }

    pub fn status(self, status: u16) -> Self {
        Response {
            status: Status::new(status).unwrap(),
            ..self
        }
    }

    pub fn headers(self, headers: Vec<Header>) -> Self {
        Response {
            headers: self
                .headers
                .iter()
                .cloned()
                .chain(headers.iter().cloned())
                .collect(),
            ..self
        }
    }

    pub fn html(self, content: String) -> Self {
        Response {
            content,
            headers: vec![Header {
                key: "Content-Type".to_string(),
                value: "text/html".to_string(),
            }],
            ..self
        }
    }

    pub fn json(self, content: String) -> Self {
        Response {
            content,
            headers: vec![Header {
                key: "Content-Type".to_string(),
                value: "application/json".to_string(),
            }],
            ..self
        }
    }

    pub fn to_str(self) -> String {
        let length = self.content.len();
        let length_str = length.to_string();
        let c_l_header = Header::new("Content-Length", &length_str);
        let c_t_header = Header::new("Content-type", &self.content_type);
        // TODO - generalise headers

        format!(
            "{}/{} {} {}\r\n{}\r\n{}\r\n\r\n{}",
            self.scheme,
            self.version,
            self.status.code(),
            self.status.message(),
            c_l_header.to_str(),
            c_t_header.to_str(),
            self.content
        )
    }
}
