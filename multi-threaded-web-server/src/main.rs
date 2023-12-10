use std::{
    fs,
    thread,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Duration
};

fn main() {
    let port = 8080;
    let ip = "127.0.0.1";
    let address = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(&address).unwrap();

    println!("Server is running on {address}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, content_file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(content_file).unwrap();
    let content_length = content.len();

    let resposne = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");

    stream.write_all(resposne.as_bytes()).unwrap();
}