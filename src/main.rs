use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for (_count, stream) in listener.incoming().enumerate() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: String = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:?}", http_request);

    let request_line = http_request
        .lines()
        .next()
        .unwrap()
        .split(':')
        .map(|s| &s[0..s.len().saturating_sub(4)])
        .collect::<Vec<_>>()[0];

    let request_line = "GET / HTTP/1.1";

    let request_vec: Vec<_> = request_line.split(' ').collect();

    let request_method = request_vec[0];

    let request_path = if request_vec[1] == "/" {
        "pages/main.html".to_string()
    } else {
        format!("pages/{}", request_vec[1].split_at(1).1)
    };

    let _http_version = request_vec[2];

    match request_method {
        "GET" => match fs::read_to_string(request_path) {
            Ok(val) => {
                let status_line = "HTTP/1.1 200 OK";
                let len = val.len();
                let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{val}");

                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(_) => {
                let status_line = "HTTP/1.1 404 OK";
                let html = fs::read_to_string("pages/404.html").unwrap();
                let len = html.len();

                let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{html}");

                stream.write_all(response.as_bytes()).unwrap();
            }
        },
        "POST" => {
            println!("Someone posted");
        }
        _ => {}
    }
}
