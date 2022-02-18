pub fn is_armstrong_number(num: u32) -> bool {
    let len = num.to_string().len() as u32;
    let mut sum = 0;
    for char in num.to_string().chars() {
        sum += char.to_digit(10).unwrap().pow(len);
    }
    if sum == num {
        return true;
    }
    false
}
