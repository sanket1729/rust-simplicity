// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Bitcoin Extensions
//!
//! Extensions to the Simplicity language to allow use on the Bitcoin
//! blockchain
//!

use std::{fmt, io};

use super::TypeName;
use crate::bitcoin_hashes::{sha256, Hash, HashEngine};
use crate::bititer::BitIter;
use crate::cmr::Cmr;
use crate::encode;
use crate::exec;
use crate::extension;
use crate::Error;

/// Set of new Simplicity nodes enabled by the Bitcoin extension
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum JetsNode {
    Adder32,
    FullAdder32,
    Subtractor32,
    FullSubtractor32,
    Multiplier32,
    FullMultiplier32,
    Sha256HashBlock,
    SchnorrAssert,
    // Temparory jets for compiler
    EqV256,
    Sha256,
    LessThanV32, // less than verify for u32
    EqV32,
}

impl fmt::Display for JetsNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            JetsNode::Adder32 => "adder32",
            JetsNode::FullAdder32 => "fulladder32",
            JetsNode::Subtractor32 => "subtractor32",
            JetsNode::FullSubtractor32 => "fullsubtractor32",
            JetsNode::Multiplier32 => "multiplier32",
            JetsNode::FullMultiplier32 => "fullmultiplier32",
            JetsNode::Sha256HashBlock => "sha256hashblock",
            JetsNode::SchnorrAssert => "schnorrassert",
            JetsNode::EqV256 => "eqv256",
            JetsNode::Sha256 => "sha256",
            JetsNode::LessThanV32 => "le32",
            JetsNode::EqV32 => "eqv32",
        })
    }
}

impl extension::Jet for JetsNode {
    type TxEnv = ();
    /// Name of the source type for this node
    fn source_type(&self) -> TypeName {
        match *self {
            JetsNode::Adder32 => TypeName(b"l"),
            JetsNode::FullAdder32 => TypeName(b"*l2"),
            JetsNode::Subtractor32 => TypeName(b"l"),
            JetsNode::FullSubtractor32 => TypeName(b"*l2"),
            JetsNode::Multiplier32 => TypeName(b"l"),
            JetsNode::FullMultiplier32 => TypeName(b"*ll"),
            JetsNode::Sha256HashBlock => TypeName(b"*h*hh"),
            JetsNode::SchnorrAssert => TypeName(b"*h*hh"),
            JetsNode::EqV256 => TypeName(b"*hh"),
            JetsNode::Sha256 => TypeName(b"*hh"),
            JetsNode::LessThanV32 => TypeName(b"l"),
            JetsNode::EqV32 => TypeName(b"l"),
        }
    }

    /// Name of the target type for this node
    fn target_type(&self) -> TypeName {
        match *self {
            JetsNode::Adder32 => TypeName(b"*2i"),
            JetsNode::FullAdder32 => TypeName(b"*2i"),
            JetsNode::Subtractor32 => TypeName(b"*2i"),
            JetsNode::FullSubtractor32 => TypeName(b"*2i"),
            JetsNode::Multiplier32 => TypeName(b"l"),
            JetsNode::FullMultiplier32 => TypeName(b"l"),
            JetsNode::Sha256HashBlock => TypeName(b"h"),
            JetsNode::SchnorrAssert => TypeName(b"1"),
            JetsNode::EqV256 => TypeName(b"1"),
            JetsNode::Sha256 => TypeName(b"h"),
            JetsNode::LessThanV32 => TypeName(b"1"),
            JetsNode::EqV32 => TypeName(b"1"),
        }
    }

    /// CMR for this node
    fn cmr(&self) -> Cmr {
        let cmr = Cmr::new(b"Simplicity\x1fJet");
        match *self {
            JetsNode::Adder32 => cmr.update_1(Cmr::from([
                0x5e, 0xa6, 0x71, 0x42, 0xf7, 0x75, 0xea, 0x2b, 0xa2, 0x85, 0xce, 0xfb, 0x39, 0xc1,
                0xa4, 0x71, 0xd9, 0x77, 0x6a, 0x6e, 0x43, 0xc5, 0x95, 0x78, 0x15, 0xf7, 0xe8, 0x41,
                0x2d, 0x32, 0x6d, 0xca,
            ])),
            JetsNode::FullAdder32 => cmr.update_1(Cmr::from([
                0xfc, 0xc5, 0xca, 0x69, 0xd1, 0x7a, 0x3f, 0x3f, 0xb9, 0xad, 0x3b, 0x8f, 0x0e, 0xfc,
                0x7a, 0xdb, 0x50, 0x78, 0x00, 0xe0, 0xb8, 0x17, 0xe7, 0xcc, 0x1f, 0xcd, 0x55, 0xa3,
                0xcf, 0xc3, 0x8d, 0xba,
            ])),
            JetsNode::Subtractor32 => cmr.update_1(Cmr::from([
                0xf6, 0x9f, 0x42, 0x44, 0xba, 0x60, 0x13, 0x46, 0x77, 0x56, 0x70, 0x93, 0x3a, 0x56,
                0x8a, 0xac, 0x76, 0x8d, 0xd4, 0x51, 0x2d, 0x58, 0xc8, 0x06, 0x8b, 0x0e, 0xd4, 0x8b,
                0x91, 0xb1, 0x71, 0x8f,
            ])),
            JetsNode::FullSubtractor32 => cmr.update_1(Cmr::from([
                0x6a, 0x29, 0xf1, 0x82, 0xb0, 0xf5, 0xfd, 0x9c, 0x15, 0x4c, 0x79, 0x21, 0x62, 0x6e,
                0xcb, 0x36, 0x0a, 0x3c, 0x9c, 0x8a, 0x2b, 0xe3, 0x2b, 0xf7, 0x8a, 0x20, 0xed, 0x1f,
                0x25, 0xb6, 0xe1, 0xfd,
            ])),
            JetsNode::Multiplier32 => cmr.update_1(Cmr::from([
                0x89, 0x00, 0x14, 0x56, 0xbc, 0x90, 0x36, 0x7f, 0x13, 0x37, 0x3b, 0x30, 0xab, 0x66,
                0xec, 0x95, 0x2b, 0xab, 0x79, 0x6e, 0x3b, 0x7a, 0xe4, 0xa0, 0x5a, 0xaf, 0x40, 0xb0,
                0x0c, 0x23, 0x97, 0x93,
            ])),
            JetsNode::FullMultiplier32 => cmr.update_1(Cmr::from([
                0xe5, 0x0a, 0x5a, 0x6f, 0x78, 0xb4, 0x09, 0x0b, 0x29, 0x1e, 0x64, 0x5c, 0x3d, 0x28,
                0x0a, 0xbb, 0x57, 0x4e, 0xa9, 0xa9, 0x44, 0xe4, 0x0c, 0x21, 0x97, 0x9e, 0xdb, 0x8c,
                0x6e, 0x35, 0xc3, 0xf4,
            ])),
            JetsNode::Sha256HashBlock => cmr.update_1(Cmr::from([
                0xc9, 0xd1, 0x32, 0x60, 0x2d, 0xb6, 0x3d, 0xd4, 0x98, 0x1d, 0xa5, 0x8c, 0x6c, 0xda,
                0xd3, 0x05, 0x9e, 0x9c, 0xa7, 0x03, 0xe9, 0x78, 0xb6, 0x27, 0xcf, 0xe5, 0xe3, 0xe5,
                0x69, 0xa2, 0xf6, 0x76,
            ])),
            JetsNode::SchnorrAssert => cmr.update_1(Cmr::from([
                0xee, 0xae, 0x47, 0xe2, 0xf7, 0x87, 0x6c, 0x3b, 0x9c, 0xbc, 0xd4, 0x04, 0xa3, 0x38,
                0xb0, 0x89, 0xfd, 0xea, 0xdf, 0x1b, 0x9b, 0xb3, 0x82, 0xec, 0x6e, 0x69, 0x71, 0x9d,
                0x31, 0xba, 0xec, 0x9b, //only last `a` changed to `b` from sha2 block cmr
            ])),
            JetsNode::EqV256 => cmr.update_1(Cmr::from([
                0xee, 0xae, 0x47, 0xe2, 0xf7, 0x87, 0x6c, 0x3b, 0x9c, 0xbc, 0xd4, 0x04, 0xa3, 0x38,
                0xb0, 0x89, 0xfd, 0xea, 0xdf, 0x1b, 0x9b, 0xb3, 0x82, 0xec, 0x6e, 0x69, 0x71, 0x9d,
                0x31, 0xba, 0xec, 0x9c, //only last `a` changed to `c` from sha2 block cmr
            ])),
            JetsNode::Sha256 => cmr.update_1(Cmr::from([
                0xee, 0xae, 0x47, 0xe2, 0xf7, 0x87, 0x6c, 0x3b, 0x9c, 0xbc, 0xd4, 0x04, 0xa3, 0x38,
                0xb0, 0x89, 0xfd, 0xea, 0xdf, 0x1b, 0x9b, 0xb3, 0x82, 0xec, 0x6e, 0x69, 0x71, 0x9d,
                0x31, 0xba, 0xec, 0x9d, //only last `a` changed to `d` from sha2 block cmr
            ])),
            JetsNode::LessThanV32 => cmr.update_1(Cmr::from([
                0xee, 0xae, 0x47, 0xe2, 0xf7, 0x87, 0x6c, 0x3b, 0x9c, 0xbc, 0xd4, 0x04, 0xa3, 0x38,
                0xb0, 0x89, 0xfd, 0xea, 0xdf, 0x1b, 0x9b, 0xb3, 0x82, 0xec, 0x6e, 0x69, 0x71, 0x9d,
                0x31, 0xba, 0xec, 0x9e, //only last `a` changed to `e` from sha2 block cmr
            ])),
            JetsNode::EqV32 => cmr.update_1(Cmr::from([
                0xee, 0xae, 0x47, 0xe2, 0xf7, 0x87, 0x6c, 0x3b, 0x9c, 0xbc, 0xd4, 0x04, 0xa3, 0x38,
                0xb0, 0x89, 0xfd, 0xea, 0xdf, 0x1b, 0x9b, 0xb3, 0x82, 0xec, 0x6e, 0x69, 0x71, 0x9d,
                0x31, 0xba, 0xec, 0x9f, //only last `a` changed to `f` from sha2 block cmr
            ])),
        }
    }

    fn wmr(&self) -> Cmr {
        self.cmr()
    }

    /// Encode the node into a bitstream
    fn encode<W: encode::BitWrite>(&self, w: &mut W) -> io::Result<usize> {
        match *self {
            JetsNode::Adder32 => w.write_u8(48 + 0, 6),
            JetsNode::Subtractor32 => w.write_u8(48 + 1, 6),
            JetsNode::Multiplier32 => w.write_u8(24 + 1, 5),
            JetsNode::FullAdder32 => w.write_u8(48 + 2, 6),
            JetsNode::FullSubtractor32 => w.write_u8(48 + 3, 6),
            JetsNode::FullMultiplier32 => w.write_u8(24 + 3, 5),
            JetsNode::Sha256HashBlock => w.write_u8(14, 4),
            JetsNode::SchnorrAssert => w.write_u8(15 * 16 + 0, 8),
            JetsNode::EqV256 => w.write_u8(15 * 16 + 1, 8),
            JetsNode::Sha256 => w.write_u8(15 * 16 + 2, 8),
            JetsNode::LessThanV32 => w.write_u8(15 * 16 + 3, 8),
            JetsNode::EqV32 => w.write_u8(15 * 16 + 3, 8),
        }
    }

    /// Decode a natural number according to section 7.2.1
    /// of the Simplicity whitepaper. Assumes that a 11 has
    /// already been read from the stream
    fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<Self, Error> {
        match iter.next() {
            Some(false) => {
                let code = match iter.read_bits_be(2) {
                    Some(code) => code,
                    None => return Err(Error::EndOfStream),
                };
                match code {
                    0 => match iter.next() {
                        Some(false) => Ok(JetsNode::Adder32),
                        Some(true) => Ok(JetsNode::Subtractor32),
                        None => Err(Error::EndOfStream),
                    },
                    1 => Ok(JetsNode::Multiplier32),
                    2 => match iter.next() {
                        Some(false) => Ok(JetsNode::FullAdder32),
                        Some(true) => Ok(JetsNode::FullSubtractor32),
                        None => Err(Error::EndOfStream),
                    },
                    3 => Ok(JetsNode::FullMultiplier32),
                    _ => unreachable!(),
                }
            }
            Some(true) => match iter.next() {
                Some(false) => Ok(JetsNode::Sha256HashBlock),
                Some(true) => {
                    // Some custom jets for fast developement
                    // FIXME: Get a consensus for encoding with Rusell
                    let code = match iter.read_bits_be(4) {
                        Some(code) => code,
                        None => return Err(Error::EndOfStream),
                    };
                    match code {
                        0 => Ok(JetsNode::SchnorrAssert),
                        1 => Ok(JetsNode::EqV256),
                        2 => Ok(JetsNode::Sha256),
                        3 => Ok(JetsNode::LessThanV32),
                        4 => Ok(JetsNode::EqV32),
                        _ => unreachable!(),
                    }
                }
                None => Err(Error::EndOfStream),
            },
            None => Err(Error::EndOfStream),
        }
    }

    fn exec(&self, mac: &mut exec::BitMachine, _tx_env: &Self::TxEnv) {
        match *self {
            JetsNode::Adder32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();
                let (res, overflow) = a.overflowing_add(b);
                mac.write_bit(overflow);
                mac.write_u32(res);
            }
            JetsNode::FullAdder32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();
                let carry = mac.read_bit();
                let (res, overflow_1) = a.overflowing_add(b);
                let (res, overflow_2) = res.overflowing_add(carry as u32);
                mac.write_bit(overflow_1 || overflow_2);
                mac.write_u32(res);
            }
            JetsNode::Subtractor32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();
                let (res, overflow) = a.overflowing_sub(b);
                mac.write_bit(overflow);
                mac.write_u32(res);
            }
            JetsNode::FullSubtractor32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();
                let carry = mac.read_bit();
                let (res, overflow_1) = a.overflowing_sub(b);
                let (res, overflow_2) = res.overflowing_sub(carry as u32);
                mac.write_bit(overflow_1 || overflow_2);
                mac.write_u32(res);
            }
            JetsNode::Multiplier32 => {
                let a = mac.read_u32() as u64;
                let b = mac.read_u32() as u64;
                mac.write_u64(a * b);
            }
            JetsNode::FullMultiplier32 => {
                let a = mac.read_u32() as u64;
                let b = mac.read_u32() as u64;
                let c = mac.read_u32() as u64;
                let d = mac.read_u32() as u64;
                mac.write_u64(a * b + c + d);
            }
            JetsNode::Sha256HashBlock => {
                let hash = mac.read_32bytes();
                let block = mac.read_bytes(64);
                let sha2_midstate = sha256::Midstate::from_inner(hash);
                let mut engine = sha256::HashEngine::from_midstate(sha2_midstate, 0);
                engine.input(&block);
                let h = engine.midstate();
                mac.write_bytes(&h.into_inner());
            }
            JetsNode::SchnorrAssert => {
                let _pubkey = mac.read_32bytes();
                let _sig = mac.read_bytes(64);
                //Check the signature here later
            }
            JetsNode::EqV256 => {
                let a = mac.read_32bytes();
                let b = mac.read_32bytes();

                // FIXME:
                // Get Error here instead of assert
                assert!(a == b);
            }
            JetsNode::Sha256 => {
                let data = mac.read_32bytes();
                let h = sha256::Hash::hash(&data);

                mac.write_bytes(&h);
            }
            JetsNode::LessThanV32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();

                // FIXME: error
                assert!(a < b);
            }
            JetsNode::EqV32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();

                // FIXME: error
                assert!(a == b);
            }
        }
    }
}
