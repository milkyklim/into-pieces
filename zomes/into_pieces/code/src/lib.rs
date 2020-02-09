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
use hdk::holochain_core_types::entry::Entry;

use hdk_proc_macros::zome;

pub mod paste;
use paste::Paste;

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
    fn get_my_address() -> ZomeApiResult<Address> {
      Ok(hdk::AGENT_ADDRESS.clone())
    }

    #[entry_def]
    fn anchor_entry_def() -> ValidatingEntryType {
        paste::anchor_entry_def()
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
        paste::create(title, text, language, timestamp, expiration)
    }

    #[zome_fn("hc_public")]
    fn get_paste(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

    #[zome_fn("hc_public")]
    fn update_paste(
        paste_address: Address,
        title: String,
        text: String,
        language: String,
        timestamp: u64,
        expiration: u64,
    ) -> ZomeApiResult<Address> {
        paste::update(&paste_address, title, text, language, timestamp, expiration)
    }

    #[zome_fn("hc_public")]
    fn remove_paste(paste_address: Address) -> ZomeApiResult<Address> {
        paste::remove(paste_address)
    }

    // TODO: this one is questionable; seems unnecessary
    #[zome_fn("hc_public")]
    fn get_all_pastes() -> ZomeApiResult<Vec<Address>> {
      paste::list()
    }

    #[zome_fn("hc_public")]
    fn get_my_pastes() -> ZomeApiResult<Vec<Address>> {
      paste::get_my_pastes()
    }

    #[zome_fn("hc_public")]
    pub fn retrieve_pastes(agent_address: Address) -> ZomeApiResult<Vec<Paste>> {
        paste::retrieve_pastes(agent_address)
    }
}
