use std::error::Error;

fn main()  {

    words().for_each(|word| println!("{word}"));
}

fn lines() -> impl Iterator<Item=String> {
    vec!(
        "the first line",
        "the second line",
    ).into_iter().map(String::from)
}

fn words() -> impl Iterator<Item=String> {
    lines()
        .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>())
}