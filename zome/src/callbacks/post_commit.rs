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
fn post_commit_app(eh: EntryHash, app_type: AppEntryType) -> ExternResult<()> {
   debug!("  - post_commit_app() {:?}",  app_type);
   match app_type.id().into() {
      0 => {
         debug!("post_commit() of Handle ; remoting call with call_remote()");
         let res = call_remote(
            agent_info()?.agent_latest_pubkey,
            zome_info()?.name,
            "set_number".to_string().into(),
            None,
            42,
         )?;
         debug!("post_commit() of Handle ; res = {:?}", res);
      }
      1 => {
         debug!("post_commit() of Real ; remoting call with call()");
         let res = call(
            CallTargetCell::Local,
            zome_info()?.name,
            "set_number".to_string().into(),
            None,
            24,
         )?;
         debug!("post_commit() of Real ; res = {:?}", res);
      }
      2 => {debug!(" post_commit() of Number"); }
      3 => {
         debug!(" post_commit() of Thing");
         let payload = CommitLinkInput {
            eh,
            to: agent_info()?.agent_latest_pubkey,
         };
         let response = call_remote(
            agent_info()?.agent_latest_pubkey,
            zome_info()?.name,
            "commit_link".to_string().into(),
            None,
            payload,
         )?;
         debug!("commit_link() response: {:?}", response);
      }
      _ => {debug!(" !!! unknown entry index: {:?}", app_type.id()); }
   }
   // Done
   Ok(())
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct CommitLinkInput {
   pub eh: EntryHash,
   pub to: AgentPubKey,
}

/// Create & Commit 'Sent' link
/// Return HeaderHash of newly created link
#[hdk_extern]
fn commit_link(input: CommitLinkInput) -> ExternResult<HeaderHash> {
   debug!("commit_link(): {:?} ", input);
   let tag = LinkTag::new([]);
   let hh = create_link(input.eh.clone(), input.eh, tag)?;
   Ok(hh)
}
