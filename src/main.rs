
use clap::Parser;
use rand::prelude::*;
// Prefer to use the Clap Derive marcros rather than the older Builder macros.
// https://docs.rs/clap/latest/clap/_faq/index.html#when-should-i-use-the-builder-vs-derive-apis
// https://docs.rs/clap/latest/clap/_derive/index.html#doc-comments

#[derive(Parser, Debug)]
/// cantstop: Simulate situations in the Can't Stop game to estimate probabilities of success.
#[command(version, arg_required_else_help = false)]
struct Args {
    /// Simulate all common scenarios
    #[arg(short, long)]
    all: bool,

    // number of simulations
    #[arg(short, long, value_name = "NUM_SIMS", default_value_t = 100000)]
    num_sims: u32,

    /// simulate a specific scenario
    #[arg(short, long, value_name = "SPECIFIC")]
    specific: Option<String>
}

fn main() {
    let args = Args::parse();
    if args.specific != None {
        specific(args.specific.unwrap(), args.num_sims);
    }
    if args.all {
        simulate(args.num_sims);
    }
}

fn specific(specific: String, num_sims: u32) -> () {
    let pattern:Vec<u32> = specific.split_whitespace().map(|val| val.parse::<u32>().unwrap()).collect();
    let mut count = 0;
    let mut rng = rand::rng();
    for _ in 0..num_sims {
        let rolls: Vec<u32> = (1..=4).map(|_x| rng.random_range(1..=6)).collect();
        let vals:[u32; 6] = [rolls[0]+rolls[1], rolls[2]+rolls[3], rolls[0]+rolls[2], rolls[1]+rolls[3], rolls[0]+rolls[3], rolls[1]+rolls[2]];
        if contains_any(&vals, &pattern) {
            count += 1;
        }
    }
    println!("Any value in {:?}: {:2.0}%", pattern, 100. * count as f32 / num_sims as f32)
}

fn simulate(num_sims: u32) {
    println!("Can't Stop: {} simulations.\n", num_sims);
    let mut rng = rand::rng();
    let mut counts: [u32; 13] = [0u32; 13];
    let mut doubles: [u32; 13] = [0u32; 13];
    let patterns: Vec<[u32; 3]> = 
        vec![[6, 7, 8], 
            [5, 6, 7], 
            [5, 6, 8],
            [5, 6, 9], 
            [4, 6, 7], 
            [4, 6, 8], 
            [4, 5, 7], 
            [4, 5, 6], 
            [4, 5, 9], 
            [3, 6, 7], 
            [3, 6, 8], 
            [3, 5, 7], 
            [3, 5, 6],
            [3, 4, 7],
            [3, 4, 6],
            [3, 4, 5],
            [3, 4, 10],
            [2, 6, 7], 
            [2, 6, 8], 
            [2, 5, 7], 
            [2, 5, 6],
            [2, 4, 7],
            [2, 4, 6],
            [2, 4, 5],
            [2, 4, 10],
            [2, 3, 7],
            [2, 3, 6],
            [2, 3, 5],
            [2, 3, 4]            
            ];
    let mut pattern_counts = vec![0u32; patterns.len() as usize];
    for _i in 0..num_sims {
        let rolls: Vec<u32> = (1..=4).map(|_x| rng.random_range(1..=6)).collect();
        let vals:[u32; 6] = [rolls[0]+rolls[1], rolls[2]+rolls[3], rolls[0]+rolls[2], rolls[1]+rolls[3], rolls[0]+rolls[3], rolls[1]+rolls[2]];
        for j in 2..=12 {
            // single value match
            if contains_any(&vals, &[j]) {
                counts[j as usize] += 1;
            }
            // single value match twice
            if vals[0] == j && vals[1] == j {
                doubles[j as usize] += 1;
            } else if vals[2] == j && vals[3] == j {
                doubles[j as usize] += 1;
            } else if vals[4] == j && vals[5] == j {
                doubles[j as usize] += 1;
            }
        }
        // multiple value match any
        for (idx, pattern) in patterns.iter().enumerate() {
            if contains_any(&vals, pattern) {
                pattern_counts[idx] += 1;
            }
        }
    }
    println!("    one two expected turns to top");
    let tops = [0, 0, 2, 4, 6, 8, 10, 12, 10, 8, 6, 4, 2];
    for i in 2..=12 {
        let expected = (counts[i] + doubles[i]) as f32/num_sims as f32;
        println!("{:2}: {:2.0}% {:2.0}% {:.2}     {:.0}", i, 
            100. * counts[i] as f32/num_sims as f32, 
            100. * doubles[i] as f32/num_sims as f32,
            expected, tops[i] as f32 / expected);
    }
    println!("\nAny value in the list");
    for (idx, count) in pattern_counts.iter().enumerate() {
        println!("{:1?}: {:2.0}%", patterns[idx], 100. * *count as f32/num_sims as f32);
    }
}

fn contains_any(vals: &[u32], of: &[u32]) -> bool {
    for i in of {
        if vals.contains(i) {
            return true;
        }
    }
    false
}
