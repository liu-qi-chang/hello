use std::{net::{TcpStream, TcpListener}, io::Write};

fn main() {
    for i in 0..10{
        let mut con = TcpStream::connect("127.0.0.1:7878").unwrap();
        con.write("hello".as_bytes());
        println!("Hello, world! {}", i);
    }
    
}
