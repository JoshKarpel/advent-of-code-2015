use std::env;
use ureq::get;

pub fn get_input(year: usize, day: usize) -> String {
    dotenv::dotenv().ok();

    return get(&format!(
        "https://adventofcode.com/{year}/day/{day}/input",
        year = year,
        day = day
    ))
    .set(
        "cookie",
        &format!("session={}", &env::var("ADVENT_SESSION").unwrap()),
    )
    .call()
    .unwrap()
    .into_string()
    .unwrap();
}
