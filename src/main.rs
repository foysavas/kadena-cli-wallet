#[macro_use]
extern crate clap;
use clap::App;

mod cmds;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    if let Some(ref _matches) = matches.subcommand_matches("gen-keypair") {
        cmds::gen_keypair::run().expect("couldn't generate keypair");
    }
}
