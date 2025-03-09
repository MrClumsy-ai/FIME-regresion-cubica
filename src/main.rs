fn main() {
    cubicas();
}

struct SistemaDeEcuaciones {
    size: usize,
    ecuaciones: Vec<Vec<f64>>,
    igualdades: Vec<f64>,
}

impl SistemaDeEcuaciones {
    fn new(size: usize) -> Self {
        SistemaDeEcuaciones {
            size,
            ecuaciones: vec![vec![0.0; size]; size],
            igualdades: vec![0.0; size],
        }
    }
    fn fill(&mut self, ecuaciones: Vec<Vec<f64>>, igualdades: Vec<f64>) {
        self.ecuaciones = ecuaciones;
        self.igualdades = igualdades;
    }
    fn show(&self) {
        let char_arr = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        for i in 0..self.size {
            for j in 0..self.size {
                print!("{}{} + ", self.ecuaciones[i][j], char_arr[j % 26]);
            }
            println!("{}", self.igualdades[i]);
        }
    }
    fn gauss_seidel(&self) -> Option<Vec<f64>> {
        let margen_de_error = 0.001;
        let mut prev = vec![0.0; self.size];
        let mut curr: Vec<f64> = Vec::new();
        for i in 1..100 {
            println!("{}, prev: {:?}, curr: {:?}", i, &prev, &curr);
            for j in 0..self.size {
                // debug start
                let char_arr = [
                    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                    'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                ];
                println!("iterating: {}", char_arr[j % 26]);
                // debug end
            }
            for i in 0..self.size {
                if (prev[i] - curr[i]).abs() > margen_de_error {
                    prev = curr.clone();
                    continue;
                }
            }
            return Some(curr);
        }
        None
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
    // println!("{}a + {}b + c + {n}d = {}", sums[3], sums[2], sums[1]);
    // println!(
    //     "{}a + {}b + {}c + {}d = {}",
    //     sums[4], sums[3], sums[2], sums[0], sums[7]
    // );
    // println!(
    //     "{}a + {}b + {}c + {}d = {}",
    //     sums[5], sums[4], sums[3], sums[2], sums[8]
    // );
    // println!(
    //     "{}a + {}b + {}c + {}d = {}",
    //     sums[6], sums[5], sums[4], sums[3], sums[9]
    // );
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
    s.show();
    s.gauss_seidel();
}
