use omnisor::{DeviceSession, CiscoVariant};

#[tokio::main]
async fn main() -> Result<(), omnisor::Error> {
    // Connexion à un commutateur/routeur Cisco (IOS)
    let mut session = DeviceSession::connect(
        ("192.168.1.1", 22),
        "controller",
        "controller123@",
        CiscoVariant::Ios, // Utilisez IosLegacy pour les anciens matériels
    ).await?;

    // Envoyer une commande et récupérer le résultat
    let output = session.send_command("show ip interface brief").await?;
    println!("{:?}", output);

    Ok(())
}
