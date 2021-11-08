pub fn fact (num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => fact(num - 1) * num
    }
}