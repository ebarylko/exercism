use itertools::iproduct;
pub type Coord = (usize, usize);

/// Takes a garden and returns a collection of
/// coordinates in the garden if it is not empty. Returns
/// None otherwise
pub fn gen_all_garden_coords(garden: &[&str]) -> Option<Vec<Coord>> {
    let size_of_row = |gard: &[&str]| -> Option<usize>  {
        gard.get(0).map(|row| row.len())
    };

    let is_not_empty_garden = |&(x, y): &(usize, usize)| -> bool {
        x != 0 && y != 0
    };

    Some(garden.len())
        .zip(size_of_row(garden))
        .filter(|row_and_col_lengths| is_not_empty_garden(row_and_col_lengths))
        .map(|(num_of_rows, num_of_cols)| iproduct!(0..num_of_cols, 0..num_of_rows))
        .map(|positions| positions.collect())

}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    vec![]
    // gen_all_garden_coords(garden).unwrap_or()
}
