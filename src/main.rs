use server::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    // thread,
    // time::Duration,
};

fn main() {
    // Create an instance of TcpListener to listen on a port.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); 

    // Create a pool of threads to manage multiple incoming requests concurrently.
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() // .take(6) 
    { 
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Make GET requests to various IP addresses.
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello_world.html"),
        // "GET /sleep HTTP/1.1" => {
        //     thread::sleep(Duration::from_secs(5));
        //     ("HTTP/1.1 200 OK", "hello_world.html")
        // }
        // "GET /hidden HTTP/1.1" => ("HTTP/1.1 200 OK", "hidden.html"),\
        "GET /text_counter HTTP/1.1" => ("HTTP/1.1 200 OK", "text_counter.html"),
        "GET /hidden HTTP/1.1" => ("HTTP/1.1 200 OK", "hidden.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}




// ########## Rocket Web App - Uncomment to Use ############
// #[macro_use] extern crate rocket;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello World!"
// }

// #[get("/hello/<name>")]
// fn hello(name: &str) -> String {
//     format!("Hello {}!", name)
// }

// #[get("/add/<num1>/<num2>")]
// fn add(num1: i64, num2: i64) -> String {
//     let new_num = num1 + num2;
//     format!("{} + {} = {}", num1, num2, new_num)
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![index])
//         .mount("/", routes![hello])
//         .mount("/", routes![add])
// }
