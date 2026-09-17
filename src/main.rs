use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcp = TcpStream::connect("192.168.1.1:22")?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password("controller", "controller123@")?;

    // Set a blocking timeout (in milliseconds) for session operations/reads
    sess.set_timeout(5000);

    let mut channel = sess.channel_session()?;
    channel.request_pty("vt100", None, None)?;
    channel.shell()?;

    // Send the command
    channel.write_all(b"show ip interface brief\n")?;
    channel.flush()?;

    // Read loop capturing chunks until the prompt appears or timeout triggers
    let mut output = String::new();
    let mut buffer = [0u8; 1024];

    loop {
        match channel.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let chunk = String::from_utf8_lossy(&buffer[..n]);
                output.push_str(&chunk);

                // Break out once the Cisco prompt is detected
                if output.contains('#') || output.contains('>') {
                    if output.lines().count() > 2 {
                        break;
                    }
                }
            }
            Err(e) => {
                // This catches the session timeout if it hits the limit without an EOF
                eprintln!("Read finished or timed out: {}", e);
                break;
            }
        }
    }

    print!("{}", output);

    channel.send_eof()?;
    channel.wait_close()?;

    Ok(())
}