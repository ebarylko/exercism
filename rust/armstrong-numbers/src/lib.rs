fn num_of_digits(num: u32) -> u32 {
    num.to_string().len() as u32
}

pub fn is_armstrong_number(num: u32) -> bool {
    num.
        to_string().
        chars().
        filter_map(|c| c.to_digit(10)).
        map(|n: u32| n.pow(num_of_digits(num))).
        sum::<u32>() == num
}
