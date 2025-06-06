use rand::Rng;

fn main()  {
    words().for_each(|word| {println!("{}", word)});
}

#[derive(Debug)]
struct MyError(String);
type MyResult<T> = Result<T, MyError>;

fn lines() -> impl Iterator<Item=Result<String, MyError>> {
     vec!(
         "the first line",
         "the second line",
         "the third line",
     ).into_iter().map(|str| Ok(String::from(str)))

}

fn words() -> MyResult<impl Iterator<Item=String>> {
    let foo = lines()
        .flat_map(|res|
            match res {
                Ok(line) => line.split_whitespace().map(|str| Ok(String::from(str))).collect::<Vec<Result<String,_>>>(),
                Err(e) => vec!(Err(e)),
            }
        );

unimplemented!()
}
