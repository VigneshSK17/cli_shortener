use serde::{Deserialize, Serialize};

pub fn gen_hash() -> String {
    format!(
        "{}-{}",
        random_word::gen_len(7).expect("Could not generate hash"),
        random_word::gen_len(7).expect("Could not generate hash")
    )
}

pub fn is_url(url: &str) -> bool {
    const REGEX: &str = r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)";

    regex::Regex::new(REGEX).unwrap().is_match(url)
}

#[derive(Deserialize, Serialize)]
pub struct CreateLink {
    pub link: String,
}

#[derive(Deserialize, Serialize)]
pub struct Shortcut {
    pub link: String,
    pub hashed_link: String,
}
