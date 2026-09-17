use ssh2::Session;
use std::io::{Read};
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
        if cmd.trim().is_empty() {
            results.push("<empty command skipped>".to_string());
            continue;
        }

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

    let mut stdout = String::new();
    channel.read_to_string(&mut stdout)?;

    // Read stderr too (important!)
    let mut stderr = String::new();
    channel.stderr().read_to_string(&mut stderr).ok();

    // Drain EOF properly
    channel.send_eof()?;
    channel.wait_eof()?;
    channel.wait_close()?;

    let status = channel.exit_status()?;

    Ok(format!(
        "STATUS: {}\nSTDOUT:\n{}\nSTDERR:\n{}",
        status, stdout, stderr
    ))
}

fn main() -> anyhow::Result<()> {
    let commands = vec![
        "sh ip int br",
      //  "sh hosts",

    ];

    let results = send_commands(
        "192.168.1.1:22",
        "controller",
        "controller123@",
        &commands,
    )?;

    for (cmd, output) in commands.iter().zip(results.iter()) {
        println!(">>> {}", cmd);
        println!("{}", output);
        println!("--------------------------------");
    }

    Ok(())
}
