use std::iter::successors;

pub fn is_armstrong_number(num: u32) -> bool {
    successors(Some(num), |&el| Some(el / 10)).
        take_while(|&el| el != 0).
        map(|num: u32| (num % 10).pow(3))
        .sum::<u32>() == num
}
