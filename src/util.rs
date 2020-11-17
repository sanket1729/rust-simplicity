// Rust Bitcoin Library
// Written by
//   The Rust Bitcoin developers
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

// utility functions from rust-bitcoin
// Fix these after bitcoin_num crate is released
macro_rules! define_slice_to_be {
    ($name: ident, $type: ty) => {
        #[inline]
        pub fn $name(slice: &[u8]) -> $type {
            assert_eq!(slice.len(), ::std::mem::size_of::<$type>());
            let mut res = 0;
            for i in 0..::std::mem::size_of::<$type>() {
                res |= (slice[i] as $type) << (::std::mem::size_of::<$type>() - i - 1) * 8;
            }
            res
        }
    };
}

macro_rules! define_be_to_array {
    ($name: ident, $type: ty, $byte_len: expr) => {
        #[inline]
        pub fn $name(val: $type) -> [u8; $byte_len] {
            assert_eq!(::std::mem::size_of::<$type>(), $byte_len); // size_of isn't a constfn in 1.22
            let mut res = [0; $byte_len];
            for i in 0..$byte_len {
                res[i] = ((val >> ($byte_len - i - 1) * 8) & 0xff) as u8;
            }
            res
        }
    };
}

define_slice_to_be!(slice_to_u64_be, u64);
define_slice_to_be!(slice_to_u32_be, u32);
define_be_to_array!(u64_to_array_be, u64, 8);

// handy function for converting bit vector to vec[u8]
// # PANIC:
// panics when bitvec length is not a multiple of 8.
pub(crate) fn bitvec_to_bytevec(bitvec: &[bool]) -> Vec<u8> {
    let mut ret = vec![];
    assert!(bitvec.len() % 8 == 0, "Bitvec len must be multiple of 8");
    let mut start = 0;
    while start < bitvec.len() {
        //read a byte
        let mut byte: u8 = 0;
        for i in 0..8 {
            byte += (bitvec[start + i] as u8) * (1u8 << (7 - i));
        }
        ret.push(byte);
        start += 8;
    }
    ret
}
