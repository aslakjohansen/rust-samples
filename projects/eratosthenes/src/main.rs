use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let max: usize = args[1].parse().expect("Argument must be an unsigned integer");
    let mut data = vec![false; max];
    let last: usize = (max as f64).sqrt() as usize;
    
    // process
    for i in 2..last {
        let mut ptr: usize = i*2;
        while ptr<max {
            data[ptr] = true;
            ptr += i;
        }
    }
    
    // display
    println!("Primes below {}:", max);
    for i in 2..max {
        if ! data[i] {
            println!("{}", i);
        }
    }
}
