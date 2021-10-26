// import modules
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    // binding a port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // get a sequence of streams
    for stream in listener.incoming() {
        // calling unwrap to terminate our program if the stream has any errors
        let stream = stream.unwrap();

        // do some work with the connection
        handle_connection(stream);
    }
}

// define the function
fn handle_connection(mut stream: TcpStream) {
    // define a buffer to hold the data that is read in
    let mut buffer = [0; 1024];
    // read bytes from the TcpStream and put them in the buffer
    stream.read(&mut buffer).unwrap();

    // print the request data
    let request = format!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // write the request data into the stream
    stream.write(request.as_bytes()).unwrap();
    // read a file
    let f = fs::read_to_string("hello.html");
    // match the result
    match f {
        // if result is OK
        Ok(file) => {
            // write data into stream
            stream.write("find the file\n".as_bytes()).unwrap();
        }
        // if result is error
        Err(err) => {
            // write data into stream
            stream.write("cannot find the file\n".as_bytes()).unwrap();
        }
    }
    // flush will wait and prevent the program from continuing until all the bytes are written to the connection
    stream.flush().unwrap();
}
