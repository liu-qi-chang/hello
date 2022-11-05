use std::{net::{TcpStream, TcpListener}, io::Write};

fn main() {
    let mut con = TcpStream::connect("127.0.0.1:3000").unwrap();
    con.write("hello".as_bytes());
    println!("Hello, world!");
}
