
use machineid_rs::{Encryption, HWIDComponent, IdBuilder};

pub fn get_hwid(use_disk_serial: bool, build_key: &str) -> String {
    let mut builder = IdBuilder::new(Encryption::SHA256);

    builder
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::CPUCores)
        .add_component(HWIDComponent::MacAddress)
        .add_component(HWIDComponent::SystemID);

    if use_disk_serial {
        builder.add_component(HWIDComponent::DriveSerial);
    }

    let hwid = builder.build(build_key).unwrap();

    hwid
}