use std::time::SystemTime;
use holochain::sweettest::*;
use holochain::conductor::{
   ConductorHandle,
};
use maplit::hashset;
use tokio::time::{sleep, Duration};
use holo_hash::*;

use crate::setup::*;



///
pub async fn test(arg: String) {
   let now = SystemTime::now();

   // Admin API test
   if arg == "" {
      test_list_apps().await;
   }
   // Handle
   if arg == "all" || arg == "call_remote" {
      test_handle().await;
   }
   // Real
   if arg == "all" || arg == "call" {
      test_real().await;
   }

   // Print elapsed
   match now.elapsed() {
      Ok(elapsed) => {
         // it prints '2'
         println!("\n *** Test(s) duration: {} secs", elapsed.as_secs());
      }
      Err(e) => {
         // an error occurred!
         println!("Error: {:?}", e);
      }
   }
}


///
pub async fn test_handle() {
   let now = SystemTime::now();
   let (mut conductor, alex, cell1) = setup_1_conductor().await;

   if let Ok(elapsed) = now.elapsed() {
      println!("\n *** Setup duration: {} secs\n\n", elapsed.as_secs());
   }

   let name = "alex";
   println!("*** Calling set_handle()");
   let handle_address1: HeaderHash = conductor.call(&cell1.zome("testground"), "set_handle", name.to_string()).await;
   println!("handle_address1: {:?}", handle_address1);

   print_chain(&conductor, &alex, &cell1).await;

   /// need a sleep before shutdown otherwise post_commit() will return with
   /// `NetworkError("Other: GhostActorDisconnected")`
   sleep(Duration::from_millis(500)).await;

   print_chain(&conductor, &alex, &cell1).await;

   /// Shutdown will not wait for post_commit() to finish :(
   conductor.shutdown().await;
}


///
pub async fn test_real() {
   let now = SystemTime::now();
   let (mut conductor, alex, cell1) = setup_1_conductor().await;

   if let Ok(elapsed) = now.elapsed() {
      println!("\n *** Setup duration: {} secs\n\n", elapsed.as_secs());
   }

   println!("*** Calling set_real()");
   let handle_address1: HeaderHash = conductor.call(&cell1.zome("testground"), "set_real", 42.0).await;
   println!("handle_address1: {:?}", handle_address1);

   print_chain(&conductor, &alex, &cell1).await;

   /// need a sleep before shutdown otherwise post_commit() will return with
   /// `NetworkError("Other: GhostActorDisconnected")`
   sleep(Duration::from_millis(500)).await;

   print_chain(&conductor, &alex, &cell1).await;

   /// Shutdown will not wait for post_commit() to finish :(
   conductor.shutdown().await;
}


///
pub async fn test_list_apps() {
   //observability::test_run().ok();

   println!("Loading DNA...");
   let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
      .await
      .unwrap();

   println!("INSTALLING TWO APPS...");
   // Install two apps on the Conductor:
   // Both share a CellId in common, and also include a distinct CellId each.
   let mut conductor = SweetConductor::from_standard_config().await;
   let alex = SweetAgents::one(conductor.keystore()).await;
   let app1 = conductor
      .setup_app_for_agent("app1", alex.clone(), &[dna.clone()])
      .await
      .unwrap();
   let _app2 = conductor
      .setup_app_for_agent("app2", alex.clone(), &[dna])
      .await
      .unwrap();

   let cell1 = app1.into_cells()[0].clone();

   println!("\n LIST RUNNING APPS...");
   let list_apps = |conductor: ConductorHandle, cell: SweetCell| async move {
      conductor
         .list_running_apps_for_required_cell_id(cell.cell_id())
         .await
         .unwrap()
   };
   let res = list_apps(conductor.clone(), cell1.clone()).await;
   println!("list_apps = {:?}", res);

   // - Ensure that the first CellId is associated with both apps,
   //   and the other two are only associated with one app each.
   assert_eq!(res, hashset!["app1".to_string(), "app2".to_string()]);
}

