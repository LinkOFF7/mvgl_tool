use std::{env, process};

mod mvgl;

fn print_usage(arg: &str) {
    println!("The Hundred Line -Last Defense Academy- MVGL Extractor");
    println!("Usage: {} extract <mvgl file>", &arg);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3{
        print_usage(&args[0]);
        process::exit(0);
    } else {
        if &args[1] == "extract" {
            match mvgl::extract(&args[2]){
                Err(e) => panic!("{}", e),
                Ok(r) => r
            };
        } else {
            print_usage(&args[0]);
        process::exit(0);
        }
    }
    let _ = mvgl::extract("app_text01.dx11.mvgl");
}
