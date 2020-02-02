extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};

use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    link::LinkMatch
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError,
};

use hdk::holochain_persistence_api::{
    cas::content::Address,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Person {
    name: String
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Paste {
    message: String,
    timestamp: u64,
    author_id: Address
}

pub fn paste_entry_def() -> ValidatingEntryType {
    entry!(
        name: "paste",
        description: "A paste",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Paste> | {
            match validation_data {
                hdk::EntryValidationData::Create{ entry, .. } => {
                    const MAX_LENGTH: usize = 140;
                    if entry.message.len() <= MAX_LENGTH {
                        Ok(())
                    } else {
                        Err("Paste too long".into())
                    }
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

pub fn create_paste(message: String, timestamp: u64) -> ZomeApiResult<Address> {
    let paste = Paste {
        message, 
        timestamp,
        author_id: hdk::AGENT_ADDRESS.clone(),
    };

    let agent_address = hdk::AGENT_ADDRESS.clone().into();
    let entry = Entry::App("paste".into(), paste.into());
    let address = hdk::commit_entry(&entry)?;

    hdk::link_entries(&agent_address, &address, "author_paste", "")?;
    Ok(address)
}

pub fn retrieve_pastes(agent_address: Address) -> ZomeApiResult<Vec<Paste>> {
    hdk::utils::get_links_and_load_type(
        &agent_address,
        LinkMatch::Exactly("author_paste"),
        LinkMatch::Any,
    )
}