use crate::paste::Paste;

pub fn validate_entry(paste: &Paste) -> Result<(), String> {
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

pub fn validate_author(paste: &Paste) -> Result<(), String> {
    let agent_address: String = hdk::AGENT_ADDRESS.to_string();
    let paste_author: String = paste.author_id.to_string();

    if agent_address == paste_author {
        Ok(())
    }
    else {
        Err(format!("Author and current agent id don't match: {} != {}", paste_author, agent_address))
    }
}
