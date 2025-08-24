use std::iter::successors;

fn num_of_digits(num: u32) -> u32 {
    num.to_string().len() as u32
}

pub fn is_armstrong_number(num: u32) -> bool {
    successors(Some(num), |&el| Some(el / 10)).
        take_while(|&el| el != 0).
        map(|n: u32| (n % 10).pow(num_of_digits(num)))
        .sum::<u32>() == num
}
