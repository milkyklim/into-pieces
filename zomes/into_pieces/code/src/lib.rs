#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};

use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use hdk_proc_macros::zome;

pub mod post;

#[zome]
mod into_pieces_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[zome_fn("hc_public")]
    pub fn hello_holo() -> ZomeApiResult<String> {
        // dummy to check that zome is initialized correctly
        Ok("Hello Holo".into())
    }

    #[zome_fn("hc_public")]
    pub fn get_agent_id() -> ZomeApiResult<Address> {
        Ok(hdk::AGENT_ADDRESS.clone())
    }

    #[entry_def]
    fn post_entry_def() -> ValidatingEntryType {
        post::post_entry_def()
    }

    #[zome_fn("hc_public")]
    pub fn create_post(message: String, timestamp: u64) -> ZomeApiResult<Address> {
        post::create_post(message, timestamp)
    }

    #[zome_fn("hc_public")]
    pub fn retrieve_posts(agent_address: Address) -> ZomeApiResult<Vec<post::Post>> {
        post::retrieve_posts(agent_address)
    }
}
