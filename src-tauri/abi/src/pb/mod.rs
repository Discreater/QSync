#[allow(non_camel_case_types)]
mod musync;
pub(crate) mod google {
  pub mod protobuf {
    include!("./google.protobuf.rs");
  }
}
pub use musync::*;
