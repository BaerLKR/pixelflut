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
fn main() {
    // print(20, 20, hex);
    // println!("ok")
    let mut stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
    thread::spawn(move || {
    let hex = getclr();
        loop {
            for x in 0..1000 {
                for y in 0..200 {
                    print(x , y , &hex, &mut stream)
                }
            }
        }
    });
    let mut stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
    let hex = getclr();
    thread::spawn(move || {
        loop {
            for x in 0..1000 {
                for y in 200..400 {
                    print(x , y , &hex, &mut stream)
                }
            }
        }
    });
    let mut stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
    let hex = getclr();
    thread::spawn(move || {
        loop {
            for x in 0..1000 {
                for y in 400..600 {
                    print(x , y , &hex, &mut stream)
                }
            }
        }
    });
    let mut stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");

    let hex = getclr();
    thread::spawn(move || {
        loop {
            for x in 0..1000 {
                for y in 600..800 {
                    print(x , y , &hex, &mut stream)
                }
            }
        }
    });
    let mut stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
    let hex = getclr();
    loop {
        for x in (0..1000).rev() {
            for y in (0..500).rev() {
                print(x , y , &hex, &mut stream)
            }
        }
    }
}
fn print(x: i32, y: i32, hex: &[u8; 4], mut stream: &TcpStream) {
    // let buf = format!("PX {x} {y} {hex}\n");
    let buf = format!("PX {x} {y} {:02x?}{:02x?}{:02x?}{:02x?}\n", hex[0], hex[1], hex[2], hex[3]);
    // println!("{buf}");
    // pintln!("{x} {y}");
    let buffer = buf.as_bytes();
    stream.write(&buffer).expect("Error writing");
}
