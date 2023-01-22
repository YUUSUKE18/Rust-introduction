use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{thread, time};

fn handle_read(mut stream: &TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_create(mut stream: &TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request_body = String::from_utf8_lossy(&buffer[..]);
    println!("Request Body: {}", request_body);
    let response = format!("HTTP/1.1 200 OK\r\n\r\nYou sent me: {}", request_body);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_update(mut stream: &TcpStream, id: &str) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request_body = String::from_utf8_lossy(&buffer[..]);
    println!("Request Body: {}", request_body);
    let response = format!("HTTP/1.1 200 OK\r\n\r\nYou updated item {} with data: {}", id, request_body);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_delete(mut stream: &TcpStream, id: &str) {
    let response = format!("HTTP/1.1 200 OK\r\n\r\nYou deleted item {}", id);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap();
    let id = parts.nth(1).unwrap();
    let id = &id[1..];
    match method {
        "GET" => handle_read(&stream),
        "POST" => handle_create(&stream),
        "PUT" => handle_update(&stream, id),
        "DELETE" => handle_delete(&stream, id),
        _ => handle_read(&stream),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running at http://127.0.0.1:7878");