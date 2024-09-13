/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/flood-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
//! A module that re-exports commonly used items for convenience.
//!
//! The `prelude` module provides easy access to commonly used types, structs, and enums across
//! various parts of the library. By including this prelude, you can reduce the number of individual
//! imports needed in your code.
pub use crate::{
    Deserialize, InOctetStream, OutOctetStream, ReadOctetStream, Serialize, WriteOctetStream,
};
