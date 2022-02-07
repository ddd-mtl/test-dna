#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

pub mod callbacks;
pub mod handle;

use hdk::prelude::*;
use crate::handle::*;

//----------------------------------------------------------------------------------------

entry_defs![
   Handle::entry_def(),
   Number::entry_def()
];


/// Zome Function
#[hdk_extern]
pub fn set_handle(new_name: String) -> ExternResult<HeaderHash> {
   let new_handle = Handle::new(new_name.to_string());
   let _new_handle_eh = hash_entry(&new_handle)?;
   let hh = create_entry(&new_handle)?;
   debug!("new handle hh = {:?}", hh);
   return Ok(hh);
}

/// Zome Function
#[hdk_extern]
pub fn set_number(number: u32) -> ExternResult<HeaderHash> {
   let number = Number {
      value: number,
   };
   let _eh = hash_entry(&number)?;
   let hh = create_entry(&number)?;
   debug!("new number hh = {:?}", hh);
   return Ok(hh);
}