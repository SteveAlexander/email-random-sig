/*
    NEXT

    - write to private project on github
    - make the quotes parser try both formats for any file offered, return vec of quotes
    - various small test cases for quotes parser
    - better naming for quotes parser implementations
    - nice error message if parser fails
    - make command line load up file(s) on offer, make quotes unique, and update Mail

 */

pub mod signatures;

use crate::signatures::{
    add_fixed_signature, add_signature, add_signatures, erase_all_signatures, Signature,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let prefix = "Autoquote: ";
    erase_all_signatures("")?;
    add_fixed_signature()?;
    add_signature(&(prefix.to_owned() + "sig 2"), "sig 2 content")?;
    erase_all_signatures(&prefix)?;
    add_signature("sig 3", "sig 3 content")?;
    add_signatures(
        &prefix,
        vec![
            Signature::new("sig 4", "content 4"),
            Signature::new("sig 5", "content 5"),
            Signature::new("sig 6", "content 6"),
        ],
    )?;
    Ok(())
}
