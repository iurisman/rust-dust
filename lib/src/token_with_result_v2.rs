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
    //         Err(error) => {
    //             vec![Err(TokenizerError::from(error))].into_iter()
    //         }
    //     }
    // }

    pub fn from_file_enum(&self, filename: &str)
                     -> impl Iterator<Item=Result<String, TokenizerError>>
    {
        match fs::File::open(filename) {
            Ok(file) => TokenizerIter::Iter1(self.from_buf_reader(file)),
            Err(error) => TokenizerIter::Iter2(vec![Err::<String, TokenizerError>(TokenizerError::from(error))].into_iter())
        }
    }

    pub fn from_file_either(&self, filename: &str)
                     -> impl Iterator<Item=Result<String, TokenizerError>>
    {
        match fs::File::open(filename) {
            Ok(file) => Either::Left(self.from_buf_reader(file)),
            Err(error) => Either::Right(vec![Err(TokenizerError::from(error))].into_iter())
        }
    }

    pub fn from_file_chain(&self, filename: &str)
        -> impl Iterator<Item=Result<String, TokenizerError>>
    {
        let (iter1_opt, iter2_opt) =
            match fs::File::open(filename) {
                Ok(file) => (Some(self.from_buf_reader(file)), None),
                Err(error) => (None, Some(vec![Err(TokenizerError::from(error))]))
            };
        iter1_opt.into_iter().flatten().chain(iter2_opt.into_iter().flatten())
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

pub enum TokenizerIter<I1,I2> {
    Iter1(I1),
    Iter2(I2),
}
impl<I1: Iterator<Item=Result<String,TokenizerError>>, I2: Iterator<Item=Result<String,TokenizerError>>>
Iterator for TokenizerIter<I1, I2> {
    type Item = Result<String, TokenizerError>;
    fn next(&mut self) -> Option<Result<String, TokenizerError>> {
        match self {
            Self::Iter1(iter1) => iter1.next(),
            Self::Iter2(iter2) => iter2.next(),
        }
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
            tokenizer.from_file_enum("./verlaine.txt")
                .partition(|res| res.is_ok());
        assert_eq!(oks.len(), 45);

        let (oks, _errs): (Vec<Result<_,_>>, Vec<Result<_, _>>) =
            tokenizer.from_file_either("./verlaine.txt")
                .partition(|res| res.is_ok());
        assert_eq!(oks.len(), 45);

        let (oks, _errs): (Vec<Result<_,_>>, Vec<Result<_, _>>) =
            tokenizer.from_file_chain("./verlaine.txt")
                .partition(|res| res.is_ok());
        assert_eq!(oks.len(), 45);
    }

    #[test]
    fn test_io_error() {
        let tokenizer = Tokenizer::new_with_validator(validator);

        let vec = tokenizer.from_file_enum("./bad.txt").collect::<Vec<_>>();
        assert_eq!(vec.len(), 1);
        match vec.first().unwrap() {
            Ok(_) => assert!(false),
            Err(err) => assert!(
                matches!(err, TokenizerError::Io(foo) if foo.kind() == io::ErrorKind::NotFound)
            ),
        }

        let vec = tokenizer.from_file_either("./bad.txt").collect::<Vec<_>>();
        assert_eq!(vec.len(), 1);
        match vec.first().unwrap() {
            Ok(_) => assert!(false),
            Err(err) => assert!(
                matches!(err, TokenizerError::Io(foo) if foo.kind() == io::ErrorKind::NotFound)
            ),
        }

        let vec = tokenizer.from_file_chain("./bad.txt").collect::<Vec<_>>();
        assert_eq!(vec.len(), 1);
        match vec.first().unwrap() {
            Ok(_) => assert!(false),
            Err(err) => assert!(
                matches!(err, TokenizerError::Io(ioerr) if ioerr.kind() == io::ErrorKind::NotFound)
            ),
        }

    }
}