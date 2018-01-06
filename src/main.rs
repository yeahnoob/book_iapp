// the main rock
extern crate clap;
// ch02
extern crate rcalc;

use clap::{Arg, App};

fn exercise_ch01() {
    println!("Exercise in Chapter 01");
}

fn exercise_ch02() {

	use rcalc::Interpreter;

    let op = vec!["+", "-", "*", "/", ""];
    // let op = vec!["*", ""];
    for i in 1000..10000 {
        let c = i.to_string();
        for j in &op {
            for k in &op {
                for l in &op {
                    let expr = format!("{}{}{}{}{}{}{}",&c[3..],j,&c[2..3],k,&c[1..2],l,&c[0..1]);
                    if expr.len() > 4 {
                        let mut pr = Interpreter::from(&expr);
                        let result = pr.interpret().unwrap().round() as i64;
                        if i == result {
                            println!("{} = {}", expr, result);
                        }
                    }
                }
            }
        }
    }
}

fn exercise_ch03() {
    const N_MAX: usize = 100;

    let mut pokerface = vec![false; N_MAX];
    for n in 1..N_MAX {
        for i in (n..N_MAX).filter(|&x| (x + 1) % (n + 1) == 0) {
            pokerface[i] = !pokerface[i];
        }
    }

    // test the result
    for (index, elem) in pokerface.iter().enumerate() {
        if !(*elem) {
            print!("{}, ", index + 1);
        }
    }
    println!();
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
        ("02", "01") => exercise_ch02(),
        ("03", "01") => exercise_ch03(),
        _ => println!("The exercise[{}] has not been done.", chapter_number),
    }
}
