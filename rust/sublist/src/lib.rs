#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list, second_list) {
        ([], []) => Comparison::Equal,
        ([], _x) => Comparison::Sublist,
        (_x, []) => Comparison::Superlist,
        (x, y) => Comparison::Equal
    }


}
