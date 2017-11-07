use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let max: u32 = args[1].parse().expect("Argument must be an unsigned integer");
    let mut data = vec![false; max as usize];
    let last: u32 = (max as f64).sqrt() as u32;
    
    // process
    for i in 2..last {
        let mut ptr: u32 = i*2;
        while ptr<max {
            data[ptr as usize] = true;
            ptr += i;
        }
    }
    
    // display
    println!("Primes below {}:", max);
    for i in 2..max {
        if ! data[i as usize] {
            println!("{}", i);
        }
    }
}
