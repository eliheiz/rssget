extern crate clap;
use clap::{App, Arg, SubCommand};

mod fetch;
mod alias;

fn main() {
    let matches = App::new("rssget")
        .version("0.1.0")
        .about("A minimal RSS client.")
        .subcommand(
            SubCommand::with_name("fetch")
                .about("fetch an RSS stream from the given feed")
                .arg(
                    Arg::with_name("feed")
                        .value_name("feed")
                        .index(1)
                        .required(true)
                        .help("feed URL or alias")
                        .takes_value(true),
                ),
          ).subcommand(
            SubCommand::with_name("alias")
                .subcommand(SubCommand::with_name("add")
                    .about("create an alias for an RSS stream")
                    .arg(
                        Arg::with_name("alias")
                            .value_name("alias_string")
                            .index(1)
                            .required(true)
                            .help("stream alias")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("url")
                            .value_name("url")
                            .index(2)
                            .required(true)
                            .help("stream URL")
                            .takes_value(true),
                    ),
                ),
        ).get_matches();

    if let Some(matches) = matches.subcommand_matches("fetch") {
        if let Some(feed) = matches.value_of("feed") {
            fetch::fetch(feed);
        }
    }
    
    if let Some(matches) = matches.subcommand_matches("alias") {
        if let Some(matches) = matches.subcommand_matches("add"){
            if let (Some(alias), Some(url)) = (matches.value_of("alias"), matches.value_of("url")) {
                alias::add(alias, url);
            }
        } 
    }
}
