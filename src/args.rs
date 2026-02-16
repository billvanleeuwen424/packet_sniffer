use clap::Parser;

#[derive(Parser)]
#[command(
    name = "Packet Sniffer",
    version = "0.1.0",
    about = "a network packet sniffing tool"
)]
#[derive(Debug)]
pub struct Args {
    /// The network interface to capture on
    #[arg(short, long)]
    pub(crate) interface: Option<String>,
}
