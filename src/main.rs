use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // bind returns a new TcpListener instance, connecting to a port
    // is known as "binding to a port"
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); 

    // incoming method returns iterator that gives us a sequence of 
    // streams of type TcpStream, a stream is an open connection 
    // between client and server
    for stream in listener.incoming() { 
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // let response = "HTTP/1.1 200 OK\r\n\r\n";

    // stream.write_all(response.as_bytes()).unwrap();

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // if request_line == "GET / HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("hello_world.html").unwrap();
    //     let length = contents.len();

    //     let response = format!(
    //         "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    //     );
    
    //     stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();

    //     let response = format!(
    //         "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    //     );

    //     stream.write_all(response.as_bytes()).unwrap();
    // }

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello_world.html")
    } else if request_line == "GET /hidden HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hidden.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

    stream.write_all(response.as_bytes()).unwrap();
}