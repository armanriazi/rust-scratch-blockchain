pub fn split_space_one_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

pub fn split_comma_wordlist(wordlist: &String) -> Vec<&str> {
    wordlist.split(',').collect()
}

pub fn split_half_len(array: &[u8; 64]) -> (&[u8], &[u8]) {
    let slice: &[u8] = array;

    let (first_half, second_half) = slice.split_at(32);
    // println!(
    //     "first_half.len()={} second_half.len()={}",
    //     first_half.len(),
    //     second_half.len()
    // );
    (&first_half, &second_half)
}
