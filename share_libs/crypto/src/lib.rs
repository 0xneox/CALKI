// CALKI
// Copyright 2016-2017 Zibbit Labs.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#[cfg(feature = "ed25519")]
extern crate calki_ed25519;
#[cfg(feature = "secp256k1")]
extern crate calki_secp256k1;
#[cfg(feature = "sm2")]
extern crate calki_sm2;
extern crate util;

#[cfg(feature = "ed25519")]
pub use calki_ed25519::*;
#[cfg(feature = "secp256k1")]
pub use calki_secp256k1::*;
#[cfg(feature = "sm2")]
pub use calki_sm2::*;
pub use util::crypto::{Sign, CreateKey};

#[cfg(feature = "ed25519")]
pub const SIGNATURE_NAME: &str = "ed25519";
#[cfg(feature = "secp256k1")]
pub const SIGNATURE_NAME: &str = "secp256k1";
#[cfg(feature = "sm2")]
pub const SIGNATURE_NAME: &str = "sm2";
