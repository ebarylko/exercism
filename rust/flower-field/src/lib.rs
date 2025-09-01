use itertools::iproduct;
pub type ValidCoord = (usize, usize);

/// Takes a garden and returns a collection of
/// coordinates in the garden if it is not empty. Returns
/// None otherwise
pub fn gen_all_garden_coords(garden: &[&str]) -> Option<Vec<ValidCoord>> {
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

fn is_non_negative(num: i32) -> bool {
    num >= 0
}

// This data type represents a coordinate that may or may not be valid,
// i.e, those whose x or y coordinates exceed the size of the garden or are negative
type PossibleCoord = (i32, i32);

// This data type represents the bounds of a row/column position for a square in the garden

#[derive(Clone, Copy)]
pub struct CoordRestriction {
    pub row_limit: usize,
    pub col_limit: usize,
}

/// Takes a position, a bound on the size of the
/// position and returns true if the position is
/// in the range [0, pos_limit]
fn is_valid_pos(pos: i32, pos_limit: usize) -> bool {
    is_non_negative(pos) && (0..=pos_limit).contains(&(pos as usize))

}

// This data type represents the state of position prior
// to converting it to a ValidCoord if possible given the current restriction of coord_limit
struct IntermediateCoordConversionAttempt {
    coord_limits: CoordRestriction,
    position: PossibleCoord
}


impl TryFrom<IntermediateCoordConversionAttempt> for ValidCoord {
    type Error = ();
    fn try_from(coord_info: IntermediateCoordConversionAttempt) -> Result<Self, Self::Error> {
        let (row_pos, col_pos) = coord_info.position;
        if is_valid_pos(row_pos, coord_info.coord_limits.row_limit) && is_valid_pos(col_pos, coord_info.coord_limits.col_limit)
        {Ok((row_pos as usize, col_pos as usize))}
        else {Err(())}
    }
}

/// Takes a restriction on what coordinates are valid, a position, and
/// returns the valid surrounding coordinates
pub fn gen_coords_of_surrounding_squares(pos_limit: CoordRestriction, (x, y): ValidCoord) -> Vec<ValidCoord> {
    let (tmp_x, tmp_y): PossibleCoord = (x as i32, y as i32);
    iproduct!(-1..2, -1..2)
        .filter(|&(x, y)| x != 0 || y != 0)
        .map(|(row_shift, col_shift)| (tmp_x + row_shift, tmp_y + col_shift))
        .filter_map(|coord: PossibleCoord| ValidCoord::try_from(IntermediateCoordConversionAttempt {position: coord, coord_limits: pos_limit}).ok())
        .collect()
}

/// Takes a garden, a coordinate, the surrounding squares, and
/// returns the number of flowers in the surrounding squares
/// if the original coordinate is not a flower. Otherwise, it returns '*'
pub fn num_of_flowers_in_surrounding_squares(garden: &[&str], orig_coord: &ValidCoord, surrounding_squares: &Vec<ValidCoord>) -> char {
    let get_square_content = |&(row, col): &ValidCoord, garden: &[&str]| -> char {
        garden.get(row).and_then(|row| row.chars().nth(col)).unwrap()
    };

    let is_not_flower = |pos: &ValidCoord, garden: &[&str]| -> bool {
        get_square_content(pos, garden) == ' '
    };

    Some(orig_coord)
        .filter(|&coord| is_not_flower(coord, garden))
        .map(|_| surrounding_squares
            .iter()
            .map(|square| get_square_content(square, garden))
            .filter(|content: &char| content == &'*')
            .count()
            .to_string()
            .chars()
            .nth(0)
            .unwrap())
        .unwrap_or('*')

}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    // gen_all_garden_coords(garden)
    //     .map(|coords| coords.iter().map(|coord| (coord, gen_coords_of_surrounding_squares(coord))))
    //     .map(|(orig_coord, surrounding_squares)| num_of_flowers_in_surrounding_squares(garden, orig_coord, surrounding_squares))
    vec![]
    // gen_all_garden_coords(garden).unwrap_or()
}
