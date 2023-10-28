/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub mod microblink_receipt;
pub mod shopify_order;
pub mod dummy;
mod utils;

#[cfg(feature = "microblink_receipt")]
pub use microblink_receipt as generic;

#[cfg(feature = "shopify_order")]
pub use shopify_order as generic;

#[cfg(feature = "dummy")]
pub use dummy as generic;
