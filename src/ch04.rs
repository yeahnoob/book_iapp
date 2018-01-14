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

// exercise ch04 ex02
// m: Men used to cut the bar,
// n: the max number of bar pieces
// piece: the count of bar pieces, in the current turn
fn cutbar(m: u64, n: u64, piece: u64) -> u64 {
    if piece >= n {
        0
    } else {
        if piece < m {
            cutbar(m, n, piece * 2) + 1
        } else {
            cutbar(m, n, piece + m) + 1
        }
    }
}

pub fn exercise_ch04_ex02() {
    const N_LEN: u64 = 100;
    const M_HUM: u64 = 5;

    println!("{}, {}", N_LEN, cutbar(M_HUM, N_LEN, 1))
}