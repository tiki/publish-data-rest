/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub mod microblink_receipt;
pub mod dummy;
mod utils;

#[cfg(feature = "microblink_receipt")]
pub use microblink_receipt as generic;

#[cfg(feature = "dummy")]
pub use dummy as generic;
