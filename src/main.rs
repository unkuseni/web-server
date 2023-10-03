use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut x = 0;
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        x += 1;
        println!("Connection established! {}", &x);
    }
}
