use clap::{App, Arg};
use std::fs::File;
use std::path::Path;

fn main() {
    let app = App::new("rjit")
        .version("0.0.1")
        .author("Motoyuki Kimura")
        .about("A toy x86 JIT Compiler.")
        .arg(
            Arg::with_name("file")
                .value_name("FILE_NAME")
                .help("specify the input file.")
                .multiple(true)
        );

    let matches = app.get_matches();

    let path;
    if let Some(file ) = matches.value_of("file") {
        path = Path::new(file);
    } else {
        panic!("-file flag required.");
    };

    let mut file = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: ", e),
        Ok(file) => file,
    };

    println!("meta: {:?}", file.metadata());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
