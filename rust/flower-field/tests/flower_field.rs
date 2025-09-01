use flower_field::*;

#[test]
fn no_rows() {
    let input = &[];
    let expected: &[&str] = &[];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
fn no_valid_coords_for_empty_garden()  {
    let expected: Option<Vec<ValidCoord>> = None;
    assert_eq!(expected, gen_all_garden_coords(&[]));
}

#[test]
fn no_valid_coords_for_garden_with_no_squares()  {
    let expected: Option<Vec<ValidCoord>> = None;
    assert_eq!(expected, gen_all_garden_coords(&["", ""]));
}

#[test]
fn garden_with_two_rows_and_columns()  {
    let expected: Option<Vec<ValidCoord>> = Some(vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
    assert_eq!(expected, gen_all_garden_coords(&["  ", "  "]));
}

#[test]
fn some_surrounding_squares_not_valid() {
    let expected = vec![(0, 1), (1, 0), (1, 1)];
    assert_eq!(expected,
               gen_coords_of_surrounding_squares(CoordRestriction{row_limit: 1, col_limit: 1},
                                                 &(0, 0)))
}

#[test]
fn when_the_original_coord_is_a_flower() {
    let garden = &["* ", " *"];
    let expected = '*';

    assert_eq!(expected, num_of_flowers_in_surrounding_squares(garden, &(0, 0), &vec![(0, 1), (1, 0), (1, 1)]));
}

#[test]
fn when_the_original_coord_is_not_a_flower() {
    let garden = &["  ", " *"];
    let expected = '1';

    assert_eq!(expected, num_of_flowers_in_surrounding_squares(garden, &(0, 0), &vec![(0, 1), (1, 0), (1, 1)]));
}

#[test]
fn when_the_original_coord_is_not_a_flower_and_there_are_no_surrounding_flowers() {
    let garden = &["  ", "  "];
    let expected = ' ';

    assert_eq!(expected, num_of_flowers_in_surrounding_squares(garden, &(0, 0), &vec![(0, 1), (1, 0), (1, 1)]));
}

#[test]
fn no_columns() {
    let input = &[""];
    let expected = &[""];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
fn no_flowers() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "   ",
        "   ",
        "   ",
    ], &[
        "   ",
        "   ",
        "   ",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
fn garden_full_of_flowers() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "***",
        "***",
        "***",
    ], &[
        "***",
        "***",
        "***",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
fn flower_surrounded_by_spaces() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "   ",
        " * ",
        "   ",
    ], &[
        "111",
        "1*1",
        "111",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
fn space_surrounded_by_flowers() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "***",
        "* *",
        "***",
    ], &[
        "***",
        "*8*",
        "***",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
fn horizontal_line() {
    let input = &[" * * "];
    let expected = &["1*2*1"];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
fn horizontal_line_flowers_at_edges() {
    let input = &["*   *"];
    let expected = &["*1 1*"];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
fn vertical_line() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        " ",
        "*",
        " ",
        "*",
        " ",
    ], &[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
fn vertical_line_flowers_at_edges() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "*",
        " ",
        " ",
        " ",
        "*",
    ], &[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
fn cross() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "  *  ",
        "  *  ",
        "*****",
        "  *  ",
        "  *  ",
    ], &[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
fn large_garden() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        " *  * ",
        "  *   ",
        "    * ",
        "   * *",
        " *  * ",
        "      ",
    ], &[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
