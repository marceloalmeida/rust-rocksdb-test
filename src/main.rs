use rocksdb::DB;
use std::env;

use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let db = DB::open_default("/alloc/data/rocksdb").unwrap();
    let length = 10;
    let mut body = "".to_string();

    for x in 0..length {
      match db.get(format!("{:b}", x)) {
         //Ok(Some(value)) => println!("retrieved value: '{}'", value.to_utf8().unwrap()),
         Ok(Some(value)) => body.push_str(&format!("{}\n", &value.to_utf8().unwrap())),
         //Ok(Some(value)) => body.push_str("\n"),
         Ok(Some(value)) => println!("retrieved value: '{}'", value.to_utf8().unwrap()),
         Ok(None) => println!("value not found"),
         Err(e) => println!("operational problem encountered: {}", e),
      }
      db.put(format!("{:b}", x), format!("{}'{}'", "my awesome value is ", x));
      //db.delete(format!("{:b}", x)).unwrap();
    }

    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>{}</body></html>\r\n", body);

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}


fn main() {
    let mut length;

    if env::args().len() > 1 {
        let args: Vec<String> = env::args().collect();
        length = (&args[1]).parse().unwrap_or(0);
    } else {
        length = 10;
    }

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }

}
