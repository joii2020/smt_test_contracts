// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;
use crate::error::Error;
use cycles_lib;

pub fn main() -> Result<(), Error> {
    if cycles_lib::cycles_c().is_err() {
        return Err(Error::MyError);
    }

    Ok(())
}

