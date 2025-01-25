extern crate clap;

use clap::{Arg, Command, crate_version, crate_authors, ArgAction};
use stdf::records::V4;
use std::fs::File;
use std::process;
use stdf::{get_endian_from_file, get_index_from_file};
use memmap::MmapOptions;
use byte::BytesExt;

fn main() {
    let matches = Command::new("stdf")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Standard Test Data Format (STDF) data processing")
        .subcommand(
            Command::new("endian")
                .about("Determines the endian of the given STDF file.")
                .arg(
                    Arg::new("input_file")
                        .short('i')
                        .long("input_file")
                        .required(true)
                        .help("Sets the input file to use"),
                ),
        )
        .subcommand(
            Command::new("records")
                .about("Lists the records in the STDF file."),
        )
        .subcommand(
            Command::new("dump")
                .about("Dumps the STDF file in a more readable form to the console.")
                .arg(
                    Arg::new("input_file")
                        .short('i')
                        .long("input_file")
                        .required(true)
                        .help("Sets the input file to use"),
                )
                .arg(
                    Arg::new("records")
                        .short('r')
                        .long("records")
                        .required(false)
                        .help("Sets the list of records to dump. (see `stdf records` for a valid list of records)`"),
                ),
        )
        .subcommand(Command::new("count")
            .about("Counts various thing in the STDF file")
            .subcommand(Command::new("records")
                .about("Counts the records in the STDF file")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("verbosity")
                    .short('v')
                    .long("verbose")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .help("Sets the verbosity to high"),
                )
                .arg(Arg::new("record_list")
                    .short('r')
                    .long("records")
                    .required(false)
                    .num_args(1..)
                    .help("Limits the records to count to the given ones"),
                ),
            )
            .subcommand(Command::new("parts")
                .about("Counts the parts in the STDF file")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("verbosity")
                    .short('v')
                    .long("verbose")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .help("Sets the verbosity to high"),
                )
            )
            .subcommand(Command::new("tests")
                .about("Counts the number of unique tests in the STDF file")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("verbosity")
                    .short('v')
                    .long("verbose")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .help("Sets the verbosity to high"),
                )
            )
                .subcommand(Command::new("sites")
                .about("Counts the number of sites in the STDF file")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            )
                .subcommand(Command::new("heads")
                .about("Counts the number of heads in the STDF file")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            ), 
         )
        .subcommand(
            Command::new("to")
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
        Some(("records", sub_m)) => {
            println!("FAR : File Attributes Record");
            println!("ATR : Audit Trail Record");
            println!("MIR : Master Information Record");
            println!("MRR : Master Results Record");
            println!("PCR : Part Count Record");
            println!("HBR : Hardware Bin Record");
            println!("SBR : Software Bin Record");
            println!("PMR : Pin Map Record");
            println!("PGR : Pin Group Record");
            println!("PLR : Pin List Record");
            println!("RDR : Retest Data Record");
            println!("SDR : Site Description Record");
            println!("WIR : Wafer Information Record");
            println!("WRR : Wafer Results Record");
            println!("WCR : Wafer Configuration Record");
            println!("PIR : Part Information Record");
            println!("PRR : Part Results Record");
            println!("TSR : Test Synopsis Record");
            println!("PTR : Parametric Test Record");
            println!("MPR : Multiple-Result Parametric Record");
            println!("FTR : Functional Test Record");
            println!("BPS : Begin Program Section Record");
            println!("EPS : End Program Section Record");
            println!("GDR : Generic Data Record");
            println!("DTR : Data Record");
        }
        Some(("endian", sub_m)) => {
            let input_file = sub_m.get_one::<String>("input_file").unwrap();
            let file = File::open(input_file);
            if file.is_err() {
                println!("Error: {}", file.err().unwrap());
                process::exit(1);
            }
            match get_endian_from_file(&mut file.unwrap()) {
                Ok(Some(endian)) => {
                    match endian {
                        byte::ctx::Endian::Big => println!("BE"),
                        byte::ctx::Endian::Little => println!("LE")
                    }
                },
                Ok(None) => {
                    println!("Error: NO STDF file!");
                    process::exit(1);
                },
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        Some(("dump", sub_m)) => {
            let input_file_name = sub_m.get_one::<String>("input_file").unwrap();
            let mut input_file = File::open(input_file_name).unwrap();
            let endian = match get_endian_from_file(&mut input_file) {
                Ok(Some(endian)) => endian,
                Ok(None) => {
                    println!("Error: NO STDF file!");
                    process::exit(1);
                },
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(1);
                }
            };

            let m = unsafe { MmapOptions::new().map(&input_file).unwrap() };
            let bytes = &m[..];
            let offset = &mut 0;
            loop {
                match bytes.read_with::<V4>(offset, endian) {
                    Ok(v4) => println!("{:?}", v4),
                    // Err(byte::Error::BadOffset(x)) => println!("Error : bad offset {} before EOF", x),
                    // Err(e) => println!("Error : {:?}", e),
                    _ => {}
                };
            }
        }
        Some(("count", sub_m)) => {
            match sub_m.subcommand() {
                Some(("records", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut file = File::open(input_file).unwrap();
                    let index = get_index_from_file(&mut file).unwrap();
                    for (key, value) in index.iter() {
                        println!("({}, {}) : {}", key.0, key.1, value.len());
                    }
                


                    // println!("{:?}", index);
                }
                Some(("parts", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                }
                Some(("tests", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                }
                Some(("sites", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                }
                Some(("heads", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                }
                _ => eprintln!("No valid subcommand was used for convert_to"),
            }
        }
        Some(("to", sub_m)) => {
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