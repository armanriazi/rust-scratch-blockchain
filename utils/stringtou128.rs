pub fn string_to_u128 (difficulty_str:&String) -> u128 {
    let diff_str = difficulty_str.trim().to_lowercase().to_string();
    let diff_digits = diff_str.strip_prefix("0x").unwrap();
    let difficulty = u128::from_str_radix(diff_digits, 16).unwrap();
    let diff_bytes = difficulty.to_le_bytes();

    let de_diff_bytes = diff_bytes;
    let de_diff = u128::from_le_bytes(de_diff_bytes);
    let de_diff_str = format!("0x{de_diff:032x}");
    assert_eq!(diff_str, de_diff_str);
difficulty
}