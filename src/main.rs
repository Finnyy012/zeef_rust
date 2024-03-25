extern crate rayon;

use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let clock = Instant::now();
    let p = zeef_sequentieel(1_000_000);
    println!("sequentieel tijd: {}", clock.elapsed().as_secs_f64());
    // println!("{}", p.iter().sum::<u32>());

    let clock = Instant::now();
    let p = zeef_parallel(1_000_000);
    println!("parallel tijd: {}", clock.elapsed().as_secs_f64());
}

fn zeef_sequentieel(N: usize) -> Vec<u32>{
    let mut p = vec![1; N + 1];
    p[0] = 0;
    p[1] = 0;

    let mut k: usize = 2;
    while k <= (N as f64).sqrt() as usize{
        if p[k] == 1 {
            let mut l = k.pow(2);
            while l <= N {
                p[l] = 0;
                l += k;
            }
        }
        k += 1;
    }
    p
}

fn zeef_parallel(N: usize) -> Vec<u32> {
    let mut p = vec![1; N + 1];
    p[0] = 0;
    p[1] = 0;

    let sqrt_N = (N as f64).sqrt() as usize;

    let result: Vec<Vec<u32>> = (2..=sqrt_N)
        .into_par_iter()
        .filter_map(|k| {
            if p[k] == 1 {
                let mut buffer = vec![0; N + 1];
                let mut l = k * k;
                while l <= N {
                    buffer[l] = 1;
                    l += k;
                }
                Some(buffer)
            } else {
                None
            }
        })
        .collect();

    for buffer in result {
        for (i, &val) in buffer.iter().enumerate() {
            if val == 1 {
                p[i] = 0;
            }
        }
    }

    p
}
