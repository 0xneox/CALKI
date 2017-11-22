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

use account_db::Factory as AccountFactory;
use evm::Factory as EvmFactory;
use native::Factory as NativeFactory;
use util::trie::TrieFactory;

/// Collection of factories.
#[derive(Default, Clone)]
pub struct Factories {
    /// factory for evm.
    pub vm: EvmFactory,
    pub native: NativeFactory,
    /// factory for tries.
    pub trie: TrieFactory,
    /// factory for account databases.
    pub accountdb: AccountFactory,
}
