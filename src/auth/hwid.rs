
use machineid_rs::{Encryption, HWIDComponent, IdBuilder};

pub async fn get_hwid(use_disk_serial: bool) -> String {
    let mut builder = IdBuilder::new(Encryption::SHA256);

    // Change components for your purposes
    builder
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::CPUCores)
        .add_component(HWIDComponent::MacAddress)
        .add_component(HWIDComponent::SystemID);

    // Conditionally add disc serial based on a parameter
    // Using disk serial may provide you more protection, but it maybe can't be used on some systems
    if use_disk_serial {
        builder.add_component(HWIDComponent::DriveSerial);
    }

    let hwid = builder.build("miracet").unwrap();

    hwid
}