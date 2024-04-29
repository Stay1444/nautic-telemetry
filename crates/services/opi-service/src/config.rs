use clap::Parser;

#[derive(Parser, Clone)]
pub struct Configuration {
    #[clap(long, env)]
    pub amqp_addr: String,

    #[clap(long, env)]
    pub tty: String,

    #[clap(long, env)]
    pub baud: u32,

    #[clap(long, env)]
    pub gpio_chip: String,
    #[clap(long, env)]
    pub gpio_pin: u32,
}
