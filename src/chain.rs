// Copyright (C) 2019-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Data structures describing a chain of blocks.
//!
//! A chain of blocks is composed of two parts:
//!
//! - A list of finalized blocks. Finalized blocks are blocks that are forever part of the chain
//! and can never be reverted.
//! - A tree of non-finalized blocks built on top of the last finalized block.
//!
//! When a block first appears it is verified and, on success, added to the tree of
//! non-finalized blocks. Later, this block might get finalized. When a block is finalized, all
//! the blocks that are not one of its ancestors or descendants is entirely discarded.
//!
//! Example chain:
//!
//! ```ignore
//!                             +-> #5
//!                             |
//!                      +-> #4 +-> #5
//!                      |
//! #0 +> #1 +> #2 +> #3 +-> #4 +-> #5 +> #6
//! ```
//!
//! In this example, #3 is the latest finalized block. Before and including #3, the chain is
//! always a simple list. After #3, the chain becomes a tree.
//!
//! > **Note**: This example is exaggerated, and in most situations the non-finalized blocks also
//! >           form a simple list with a few extra individual leaves.
//!
//! Amongst the non-finalized blocks, one block is chosen as the *best* block. When authoring
//! blocks, the *best block* is the one upon new blocks will be built upon. When finalizing
//! blocks, the *best block* is the one that will be voted for finalization. If there isn't any
//! non-finalized block, the latest finalized block is also the best block.

pub mod blocks_tree;
pub mod chain_information;
pub mod fork_tree;
pub mod sync;
