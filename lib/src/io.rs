use std::fs::{File};
use std::io::{BufRead, BufReader, Read, Error};
///Read tokens from a file
pub fn tokenize_file(filename: &str) -> Result<impl Iterator<Item=String>, Error> {
    let file = File::open(filename)?;
    from_buf_reader(file)
}

pub fn from_buf_reader(reader: impl Read) -> Result<impl Iterator<Item=String>, Error> {
    let res = BufReader::new(reader).lines()
        .map(|res| res.unwrap())
        .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>())
        .map(|str| str.chars().filter(|c| c.is_alphanumeric()).collect::<String>());
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_punctuation() {
        from_buf_reader(BufReader::new("oh, la , la!".as_bytes())).unwrap()
            .for_each(|token| {println!("{}", token)})
    }
    #[test]
    fn test_verlaine() {
        tokenize_file("./verlaine.txt").unwrap()
            .for_each(|token| {println!("{}", token)})
    }
}