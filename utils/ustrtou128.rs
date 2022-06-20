pub fn safe_string_to_u128u(input:&str){

    let mut buffer = std::io::BufReader::new(input.as_bytes());
    let mut first= String::new();
    std::io::BufRead::read_line(&mut buffer, &mut first);    
    
}


//let difficulty = e.trim().parse::<u128>()).as_ref().unwrap();     