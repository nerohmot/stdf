extern crate clap;
use clap::{Arg, Command, crate_version, crate_authors};

fn main() {
    let matches = Command::new("stdf")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Standard Test Data Format (STDF) serialization and data processing")
        .subcommand(
            Command::new("dump")
                .about("Dumps the content of the input file")
                .arg(
                    Arg::new("input_file")
                        .short('i')
                        .long("input_file")
                        .required(true)
                        .help("Sets the input file to use"),
                ),
        )
        .subcommand(
            Command::new("count")
                .about("Counts the records in the input file")
                .arg(
                    Arg::new("input_file")
                        .short('i')
                        .long("input_file")
                        .required(true)
                        .help("Sets the input file to use"),
                ),
        )
        .subcommand(
            Command::new("convert_to")
                .about("Converts the input file to another format")
                .subcommand(
                    Command::new("csv")
                        .about("Converts the input file to CSV format")
                        .arg(
                            Arg::new("input_file")
                                .short('i')
                                .long("input_file")
                                .required(true)
                                .help("Sets the input file to use"),
                        ),
                )
                .subcommand(
                    Command::new("xlsx")
                        .about("Converts the input file to XLSX format")
                        .arg(
                            Arg::new("input_file")
                                .short('i')
                                .long("input_file")
                                .required(true)
                                .help("Sets the input file to use"),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("dump", sub_m)) => {
            let input_file = sub_m.get_one::<String>("input_file").unwrap();
            println!("Dumping the content of the file: {}", input_file);
            // Add your logic for the "dump" subcommand here
        }
        Some(("count", sub_m)) => {
            let input_file = sub_m.get_one::<String>("input_file").unwrap();
            println!("Counting the records in the file: {}", input_file);
            // Add your logic for the "count" subcommand here
        }
        Some(("convert_to", sub_m)) => {
            match sub_m.subcommand() {
                Some(("csv", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    println!("Converting the file to CSV: {}", input_file);
                    // Add your logic for the "csv" subcommand here
                }
                Some(("xlsx", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    println!("Converting the file to XLSX: {}", input_file);
                    // Add your logic for the "xlsx" subcommand here
                }
                _ => eprintln!("No valid subcommand was used for convert_to"),
            }
        }
        _ => eprintln!("No valid subcommand was used"),
    }
}