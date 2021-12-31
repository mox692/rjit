use clap::{App, Arg};
use std::fs;
mod asm;

use asm::Asm;

fn main() {
    let app = App::new("rjit")
        .version("0.0.1")
        .author("Motoyuki Kimura")
        .about("A toy x86 JIT Compiler.")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .help("The input file"),
        );

    let matches = app.get_matches();

    let path = matches
        .value_of("file")
        .unwrap_or_else(|| panic!("file flag not found."));

    let input = match fs::read_to_string(&path) {
        Ok(asm) => asm,
        Err(e) => panic!("wrong, path: {:?}. err: {}", &path, e),
    };

    println!("asm: {}", input);

    let asm = Asm::new(input);
    asm.list();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
