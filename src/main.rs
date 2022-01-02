#![feature(asm)]
#![feature(with_options)]
mod asm;
use asm::Asm;
use clap::{App, Arg};
use libc::{mmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};
use std::fs;
use std::ptr;

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

    let mmap = unsafe {
        mmap(
            ptr::null_mut(),
            1000,
            PROT_EXEC | PROT_READ | PROT_WRITE,
            MAP_ANONYMOUS | MAP_PRIVATE,
            -1,
            0,
        )
    };

    let mut asm = Asm::new(input);

    asm.list(); // debug

    asm.set_mapped_mem(mmap as *mut u8, 1000);

    asm.assemble();

    asm.run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
