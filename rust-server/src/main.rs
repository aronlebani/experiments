use std::str;

use async_std::fs;
use async_std::io::{ReadExt, WriteExt};
use async_std::net::{TcpListener, TcpStream};
use async_std::task;

use rust_server::{Method, Request, Response};

async fn handler(req: Request) -> Response {
    match req.path.as_str() {
        "/" => match req.method {
            Method::GET => {
                let html = fs::read_to_string("public/hello.html").await.unwrap();
                Response::html(html).status(200)
            }
            _ => {
                let html = fs::read_to_string("public/hello.html").await.unwrap();
                Response::html(html).status(405)
            }
        },
        "/bye" => match req.method {
            Method::GET => {
                let html = fs::read_to_string("public/bye.html").await.unwrap();
                Response::html(html).status(200)
            }
            _ => {
                let html = fs::read_to_string("public/hello.html").await.unwrap();
                Response::html(html).status(405)
            }
        },
        _ => {
            let html = fs::read_to_string("public/404.html").await.unwrap();
            Response::html(html).status(404)
        }
    }
}

async fn handle_connection(mut connection: TcpStream) {
    let mut buffer = [0; 1024];

    connection.read(&mut buffer).await.unwrap();

    let req_text = str::from_utf8(&buffer).unwrap().trim_end_matches("\0");

    let req = Request::from_string(req_text);
    let res = handler(req).await.to_string();

    let res_bytes = res.as_bytes();

    connection.write(res_bytes).await.unwrap();
    connection.flush().await.unwrap();
}

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3333").await.unwrap();

    print!("     > Listening on http://localhost:3333\n     > Type Ctrl+C to stop\n");

    loop {
        let (connection, _) = listener.accept().await.unwrap();
        task::spawn(async move {
            handle_connection(connection).await;
        });
    }
}
