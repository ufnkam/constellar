use clap::{arg, Arg, Command};

fn client_cmd() -> Command {
    let cmd = clap::Command::new("client")
        .about("Contains client command set")
        .arg(Arg::new("host").short('h').help("Client host"))
        .arg(Arg::new("port").short('p').help("Client port"));
    cmd
}
