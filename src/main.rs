#![feature(asm)]
use clap::{App, Arg};
use memmap::MmapOptions;
use memmap::Mmap;
use std::fs::File;
use std::ops::Deref;
mod asm;

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

    // let input = match fs::read_to_string(&path) {
    //     Ok(asm) => asm,
    //     Err(e) => panic!("wrong, path: {:?}. err: {}", &path, e),
    // };

    // println!("asm: {}", input);

    // let asm = Asm::new(input);

    // asm.list(); // debug

    let file = File::open(path).unwrap_or_else(|e| panic!("{}", e));
    let mmap = unsafe { MmapOptions::new().map_exec(&file) }.unwrap_or_else(|e| panic!("{}", e));
    mmmmmmmmmmmm(mmap);

    // function pointerに変換
    // ref; https://rust-lang.github.io/unsafe-code-guidelines/layout/function-pointers.html
    /* 
        将来的にはこうする

    let f = addr as *const ();
    let ff = unsafe { std::mem::transmute::<*const (), fn()>(f) };
    ff();
    */
}

fn mmmmmmmmmmmm(mmap: Mmap) {
    // inline-asm in rust 
    // ref: https://rust-lang.github.io/rfcs/2873-inline-asm.html
    let target_addr = mmap.deref().as_ptr();
    unsafe {
        asm!("nop");
        asm!("nop");
        asm!("nop");
        asm!("jmp {0}", in(reg) target_addr);
    }
    println!("mapped data: {:?}", mmap.deref());
    let addr = mmap.deref().as_ptr();
    println!("mapped addr: {:?}", addr);
    unsafe {
        asm!("nop");
        asm!("nop");
        asm!("nop");
        asm!("nop");
        asm!("nop");
        asm!("nop");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
