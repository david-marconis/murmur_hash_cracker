use rayon::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = hash_or_crack(&args) {
        exit_with_error(&e, &args[0]);
    }
}

fn hash_or_crack(args: &[String]) -> Result<(), String> {
    if args.len() < 3 {
        return Err("Invalid input count".to_owned());
    }
    let mode = &args[1];
    let number = &args[2].parse::<u64>().map_err(|e| e.to_string())?;

    match mode.as_str() {
        "hash" => Ok(hash(*number)),
        "crack" => Ok(crack(*number as u32)),
        _ => Err(format!("Invalid input: {mode}")),
    }
}

fn hash(number: u64) {
    let hash = fasthash::murmur::hash32(number.to_be_bytes());
    println!("{hash}");
}

fn crack(number: u32) {
    (10000000000u64..99999999999).into_par_iter().for_each(|n| {
        let try_hash = fasthash::murmur::hash32(n.to_be_bytes());
        if try_hash == number {
            println!("{n}");
        }
    });
}

fn exit_with_error(error: &str, arg_0: &str) {
    eprintln!(
        "{error}
Usage examples:
    Hash account nubmer:  {arg_0} hash 12345678901
    Crack account number: {arg_0} crack 537983508"
    );
    std::process::exit(1);
}
