use std::io::prelude::*;
use std::time;
use std::net::TcpStream;
use rand::Rng;
use std::thread;
use image::io::Reader as ImageReader;
fn getmode() {
    println!("Please enter mode:");
    let i = pixelflut::input(); 
    match i.trim() {
        "void" => {
            for n in 0..=50 {
                println!("Spawned tread! {n}");
                let stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
                thread::spawn(move || {
                    print(getcoorx(), getcoory(), &getclr(), &stream)
                }); 
            }
            let stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
            loop {
                print(getcoorx(), getcoory(), &getclr(), &stream)
            };
        },
        "image" | "img" => {
            img();
        },
        _ => help(),
    };
}
fn help() {
    println!("help");
}
fn getclr() -> [u8; 3] {
    [
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
    getmode();
}
fn print(x: u32, y: u32, hex: &[u8], mut stream: &TcpStream) {
    let buf = format!("PX {x} {y} {:02x?}{:02x?}{:02x?}\n", hex[0], hex[1], hex[2]);
    let buffer = buf.as_bytes();
    stream.write(&buffer).expect("Error writing");
}
fn img() {
    let img = ImageReader::open("./myimage.jpg").unwrap().decode().unwrap();
    let w = img.width();
    let height = img.height();
    let mut c = 0;
    let mut l = 0;
    for n in 1..=3 {
        let ten_millis = time::Duration::from_secs(2);
        thread::sleep(ten_millis);
        println!("Spawned thread! {n}");
        let img = ImageReader::open("./myimage.jpg").unwrap().decode().unwrap();
        let stream = TcpStream::connect("192.168.120.13:1337").expect("Error connecting");
        thread::spawn(move || {
            loop {
                for n in img.as_rgb8().unwrap().chunks(3) {
                    c += 1;
                    print(c+20, l+20, n, &stream);
                    if c == w {
                        c = 0;
                        l += 1;
                    }
                    if l == height {
                        l = 0;
                    }
                }   
            }
        });
    }
    pixelflut::input();
    //scheiße, aber geht
}
