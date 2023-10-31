use std::fs;
use std::str;

use async_std::task;
use async_std::net::{TcpListener, TcpStream};
use async_std::io::{ReadExt, WriteExt};

mod http;

use crate::http::{Request, Response, Method};

fn log_request(buffer: &[u8; 1024]) {
    let result = str::from_utf8(buffer).unwrap();
    print!("{}\n", result);
}

async fn handler(req: Request) -> Response {
    match req.path {
        "/" => match req.method {
            Method::GET => Response::new().status(200).html(fs::read_to_string("public/hello.html").unwrap()),
            _ => Response::new().status(405).html(fs::read_to_string("public/hello.html").unwrap()),
        },
        "/bye" => match req.method {
            Method::GET => Response::new().status(200).html(fs::read_to_string("public/bye.html").unwrap()),
            _ => Response::new().status(405).html(fs::read_to_string("public/hello.html").unwrap()),
        },
        _ => Response::new().status(404).html(fs::read_to_string("public/404.html").unwrap()),
    }
}

async fn handle_connection(mut connection: TcpStream) {
    let mut buffer = [0; 1024];
    connection.read(&mut buffer).await.unwrap();

    log_request(&buffer);

    let req = Request::from_buffer(&buffer);

    // let (status_line, filename) = match req.path {
    //     "/" => match req.method {
    //         Method::GET => ("HTTP/1.1 200 OK", "public/hello.html"),
    //         _ => ("HTTP/1.1 405 NOT ALLOWED", "public/405.html"),
    //     },
    //     "/bye" => match req.method {
    //         Method::GET => ("HTTP/1.1 200 OK", "public/bye.html"),
    //         _ => ("HTTP/1.1 405 NOT ALLOWED", "public/405.html"),
    //     },
    //     _ => ("HTTP/1.1 404 NOT FOUND", "public/404.html"),
    // };
    //
    // let contents = fs::read_to_string(filename).unwrap();
    // let length = contents.len();
    // let response = format!("{status_line}\r\nContent-Length: {length}\nContent-Type: text/html\r\n\r\n{contents}");

    let res = handler(req).await;

    connection.write(res.to_str().as_bytes()).await.unwrap();
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
