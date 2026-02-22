use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "Packet Sniffer",
    version = env!("CARGO_PKG_VERSION"),
    about = "a network packet sniffing tool"
)]
pub struct Args {
    /// The network interface to capture on
    #[arg(short, long)]
    pub(crate) interface: Option<String>,
}
