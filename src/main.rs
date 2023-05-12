use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
fn main() {
    let hex = String::from("000000");
    // print(20, 20, hex);
    // println!("ok")
    let mut stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
    // loop {
    //     for x in 0..1000 {
    //         for y in 0..1000 {
    //             print(x , y , &hex, &mut stream)
    //         }
    //     }
    // }
    thread::spawn(move || {
        loop {
            for x in 0..500 {
                for y in 500..1000 {
                    print(x , y , &hex, &mut stream)
                }
            }
        }
    });
    let mut stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
    let hex = String::from("333333");
    thread::spawn(move || {
        loop {
            for x in 0..500 {
                for y in 0..1000 {
                    print(x , y , &hex, &mut stream)
                }
            }
        }
    });
    let mut stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
    let hex = String::from("ffffff");
    thread::spawn(move || {
        loop {
            for x in 100..200 {
                for y in 0..200 {
                    print(x , y , &hex, &mut stream)
                }
            }
        }
    });
    let mut stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
    let hex = String::from("ffffff");
    thread::spawn(move || {
        loop {
            for x in 0..100 {
                for y in 0..200 {
                    print(x , y , &hex, &mut stream)
                }
            }
        }
    });
    let mut stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
    let hex = String::from("000000");
    loop {
        for x in (500..1000).rev() {
            for y in (0..500).rev() {
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
