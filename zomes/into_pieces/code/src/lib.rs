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

use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

pub mod paste;

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
    fn paste_entry_def() -> ValidatingEntryType {
        paste::paste_entry_def()
    }

    #[zome_fn("hc_public")]
    pub fn create_paste(
        title: String,
        text: String,
        language: String,
        timestamp: u64,
        expiration: u64,
    ) -> ZomeApiResult<Address> {
        paste::create_paste(title, text, language, timestamp, expiration)
    }

    #[zome_fn("hc_public")]
    pub fn retrieve_pastes(agent_address: Address) -> ZomeApiResult<Vec<paste::Paste>> {
        paste::retrieve_pastes(agent_address)
    }
}
