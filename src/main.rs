use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Connect to the Cisco node's TCP port 22 (SSH)
    let tcp = TcpStream::connect("192.168.1.1:22")?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    // 2. Authenticate with username and password
    sess.userauth_password("controller", "controller123@")?;

    // 3. Open an SSH channel
    let mut channel = sess.channel_session()?;

    // 4. Send the command to the Cisco node
    channel.exec("show version")?;

    // 5. Read the output from the node
    let mut s = String::new();
    channel.read_to_string(&mut s)?;
    print!("{}", s);

    // 6. Close the channel cleanly
    channel.wait_close()?;

    Ok(())
}