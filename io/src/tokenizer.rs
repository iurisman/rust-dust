use std::fs::{File};
use std::io::{BufRead, BufReader};
///Read tokens from a file
fn read_tokens(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines()
        .map(|res| res.unwrap())
        .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_tokes() {
        for token in  read_tokens("./verlaine.txt") {
            println!("{}", token);
        }
    }
}