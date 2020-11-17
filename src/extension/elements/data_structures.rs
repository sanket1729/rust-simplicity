// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//   Sanket kanjalkar <sanket1729@gmail.com>
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Elements Extensions: Data Structures
//!
//! Data Structures in rust-elements cannot be directly used as-is in
//! rust-simplicity. This file has additional data-structures for
//! simplicity transactions

use crate::cmr::Cmr;
use crate::exec;
use bitcoin_hashes::{sha256, Hash, HashEngine};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use elements::confidential::{Asset, Nonce, Value};
use elements::{confidential, AssetIssuance};

/// Helper trait for writing various components of
/// Simplicity transactions(Assets, Values) into bit machine.
pub(in crate::extension::elements) trait SimplicityEncodable {
    // write the simplicity encoding of `self` on bitmachine
    // at the current write cursor.
    fn simplicity_encode(self, mac: &mut exec::BitMachine);
}

/// A simplicity representation of elements confidential asset is then:
/// (prefix, asset) = ((is_explicit, is_odd),[u8; 32])
/// Write an confidential asset to write frame
/// advancing the cursor 258 cells, unless asset is not None
//FIXME: Change to errors
impl SimplicityEncodable for confidential::Asset {
    fn simplicity_encode(self, mac: &mut exec::BitMachine) {
        match self {
            // todo: Make appropriate errors
            Asset::Null => unreachable!(),
            Asset::Explicit(data) => {
                mac.write_bit(true);
                mac.skip(1);
                debug_assert!(data.len() == 32);
                mac.write_bytes(&data);
            }
            // consensus rules state that asset must be 0x0a or 0x0b
            Asset::Confidential(prefix, comm) => {
                if prefix != 0x0a || prefix != 0x0b {
                    unimplemented!()
                }
                mac.write_bit(false); //not explicit
                mac.write_bit(prefix == 0x0b);
                debug_assert!(comm.len() == 32);
                mac.write_bytes(&comm);
            }
        }
    }
}

/// In Elements, many fields can optionally be blinded and encoded as a
/// point on the secp256k1 curve.
/// The prefix determines the parity of the y-coordinate of that point, or
/// indicates the value is explicit.
/// In few cases, values are entirely optional, in which case
/// 'NONE' is a possibility.
/// Following are the possibilites of prefix: [None, Explicit, EvenY, OddY]
/// A simplicity representation of elements confidential value is then:
/// (prefix, value) = ((is_explicit, is_odd),[u8; 32])
/// Write an confidential asset to write frame
/// advancing the cursor 258 cells, unless asset is not None
//FIXME: Change to errors
impl SimplicityEncodable for confidential::Value {
    fn simplicity_encode(self, mac: &mut exec::BitMachine) {
        match self {
            // todo: Make appropriate errors
            Value::Null => unreachable!(),
            Value::Explicit(data) => {
                mac.write_bit(true);
                mac.skip(1 + 256 - 64);
                mac.write_u64(data);
            }
            // consensus rules state that prefix value must be 0x08 or 0x09
            Value::Confidential(prefix, comm) => {
                if prefix != 0x08 || prefix != 0x09 {
                    unimplemented!()
                }
                mac.write_bit(false); //not explicit
                mac.write_bit(prefix == 0x09);
                debug_assert!(comm.len() == 32);
                mac.write_bytes(&comm);
            }
        }
    }
}

/// In Elements, many fields can optionally be blinded and encoded as a
/// point on the secp256k1 curve.
/// The prefix determines the parity of the y-coordinate of that point, or
/// indicates the value is explicit.
/// In few cases, values are entirely optional, in which case
/// 'NONE' is a possibility.
/// Following are the possibilites of prefix: [None, Explicit, EvenY, OddY]
/// A simplicity representation of elements confidential none is then:
/// (prefix, value) = ((is_not_null, is_explicit, is_odd),[u8; 32])
/// Write an confidential asset to write frame
/// advancing the cursor 259 cells, unless asset is not None
//FIXME: Change to errors
impl SimplicityEncodable for confidential::Nonce {
    fn simplicity_encode(self, mac: &mut exec::BitMachine) {
        // all paths should write 259 bits
        match self {
            // todo: Make appropriate errors
            Nonce::Null => {
                mac.write_bit(false);
                mac.skip(258);
            }
            Nonce::Explicit(data) => {
                mac.write_bit(true); // not null
                mac.write_bit(true); // is explicit
                mac.skip(1);
                mac.write_bytes(&data);
            }
            // consensus rules state that prefix nocne must be 0x02 or 0x03
            Nonce::Confidential(prefix, comm) => {
                if prefix != 0x02 || prefix != 0x03 {
                    unimplemented!()
                }
                mac.write_bit(true); // not null
                mac.write_bit(false); // not explicit
                mac.write_bit(prefix == 0x03); // oddY
                debug_assert!(comm.len() == 32);
                mac.write_bytes(&comm);
            }
        }
    }
}
/// Simplicity has a different logic for computing the transactoin input and output
/// digest. This trait defines the method for computation of such digests.
pub(super) trait SimplicityHash {
    /// Add the hash of current tx component
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine);
}

impl SimplicityHash for confidential::Asset {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        match *self {
            Asset::Null => {
                eng.write_u8(0).unwrap();
            }
            Asset::Explicit(data) => {
                eng.write_u8(1).unwrap();
                eng.input(&data);
            }
            Asset::Confidential(prefix, data) => {
                assert!(prefix == 0x0a || prefix == 0x0b);
                eng.write_u8(prefix).unwrap();
                eng.input(&data);
            }
        }
    }
}

impl SimplicityHash for confidential::Value {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        match *self {
            Value::Null => {
                eng.write_u8(0).unwrap();
            }
            Value::Explicit(data) => {
                eng.write_u8(1).unwrap();
                eng.write_u64::<BigEndian>(data).unwrap();
            }
            Value::Confidential(prefix, data) => {
                assert!(prefix == 0x08 || prefix == 0x09);
                eng.write_u8(prefix).unwrap();
                eng.input(&data);
            }
        }
    }
}

impl SimplicityHash for confidential::Nonce {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        match *self {
            Nonce::Null => {
                eng.write_u8(0).unwrap();
            }
            Nonce::Explicit(data) => {
                eng.write_u8(1).unwrap();
                eng.input(&data);
            }
            Nonce::Confidential(prefix, data) => {
                assert!(prefix == 0x02 || prefix == 0x03);
                eng.write_u8(prefix).unwrap();
                eng.input(&data);
            }
        }
    }
}

impl SimplicityHash for bitcoin::Script {
    /// All scripts are first hashed to sha256 to get a scriptpubkey
    /// equivalent and then added to current sha256 context.
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        let script_hash = sha256::Hash::hash(&self.to_bytes());
        eng.input(&script_hash);
    }
}

// I think this should belong in rust-elements
pub(super) fn is_asset_reissue(asset: &AssetIssuance) -> bool {
    asset.asset_blinding_nonce != [0; 32]
}

// I think this should belong in rust-elements
pub(super) fn is_asset_new_issue(asset: &AssetIssuance) -> bool {
    asset.asset_blinding_nonce == [0; 32]
}

impl SimplicityHash for AssetIssuance {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        let is_new_issue = is_asset_new_issue(&self);
        if is_new_issue {
            self.amount.simplicity_hash(eng);
            self.inflation_keys.simplicity_hash(eng);
            // asset blinding nonce here must be zero
            eng.input(&self.asset_blinding_nonce);
            eng.input(&self.asset_entropy);
        } else {
            debug_assert!(is_asset_reissue(&self));
            self.amount.simplicity_hash(eng);
            // The inflation keys here must be zero
            // Review this assertion
            let null_amt = Value::Null;
            null_amt.simplicity_hash(eng);
            eng.input(&self.asset_blinding_nonce);
            eng.input(&self.asset_entropy);
        }
    }
}

impl SimplicityHash for elements::TxIn {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        eng.input(&self.previous_output.txid);
        eng.write_u32::<LittleEndian>(self.previous_output.vout)
            .unwrap();
        eng.write_u32::<LittleEndian>(self.sequence).unwrap();
        if self.has_issuance() {
            self.asset_issuance.simplicity_hash(eng);
        } else {
            let null_amt = confidential::Value::Null;
            null_amt.simplicity_hash(eng);
            null_amt.simplicity_hash(eng);
        }
    }
}

impl SimplicityHash for Vec<elements::TxIn> {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        for i in self {
            i.simplicity_hash(eng);
        }
    }
}

impl SimplicityHash for elements::TxOut {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        self.asset.simplicity_hash(eng);
        self.value.simplicity_hash(eng);
        self.nonce.simplicity_hash(eng);
        self.script_pubkey.simplicity_hash(eng);
    }
}

impl SimplicityHash for Vec<elements::TxOut> {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        for i in self {
            i.simplicity_hash(eng);
        }
    }
}

/// An elementsUTXO.
/// The data held by an Elements unspent transaction output database.
/// This `scriptPubKey` of the unspent transaction output,
/// which in our application is digested as a SHA-256 hash of simplicity program.
/// This also includes the asset and amout of the output, each of
/// which may or may not be blinded.
// This is not a complete TxOut as it does not contain the nonce that
// is sent to the recipient.
pub struct ElementsUtxo {
    /// the scriptpubkey of the elements transaction
    pub(super) script_pubkey: Cmr,
    /// The confidential asset
    pub(super) asset: confidential::Asset,
    /// The confidential transaction value
    pub(super) value: confidential::Value,
}

/// Transaction environment for Bitcoin Simplicity programs
///  * This includes
/// 1. the transaction data, which may be shared when Simplicity expressions
/// 2. The Utxos corresponding to value
/// 3. the input index under consideration,
/// 4. and the commitment Merkle root of the Simplicity expression being executed.
/// #NOTE:
/// The order of `utxos` must be same as of the order of inputs in the
/// transaction.
// FIXME: tx can be shared across multiple inputs in the same transaction.
// Changing tx to reference does not directly work as the trait declarations do
// not support generic assiciated types. Look out for other ways to fix this.
pub struct TxEnv {
    // The elements transaction
    pub(super) tx: elements::Transaction,
    // The input utxo information corresponding to outpoint being spent.
    pub(super) utxos: Vec<ElementsUtxo>,
    // the current index of the input
    pub(super) ix: u32,
    // Commitment merkle root of the script
    pub(super) script_cmr: Cmr,
    // cached InputHash
    pub(super) inputs_hash: sha256::Hash,
    // cached OutputHash
    pub(super) outputs_hash: sha256::Hash,
}

impl TxEnv {
    /// Constructor from a transaction
    pub fn from_txenv(
        tx: elements::Transaction,
        utxos: Vec<ElementsUtxo>,
        ix: u32,
        script_cmr: Cmr,
    ) -> TxEnv {
        let mut inp_eng = sha256::Hash::engine();
        let mut output_eng = sha256::Hash::engine();
        // compute the hash
        tx.input.simplicity_hash(&mut inp_eng);
        tx.output.simplicity_hash(&mut output_eng);
        let inputs_hash = sha256::Hash::from_engine(inp_eng);
        let outputs_hash = sha256::Hash::from_engine(output_eng);
        TxEnv {
            tx: tx,
            utxos: utxos,
            ix: ix,
            script_cmr: script_cmr,
            inputs_hash: inputs_hash,
            outputs_hash: outputs_hash,
        }
    }
}
