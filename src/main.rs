use std::io::prelude::*;
use std::net::TcpStream;
use rand::Rng;
use std::thread;
fn getclr() -> [u8; 4] {
    [
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
    ]
}
fn getcoorx() -> u32 {
    rand::thread_rng().gen_range(0..1080)
}
fn getcoory() -> u32 {
    rand::thread_rng().gen_range(0..1000)
}
fn main() {
    for _ in 0..25 {
        let stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
        thread::spawn(move || {
            print(getcoorx(), getcoory(), &getclr(), &stream)
        }); 
    }
    let stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
    loop {
        print(getcoorx(), getcoory(), &getclr(), &stream)
    };
}
fn print(x: u32, y: u32, hex: &[u8; 4], mut stream: &TcpStream) {
    // let buf = format!("PX {x} {y} {hex}\n");
    let buf = format!("PX {x} {y} {:02x?}{:02x?}{:02x?}{:02x?}\n", hex[0], hex[1], hex[2], hex[3]);
    let buffer = buf.as_bytes();
    stream.write(&buffer).expect("Error writing");
}
