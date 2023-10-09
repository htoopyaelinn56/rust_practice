pub fn test() {
    use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
    let mut builder = IdBuilder::new(Encryption::SHA256);
    builder.add_component(HWIDComponent::SystemID);
    let my_id = builder.build("mg_zaw_min_aung").unwrap();
    println!("{}", my_id)
}
