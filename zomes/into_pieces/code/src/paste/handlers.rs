use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        entry::Entry,
    },
    holochain_persistence_api::cas::content::Address,
    prelude::*,
};

use crate::paste::{
    anchor_address,
    Paste,
    PASTE_ENTRY_NAME,
    PASTE_LINK_TYPE,
    ANCHOR_LINK_TYPE
};

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

    let entry = Entry::App(PASTE_ENTRY_NAME.into(), paste.into());
    let address = hdk::commit_entry(&entry)?;

    hdk::link_entries(&hdk::AGENT_ADDRESS, &address, PASTE_LINK_TYPE, "")?;
    Ok(address)
}

pub fn remove(paste_address: &Address) -> ZomeApiResult<Address> {
    // FIXME: add link removal that points to the entry
    // hdk::remove_link(&anchor_address()?, paste_address, PASTE_LINK_TYPE, "")?;
    hdk::remove_entry(paste_address)
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
        LinkMatch::Exactly(ANCHOR_LINK_TYPE),
        LinkMatch::Any,
    )?
    .addresses();

    Ok(addresses)
}

// TODO: return actual pastes not addresses
pub fn get_my_pastes() -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &hdk::AGENT_ADDRESS,
        LinkMatch::Exactly(PASTE_LINK_TYPE),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}
