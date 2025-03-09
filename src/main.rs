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
    println!("paso 1:");
    println!("x\ty\tx2\tx3\tx4\tx5\tx6\txy\tx2y\tx3y");
    println!("------------------------------------------------------------------------------");
    for i in 0..n {
        results.push(Vec::new());
        sums[0] += x[i];
        sums[1] += y[i];
        print!("{}\t{}\t", x[i], y[i]);
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
            print!("{}\t", results[i][j]);
        }
        println!();
    }
    println!("\npaso 2:");
    println!("x\ty\tx2\tx3\tx4\tx5\tx6\txy\tx2y\tx3y");
    for sum in &sums {
        print!("{sum}\t");
    }
    println!("\n\npaso 3:");
    // a*x^3 + b*x^2 + c + dn = y
    println!("{}a + {}b + c + {n}d = {}", sums[3], sums[2], sums[1]);
    println!(
        "{}a + {}b + {}c + {}d = {}",
        sums[4], sums[3], sums[2], sums[0], sums[7]
    );
    println!(
        "{}a + {}b + {}c + {}d = {}",
        sums[5], sums[4], sums[3], sums[2], sums[8]
    );
    println!(
        "{}a + {}b + {}c + {}d = {}",
        sums[6], sums[5], sums[4], sums[3], sums[9]
    );
}
