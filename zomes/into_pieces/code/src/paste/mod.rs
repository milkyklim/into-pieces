extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::entry_definition::ValidatingEntryType;

use hdk::holochain_core_types::{dna::entry_types::Sharing, entry::Entry};

use hdk::holochain_json_api::{error::JsonError, json::JsonString};

use hdk::holochain_persistence_api::cas::content::Address;

pub mod handlers;
pub mod validation;

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
    author_id: Address,
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
        author_id: Address,
        // reported: bool
    ) -> Self {
        Paste {
            title,
            text,
            language,
            timestamp,
            expiration,
            author_id,
            reported: false
        }
    }

    pub fn from(
        title: String,
        text: String,
        language: String,
        timestamp: u64,
        expiration: u64,
        author_id: Address,
        // reported: bool
    ) -> Self {
        Paste {
            title,
            text,
            language,
            timestamp,
            expiration,
            author_id,
            reported: false
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("paste".into(), self.into())
    }
}

pub fn paste_entry_def() -> ValidatingEntryType {
    entry!(
        name: "paste",
        description: "A piece of text",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Paste> | {
            match validation_data {
                hdk::EntryValidationData::Create { entry, validation_data } => {
                    validation::validate_entry(&entry, &validation_data)
                },
                hdk::EntryValidationData::Delete { old_entry, old_entry_header, validation_data } => {
                    validation::validate_author(&old_entry, &old_entry_header, &validation_data)
                },
                _ => Ok(()),
            }
        },
        links: [
            from!(
                "%agent_id",
                link_type: "author_paste",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}
