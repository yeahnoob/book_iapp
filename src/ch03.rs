pub fn exercise_ch03_ex01() {

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

