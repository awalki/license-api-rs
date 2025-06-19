use machineid_rs::{Encryption, HWIDComponent, IdBuilder};

pub async fn get_hwid() -> String {
    let mut builder = IdBuilder::new(Encryption::SHA256);

    // Change components on your purposes
    builder
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::CPUCores)
        .add_component(HWIDComponent::DriveSerial)
        .add_component(HWIDComponent::MacAddress)
        .add_component(HWIDComponent::SystemID);

    let hwid = builder.build("miracet").unwrap();

    hwid
}
