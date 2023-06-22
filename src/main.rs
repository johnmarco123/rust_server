use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs };
use std::fs::File;
use std::io::Read;

fn main() {
    let address = "127.0.0.1:80";
    let listener = TcpListener::bind(address).unwrap();
    // accept connections and process them serially
    println!("Listening on address {}", address);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn read_image_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    // let mut file = File::open(path)?;
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let mut start_idx: usize = 0;
    let mut end_idx: usize = 0;
    for (i, c) in request_line[..].chars().enumerate() {
        if start_idx == 0 {
            if c == '/' { start_idx = i; }
        } else {
            if c == ' ' {
                end_idx = i - 1
            }
        }
    }
    let requested_file = &request_line[start_idx..=end_idx];

    let mut file_name = match requested_file {
        "/" => "index.html",
        _ => requested_file,
    };

    if file_name.starts_with('/') {
        file_name = &requested_file[1..]
    };

    if requested_file.ends_with(".jpg") || requested_file.ends_with(".png") {
        let contents = read_image_file(file_name).unwrap();
        let length = contents.len();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: image/jpg\r\nContent-Length: {length}\r\n\r\n");
        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    } else {
        let contents = fs::read_to_string(file_name);
        let (status_line, contents) = match contents {
            Ok(contents) => ("HTTP/1.1 200 OK", contents),
            Err(_) => ("HTTP/1.1 404 NOT FOUND", String::from("File not found")),
        };

        let length = contents.len();
        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);
        stream.write_all(response.as_bytes()).unwrap();
    }

}
