// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
//use alloc::{vec, vec::Vec};

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
/*use ckb_std::{
    debug,
    high_level::{load_script, load_tx_hash},
    ckb_types::{bytes::Bytes, prelude::*},
};*/

use crate::error::Error;
use sparse_merkle_tree::{SMTBuilder, SMT, H256};
use hex::decode;
use alloc::vec::Vec;
use alloc::str;
use core::convert::TryInto;

fn str_to_h256(src: &str) -> Result<H256, Error> {
    let src: Vec<u8> = decode(src).unwrap();
    if src.len() != 32 {
        return Err(Error::ParseDataError);
    }
    
    let data: [u8; 32] = src.try_into().unwrap();
    Ok(H256::from(data))
}

fn str_to_vec(src: &str) -> Vec<u8> {
    decode(src).unwrap()
}

fn test_verify() -> Result<(), Error> {
    let key = str_to_h256("381dc5391dab099da5e28acd1ad859a051cf18ace804d037f12819c6fbc0e18b");
    if key.is_err() {
        return Err(Error::ParseDataError);
    }
    let key = key.unwrap_or(H256::from([0u8; 32]));
    let val = str_to_h256("9158ce9b0e11dd150ba2ae5d55c1db04b1c5986ec626f2e38a93fe8ad0b2923b");
    if val.is_err() {
        return Err(Error::ParseDataError);
    }
    let val = val.unwrap_or(H256::from([0u8; 32]));
    let root_hash = str_to_h256("ebe0fab376cd802d364eeb44af20c67a74d6183a33928fead163120ef12e6e06");
    if root_hash.is_err() {
        return Err(Error::ParseDataError);
    }
    let root_hash = root_hash.unwrap_or(H256::from([0u8; 32]));
    let proof = str_to_vec(
        "4c4fff51ff322de8a89fe589987f97220cfcb6820bd798b31a0b56ffea221093d35f909e580b00000000000000000000000000000000000000000000000000000000000000");

    let builder = SMTBuilder::new();
    let builder = builder.insert(&key, &val).unwrap();

    let smt: SMT = builder.build().unwrap();
    if smt.verify(&root_hash, &proof).is_err() {
        return Err(Error::VerifyError);
    }

    Ok(())
}

fn test_verify_fail() -> Result<(), Error> {
    let key = str_to_h256("281dc5391dab099da5e28acd1ad859a051cf18ace804d037f12819c6fbc0e18b");
    if key.is_err() {
        return Err(Error::ParseDataError);
    }
    let key = key.unwrap_or(H256::from([0u8; 32]));
    let val = str_to_h256("9158ce9b0e11dd150ba2ae5d55c1db04b1c5986ec626f2e38a93fe8ad0b2923b");
    if val.is_err() {
        return Err(Error::ParseDataError);
    }
    let val = val.unwrap_or(H256::from([0u8; 32]));
    let root_hash = str_to_h256("ebe0fab376cd802d364eeb44af20c67a74d6183a33928fead163120ef12e6e06");
    if root_hash.is_err() {
        return Err(Error::ParseDataError);
    }
    let root_hash = root_hash.unwrap_or(H256::from([0u8; 32]));
    let proof = str_to_vec(
        "4c4fff51ff322de8a89fe589987f97220cfcb6820bd798b31a0b56ffea221093d35f909e580b00000000000000000000000000000000000000000000000000000000000000");

    let builder = SMTBuilder::new();
    let builder = builder.insert(&key, &val).unwrap();

    let smt: SMT = builder.build().unwrap();
    if smt.verify(&root_hash, &proof).is_ok() {
        return Err(Error::VerifyError);
    }

    Ok(())
}

pub fn main() -> Result<(), Error> {
    let ret = test_verify();
    if ret.is_err() {
        return ret;
    }

    let ret = test_verify_fail();
    if ret.is_err() {
        return ret;
    }

    Ok(())
}

