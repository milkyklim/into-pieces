extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};

use hdk::holochain_core_types::{dna::entry_types::Sharing, entry::Entry, link::LinkMatch};

use hdk::holochain_json_api::{error::JsonError, json::JsonString};

use hdk::holochain_persistence_api::cas::content::Address;

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
                    const MAX_LENGTH: usize = 1024;
                    if entry.text.len() <= MAX_LENGTH {
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

// title: String
// text: String,
// language: String,
// timestamp: u64,
// expiration: u64,
// author_id: Address,
// reported: bool

pub fn create_paste(
    title: String,
    text: String,
    language: String,
    timestamp: u64,
    expiration: u64,
) -> ZomeApiResult<Address> {
    let paste = Paste {
        title,
        text,
        language,
        timestamp,
        expiration,
        author_id: hdk::AGENT_ADDRESS.clone(),
        reported: false,
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
