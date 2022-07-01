pub fn string_to_u128(difficulty_str: &String) -> u128 {
    std::process::exit(match is_run_app_stringtou128(&difficulty_str) {
        Ok(str) => {
            let diff_str = str.trim().to_lowercase().to_string();
            let diff_digits = diff_str.strip_prefix("0x").unwrap();
            let difficulty = u128::from_str_radix(diff_digits, 16).unwrap();
            let diff_bytes = difficulty.to_le_bytes();

            let de_diff_bytes = diff_bytes;
            let de_diff = u128::from_le_bytes(de_diff_bytes);
            let de_diff_str = format!("0x{de_diff:032x}");
            assert_eq!(diff_str, de_diff_str);
            return difficulty;
        }
        Err(err) => {
            eprintln!("error: {err:?}");
            1
        }
    });
}

fn is_run_app_stringtou128(difficulty_str: &String) -> Result<&str, &str> {
    let l = (&difficulty_str).as_str().len();
    match l {
        34 => Ok(&(difficulty_str).as_str()),
        _ => Err("difficulty length must be 34 char"),
    }
    //Ok(()))
}
