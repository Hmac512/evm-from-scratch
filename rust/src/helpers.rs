use primitive_types::U256;

pub fn concat_decimals(arr: &[u8]) -> U256 {
    let hexadecimal_concat: String = arr
        .iter()
        .map(|x| format!("{:X}", x))
        .collect::<Vec<String>>()
        .join("");

    return U256::from_str_radix(&hexadecimal_concat, 16).unwrap();
}

pub fn convert_twos_compliment(x: U256) -> U256 {
    let mut y = x;
    // We do this by first doing a bitwise negation
    y = !x;
    // Then adding one
    y += U256::one();
    y
}
