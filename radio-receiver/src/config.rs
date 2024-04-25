use clap::Parser;

#[derive(Parser)]
pub struct Config {
    pub tty: String,
    pub baud: u32,
}
