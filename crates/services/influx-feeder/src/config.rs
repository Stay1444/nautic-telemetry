use clap::Parser;

#[derive(Parser, Clone)]
pub struct Configuration {
    #[clap(long, env)]
    pub amqp_addr: String,
    #[clap(long, env)]
    pub influx_addr: String,
}
