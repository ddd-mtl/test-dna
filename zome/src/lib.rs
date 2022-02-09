#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

pub mod callbacks;
pub mod entries;

use hdk::prelude::*;
use crate::entries::*;

//----------------------------------------------------------------------------------------



/// Zome Function
#[hdk_extern]
pub fn set_handle(new_name: String) -> ExternResult<EntryHash> {
   let new_handle = Handle::new(new_name.to_string());
   let new_handle_eh = hash_entry(&new_handle)?;
   let hh = create_entry(&new_handle)?;
   debug!("new Handle hh = {:?}", hh);
   return Ok(new_handle_eh);
}

/// Zome Function
#[hdk_extern]
pub fn set_number(value: u32) -> ExternResult<EntryHash> {
   let number = Number {
      value,
   };
   let eh = hash_entry(&number)?;
   let hh = create_entry(&number)?;
   debug!("new Number hh = {:?}", hh);
   return Ok(eh);
}

/// Zome Function
#[hdk_extern]
pub fn set_real(value: f32) -> ExternResult<HeaderHash> {
   let real = Real {
      value,
   };
   let _eh = hash_entry(&real)?;
   let hh = create_entry(&real)?;
   debug!("new Real hh = {:?}", hh);
   return Ok(hh);
}

/// Zome Function
#[hdk_extern]
pub fn set_thing(value: u32) -> ExternResult<HeaderHash> {
   let number = Thing {
      value,
   };
   let _eh = hash_entry(&number)?;
   let hh = create_entry(&number)?;
   debug!("new Thing hh = {:?}", hh);
   return Ok(hh);
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitLinkInput {
   pub base: EntryHash,
   pub target: EntryHash,
}

/// Create & Commit 'Sent' link
/// Return HeaderHash of newly created link
#[hdk_extern]
fn commit_link(input: CommitLinkInput) -> ExternResult<HeaderHash> {
   debug!("commit_link(): {:?} ", input);
   let tag = LinkTag::new([]);
   let hh = create_link(input.base.clone(), input.target, tag)?;
   Ok(hh)
}
