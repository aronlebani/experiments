use std::str;

use async_std::fs;
use async_std::task;
use async_std::net::{TcpListener, TcpStream};
use async_std::io::{ReadExt, WriteExt};

mod http;

use crate::http::{Request, Response, Method};

fn log_request(buffer: &[u8; 1024]) {
    let result = str::from_utf8(buffer).unwrap();
    print!("{}\n", result);
}

fn log_response(response: &Response) {
    print!("{:?}\n", response);
}

async fn handler(req: Request) -> Response {
    match req.path.as_str() {
        "/" => match req.method {
            Method::GET => Response::new()
                .status(200)
                .html(fs::read_to_string("public/hello.html").await.unwrap()),
            _ => Response::new()
                .status(405)
                .html(fs::read_to_string("public/hello.html").await.unwrap()),
        },
        "/bye" => match req.method {
            Method::GET => Response::new()
                .status(200)
                .html(fs::read_to_string("public/bye.html").await.unwrap()),
            _ => Response::new()
                .status(405)
                .html(fs::read_to_string("public/hello.html").await.unwrap()),
        },
        _ => Response::new()
            .status(404)
            .html(fs::read_to_string("public/404.html").await.unwrap()),
    }
}

async fn handle_connection(mut connection: TcpStream) {
    let mut buffer = [0; 1024];
    connection.read(&mut buffer).await.unwrap();

    log_request(&buffer);

    let req = Request::from_buffer(&buffer);

    let res = handler(req).await;

    log_response(&res);

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
