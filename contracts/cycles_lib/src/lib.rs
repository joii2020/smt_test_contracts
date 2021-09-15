#![no_std]

extern crate alloc;

use sparse_merkle_tree;
use sparse_merkle_tree::H256;
use hex::decode;
use core::convert::TryInto;
use alloc::vec::Vec;

#[link(name = "smt-test-c-impl", kind = "static")]
extern "C" {
    fn test_verify() -> i32;
}

fn str_to_h256(src: &str) -> H256 {
    let src = decode(src).unwrap();
    assert!(src.len() == 32);
    let data: [u8; 32] = src.try_into().unwrap();
    H256::from(data)
}

fn str_to_vec(src: &str) -> Vec<u8> {
    decode(src).unwrap()
}

pub fn cycles_rust() -> Result<(), i32> {
    let _key = str_to_h256("381dc5391dab099da5e28acd1ad859a051cf18ace804d037f12819c6fbc0e18b");
    let _val = str_to_h256("9158ce9b0e11dd150ba2ae5d55c1db04b1c5986ec626f2e38a93fe8ad0b2923b");
    let _root_hash = str_to_h256("ebe0fab376cd802d364eeb44af20c67a74d6183a33928fead163120ef12e6e06");
    let _proof = str_to_vec(
        "4c4fff51ff322de8a89fe589987f97220cfcb6820bd798b31a0b56ffea221093d35f909e580b00000000000000000000000000000000000000000000000000000000000000");

    let builder = sparse_merkle_tree::SMTBuilder::new();
    let builder = builder.insert(&_key, &_val).unwrap();
    
    let _smt = builder.build().unwrap();
    if _smt.verify(&_root_hash, &_proof).is_err() {
        return Err(5);
    }

    Ok(())
}

pub fn cycles_c() -> Result<(), i32> {
    unsafe {
        let ret = test_verify();
        if ret != 0 {
            return Err(ret)
        }
    }
    Ok(())
}
