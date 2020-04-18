use hdk::holochain_core_types::{chain_header::ChainHeader, link::link_data::LinkData};

use crate::paste::Paste;

// TODO: update _validation_data check
pub fn validate_entry_create(entry: &Paste, _validation_data: &hdk::ValidationData) -> Result<(), String> {
    validate_title(&entry.title)?;
    validate_text(&entry.text)?;
    validate_language(&entry.language)?;
    Ok(())
}

pub fn validate_entry_update(_new_entry: &Paste, _old_entry: &Paste, _old_entry_header: &ChainHeader, _validation_data: &hdk::ValidationData) -> Result<(), String> {
    // we don't need author validation, cause we essentially create fork
    // validate_author(old_entry_header, validation_data)?;
    Ok(())
}

pub fn validate_entry_delete(_old_entry: &Paste, old_entry_header: &ChainHeader, validation_data: &hdk::ValidationData) -> Result<(), String> {
    validate_author(old_entry_header, validation_data)?;
    Ok(())
}

pub fn validate_link_add(_link: &LinkData, _validation_data: &hdk::ValidationData) -> Result<(), String> {
    // TODO:
    Ok(())
}

pub fn validate_link_remove(_link: &LinkData, _validation_data: &hdk::ValidationData) -> Result<(), String> {
    // TODO:
    Ok(())
}

pub fn validate_title(title: &str) -> Result<(), String> {
    const MAX_TITLE_LENGTH: usize = 50;
    const INFO_TEXT: &str = &"Symbols in title above";

    check_length(title, MAX_TITLE_LENGTH, INFO_TEXT)
}

pub fn validate_text(text: &str) -> Result<(), String> {
    const MAX_TEXT_LENGTH: usize = 1024;
    const INFO_TEXT: &str = &"Symbols in text above";

    check_length(text, MAX_TEXT_LENGTH, INFO_TEXT)
}

pub fn validate_language(_language: &str) -> Result<(), String> {
    // TODO:
    Ok(())
}

pub fn check_length(s: &str, max_length: usize, info_text: &str) -> Result<(), String> {
    if s.len() < max_length {
        Ok(())
    }
    else {
        Err(format!("{} {}", info_text, max_length))
    }
}

pub fn validate_author(old_entry_header: &ChainHeader, validation_data: &hdk::ValidationData) -> Result<(), String> {
    if let (Some(o), Some(p)) = (old_entry_header.provenances().get(0), validation_data.package.chain_header.provenances().get(0)) {
        if o.source() == p.source() {
          Ok(())
        }
        else {
          Err("Agent who did not author is trying to call function".to_string())
        }
    }
    else {
      Err("No provenance on this validation_data".to_string())
    }
}