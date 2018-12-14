#![feature(try_from)]
use std::convert::TryFrom;
#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;
#[macro_use]
extern crate serde_json;

use hdk::holochain_core_types::{    
    hash::HashString,    
    error::HolochainError,    
    entry::Entry,    
    dna::zome::entry_types::Sharing,    
    entry::entry_type::EntryType,    
    json::JsonString,    
    cas::content::Address
};

// see https://developer.holochain.org/api/0.0.2/hdk/ for info on using the hdk library

define_zome! {
    entries: []

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            add_question: {
                inputs: |question: Question|,
                outputs: |result: JsonString|,
                handler: handle_add_person
            }
        }
    }
}
