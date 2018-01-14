pub fn exercise_ch04_ex01() {
    const N_LEN: u64 = 100;
    const M_HUM: u64 = 5;

    let mut n_cut = 0;
    let mut n_part = N_LEN;
    println!("parts, cuts");
    while n_part > 1 {
        n_part = n_part / 2;
        match n_part / M_HUM {
            0 => n_cut += 1,
            x if n_part % M_HUM == 0 => n_cut += x,
            x if n_part % M_HUM != 0 => n_cut += x + 1,
            _ => println!("Something Wrong!"),
        }
        println!("{}, {}", n_part, n_cut);
    }
}
