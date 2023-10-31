#[derive(Debug)]
struct Header<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> Header<'a> {
    fn new(key: &'a str, value: &'a str) -> Self {
        Header {
            key,
            value,
        }
    }

    fn to_str(&self) -> String {
        format!("{}: {}", self.key, self.value)
    }
}

#[derive(Debug)]
enum Method {
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
enum Status {
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

    fn code(&self) -> Result<u16, &str> {
        match self {
            Status::Ok => Ok(200),
            Status::SeeOther => Ok(303),
            Status::BadRequest => Ok(400),
            Status::Unauthorized => Ok(401),
            Status::Forbidden => Ok(403),
            Status::NotFound => Ok(404),
            Status::NotAllowed => Ok(405),
            Status::InternalServerError => Ok(500),
            _ => Err("Invalid code"),
        }
    }

    fn message(&self) -> Result<&str, &str> {
        match self {
            Status::Ok => Ok("OK"),
            Status::SeeOther => Ok("SEE OTHER"),
            Status::BadRequest => Ok("BAD REQUEST"),
            Status::Unauthorized => Ok("UNAUTHORIZED"),
            Status::Forbidden => Ok("FORBIDDEN"),
            Status::NotFound => Ok("NOT FOUND"),
            Status::NotAllowed => Ok("NOT ALLOWED"),
            Status::InternalServerError => Ok("INTERNAL SERVER ERROR"),
            _ => Err("Invalid code"),
        }
    }
}

#[derive(Debug)]
struct Request<'a> {
    method: Method,
    path: &'a str,
    scheme: &'a str,
    version: &'a str,
    headers: Vec<Header<'a>>,
    body: String,
}

impl<'a> Request<'a> {
    fn from_buffer(from: &'a [u8; 1024]) -> Self {
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

        let body: String = parts
            .clone()
            .skip_while(|x| x.to_owned() != "")
            .collect();

        Request {
            method, 
            path,
            scheme,
            version,
            headers,
            body,
        }
    }

    fn parse_header(line: &str) -> Header {
        let mut parts = line.split(": ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();

        Header {
            key,
            value,
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

struct Response<'a> {
    scheme: &'a str,
    version: &'a str,
    status: Status,
    headers: Vec<Header<'a>>,
    content_type: &'a str,
    content: String,
}

impl<'a> Response<'a> {
    fn new() -> Self {
        Response {
            scheme: "HTTP",
            version: "1.1",
            status: Status::new(200).unwrap(),
            headers: Vec::new(),
            content_type: "",
            content: String::new(),
        }
    }

    fn status(self, status: u16) -> Self {
        Response {
            status: Status::new(status).unwrap(),
            ..self
        }
    }

    fn html(self, content: String) -> Self {
        Response {
            content,
            content_type: "text/html",
            ..self
        }
    }

    fn json(self, content: String) -> Self {
        Response {
            content,
            content_type: "application/json",
            ..self
        }
    }

    fn to_str(self) -> String {
        let length = self.content.len();
        let length_str = length.to_string();
        let c_l_header = Header::new("Content-Length", &length_str);
        let c_t_header = Header::new("Content-type", self.content_type);

        format!(
            "{}/{} {} {}\r\n{}\r\n{}\r\n\r\n{}",
            self.scheme,
            self.version,
            self.status.code().unwrap(),
            self.status.message().unwrap(),
            c_l_header.to_str(),
            c_t_header.to_str(),
            self.content
        )
    }
}
