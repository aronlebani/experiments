use std::fs;
use std::str;
use std::time::Duration;

use async_std::task;
use async_std::net::{TcpListener, TcpStream};
use async_std::io::{ReadExt, WriteExt};

fn log_request(buffer: &[u8; 1024]) {
    let result = str::from_utf8(buffer).unwrap();
    print!("{}\n", result);
}

#[derive(Debug)]
struct Header<'a> {
    key: &'a str,
    value: &'a str,
}

enum Method {
    HEAD,
    GET,
    POST,
    PUT,
    DELETE,
}

impl Method {
    fn from_str(from: &str) -> Self {
        match from {
            "HEAD" => Method::HEAD,
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
        }
    }
}

struct Request<'a> {
    method: Method,
    path: &'a str,
    version: &'a str,
    headers: Vec<Header<'a>>,
    body: String,
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

fn parse_start_line(line: &str) -> (Method, &str, &str) {
    let mut parts = line.split(" ");

    let method = parts.next();
    let path = parts.next();
    let protocol = parts.next();

    (Method::from_str(method), path, protocol)
}

fn parse(buffer: &[u8; 1024]) -> &str {
    let text = str::from_utf8(buffer).unwrap().trim_end_matches("\0");
    let mut parts = text.split("\r\n");

    let start_line = parts.next().unwrap();

    let headers: Vec<Header> = parts
        .clone()
        .take_while(|x| x.to_owned() != "")
        .map(|x| parse_header(x))
        .collect();

    let body: String = parts
        .clone()
        .skip_while(|x| x.to_owned() != "")
        .collect();

    (method, path, version) = parse_start_line(start_line);

    Request {
        method, 
    }

    println!("---start-{:?}", start_line);
    println!("---headers-{:?}", headers);
    println!("---body-{:?}", body);

    "hello"
}

async fn handle_connection(mut connection: TcpStream) {
    let mut buffer = [0; 1024];
    connection.read(&mut buffer).await.unwrap();

    log_request(&buffer);

    parse(&buffer);

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = match buffer {
        b if b.starts_with(get) => ("HTTP/1.1 200 OK", "public/hello.html"),
        b if b.starts_with(sleep) => {
            task::sleep(Duration::from_secs(5)).await;
            ("HTTP/1.1 200 OK", "public/hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "public/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\nContent-Type: text/html\r\n\r\n{contents}");

    connection.write(response.as_bytes()).await.unwrap();
    connection.flush().await.unwrap();
}

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3333").await.unwrap();

    loop {
        let (connection, _) = listener.accept().await.unwrap();
        task::spawn(async move {
            handle_connection(connection).await;
        });
    };
}
