#![allow(dead_code)]

pub mod exec;
mod frame;

/// Trait for writing various components of
/// Simplicity transactions(Assets, Values) into bit machine.
// FIXME: Consider implementing simplicity encode for all encodable
// things?
pub(crate) trait SimplicityEncodable {
    // write the simplicity encoding of `self` on bitmachine
    // at the current write cursor.
    fn simplicity_encode(self, mac: &mut exec::BitMachine);
}
