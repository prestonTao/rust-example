use std::io::{Read, Write};
use std::net::TcpListener;

// 简单检查一下是否http请求
fn http_check(buf: &Vec<u8>) -> bool {
    let l = buf.len();
    return if l <= 4 {
        false
    } else {
        buf[l - 1] == b'\n' && buf[l - 2] == b'\r' && buf[l - 3] == b'\n' && buf[l - 4] == b'\r'
    };
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2233").unwrap();
    let mut buf = vec![0u8; 0];

    let (mut socket, _) = listener.accept().unwrap();
    loop {
        let mut b = vec![0u8; 1024];
        if http_check(&buf) {
            break;
        }

        let n = socket.read(&mut b).unwrap();
        if 0 == n {
            println!("eof");
            return;
        } else {
            buf.append(&mut b[..n].to_vec());
        }
    }

    socket
        .write(b"HTTP/1.1 200\r\nContent-Length: 0\r\n\r\n")
        .unwrap();
    println!("{}", String::from_utf8_lossy(&buf));
}