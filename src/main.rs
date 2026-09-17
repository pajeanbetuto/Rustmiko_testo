use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let tcp = TcpStream::connect("192.168.1.1:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("controller", "controller123@")
        .unwrap();
    let mut channel = sess.channel_session().unwrap();
    channel.exec("sh mac address-table ").unwrap();

    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().ok();
    println!("{}", channel.exit_status().unwrap());
}