fn main() {
    cubicas();
}

struct SistemaDeEcuaciones {
    size: usize,
    equations: Vec<Vec<f64>>,
    equalities: Vec<f64>,
}

impl SistemaDeEcuaciones {
    fn new(size: usize) -> Self {
        SistemaDeEcuaciones {
            size,
            equations: vec![vec![0.0; size]; size],
            equalities: vec![0.0; size],
        }
    }

    fn fill(&mut self, equations: Vec<Vec<f64>>, equalities: Vec<f64>) {
        if equations.len() != equalities.len() {
            panic!("the length of the equations and equalities don't match");
        }
        if equations.len() != self.size || equalities.len() != self.size {
            panic!("the length doesn't match the new function");
        }
        self.equations = equations;
        self.equalities = equalities;
    }

    fn show(&self) {
        let char_arr = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        for i in 0..self.size {
            for j in 0..self.size {
                print!("{}{} + ", self.equations[i][j], char_arr[j % 26]);
            }
            println!("{}", self.equalities[i]);
        }
    }

    fn montante(&mut self) {
        self.organize();
        let mut pivote = 1.0;
        let mut prev_equations = self.equations.clone();
        let mut prev_equalities = self.equalities.clone();
        for i in 0..self.size {
            println!("\niteration: {i}");
            let next_pivote = self.equations[i][i];
            println!("next pivote 49: {}", next_pivote);
            for column in 0..self.size + 1 {
                for row in 0..self.size {
                    // skips itself
                    if column == row && column == i {
                        continue;
                    }
                    // every item in the same column as the pivote, set to 0
                    // and skip the operations
                    else if column == i {
                        println!("setting to 0: {:?}", self.equations[column][row]);
                        self.equations[column][row] = 0.0;
                        continue;
                    }
                    // equalities
                    if column == self.size {
                        println!("equality {:?}", self.equalities[row]);
                    }
                    //equations
                    else {
                        println!("equation: {:?}", self.equations[column][row]);
                    }
                }
            }
            self.equations = prev_equations.clone();
            self.equalities = prev_equalities.clone();
            pivote = next_pivote;
        }
    }

    fn organize(&mut self) {
        for i in 0..self.size {
            if self.equations[i][i] != 0.0 {
                continue;
            }
            for j in 0..self.size {
                // skips itself
                if i == j {
                    continue;
                }
                if self.equations[j][i] != 0.0 {
                    self.equations.swap(j, i);
                    break;
                }
            }
        }
    }
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
    let mut s = SistemaDeEcuaciones::new(4);
    s.fill(
        vec![
            vec![sums[3], sums[2], 1.0, n as f64],
            vec![sums[4], sums[3], sums[2], sums[0]],
            vec![sums[5], sums[4], sums[3], sums[2]],
            vec![sums[6], sums[5], sums[4], sums[3]],
        ],
        vec![sums[1], sums[7], sums[8], sums[9]],
    );
    println!("\npaso 4:");
    s.organize();
    s.show();
    s.montante();
}
