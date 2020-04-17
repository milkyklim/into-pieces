use hdk::holochain_core_types::{chain_header::ChainHeader};

use crate::paste::Paste;

// TODO: update _validation_data check
pub fn validate_entry(paste: &Paste, _validation_data: &hdk::ValidationData) -> Result<(), String> {
    validate_title(&paste.title).and_then(|_| validate_title(&paste.title))?;
    validate_text(&paste.text).and_then(|_| validate_text(&paste.text))?;
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

pub fn check_length(s: &str, max_length: usize, info_text: &str) -> Result<(), String> {
    if s.len() < max_length {
        Ok(())
    }
    else {
        Err(format!("{} {}", info_text, max_length))
    }
}

pub fn validate_author(_paste: &Paste, old_entry_header: &ChainHeader, validation_data: &hdk::ValidationData) -> Result<(), String> {
    // TODO: should we check that paste_author is the same as in provenances()?
    // let paste_author: String = paste.author_id.to_string();

    if let (Some(o), Some(p)) = (old_entry_header.provenances().get(0), validation_data.package.chain_header.provenances().get(0)) {
        if o.source() == p.source() {
          Ok(())
        }
        else {
          Err("Agent who did not author is trying to delete".to_string())
        }
    }
    else {
      Err("No provenance on this validation_data".to_string())
    }
}