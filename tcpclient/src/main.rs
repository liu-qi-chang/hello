use std::{net::{TcpStream, TcpListener}, io::{Write, Read}};

fn main() {
    for i in 0..10000 {
        let mut con = TcpStream::connect("127.0.0.1:7878").unwrap();
        let s = "GET /hello HTTP/1.1\r\nHost:localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */8\r\n\r\n";
        con.write(s.as_bytes()).unwrap();
        let mut buffer = [0; 1024];
        con.read(&mut buffer).unwrap();
        println!("Hello, world! {}", String::from_utf8_lossy(&buffer));
    }
    
}
