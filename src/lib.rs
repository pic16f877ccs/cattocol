#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links
)]

//! This crate provided various options for combining text.
//! - Without the ansi escpe sequences.
//! - With the ansi escpe sequences.
//! - Along the lines of the first text.
//! - Iteration on non empty lines.
//! # Examples
//!
//! ```
//! # use cattocol::{cat_to_col, CatToCol};
//! let first_txt = String::from("It's a\nit's raining\nnortherly wind.");
//! let second_txt = String::from("beautiful day,\nwith a\n\n");
//! let cattocol = CatToCol::new().fill(' ').repeat(0);
//! let text = "It's a         beautiful day,\nit's raining   with a\nnortherly wind.\n";
//! let concatenated_txt = cattocol.combine_col(&first_txt, &second_txt).collect::<String>();
//!
//! assert_eq!(concatenated_txt, text);
//!
//! println!("{}", concatenated_txt);
//!
//! //It's a         beautiful day,
//! //it's raining   with a
//! //northerly wind.
//!
//! let text = "It's a beautiful day,\nit's raining with a\nnortherly wind. \n";
//! let concatenated_txt = cat_to_col(&first_txt, &second_txt).collect::<String>();
//!
//! assert_eq!(concatenated_txt, text);
//!
//! println!("{}", concatenated_txt);
//!
//! //It's a beautiful day,
//! //it's raining with a
//! //northerly wind.
//! ```

#[doc = include_str!("../README.md")]
use smallstr::SmallString;
use std::cmp::min;
use std::iter;
use strip_ansi_escapes::strip;

impl Default for CatToCol {
    fn default() -> Self {
        Self::new()
    }
}

/// A structure to store the delimiter character and its repetition value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CatToCol {
    fill: SmallString<[u8; 4]>,
    repeat: usize,
}

impl CatToCol {
    /// Create a new CatToCol.
    pub fn new() -> Self {
        Self {
            fill: ' '.into(),
            repeat: 0,
        }
    }

    /// Changes the separator character.
    #[inline]
    pub fn fill(mut self, fill: char) -> Self {
        self.fill = fill.into();
        self
    }

    /// Changes the repetition values.
    #[inline]
    pub fn repeat(mut self, repeat: usize) -> Self {
        self.repeat = repeat;
        self
    }

    /// Combining two texts in columns separated by a character repeated N times.
    ///
    /// - Without the ansi escpe sequences.
    #[inline]
    pub fn combine_col<'a>(
        &'a self,
        str_one: &'a str,
        str_two: &'a str,
    ) -> impl Iterator<Item = &str> {
        let max_line_one = max_line_len(str_one);
        let iter_one = str_one.lines();
        let iter_two = str_two.lines();
        let len_min = min(iter_one.clone().count(), iter_two.clone().count());
        let txt_iter = iter_one.clone().zip(iter_two.clone());

        txt_iter
            .flat_map(move |item| {
                let just_len = max_line_one - max_line_len(item.0);
                iter::once(item.0)
                    .chain(iter::repeat(self.fill.as_str()).take(just_len + self.repeat))
                    .chain(iter::once(item.1))
                    .chain(iter::once("\n"))
            })
            .chain(
                iter_one
                    .skip(len_min)
                    .flat_map(|line| iter::once(line).chain(iter::once("\n"))),
            )
            .chain(iter_two.skip(len_min).flat_map(move |line| {
                iter::repeat(self.fill.as_str())
                    .take(max_line_one + self.repeat)
                    .chain(iter::once(line))
                    .chain(iter::once("\n"))
            }))
    }

    /// Combining two texts in columns separated by a character repeated N times.
    ///
    /// - With the ansi escpe sequences.  
    #[inline]
    pub fn combine_col_esc<'a>(
        &'a self,
        str_one: &'a str,
        str_two: &'a str,
    ) -> impl Iterator<Item = &str> {
        let max_line_one = max_line_len_no_esc(str_one);
        let iter_one = str_one.lines();
        let iter_two = str_two.lines();
        let len_min = min(iter_one.clone().count(), iter_two.clone().count());
        let txt_iter = iter_one.clone().zip(iter_two.clone());

        txt_iter
            .flat_map(move |item| {
                let just_len = max_line_one - max_line_len_no_esc(item.0);
                iter::once(item.0)
                    .chain(iter::repeat(self.fill.as_str()).take(just_len + self.repeat))
                    .chain(iter::once(item.1))
                    .chain(iter::once("\n"))
            })
            .chain(
                iter_one
                    .skip(len_min)
                    .flat_map(|line| iter::once(line).chain(iter::once("\n"))),
            )
            .chain(iter_two.skip(len_min).flat_map(move |line| {
                iter::repeat(self.fill.as_str())
                    .take(max_line_one + self.repeat)
                    .chain(iter::once(line))
                    .chain(iter::once("\n"))
            }))
    }
}

/// Concatenating two texts line by line returns an iterator.
///
/// - Empty lines of the first text are concatenated with spaces.
/// - No lines are ignored.
/// # Examples
///
/// ```
/// use cattocol::cat_to_col;
/// let first_txt = "Combine\ntexts\none\nlinewise.\n\n";
/// let second_txt = "two\ninto\ntext\n";
/// let text = "Combine two\ntexts into\none text\nlinewise.\n\n";
/// let concatenated_txt = cat_to_col(&first_txt, &second_txt).collect::<String>();
///
/// assert_eq!(concatenated_txt, text);
///
/// let first_txt = "Combine\ntexts\none\n";
/// let second_txt = "two\ninto\ntext\nlinewise.\n\n";
/// let text = "Combine two\ntexts into\none text\nlinewise.\n\n";
/// let concatenated_txt = cat_to_col(&first_txt, &second_txt).collect::<String>();
///
/// assert_eq!(concatenated_txt, text);
/// ```
#[inline]
pub fn cat_to_col<'a>(str_one: &'a str, str_two: &'a str) -> impl Iterator<Item = &'a str> {
    let iter_one = str_one.lines();
    let iter_two = str_two.lines();
    let len_min = min(iter_one.clone().count(), iter_two.clone().count());
    let txt_iter = iter_one.clone().zip(iter_two.clone());

    txt_iter
        .flat_map(move |item| {
            iter::once(item.0)
                .chain(iter::once(" "))
                .chain(iter::once(item.1))
                .chain(iter::once("\n"))
        })
        .chain(
            iter_one
                .skip(len_min)
                .flat_map(|line| iter::once(line).chain(iter::once("\n"))),
        )
        .chain(
            iter_two
                .skip(len_min)
                .flat_map(|line| iter::once(line).chain(iter::once("\n"))),
        )
}

/// Concatenating two texts along the lines of the first text returns an iterator.
///
/// - Lines are joined by whitespace.
/// - If the first text ends, the remaining lines of the second text are ignored.
/// - No spaces are inserted before or after empty lines.
/// # Examples
///
/// ```
/// use cattocol::by_lines;
///
/// let first_txt = "One green\nrides down";
/// let second_txt = "brutal tractor\nthe street.";
/// let concatenated_txt = by_lines(first_txt, second_txt).collect::<String>();
///
/// assert_eq!(&concatenated_txt, "One green brutal tractor\nrides down the street.\n");
///
/// let first_txt = "One green\nrides down";
/// let second_txt = "brutal tractor\nthe street.\nThe tractor\nhums and smokes.";
/// let concatenated_txt = by_lines(first_txt, second_txt).collect::<String>();
///
/// assert_eq!(&concatenated_txt, "One green brutal tractor\nrides down the street.\n");
/// ```
#[inline]
pub fn by_lines<'a>(first_str: &'a str, second_str: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    let first_iter = first_str.lines();
    let mut second_iter = second_str.lines();

    first_iter.flat_map(move |first_line| {
        let mut space_take = 0;
        let second_line = if let Some(line) = second_iter.next() {
            if first_line.is_empty() || line.is_empty() {
                space_take = 0
            } else {
                space_take = 1
            };
            line
        } else {
            ""
        }
        .lines();
        iter::once(first_line).chain(
            iter::once(" ")
                .take(space_take)
                .chain(second_line)
                .chain(iter::once("\n")),
        )
    })
}

/// Concatenating two texts by lines parwise returns an iterator.
///
/// - Lines are joined by whitespace.
/// - Unpaired and empty lines are ignored.
/// # Examples
///
/// ```
/// use cattocol::by_pairs;
///
/// let first_txt = "one horsepower\ntwo horsepower\nthree horsepower\nfour horsepower\n";
/// let second_txt = "per horse\ntwo horses\n";
/// let concatenated_txt = by_pairs(first_txt, second_txt).collect::<String>();
///
/// assert_eq!( &concatenated_txt, "one horsepower per horse\ntwo horsepower two horses\n");
///
/// let first_txt = "red apple\ngreen pear\nyellow tomato\npurple eggplant\n";
/// let second_txt = "";
/// let concatenated_txt = by_pairs(first_txt, second_txt).collect::<String>();
///
/// assert_eq!( &concatenated_txt, "");
/// ```
#[inline]
pub fn by_pairs<'a>(first_str: &'a str, second_str: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    let first_iter = first_str.lines();
    let mut second_iter = second_str.lines();

    first_iter.flat_map(move |first_line| {
        let mut takes = 0;
        let second_line = if let Some(line) = second_iter.next() {
            takes = usize::MAX;
            line
        } else {
            ""
        };

        if first_line.is_empty() || second_line.is_empty() {
            takes = 0;
        };

        iter::once(first_line)
            .chain(
                iter::once(" ")
                    .chain(second_line.lines())
                    .chain(iter::once("\n")),
            )
            .take(takes)
    })
}

/// Concatenating three texts along the lines of the first text returns an iterator.
///
/// - Lines are joined by whitespace.
/// - If the first text ends, the remaining lines of the second text are ignored.
/// - No spaces are inserted before or after empty lines.
/// # Examples
///
/// ```
/// use cattocol::by_three_lines;
///
/// let first_txt = "One season\nDecembre,\nIt's cold.\n";
/// let second_txt = "a year\nJanuary,\n";
/// let third_txt = "is winter.\nFebruary.\n";
/// let text = "One season a year is winter.\nDecembre, January, February.\nIt's cold.\n";
/// let concatenated_txt = by_three_lines(first_txt, second_txt, third_txt).collect::<String>();
///
/// assert_eq!( &concatenated_txt, text);
///
/// ```
#[inline]
pub fn by_three_lines<'a>(
    first_str: &'a str,
    second_str: &'a str,
    third_str: &'a str,
) -> impl Iterator<Item = &'a str> {
    let first_iter = first_str.lines();
    let mut second_iter = second_str.lines();
    let mut third_iter = third_str.lines();

    first_iter.flat_map(move |first_line| {
        let mut first_space_take = 0;
        let mut second_space_take = 0;
        let first_line_notempty = !first_line.is_empty();
        let mut second_line_notempty = false;
        let mut second_line = "";
        let mut third_line = "";

        if let Some(line) = second_iter.next() {
            second_line_notempty = !line.is_empty();
            if first_line_notempty && second_line_notempty {
                first_space_take = 1;
            };
            second_line = line;
        }

        if let Some(line) = third_iter.next() {
            if (first_line_notempty || second_line_notempty) && !line.is_empty() {
                second_space_take = 1;
            };
            third_line = line;
        }

        iter::once(first_line)
            .chain(
                iter::once(" ")
                    .take(first_space_take)
                    .chain(second_line.lines()),
            )
            .chain(
                iter::once(" ")
                    .take(second_space_take)
                    .chain(third_line.lines()),
            )
            .chain(iter::once("\n"))
    })
}

/// Concatenating four texts along the lines of the first text returns an iterator.
///
/// - Lines are joined by whitespace.
/// - If the first text ends, the remaining lines of the second text are ignored.
/// - No spaces are inserted before or after empty lines.
/// # Examples
///
/// ```
/// use cattocol::by_four_lines;
///
/// let first_txt = "One in english,\nEin in german,\n";
/// let second_txt = "two in english,\nzwei in german,\n";
/// let third_txt = "three in english,\ndrei in german,\n";
/// let fourth_txt = "four in english.\nvier in german.\n";
/// let text = "One in english, two in english, three in english, four in english.\n\
///     Ein in german, zwei in german, drei in german, vier in german.\n";
/// let concatenated_txt = by_four_lines(first_txt, second_txt, third_txt, fourth_txt).collect::<String>();
///
/// assert_eq!(&concatenated_txt, text);
/// ```
#[inline]
pub fn by_four_lines<'a>(
    first_str: &'a str,
    second_str: &'a str,
    third_str: &'a str,
    fourth_str: &'a str,
) -> impl Iterator<Item = &'a str> {
    let first_iter = first_str.lines();
    let mut second_iter = second_str.lines();
    let mut third_iter = third_str.lines();
    let mut fourth_iter = fourth_str.lines();

    first_iter.flat_map(move |first_line| {
        let mut first_space_take = 0;
        let mut second_space_take = 0;
        let mut third_space_take = 0;
        let first_line_notempty = !first_line.is_empty();
        let mut second_line_notempty = false;
        let mut third_line_notempty = false;
        let mut second_line = "";
        let mut third_line = "";
        let mut fourth_line = "";

        if let Some(line) = second_iter.next() {
            second_line_notempty = !line.is_empty();
            if first_line_notempty && second_line_notempty {
                first_space_take = 1;
            };
            second_line = line;
        }

        if let Some(line) = third_iter.next() {
            third_line_notempty = !line.is_empty();
            if (first_line_notempty || second_line_notempty) && !line.is_empty() {
                second_space_take = 1;
            };
            third_line = line;
        }

        if let Some(line) = fourth_iter.next() {
            if (first_line_notempty || second_line_notempty || third_line_notempty)
                && !line.is_empty()
            {
                third_space_take = 1;
            };
            fourth_line = line;
        }

        iter::once(first_line)
            .chain(
                iter::once(" ")
                    .take(first_space_take)
                    .chain(second_line.lines()),
            )
            .chain(
                iter::once(" ")
                    .take(second_space_take)
                    .chain(third_line.lines()),
            )
            .chain(
                iter::once(" ")
                    .take(third_space_take)
                    .chain(fourth_line.lines()),
            )
            .chain(iter::once("\n"))
    })
}

#[inline]
fn max_line_len(text: &str) -> usize {
    text.lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

#[inline]
fn max_line_len_no_esc(text: &str) -> usize {
    max_line_len(std::str::from_utf8(&strip(text).unwrap()).unwrap())
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cat_one_two_txt() {
        let txt_col = "Combine two texts Returns an iterator\ninto one text from one\nfrom two columns. text of two\nmerged columns.\nCollect to String.\n";
        let txt_one = "Combine two texts\ninto one text\nfrom two columns.";
        let txt_two =
            "Returns an iterator\nfrom one\ntext of two\nmerged columns.\nCollect to String.";
        let texts = cat_to_col(&txt_one, &txt_two).collect::<String>();
        println!("\n{txt_one}");
        println!("\n{txt_two}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn cat_two_one_txt() {
        let txt_col = "Returns an iterator Combine two texts\nfrom one into one text\ntext of two from two columns.\nmerged columns.\nCollect to String.\n";
        let txt_one = "Combine two texts\ninto one text\nfrom two columns.";
        let txt_two =
            "Returns an iterator\nfrom one\ntext of two\nmerged columns.\nCollect to String.";
        let texts = cat_to_col(&txt_two, &txt_one).collect::<String>();
        println!("\n{txt_one}");
        println!("\n{txt_two}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn cat_one_empty_txt() {
        let txt_col = "Combine two texts\ninto one text\nfrom two columns.\n";
        let txt_one = "Combine two texts\ninto one text\nfrom two columns.";
        let texts = cat_to_col(&txt_one, "").collect::<String>();
        println!("\n{txt_one}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn cat_one_empty_line_txt() {
        let txt_col = "Combine two texts\n\ninto one text\nfrom two columns.\n";
        let txt_one = "Combine two texts\n\ninto one text\nfrom two columns.";
        let texts = cat_to_col(&txt_one, "").collect::<String>();
        println!("\n{txt_one}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn cat_one_newline_txt() {
        let txt_col = " Combine two texts\n into one text\nfrom two columns.\n";
        let txt_two = "Combine two texts\ninto one text\nfrom two columns.";
        let texts = cat_to_col("\n\n", &txt_two).collect::<String>();
        println!("\n{txt_two}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn cat_empty_txt() {
        let texts = cat_to_col("", "").collect::<String>();
        assert_eq!(texts, "");
    }

    #[test]
    fn cat_one_space_txt() {
        let txt_col = "Combine two texts  \ninto one text  \nfrom two columns.  \n";
        let txt_one = "Combine two texts\ninto one text\nfrom two columns.";
        let texts = cat_to_col(&txt_one, " \n \n ").collect::<String>();
        println!("\n{txt_one}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn combine_one_two_txt() {
        let cat_to_col = CatToCol::new().fill(' ').repeat(1);
        let txt_col = "Combine two texts Returns an iterator\ninto one text     from one\nfrom two columns. text of two\n                  merged columns.\n                  Collect to String.\n";
        let txt_one = "Combine two texts\ninto one text\nfrom two columns.";
        let txt_two =
            "Returns an iterator\nfrom one\ntext of two\nmerged columns.\nCollect to String.";
        let texts = cat_to_col.combine_col(&txt_one, &txt_two).collect::<String>();
        println!("\n{txt_one}");
        println!("\n{txt_two}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn combine_one_newline_two_txt() {
        let cat_to_col = CatToCol::new().fill(' ').repeat(1);
        let txt_col = "                  Returns an iterator\nfrom two columns. text of two\n                  merged columns.\n                  Collect to String.\n";
        let txt_one = "\nfrom two columns.\n\n";
        let txt_two =
            "Returns an iterator\ntext of two\nmerged columns.\nCollect to String.";
        let texts = cat_to_col.combine_col(&txt_one, &txt_two).collect::<String>();
        println!("\n{txt_one}");
        println!("\n{txt_two}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn combine_two_one_txt() {
        let cat_to_col = CatToCol::new().fill(' ').repeat(1);
        let txt_col = "Returns an iterator Combine two texts\nfrom one            into one text\ntext of two         from two columns.\nmerged columns.\nCollect to String.\n";
        let txt_one = "Combine two texts\ninto one text\nfrom two columns.";
        let txt_two =
            "Returns an iterator\nfrom one\ntext of two\nmerged columns.\nCollect to String.";
        let texts = cat_to_col.combine_col(&txt_two, &txt_one).collect::<String>();
        println!("\n{txt_one}");
        println!("\n{txt_two}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn combine_one_two_repeat_txt() {
        let cat_to_col = CatToCol::new().fill(' ').repeat(10);
        let txt_col = "Combine two texts          Returns an iterator\ninto one text              from one\nfrom two columns.          text of two\n                           merged columns.\n                           Collect to String.\n";
        let txt_one = "Combine two texts\ninto one text\nfrom two columns.";
        let txt_two =
            "Returns an iterator\nfrom one\ntext of two\nmerged columns.\nCollect to String.";
        let texts = cat_to_col.combine_col(&txt_one, &txt_two).collect::<String>();
        println!("\n{txt_one}");
        println!("\n{txt_two}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn combine_one_empty_repeat_txt() {
        let cat_to_col = CatToCol::new().fill(' ').repeat(10);
        let txt_col = "Combine two texts\ninto one text\nfrom two columns.\n";
        let txt_one = "Combine two texts\ninto one text\nfrom two columns.";
        let texts = cat_to_col.combine_col(&txt_one, "").collect::<String>();
        println!("\n{txt_one}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn combine_empty_one_repeat_txt() {
        let cat_to_col = CatToCol::new().fill(' ').repeat(10);
        let txt_col = "          Combine two texts\n          into one text\n          from two columns.\n";
        let txt_one = "Combine two texts\ninto one text\nfrom two columns.";
        let texts = cat_to_col.combine_col("", &txt_one).collect::<String>();
        println!("\n{txt_one}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn combine_one_two_repeat_fill_txt() {
        let cat_to_col = CatToCol::new().repeat(10).fill('╍');
        let txt_col = "Combine two texts╍╍╍╍╍╍╍╍╍╍Returns an iterator\ninto one text╍╍╍╍╍╍╍╍╍╍╍╍╍╍from one\nfrom two columns.╍╍╍╍╍╍╍╍╍╍text of two\n╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍merged columns.\n╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍Collect to String.\n";
        let txt_one = "Combine two texts\ninto one text\nfrom two columns.";
        let txt_two =
            "Returns an iterator\nfrom one\ntext of two\nmerged columns.\nCollect to String.";
        let texts = cat_to_col.combine_col(&txt_one, &txt_two).collect::<String>();
        println!("\n{txt_one}");
        println!("\n{txt_two}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn combine_one_two_repeat_zero_fill_txt() {
        let cat_to_col = CatToCol::new().repeat(0).fill('╍');
        let txt_col = "Combine two textsReturns an iterator\ninto one text╍╍╍╍from one\nfrom two columns.text of two\n╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍merged columns.\n╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍Collect to String.\n";
        let txt_one = "Combine two texts\ninto one text\nfrom two columns.";
        let txt_two =
            "Returns an iterator\nfrom one\ntext of two\nmerged columns.\nCollect to String.";
        let texts = cat_to_col.combine_col(&txt_one, &txt_two).collect::<String>();
        println!("\n{txt_one}");
        println!("\n{txt_two}");
        println!("\n{txt_col}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn combine_esc_one_two_txt() {
        let cat_to_col = CatToCol::new().fill(' ').repeat(1);
        let txt_col = "\x1b[33mCombine\x1b[0m \x1b[36mtwo\x1b[0m texts Returns an iterator\ninto one text     from one\nfrom two columns. text of two\n                  merged columns.\n                  Collect to String.\n";
        let txt_one = "\x1b[33mCombine\x1b[0m \x1b[36mtwo\x1b[0m texts\ninto one text\nfrom two columns.";
        let txt_two =
            "Returns an iterator\nfrom one\ntext of two\nmerged columns.\nCollect to String.";
        let texts = cat_to_col.combine_col_esc(&txt_one, &txt_two).collect::<String>();
        println!("\n{txt_one}");
        println!("\n{txt_two}");
        println!("\n{texts}");
        assert_eq!(texts, txt_col);
    }

    #[test]
    fn test_by_lines_first_gt_second() {
        let iter = by_lines("one\ntwo\nthree\nprimary\nsecondary\n", "first\nsecond\n");
        assert_eq!(
            &iter.collect::<String>(),
            "one first\ntwo second\nthree\nprimary\nsecondary\n"
        );
    }

    #[test]
    fn test_by_lines_first_eq_second() {
        let iter = by_lines("one\ntwo\nthree\n", "first\nsecond\nthird\n");
        assert_eq!(
            &iter.collect::<String>(),
            "one first\ntwo second\nthree third\n"
        );
    }

    #[test]
    fn test_by_lines_first_lt_second() {
        let iter = by_lines("one\ntwo\nthree\n", "first\nsecond\nthird\nfourth\nfifth\n");
        assert_eq!(
            &iter.collect::<String>(),
            "one first\ntwo second\nthree third\n"
        );
    }

    #[test]
    fn test_by_lines_first_empty() {
        let iter = by_lines("", "first\nsecond\nthird\nfourth\nfifth\n");
        assert_eq!(&iter.collect::<String>(), "");
    }

    #[test]
    fn test_by_lines_second_empty() {
        let iter = by_lines("one\ntwo\nthree\n", "");
        assert_eq!(&iter.collect::<String>(), "one\ntwo\nthree\n");
    }

    #[test]
    fn test_by_lines_first_second_empty() {
        let iter = by_lines("", "");
        assert_eq!(&iter.collect::<String>(), "");
    }

    #[test]
    fn test_by_lines_first_second_newline() {
        let iter = by_lines("\n", "\n");
        assert_eq!(&iter.collect::<String>(), "\n");
    }

    #[test]
    fn test_by_lines_first_empty_second_newline() {
        let iter = by_lines("", "\n");
        assert_eq!(&iter.collect::<String>(), "");
    }

    #[test]
    fn test_by_lines_first_newline_second_empty() {
        let iter = by_lines("\n", "");
        assert_eq!(&iter.collect::<String>(), "\n");
    }

    #[test]
    fn test_by_lines_first_two_newline_second_empty() {
        let iter = by_lines("one\n\ntwo\n\nthree\n\n", "");
        assert_eq!(&iter.collect::<String>(), "one\n\ntwo\n\nthree\n\n");
    }

    #[test]
    fn test_by_lines_first_two_newline() {
        let iter = by_lines("one\n\ntwo\n\nthree\n\n", "first\nsecond\n");
        assert_eq!(
            &iter.collect::<String>(),
            "one first\nsecond\ntwo\n\nthree\n\n"
        );
    }

    #[test]
    fn test_by_lines_first_second_two_newline() {
        let iter = by_lines("one\ntwo\n\nfour\n\nsix\n", "first\n\nthird\nfourth\n\n");
        assert_eq!(
            &iter.collect::<String>(),
            "one first\ntwo\nthird\nfour fourth\n\nsix\n"
        );
    }

    #[test]
    fn test_by_lines_first_newline() {
        let iter = by_lines("\n\n", "first\nsecond\n");
        assert_eq!(&iter.collect::<String>(), "first\nsecond\n");
    }

    #[test]
    fn test_by_lines_first_newline_gt_second() {
        let iter = by_lines("\n\n\n\n", "first\nsecond\n");
        assert_eq!(&iter.collect::<String>(), "first\nsecond\n\n\n");
    }

    #[test]
    fn test_by_lines_newlines() {
        let iter = by_lines("\n\n\n\n", "\n\n\n\n");
        assert_eq!(&iter.collect::<String>(), "\n\n\n\n");
    }

    #[test]
    fn test_by_pairs_first_gt_second() {
        let iter = by_pairs("one\ntwo\nthree\nprimary\nsecondary\n", "first\nsecond\n");
        assert_eq!(
            &iter.collect::<String>(),
            "one first\ntwo second\n"
        );
    }

    #[test]
    fn test_by_pairs_first_eq_second() {
        let iter = by_pairs("one\ntwo\nthree\n", "first\nsecond\nthird\n");
        assert_eq!(
            &iter.collect::<String>(),
            "one first\ntwo second\nthree third\n"
        );
    }

    #[test]
    fn test_by_pairs_first_lt_second() {
        let iter = by_pairs("one\ntwo\nthree\n", "first\nsecond\nthird\nfourth\nfifth\n");
        assert_eq!(
            &iter.collect::<String>(),
            "one first\ntwo second\nthree third\n"
        );
    }

    #[test]
    fn test_by_pairs_first_empty() {
        let iter = by_pairs("", "first\nsecond\nthird\nfourth\nfifth\n");
        assert_eq!(&iter.collect::<String>(), "");
    }

    #[test]
    fn test_by_pairs_second_empty() {
        let iter = by_pairs("one\ntwo\nthree\n", "");
        assert_eq!(&iter.collect::<String>(), "");
    }

    #[test]
    fn test_by_pairs_first_second_empty() {
        let iter = by_pairs("", "");
        assert_eq!(&iter.collect::<String>(), "");
    }

    #[test]
    fn test_by_pairs_first_newline() {
        let iter = by_pairs("\n\n", "first\nsecond\n");
        assert_eq!(&iter.collect::<String>(), "");
    }

    #[test]
    fn test_by_pairs_first_newline_gt_second() {
        let iter = by_pairs("one\ntwo\n\n\n", "first\nsecond\n");
        assert_eq!(&iter.collect::<String>(), "one first\ntwo second\n");
    }

    #[test]
    fn test_by_pairs_newlines() {
        let iter = by_pairs("\n\n\n\n", "\n\n\n\n");
        assert_eq!(&iter.collect::<String>(), "");
    }

    #[test]
    fn test_by_pairs_second_newline() {
        let iter = by_pairs("one\ntwo\n\n\n", "\n\n");
        assert_eq!(&iter.collect::<String>(), "");
    }

    #[test]
    fn test_by_three_lines_first_gt_second() {
        let iter = by_three_lines("one\ntwo\nthree\nfour\n", "first\nsecond\n", "primary\nsecondary\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one first primary\ntwo second secondary\nthree\nfour\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_first_eq_second() {
        let iter = by_three_lines("one\ntwo\nthree\n", "first\nsecond\nthird\n", "primary\nsecondary\ntertiary\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one first primary\ntwo second secondary\nthree third tertiary\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_first_lt_second() {
        let iter = by_three_lines("one\ntwo\nthree\n", "first\nsecond\nthird\nfourth\nfifth\n", "primary\nsecondary\ntertiary\nquaternary\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one first primary\ntwo second secondary\nthree third tertiary\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_first_empty() {
        let iter = by_three_lines("", "first\nsecond\nthird\nfourth\nfifth\n", "primary\nsecondary\ntertiary\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_second_empty() {
        let iter = by_three_lines("one\ntwo\nthree\n", "", "primary\nsecondary\ntertiary\nquaternary\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one primary\ntwo secondary\nthree tertiary\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_third_empty() {
        let iter = by_three_lines("one\ntwo\nthree\n", "first\nsecond\nthird\nfourth\nfifth\n", "");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one first\ntwo second\nthree third\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_first_newline() {
        let iter = by_three_lines("\n\n\n", "first\nsecond\nthird\nfourth\nfifth\n", "primary\nsecondary\ntertiary\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "first primary\nsecond secondary\nthird tertiary\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_first_second_newline() {
        let iter = by_three_lines("\n\n\n", "\n\n\n", "primary\nsecondary\ntertiary\nquaternary\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "primary\nsecondary\ntertiary\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_newlines() {
        let iter = by_three_lines("\n\n\n\n", "\n\n\n\n", "\n\n\n\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "\n\n\n\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_second_third_newline() {
        let iter = by_three_lines("one\ntwo\nthree\nfour\n", "\n\n", "\n\n\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one\ntwo\nthree\nfour\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_third_newline() {
        let iter = by_three_lines("one\ntwo\nthree\nfour\n", "first\nsecond\n", "\n\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one first\ntwo second\nthree\nfour\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_first_third_newline() {
        let iter = by_three_lines("\n\n\n", "first\nsecond\nthird\nfourth\nfifth\n", "\n\n\n\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "first\nsecond\nthird\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_second_newline() {
        let iter = by_three_lines("one\ntwo\nthree\nfour\n", "\n\n", "primary\nsecondary\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one primary\ntwo secondary\nthree\nfour\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_first_skip_one() {
        let iter = by_three_lines("one\n\nthree\n", "first\nsecond\nthird\nfourth\nfifth\n", "primary\nsecondary\ntertiary\nquaternary\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one first primary\nsecond secondary\nthree third tertiary\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_by_three_lines_skip_one() {
        let iter = by_three_lines("one\n\nthree\n", "\nsecond\nthird\nfourth\nfifth\n", "primary\nsecondary\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one primary\nsecond secondary\nthree third\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_first_gt_second() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "first\nsecond\n", "primary\nsecondary\n", "uno\ndue\ntre\nquattro\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one first primary uno\ntwo second secondary due\nthree tre\nfour quattro\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_first_empty() {
        let iter = by_four_lines("", "first\nsecond\n", "primary\nsecondary\n", "uno\ndue\ntre\nquattro\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_second_empty() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "", "primary\nsecondary\n", "uno\ndue\ntre\nquattro\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one primary uno\ntwo secondary due\nthree tre\nfour quattro\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_third_empty() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "first\nsecond\n", "", "uno\ndue\ntre\nquattro\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one first uno\ntwo second due\nthree tre\nfour quattro\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_fourth_empty() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "first\nsecond\n", "primary\nsecondary\n", "");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one first primary\ntwo second secondary\nthree\nfour\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_second_third_empty() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "", "", "uno\ndue\ntre\nquattro\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one uno\ntwo due\nthree tre\nfour quattro\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_second_third_fourth_empty() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "", "", "");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one\ntwo\nthree\nfour\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_second_fourth_empty() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "", "primary\nsecondary\n", "");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one primary\ntwo secondary\nthree\nfour\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_first_newline() {
        let iter = by_four_lines("\n\n\n\n", "first\nsecond\n", "primary\nsecondary\n", "uno\ndue\ntre\nquattro\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "first primary uno\nsecond secondary due\ntre\nquattro\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_first_second_newline() {
        let iter = by_four_lines("\n\n\n\n", "\n\n", "primary\nsecondary\n", "uno\ndue\ntre\nquattro\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "primary uno\nsecondary due\ntre\nquattro\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_first_second_third_fourth_newline() {
        let iter = by_four_lines("\n\n\n\n", "\n\n", "\n\n", "\n\n\n\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "\n\n\n\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_fourth_newline() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "first\nsecond\n", "primary\nsecondary\n", "\n\n\n\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one first primary\ntwo second secondary\nthree\nfour\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_third_fourth_newline() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "first\nsecond\n", "\n\n", "\n\n\n\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one first\ntwo second\nthree\nfour\n");

        println!("{:?}", com_text);
    }
 
    #[test]
    fn test_cat_four_lines_second_third_fourth_newline() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "\n\n", "\n\n", "\n\n\n\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one\ntwo\nthree\nfour\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_second_fourth_newline() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "\n\n", "primary\nsecondary\n", "\n\n\n\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one primary\ntwo secondary\nthree\nfour\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_first_thirdd_newline() {
        let iter = by_four_lines("\n\n\n\n", "first\nsecond\n", "\n\n", "uno\ndue\ntre\nquattro\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "first uno\nsecond due\ntre\nquattro\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_second_third() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "\n\n", "\n\n", "uno\ndue\ntre\nquattro\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one uno\ntwo due\nthree tre\nfour quattro\n");

        println!("{:?}", com_text);
    }

    #[test]
    fn test_cat_four_lines_fourth_skip() {
        let iter = by_four_lines("one\ntwo\nthree\nfour\n", "first\nsecond\n", "primary\nsecondary\n", "uno\n\n\nquattro\n");
        let com_text = &iter.collect::<String>();

        assert_eq!(com_text, "one first primary uno\ntwo second secondary\nthree\nfour quattro\n");

        println!("{:?}", com_text);
    }
}
