use rustmiko::devices::cisco::CiscoSSH;

fn main() -> anyhow::Result<()> {
    let mut cisco = match CiscoSSH::connect("192.168.1.1:23", "controller", "controller123@") {
        Ok(cisco) => {
            println!("Connected successfully");
            cisco
        },
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return Ok(());
        },
    };

    Ok(())
}