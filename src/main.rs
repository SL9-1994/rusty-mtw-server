use std::fs::File;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

// region:          --- Constants

const ADDRESS: &str = "127.0.0.1:7878";

// end region:      --- Constants

fn main() {
    // Listen for tcp connection.
    match TcpListener::bind(ADDRESS) {
        Ok(listener) => {
            println!("Connection established!");
            // three-way handshaking => connection => start streaming
            // tcpストリーム接続を待ち受ける。
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                handle_connection(stream);
            }
        }

        Err(err) => match err.kind() {
            std::io::ErrorKind::PermissionDenied => {
                eprintln!("Error: Port requiring administrator privileges.");
            }
            std::io::ErrorKind::AddrInUse => {
                eprintln!("Error: Duplicate port connection.");
            }
            _ => {
                eprintln!("Error: {}", err);
            }
        },
    };
}

/// Handles a TCP connection by reading data from the stream and printing the request.
///
/// ## Arguments
///
/// * `stream` - A mutable reference to a `TcpStream` representing the TCP connection.
///
/// ## Returns
///
/// * `None`
fn handle_connection(mut stream: TcpStream) {
    // 読み取りデータを1024byte保持するスタック
    // TODO: 任意のサイズのリクエストを取り扱うために、Bufferの管理方法を変更
    let mut buffer = [0; 1024];

    // TcpStreamからByteを読み込む。
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "response_test.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
