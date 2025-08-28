use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Takes two lists, A, B, where A is smaller than B.
/// Returns Unequal if A is not a proper sublist of B
///  and returns Sublist otherwise
fn sublist_or_unequal(a: &[i32], b: &[i32]) -> Comparison {

    let is_not_a_sublist = |coll1: &[i32], coll2: &[i32]| -> bool {
        coll2.windows(coll1.len())
            .filter(|&sublist| sublist == coll1)
            .collect::<Vec<_>>()
            .is_empty()
    };

    if is_not_a_sublist(a, b) {Comparison::Unequal} else {Comparison::Sublist}
}


fn invert_sublist_decision(decision: Comparison) -> Comparison {
    match decision {
        Comparison::Sublist => Comparison::Superlist,
        _ => decision
    }
}

/// Takes two non-empty lists, A, B, and categorizes them as either
/// A being a proper superlist/sublist of B or A and B not being equal
fn categorize_non_empty_lists(fst: &[i32], snd: &[i32]) -> Comparison {
    match fst == snd {
        true => Comparison::Equal,
        _ => match fst.len().cmp(&snd.len()) {
            Ordering::Equal => Comparison::Unequal,
            Ordering::Less => sublist_or_unequal(fst, snd),
            _ => invert_sublist_decision(sublist_or_unequal(snd, fst))
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
