fn main() {
    let life  = "生";
    let death = "死";
    let life_bytes  = life.as_bytes();
    let death_bytes = death.as_bytes();
    let mut result_bytes = Vec::<u8>::new();
    for i in 0..life_bytes.len()-1 {
        result_bytes.push(life_bytes[i] & death_bytes[i]);
    }
    println!("{}", std::string::String::from_utf8_lossy(&result_bytes));
}

