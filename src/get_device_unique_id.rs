use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use std::env;
pub fn test() {
    let mut builder = IdBuilder::new(Encryption::SHA256);
    builder.add_component(HWIDComponent::SystemID);
    let my_id = builder.build("mg_zaw_min_aung").unwrap();
    println!("{}", my_id)
}

pub fn get_os() {
    println!("{}", env::consts::OS); /* Prints the current OS. */
}
