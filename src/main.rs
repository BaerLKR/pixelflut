use std::io::prelude::*;
use std::net::TcpStream;
fn main() {
    let hex = String::from("F4511E");
    // print(20, 20, hex);
    // println!("ok")
    let mut stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
    loop {
        for x in 0..1000 {
            for y in 0..1000 {
                print(x , y , &hex, &mut stream)
            }
        }
    }
}
fn print(x: i32, y: i32, hex: &String, mut stream: &TcpStream) {
    let buf = format!("PX {x} {y} {hex}\n");
    // println!("{buf}");
    // pintln!("{x} {y}");
    let buffer = buf.as_bytes();
    stream.write(&buffer).unwrap();
}
