use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        entry::Entry,
    },
    holochain_persistence_api::cas::content::Address,
    prelude::*,
};

use crate::paste::Paste;

pub fn anchor_address() -> ZomeApiResult<Address> {
    hdk::entry_address(&anchor_entry())
}

pub fn anchor_entry() -> Entry {
    Entry::App("anchor".into(), "paste".into())
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

pub fn create(
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
        reported: false,
    };

    let entry = Entry::App("paste".into(), paste.into());
    let address = hdk::commit_entry(&entry)?;

    hdk::link_entries(&hdk::AGENT_ADDRESS, &address, "author_paste", "")?;
    Ok(address)
}

pub fn remove(paste_address: &Address) -> ZomeApiResult<Address> {
    hdk::remove_entry(&paste_address)
}

pub fn update(
    paste_address: &Address,
    title: String,
    text: String,
    language: String,
    timestamp: u64,
    expiration: u64,
) -> ZomeApiResult<Address> {
    let new_version_paste = Paste::from(
        title,
        text,
        language,
        timestamp,
        expiration
    );
    let new_version_paste_entry = new_version_paste.entry();

    hdk::update_entry(new_version_paste_entry, paste_address)
}

// TODO: return actual pastes not addresses
pub fn get_all_pastes() -> ZomeApiResult<Vec<Address>> {
    let addresses = hdk::get_links(
        &anchor_address()?,
        LinkMatch::Exactly("paste_list"),
        LinkMatch::Any,
    )?
    .addresses();

    Ok(addresses)
}

// TODO: return actual pastes not addresses
pub fn get_my_pastes() -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &hdk::AGENT_ADDRESS,
        LinkMatch::Exactly("author_paste"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}
