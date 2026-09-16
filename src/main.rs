use std::io::prelude::*;
use std::net::TcpStream;
use ssh2::Session;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Establish TCP connection
    let tcp = TcpStream::connect("192.168.1.1:22")?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    // 2. Authenticate
    sess.userauth_password("controller", "controller123@")?;

    // Check if step succeeded
    assert!(sess.authenticated());

    let commands=["sh ip int br",
                          "sh ip int stats",
                         ];
    for cmd in commands {

        // 3. Execute command via channel
        let mut channel = sess.channel_session()?;
        channel.exec(cmd)?;

        let mut output = String::new();
        channel.read_to_string(&mut output)?;
        channel.wait_close()?;

        println!("Output:\n{}", output);

    }
    Ok(())
}