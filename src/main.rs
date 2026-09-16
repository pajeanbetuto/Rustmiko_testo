use rustmiko::devices::cisco::CiscoSSH;
use rustmiko::devices::generic::device_types::config::Configurable;

fn main() -> anyhow::Result<()> {
    let mut cisco = match CiscoSSH::connect("192.168.1.1:22", "controller", "controller123@") {
        Ok(cisco) => {
            println!("Connected successfully");
            cisco
        },
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return Ok(());
        },
    };

    let output= cisco.execute_raw("sh ip int br");
    println!("{:?}", output);



    Ok(())
}