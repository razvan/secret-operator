//! Generated gRPC definitions

pub static FILE_DESCRIPTOR_SET_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));

pub mod csi {
    pub mod v1 {
        tonic::include_proto!("csi.v1");
    }
}
