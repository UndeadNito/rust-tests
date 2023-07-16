use std::env::args;

use fib::use_fib;

mod fib;


enum PROGRAMS{
    Fib,
    Nothing
}

fn main() {
    let program_string = args().nth(1);

    let program: PROGRAMS = match program_string{
        None => {
            println!("Program name expected as first argument");
            return;
        },

        Some(value) => 
            match value.as_str(){
                "fib" => PROGRAMS::Fib,
        
                _ => PROGRAMS::Nothing,
            }
        
    };

    match program{
        PROGRAMS::Fib => use_fib(args().nth(2)),

        PROGRAMS::Nothing => println!("Program name expected as first argument"),
    };
}

