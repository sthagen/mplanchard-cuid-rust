use cuid1::{one_off_cuid1, one_off_cuid1_slug};
use std::{
    env::{self, Args},
    process::exit,
};

/// Generate a new CUID and print it to stdout
pub fn main() {
    let args: CuidArgs = env::args().into();

    let id = if args.slug {
        one_off_cuid1_slug()
    } else {
        one_off_cuid1()
    };
    println!("{}", id);
}

const HELP: &str = r#"Usage: cuid [OPTION]...
Generate and print a CUID.

Options:
  --slug         generate a slug instead of a full CUID
  -h, --help     display this help and exit
  -v, --version  display version information and exit"#;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Commandline arguments for the CUID binary
#[derive(Debug)]
struct CuidArgs {
    /// Whether to produce a slug instead of a CUID
    slug: bool,
}
impl From<Args> for CuidArgs {
    fn from(args: Args) -> Self {
        let mut slug = false;

        // The first argument should be the binary name. Skip it.
        args.skip(1).for_each(|arg| match arg.as_str() {
            "-h" | "--help" => {
                println!("{}", HELP);
                exit(0);
            }
            "-v" | "--version" => {
                println!("{}", VERSION);
                exit(0);
            }
            "--slug" => slug = true,
            _ => {
                println!("error: unrecognized argument {}", arg);
                println!();
                println!("{}", HELP);
                exit(1);
            }
        });

        CuidArgs { slug }
    }
}
