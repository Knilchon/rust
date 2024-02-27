use std::net::UdpSocket;

pub fn main(){
    let udp_listeer = UdpSocket::bind("0.0.0.0:8080").expect("Failed to bind to address");

    loop {
        let mut buf = [0; 1024];

        let (amt, src) = udp_listeer.recv_from(&mut buf).unwrap();
        println!("Received {} bytes from {}", amt, src);    

        let data = &buf[..amt];
        println!("{:?}",String::from_utf8_lossy(data).trim());  

        udp_listeer.send_to(data, src).unwrap();
    }
}