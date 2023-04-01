pub fn gen_hash() -> String {

    format!(
        "{}-{}",
        random_word::gen_len(7).expect("Could not generate hash"),
        random_word::gen_len(7).expect("Could not generate hash")
    )

}
