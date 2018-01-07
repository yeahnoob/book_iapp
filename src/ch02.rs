pub fn exercise_ch02_ex01() {

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

