use std::io;
pub use std::str::FromStr;

pub use anyhow::{bail, Context, Error};

pub fn read_stdin() -> Result<String, io::Error> {
    let mut buf = String::new();
    io::Read::read_to_string(&mut io::stdin(), &mut buf)?;
    Ok(buf)
}

pub trait AocParse<'a> {
    fn try_from_lines<F>(&'a self) -> Result<Vec<F>, <F as FromStr>::Err>
    where
        F: FromStr;
}

impl<'a, T: AsRef<str>> AocParse<'a> for T {
    fn try_from_lines<F>(&'a self) -> Result<Vec<F>, <F as FromStr>::Err>
    where
        F: FromStr,
    {
        parser(self.as_ref(), FromStr::from_str)
    }
}

fn parser<'a, T, E>(input: &'a str, parse: impl Fn(&'a str) -> Result<T, E>) -> Result<Vec<T>, E> {
    input
        .trim()
        .split('\n')
        .map(parse)
        .collect::<Result<_, _>>()
}
