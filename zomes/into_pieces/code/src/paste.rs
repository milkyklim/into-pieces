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


fn validate_entry(paste: &Paste) -> Result<(), String> {
    // TODO: verify that this one is correct
    validate_title(&paste.title).and_then(|_| validate_text(&paste.text))
}

fn validate_title(title: &str) -> Result<(), String> {
    const MAX_TITLE_LENGTH: usize = 50;
    const INFO_TEXT: &str = &"Symbols in title above";

    check_length(title, MAX_TITLE_LENGTH, INFO_TEXT)
}

fn validate_text(text: &str) -> Result<(), String> {
    const MAX_TEXT_LENGTH: usize = 1024;
    const INFO_TEXT: &str = &"Symbols in text above";

    check_length(text, MAX_TEXT_LENGTH, INFO_TEXT)
}

fn check_length(s: &str, max_length: usize, info_text: &str) -> Result<(), String> {
    match s.len() < max_length {
        true => Ok(()),
        false => Err(format!("{} {}", info_text, max_length).to_string()),
    }
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
                    validate_entry(&entry)
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

pub fn remove_paste(paste_address: Address) -> ZomeApiResult<Address> {
    hdk::remove_entry(&paste_address)
}

pub fn update_paste(
    paste_address: &Address,
    title: String,
    text: String,
    language: String,
    timestamp: u64,
    expiration: u64,
) -> ZomeApiResult<Address> {
    // let paste: Paste = hdk::utils::get_as_type(paste_address.clone())?;

    let new_version_paste = Paste::from(
        title,
        text,
        language,
        timestamp,
        expiration,
        hdk::AGENT_ADDRESS.clone() // TODO: check this one
    );
    let new_version_paste_entry = new_version_paste.entry();

    hdk::update_entry(new_version_paste_entry, paste_address)
}

pub fn retrieve_pastes(agent_address: Address) -> ZomeApiResult<Vec<Paste>> {
    hdk::utils::get_links_and_load_type(
        &agent_address,
        LinkMatch::Exactly("author_paste"),
        LinkMatch::Any,
    )
}

pub fn anchor_entry() -> Entry {
    Entry::App("anchor".into(), "paste".into())
}

pub fn anchor_address() -> ZomeApiResult<Address> {
    hdk::entry_address(&anchor_entry())
}

pub fn list() -> ZomeApiResult<Vec<Address>> {
    let addresses = hdk::get_links(
        &anchor_address()?,
        LinkMatch::Exactly("paste_list"),
        LinkMatch::Any,
    )?
    .addresses();

    Ok(addresses)
}

pub fn get_my_pastes() -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &hdk::AGENT_ADDRESS,
        LinkMatch::Exactly("author_paste"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}


pub fn anchor_entry_def() -> ValidatingEntryType {
    entry!(
        name: "anchor", 
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
                "paste",
                link_type: "paste_list",
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