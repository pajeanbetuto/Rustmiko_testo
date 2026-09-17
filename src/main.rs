use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Connect to the Cisco node via TCP and authenticate
    let tcp = TcpStream::connect("192.168.1.1:22")?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password("controller", "controller123@")?;

    // 2. Open an SSH channel and request a PTY shell
    let mut channel = sess.channel_session()?;
    channel.request_pty("vt100", None, None)?;
    channel.shell()?;

    // 3. Send the command to the interactive shell
    // Note: Including a newline/carriage return is necessary to execute the command
    channel.write_all(b"show ip interface brief\n")?;
    channel.flush()?;

    // 4. Read the output from the channel
    let mut output = String::new();

    // Give the device a brief moment to respond or read until buffer blocks/completes
    // (A more robust production solution would handle reading chunks in a loop until the prompt returns)
    let _ = channel.read_to_string(&mut output);

    print!("{}", output);

    // 5. Clean up
    channel.send_eof()?;
    channel.wait_close()?;

    Ok(())
}