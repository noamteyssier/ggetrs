use std::io;
use clap::Command;
use clap_complete::{Generator, generate};

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
