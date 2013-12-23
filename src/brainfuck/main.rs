#[link(
    uuid = "c78f114a-2117-4912-8ccd-6cb8e2bc4480"
)];

#[license = "MIT"];

#[crate_id = "github.com/KokaKiwi/brainfuck-rs#0.1.0"];
#[crate_type = "bin"];

#[feature(macro_rules)];

extern mod argparse;

use std::os;
use std::io;
use std::str;

use argparse::ArgumentParser;
use argparse::arg;

#[path = "../../deps/argparse-rs/src/argparse/macros.rs"]
mod macros;

mod bf;

fn read_file(filename: &str) -> ~str
{
    let content = if filename == "-" {
        io::stdin().read_to_end()
    } else {
        let path = Path::new(filename);

        match io::File::open(&path)
        {
            Some(mut f) => f.read_to_end(),
            None => fail!("Error reading file: {}", filename),
        }
    };

    str::from_utf8(content).to_owned()
}

fn main_args(args: &[~str]) -> int
{
    let mut parser = ArgumentParser::new();
    parser.description = Some("Brainfuck interpreter.");

    let opts = ~[
        create_arg!("-h", "--help"; ty = arg::ArgTyBool, help = Some("Show this help and exit.")),
        create_arg!("-f", "--file"; ty = arg::ArgTyBool, help = Some("Execute file instead of string.")),
        create_arg!("program"; required = false, help = Some("Program to execute, file or string.")),
    ];
    parser.add_arguments(opts);

    let args = match parser.parse_args(args.tail()) {
        Ok(args) => args,
        Err(e) => {
            println(e.to_str());
            parser.print_help();
            return 1;
        }
    };

    if args.get::<bool>("help")
    {
        parser.print_help();
        return 0;
    }

    let program_file = args.get::<bool>("file");

    let program = if program_file {
        read_file(args.get::<~str>("program"))
    } else {
        args.get::<~str>("program")
    };

    let mut interpreter = bf::BrainfuckInterpreter::new();
    interpreter.run(program);

    return 0;
}

fn main()
{
    os::set_exit_status(main_args(os::args()));
}
