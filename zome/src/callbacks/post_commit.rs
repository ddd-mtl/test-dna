use hdk::prelude::*;

//use crate::strum::AsStaticRef;

/// Zome Callback
#[hdk_extern(infallible)]
fn post_commit(signedHeaderList: Vec<SignedHeaderHashed>) {
   //debug!("post_commit() called: {:?}", hhList);
   debug!("post_commit() call - START");
   for signedHeader in signedHeaderList {
      //debug!(" - {:?}", signedHeader.header().entry_type());
      let header = signedHeader.header();

      //let hash = signedHeader.as_hash().get_raw_39();
      //let hash64 = format!("u{}", base64::encode_config(hash, base64::URL_SAFE_NO_PAD));
      // debug!(" - {} ({:?})", hash64, signedHeader.header().entry_type());

      if header.entry_type().is_none() {
         continue;
      }
      let (entry_hash, entry_type) = header.entry_data().unwrap();

      match entry_type {
         EntryType::AgentPubKey => {},
         EntryType::CapClaim => {},
         EntryType::CapGrant => {},
         EntryType::App(app_type) => {
            let res = post_commit_app(entry_hash.clone(), app_type.clone());
            if let Err(e) = res {
               error!("post_commit() error: {:?}", e);
            }
         },
      }
   }
   debug!("post_commit() call - END");
}


/**
 *
 */
fn post_commit_app(_eh: EntryHash, app_type: AppEntryType) -> ExternResult<()> {
   debug!("  - post_commit_app() {:?}",  app_type);
   match app_type.id().into() {
      0 => {
         debug!("post_commit() of Handle ; remoting call to create random number");
         let res = call_remote(
            agent_info()?.agent_latest_pubkey,
            zome_info()?.name,
            "set_number".to_string().into(),
            None,
            42,
         )?;
         debug!("post_commit() of Handle ; res = {:?}", res);
      }
      1 => {debug!(" post_commit() of Number"); }
      _ => {debug!(" !!! unknown entry index: {:?}", app_type.id()); }
   }
   // Done
   Ok(())
}

