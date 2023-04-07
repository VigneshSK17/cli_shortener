use serde::{Deserialize, Serialize};

pub fn gen_hash() -> String {
    format!(
        "{}-{}",
        random_word::gen_len(7).expect("Could not generate hash"),
        random_word::gen_len(7).expect("Could not generate hash")
    )
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
