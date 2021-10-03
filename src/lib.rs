// Copyright 2021 Colin Finck <colin@reactos.org>
// SPDX-License-Identifier: GPL-2.0-or-later

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

extern crate alloc;

#[macro_use]
mod helpers;

mod attribute;
pub mod attribute_value;
mod boot_sector;
mod error;
mod file;
mod file_reference;
mod guid;
mod index;
mod index_entry;
mod index_record;
pub mod indexes;
mod ntfs;
mod record;
mod string;
pub mod structured_values;
mod time;
mod traits;
mod types;
mod upcase_table;

pub use crate::attribute::*;
pub use crate::error::*;
pub use crate::file::*;
pub use crate::file_reference::*;
pub use crate::guid::*;
pub use crate::index::*;
pub use crate::ntfs::*;
pub use crate::string::*;
pub use crate::time::*;
pub use crate::traits::*;
pub use crate::types::*;
