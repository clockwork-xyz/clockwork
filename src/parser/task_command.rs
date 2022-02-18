use clap::ArgMatches;

use crate::{command::CliCommand, error::CliError};

use super::utils::{parse_i64_optional, parse_instruction, parse_pubkey, parse_string};

pub fn task_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("new", matches)) => {
            // TODO parse arguments
            let filepath = parse_string(&String::from("filepath"), matches)?;
            println!("Filepath: {}", filepath);
            let ix = parse_instruction(&filepath)?;
            println!("Ix: {:#?}", ix);
            let exec_at = parse_i64_optional(&String::from("exec_at"), matches)?;
            println!("Exec at: {:?}", exec_at);
            let stop_at = parse_i64_optional(&String::from("stop_at"), matches)?;
            println!("Stop at: {:?}", stop_at);
            let recurr = parse_i64_optional(&String::from("recurr"), matches)?;
            println!("Recurr: {:?}", recurr);
            Ok(CliCommand::TaskNew {
                ix,
                exec_at,
                stop_at,
                recurr,
            })
        }
        _ => Ok(CliCommand::TaskData {
            address: parse_pubkey(&String::from("address"), matches)?,
        }),
    }
}
