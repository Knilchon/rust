mod utils;
use std::net::{TcpListener, TcpStream, UdpSocket,SocketAddrV4,Ipv4Addr};
use std::io::{Read, Write};
use std::fs;

fn main(){
    // let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind to address");
    let udp_listeer = UdpSocket::bind("0.0.0.0:8080").expect("Failed to bind to address");

    loop {
        let mut buf = [0; 1024];

        let (amt, src) = udp_listeer.recv_from(&mut buf).unwrap();
        println!("Received {} bytes from {}", amt, src);    

        let data = &buf[..amt];
        println!("{:?}",String::from_utf8_lossy(data).trim());  

        udp_listeer.send_to(data, src).unwrap();
    }

    // println!("Running on port 8080!");
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             // Spawn a new thread or use async/await for concurrency
    //             println!("Connetion");
    //             std::thread::spawn(|| handle_client(stream));
    //         }
    //         Err(e) => {
    //             eprintln!("Error accepting connection: {}", e);
    //         }
    //     }
    // }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // Read data from the client
    stream.read(&mut buffer).unwrap();
    // Process the received data

    let mut received_data = String::from_utf8_lossy(&buffer).into_owned();
    let path = utils::extract_path_method(& received_data.as_str()).expect("path Error");
    println!("{:?}", path);
    let response: String = match path {
        ("/","GET") => handle_root(&mut received_data).expect("handle root failed"),
        _ => handle_not_found(&mut received_data).expect("handle not found failed"),
    };

    stream.write(response.as_bytes()).unwrap();
}

// ROUTE /
fn handle_root(_req: &mut String) -> Result<String,std::io::Error>{
    let html_content = match fs::read_to_string("src/html.html") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading HTML file: {}", e);
            return Err(e);
        }
    };
    // Send a response back to the client
    Ok(format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        html_content.len(),
        html_content
    ))
}

// Route 404
fn handle_not_found(_req: &mut String) -> Result<String,std::io::Error>{
    Ok(String::from("HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n"))
}