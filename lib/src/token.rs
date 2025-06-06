use std::fs::{File};
use std::io::{BufRead, BufReader, Read};
pub struct Tokenizer {
    validator: fn(&char) -> bool,
}
impl Tokenizer {
    fn default_validator(_: &char) -> bool {true}
    pub fn new() -> Tokenizer {
        Tokenizer { validator: Self::default_validator}
    }
    pub fn new_with_validator(validator: fn(&char) -> bool) -> Tokenizer {
        Tokenizer {validator}
    }

    ///Read tokens from a file
    pub fn from_file(&self, filename: &str) -> impl Iterator<Item=String> {
        let file = File::open(filename).unwrap();
        self.from_buf_reader(file)
    }

    pub fn from_buf_reader<R: Read>(&self, reader: R) -> impl Iterator<Item=String> {
        BufReader::new(reader).lines()
            .map(|res| res.unwrap())
            .map(|str| str.chars().filter(|c| (self.validator)(c)).collect::<String>())
            .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    fn validator(c: &char) -> bool {
        // Chars we care about plus white space to split on.
        Regex::new(r#"[^\p{Punct}]"#).unwrap().is_match(&c.to_string())
    }
    #[test]
    fn test_default_validator() {
        let tokenizer = Tokenizer::new();
        assert_eq!(
            tokenizer.from_buf_reader(BufReader::new("oh, la , la!".as_bytes())).collect::<Vec<String>>(),
            vec!["oh,".to_string(), "la".to_string(), ",".to_string(), "la!".to_string()]
        );
    }

    #[test]
    fn test_punctuation() {
        let tokenizer = Tokenizer::new_with_validator(validator);
        tokenizer.from_buf_reader(BufReader::new("oh, la , la!".as_bytes()))
            .for_each(|token| {
                assert!(token.chars().count() > 0 && token.chars().all(|c| c.is_alphanumeric()));
            })
    }
    #[test]
    fn test_verlaine() {
        let tokenizer = Tokenizer::new_with_validator(validator);
        let token_count = tokenizer.from_file("./verlaine.txt").count();
        assert_eq!(token_count, 45);
    }
}