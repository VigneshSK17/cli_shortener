use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateLink {
    pub link: String,
}

#[derive(askama_axum::Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub url: String,
}

/// Generates hash for link, roughly 2.5 million permutations
pub fn gen_hash() -> String {
    let word_len_1 = rand::thread_rng().gen_range(5..=7);
    let word_len_2 = rand::thread_rng().gen_range(5..=7);

    format!(
        "{}-{}",
        random_word::gen_len(word_len_1).expect("Could not generate hash"),
        random_word::gen_len(word_len_2).expect("Could not generate hash")
    )
}

pub fn is_url(url: &str) -> bool {
    const REGEX: &str = r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)";
    regex::Regex::new(REGEX).unwrap().is_match(url)
}

#[test]
fn test_is_url() {
    let ok_urls = [
        "http://foo.com/blah_blah",
        "http://foo.com/blah_blah",
        "http://foo.com/blah_blah/",
        "http://foo.com/blah_blah_(wikipedia)",
        "http://foo.com/blah_blah_(wikipedia)_(again)",
        "http://www.example.com/wpstyle/?p=364",
        "https://www.example.com/foo/?bar=baz&inga=42&quux",
        "http://code.google.com/events/#&product=browser",
    ];

    let bad_urls = ["http://", "//a", "foo.com", "h://test"];

    for url in ok_urls.iter() {
        assert_eq!(is_url(url), true);
    }
    for url in bad_urls.iter() {
        assert_eq!(is_url(url), false);
    }
}
