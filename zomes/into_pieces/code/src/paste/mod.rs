extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::entry_definition::ValidatingEntryType;

use hdk::{
    holochain_core_types::{
        dna::entry_types::Sharing,
        entry::Entry
    },
    error::ZomeApiResult,
    holochain_persistence_api::cas::content::Address
};

use hdk::holochain_json_api::{error::JsonError, json::JsonString};

pub mod handlers;
pub mod validation;

const PASTE_ENTRY_NAME: &str = "paste";
const PASTE_LINK_TYPE: &str = "paste_link";
const PASTES_ANCHOR_TYPE: &str = "pastes";
const PASTES_ANCHOR_TEXT: &str = "pastes";

const ANCHOR_TYPE: &str = "anchor";
const ANCHOR_LINK_TYPE: &str = "anchor_link";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Paste {
    title: String,
    text: String,
    language: String,
    timestamp: u64,
    // Probably be UNIX end date
    expiration: u64,
    // Probably a privat entry rather than password
    // password: String
    // Probably marked if not directly removed
    reported: bool, // Probably counter signing or smth like that
                    // This one probably will be a list of all links
                    // rather than one link in total
                    // edit_link: String
}

impl Paste {
    // Constructor
    pub fn new(
        title: String,
        text: String,
        language: String,
        timestamp: u64,
        expiration: u64,
        // reported: bool
    ) -> Self {
        Paste {
            title,
            text,
            language,
            timestamp,
            expiration,
            reported: false
        }
    }

    pub fn from(
        title: String,
        text: String,
        language: String,
        timestamp: u64,
        expiration: u64,
        // reported: bool
    ) -> Self {
        Paste {
            title,
            text,
            language,
            timestamp,
            expiration,
            reported: false
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App(PASTE_ENTRY_NAME.into(), self.into())
    }
}

pub fn paste_entry_def() -> ValidatingEntryType {
    entry!(
        name: PASTE_ENTRY_NAME,
        description: "A piece of text",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Paste> | {
            match validation_data {
                hdk::EntryValidationData::Create { entry, validation_data } => {
                    validation::validate_entry_create(&entry, &validation_data)
                },
                hdk::EntryValidationData::Modify { new_entry, old_entry, old_entry_header, validation_data } => {
                    validation::validate_entry_update(&new_entry, &old_entry, &old_entry_header, &validation_data)
                },
                hdk::EntryValidationData::Delete { old_entry, old_entry_header, validation_data } => {
                    validation::validate_entry_delete(&old_entry, &old_entry_header, &validation_data)
                }
            }
        },
        links: [
            from!(
                "%agent_id",
                link_type: PASTE_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | validation_data: hdk::LinkValidationData | {
                    match validation_data {
                        hdk::LinkValidationData::LinkAdd{link, validation_data} => {
                            validation::validate_link_add(&link, &validation_data)
                        },
                        hdk::LinkValidationData::LinkRemove{link, validation_data} => {
                            validation::validate_link_remove(&link, &validation_data)
                        }
                    }
                }
            )
        ]
    )
}

pub fn anchor_address() -> ZomeApiResult<Address> {
    hdk::entry_address(&anchor_entry())
}

pub fn anchor_entry() -> Entry {
    Entry::App(PASTES_ANCHOR_TYPE.into(), PASTES_ANCHOR_TEXT.into())
}

pub fn anchor_entry_def() -> ValidatingEntryType {
    entry!(
        name: ANCHOR_TYPE,
        description: "Anchor to all pastes",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<String>| {
            Ok(())
        },
        links: [
            to!(
                PASTE_ENTRY_NAME,
                link_type: ANCHOR_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
