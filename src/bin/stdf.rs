extern crate clap;

use clap::{Arg, Command, crate_version, crate_authors, ArgAction};

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
                        )
                        .arg(
                            Arg::new("output_file")
                                .short('o')
                                .long("output_file")
                                .required(false)
                                .help("Sets the output file to use"),
                        )
                        .arg(
                            Arg::new("progress_bar")
                                .short('p')
                                .long("progress_bar")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .help("Displays a status bar while processing"),
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
                        )
                        .arg(
                            Arg::new("output_file")
                                .short('o')
                                .long("output_file")
                                .required(false)
                                .help("Sets the output file to use"),
                        )
                        .arg(
                            Arg::new("progress_bar")
                                .short('p')
                                .long("progress_bar")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .help("Displays a status bar while processing"),
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
                    let default_output_file = format!("{}.csv", input_file);
                    let output_file = sub_sub_m.get_one::<String>("output_file").unwrap_or(&default_output_file);
                    println!("Converting the STDF file '{}' to CSV file '{}'", input_file, output_file);
                    println!("Progress bar: {}", sub_sub_m.get_flag("progress_bar"));
                    // Add your logic for the "csv" subcommand here
                }
                Some(("xlsx", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let default_output_file = format!("{}.xlsx", input_file);
                    let output_file = sub_sub_m.get_one::<String>("output_file").unwrap_or(&default_output_file);
                    println!("Converting the STDF file '{}' to XLSX file '{}'", input_file, output_file);
                    // Add your logic for the "xlsx" subcommand here
                }
                _ => eprintln!("No valid subcommand was used for convert_to"),
            }
        }
        _ => eprintln!("No valid subcommand was used"),
    }
}