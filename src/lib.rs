/*
pub mod arrow_flight_protocol {
   tonic::include_proto!("arrow.flight.protocol"); // The string specified here must match the proto package name
}

*/

#[path = "helloworld.rs"]
pub mod helloworld;
pub const HELLOWORLD_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("helloworld");