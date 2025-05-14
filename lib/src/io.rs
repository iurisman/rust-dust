use std::fs::{File};
use std::io::{BufRead, BufReader};
///Read tokens from a file
pub fn tokenize(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines()
        .map(|res| res.unwrap())
        .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>())
        .map(|str| str.chars().filter(|c| c.is_alphanumeric()).collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenize() {
        tokenize("./verlaine.txt")
            .for_each(|token| {println!("{}", token)})
    }
}