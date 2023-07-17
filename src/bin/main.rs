extern crate web_demo;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;
use std::thread;
use std::time::Duration;
use web_demo::ThreadPool;

const SUCCESS_STATUS_LINE: &str = "HTTP/1.1 200 OK\r\n\r\n";
const FAILURE_STATUS_LINE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 最初にsize個のthreadを作り、プールに溜めておく
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_data = String::from_utf8_lossy(&buffer[..]);
    let route = request_data.split('\n').collect::<Vec<_>>()[0].split(' ').collect::<Vec<_>>()[1];

    let (status_line, filename) = get_status_filename(route);

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


fn get_status_filename(route: &str) -> (&str, &str) {
    match route {
        "/" => ("HTTP/1.1 200 NOT FOUND\r\n\r\n", "index.html"),
        "/index" => (SUCCESS_STATUS_LINE, "index.html"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            (SUCCESS_STATUS_LINE, "sleep.html")
        }
        _ => (FAILURE_STATUS_LINE, "404.html")
    }
}
