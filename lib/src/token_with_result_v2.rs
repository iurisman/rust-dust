use std::{fs, io};
use std::io::{BufRead};
use either::Either;

#[derive(Debug)]
pub enum TokenizerError {
    Io(io::Error),
}

impl From<io::Error> for TokenizerError {
    fn from(error: io::Error) -> Self {
        TokenizerError::Io(error)
    }
}

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

    /// Does not compile
    // pub fn from_file(&self, filename: &str)
    //     -> impl Iterator<Item=Result<String, TokenizerError>>
    // {
    //     match fs::File::open(filename) {
    //         Ok(file) => self.from_buf_reader(file),
    //         Err(error) => vec![Err(TokenizerError::from(error))].into_iter()
    //     }
    // }

    pub fn from_file(&self, filename: &str)
        -> impl Iterator<Item=Result<String, TokenizerError>>
    {
        match fs::File::open(filename) {
            Ok(file) => Either::Left(self.from_buf_reader(file)),
            Err(error) =>
                match error.kind() {
                    io::ErrorKind::NotFound => Either::Right(Either::Left(vec![Err(TokenizerError::from(error))].into_iter())),
                    _ => Either::Right(Either::Right(vec![Err(TokenizerError::from(error))].into_iter()))
                }

        }
    }

    /// Read tokens from a reader
    pub fn from_buf_reader<R: io::Read>(&self, reader: R) -> impl Iterator<Item=Result<String, TokenizerError>> {
        io::BufReader::new(reader).lines()
            .map(|res_line|
                res_line.map(|line|
                    line.chars().filter(|c| (self.validator)(c)).collect::<String>()
                )
            )
            .flat_map(|res_line|
                match res_line {
                    Err(err) =>
                        vec![Err(TokenizerError::from(err))],
                    Ok(line) =>
                        line.split_whitespace()
                            .map(|str| Ok(String::from(str)))
                            .collect::<Vec<Result<String, _>>>()
                }
            )
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
            tokenizer.from_buf_reader(io::BufReader::new("oh, la , la!".as_bytes()))
                .map(|res| res.unwrap())
                .collect::<Vec<String>>(),
            vec!["oh,".to_string(), "la".to_string(), ",".to_string(), "la!".to_string()]
        );
    }

    #[test]
    fn test_punctuation() {
        let tokenizer = Tokenizer::new_with_validator(validator);
        tokenizer.from_buf_reader(io::BufReader::new("oh, la , la!".as_bytes()))
            .for_each(|res| {
                let token = res.unwrap();
                assert!(token.chars().count() > 0 && token.chars().all(|c| c.is_alphanumeric()));
            })
    }
    #[test]
    fn test_verlaine() {
        let tokenizer = Tokenizer::new_with_validator(validator);
        let (oks, _errs): (Vec<Result<_,_>>, Vec<Result<_, _>>) =
            tokenizer.from_file("./verlaine.txt")
                //.unwrap()
                .partition(|res| res.is_ok());
        assert_eq!(oks.len(), 45);
    }

    #[test]
    fn test_io_error() {
        let tokenizer = Tokenizer::new_with_validator(validator);
        let vec = tokenizer.from_file("./bad.txt").collect::<Vec<_>>();
        assert_eq!(vec.len(), 1);
        match vec.first().unwrap() {
            Ok(_) => assert!(false),
            Err(err) => assert!(
                matches!(err, TokenizerError::Io(foo) if foo.kind() == io::ErrorKind::NotFound)
            ),
        }
    }
}