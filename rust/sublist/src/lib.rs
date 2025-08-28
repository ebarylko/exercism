use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Takes two lists, a, b, where a is smaller than b
/// and returns Unequal if a is not a proper sublist of b.
/// Returns Sublist otherwise
fn is_proper_sublist_of(a: &[i32], b: &[i32]) -> Comparison {
    Some(b)
        .map(|coll| coll.windows(a.len()))
        .filter(|coll| coll.clone().any(|sub_list| sub_list == a))
        .map(|_| Comparison::Sublist)
        .unwrap_or(Comparison::Unequal)
}

fn categorize_non_empty_lists(fst: &[i32], snd: &[i32]) -> Comparison {
    match fst == snd {
        true => Comparison::Equal,
        _ => match fst.len().cmp(&snd.len()) {
            Ordering::Equal => Comparison::Unequal,
            Ordering::Less => is_proper_sublist_of(fst, snd),
            _ => todo!("Need to implement this")
        }
    }
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list, second_list) {
        ([], []) => Comparison::Equal,
        ([], _x) => Comparison::Sublist,
        (_x, []) => Comparison::Superlist,
        _ => categorize_non_empty_lists(first_list, second_list)
    }


}
