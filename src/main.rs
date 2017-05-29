extern crate argparse;
#[cfg(feature = "interpreter")]
extern crate brainfuck;

use argparse::{ArgumentParser, Store};
#[cfg(feature = "interpreter")]
use argparse::StoreTrue;

mod hashfuck;
use hashfuck::compile_hashfuck;

#[cfg(feature = "interpreter")]
fn interpret_brainfuck(bf: String, option_interpret: bool) {
    if option_interpret {
        brainfuck::eval_string(&bf).expect("Interpreter error")
    }
}

fn main() {
    let mut program = String::new();
    #[cfg(feature = "interpreter")]
    let mut option_interpret = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Hashfuck Interpreter");
        ap.refer(&mut program).add_argument("program", Store, "Hashfuck program code");
        #[cfg(feature = "interpreter")]
        ap.refer(&mut option_interpret).add_option(&["-i", "--interpret"], StoreTrue, "Run BF interpreter");
        ap.parse_args_or_exit();
    }

    match program.is_empty() {
        false => {
            let bf = compile_hashfuck(program);
            println!("Brainfuck: {}", bf);

            #[cfg(feature = "interpreter")]
                interpret_brainfuck(bf, option_interpret)
        }
        true => { /* noop */ }
    }
}
