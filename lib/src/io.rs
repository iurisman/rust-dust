use std::fs::{File};
use std::io::{BufRead, BufReader, Read};
use regex::Regex;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
///Read tokens from a file
pub fn tokenize_file(filename: &str) -> Result<impl Iterator<Item=String>> {
    let file = File::open(filename)?;
    from_buf_reader(file)
}

pub fn from_buf_reader(reader: impl Read) -> Result<impl Iterator<Item=String>> {
    let regex = Regex::new(r#"[a-zA-Z0-9\d\s:]"#).unwrap();
    let res = BufReader::new(reader).lines()
        .map(|res| res.unwrap())
        .map(move |str| str.chars().filter(|c| regex.is_match(&c.to_string())).collect::<String>())
        .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>());
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_punctuation() {
        from_buf_reader(BufReader::new("oh, la , la!".as_bytes())).unwrap()
            .for_each(|token| {
                println!("'{}'", token);
                assert!(token.chars().count() > 0 && token.chars().all(|c| c.is_alphanumeric()));
            })
    }
    #[test]
    fn test_verlaine() {
        let mut token_count = 0;
        tokenize_file("./verlaine.txt").unwrap()
            .for_each(|_token| token_count += 1 );
        assert_eq!(token_count, 45);
    }
}