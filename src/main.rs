// the main rock
extern crate clap;
// ch02
extern crate rcalc;

use clap::{Arg, App};

mod ch02;
mod ch03;
mod ch04;
mod ch05;

fn exercise_ch01() {
    println!("Exercise in Chapter 01");
}


fn main() {
    let matches = App::new("Book IAPP's exercises")
        .version("0.1.0")
        .author("Xanadu Zhang <zgemlbox@gmail.com>")
        .about("Interesting Algorithms of Programmer's Puzzle")
        .arg(Arg::with_name("chapter number")
             .short("c")
             .long("chapter")
             .takes_value(true)
             .help("Book Chapter Number"))
        .arg(Arg::with_name("exercise number")
             .short("e")
             .long("exercise")
             .takes_value(true)
             .help("Exercise Number in One Chapter"))
        .get_matches();
    let chapter_number = matches.value_of("chapter number").unwrap_or("01");
    let exercise_number = matches.value_of("exercise number").unwrap_or("01");
    println!("Chapter Number: {}", chapter_number);
    match (chapter_number, exercise_number) {
        ("01", "01") => exercise_ch01(),
        ("02", "01") => ch02::exercise_ch02_ex01(),
        ("03", "01") => ch03::exercise_ch03_ex01(),
        ("04", "01") => ch04::exercise_ch04_ex01(),
        ("05", "01") => ch05::exercise_ch05_ex01(),
        _ => println!("The exercise[{}] has not been done.", chapter_number),
    }
}
