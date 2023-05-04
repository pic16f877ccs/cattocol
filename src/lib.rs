#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links
)]

//! Combine two text into one text as columns or by lines.
//!
//! - Without the ansi escpe sequences.
//! - With the ansi escpe sequences.
//! # Examples
//!
//! ```
//! # use cattocol::CatToCol;
//!
//! let txt_one = String::from("Text cat\nby line.\nTest line.");
//! let txt_two = String::from("Concat text.\nTwo line.\nMin.\nMax");
//! let cat_to_col = CatToCol::new().fill(' ').repeat(1);
//! let combine_iter = cat_to_col.combine_col(&txt_one, &txt_two);
//!
//! println!("{}", combine_iter.collect::<String>());
//!
//! //Text cat   Concat text.
//! //by line.   Two line.
//! //Test line. Min.
//! //           Max
//!
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
    pub fn fill(mut self, fill: char) -> Self {
        self.fill = fill.into();
        self
    }

    /// Changes the repetition values.
    pub fn repeat(mut self, repeat: usize) -> Self {
        self.repeat = repeat;
        self
    }

    /// Combining two texts in columns separated by a character repeated N times.
    ///
    /// - Without the ansi escpe sequences.
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

fn max_line_len(text: &str) -> usize {
    text.lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

fn max_line_len_no_esc(text: &str) -> usize {
    max_line_len(std::str::from_utf8(&strip(text).unwrap()).unwrap())
}
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

}


