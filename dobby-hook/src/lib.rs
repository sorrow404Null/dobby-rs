#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::missing_safety_doc)]

pub use dobby_hook_core::*;

pub mod core {
    pub use dobby_hook_core::*;
}

#[cfg(feature = "framework")]
pub mod framework {
    pub use dobby_rs_framework::*;
}

#[cfg(feature = "framework")]
pub mod prelude {
    pub use dobby_rs_framework::prelude::*;
}
