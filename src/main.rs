fn main() {
    cubicas();
}

fn cubicas() {
    let x: Vec<f64> = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
    let y: Vec<f64> = vec![3.0, 0.0, 2.0, 4.0, 4.0];
    if x.len() != y.len() {
        panic!("the length of x and y are different!");
    }
    let n = x.len();
    let mut results: Vec<Vec<f64>> = Vec::new();
    let mut sums: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    for i in 0..n {
        results.push(Vec::new());
        sums[0] += x[i];
        sums[1] += y[i];
        for j in 0..8 {
            let remainder = j % 8;
            results[i].push(match remainder {
                0 => x[i].powi(2),
                1 => x[i].powi(3),
                2 => x[i].powi(4),
                3 => x[i].powi(5),
                4 => x[i].powi(6),
                5 => x[i] * y[i],
                6 => x[i].powi(2) * y[i],
                7 => x[i].powi(3) * y[i],
                _ => panic!("ERRORRRRRRR!!!!"),
            });
            sums[j + 2] += results[i][j];
        }
    }
    println!("paso 1:");
    println!("x\ty\tx2\tx3\tx4\tx5\tx6\txy\tx2y\tx3y");
    for (i, row) in results.iter().enumerate() {
        print!("{}\t{}\t", x[i], y[i]);
        for element in row {
            print!("{element}\t");
        }
        println!();
    }
    println!("paso 2:");
    for sum in sums {
        print!("{sum}\t");
    }
    println!();
}
