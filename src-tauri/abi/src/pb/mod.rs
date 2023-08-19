#[allow(non_camel_case_types, non_snake_case)]
mod musync;
pub(crate) mod google {
  #[allow(non_camel_case_types, non_snake_case)]
  pub mod protobuf {
    include!("./google.protobuf.rs");
  }
}
pub use musync::*;
