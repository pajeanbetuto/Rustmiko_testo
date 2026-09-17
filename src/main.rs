use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

pub fn send_commands(
    addr: &str,
    username: &str,
    password: &str,
    commands: &[&str],
) -> anyhow::Result<Vec<String>> {
    let sess = connect_and_auth(addr, username, password)?;

    let mut results = Vec::new();
    for cmd in commands {
        let output = exec_remote_command(&sess, cmd)?;
        results.push(output);
    }

    Ok(results)
}

fn connect_and_auth(addr: &str, username: &str, password: &str) -> anyhow::Result<Session> {
    let tcp = TcpStream::connect(addr)?;

    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    sess.userauth_password(username, password)?;

    if !sess.authenticated() {
        anyhow::bail!("Authentication failed");
    }

    Ok(sess)
}

fn exec_remote_command(sess: &Session, cmd: &str) -> anyhow::Result<String> {
    let mut channel = sess.channel_session()?;
    channel.exec(cmd)?;

    let mut output = String::new();
    channel.read_to_string(&mut output)?;

    channel.wait_close()?;
    let _status = channel.exit_status()?;

    Ok(output)
}

fn main() -> anyhow::Result<()> {
    let commands = vec![
        "uname -a",
        "hostname",
        "uptime",
    ];

    let results = send_commands(
        "127.0.0.1:22",
        "pajeanbe",
        "G3od&sIc189)",
        &commands,
    )?;

    for (cmd, output) in commands.iter().zip(results.iter()) {
        println!(">>> Command: {cmd}");
        println!("{output}");
        println!("--------------------------------");
    }

    Ok(())
}
