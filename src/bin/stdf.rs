extern crate clap;

use clap::{Arg, Command, crate_version, crate_authors, ArgAction, value_parser};
use stdf::records::{PRR, V4, typ_sub_to_name, is_supported_records};

use std::{fs::File, io::{Seek, SeekFrom}};
use std::process;

use stdf::{get_endian_from_file, get_index_from_file, mrr_offset_in_file};
use stdf::conversions::dummy_function;
use stdf::counts::count_records;

use memmap::MmapOptions;
use byte::BytesExt;

fn main() {
    let matches = Command::new("stdf")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Standard Test Data Format (STDF) data processing.")
        .subcommand(Command::new("endian")
            .about("Determines the endian of the given STDF file.")
            .arg(Arg::new("input_file")
                .short('i')
                .long("input_file")
                .required(true)
                .help("Sets the input file to use"),
            ),
        )
        .subcommand(Command::new("play")
            .about("Play with the STDF file.")
            .arg(Arg::new("input_file")
                .short('i')
                .long("input_file")
                .required(true)
                .help("Sets the input file to use"),
            ),
        )
        .subcommand(Command::new("list")
            .about("List various capabilities.")
            .alias("show")
            .subcommand(Command::new("records")
                .about("Lists the supported record names."),
            )
            .subcommand(Command::new("types")
                .about("Lists the supported types."),
            )
        )
        .subcommand(Command::new("create")
            .about("Creates (duplicates) the input file to the output file, but waits some time between the writing of the records.")
            .alias("dupicate")
            .arg(Arg::new("input_file")
                .short('i')
                .long("input")
                .required(true)
                .help("Sets the input file to use"),
            )
            .arg(Arg::new("output_file")
                .short('o')
                .long("output")
                .required(true)
                .help("Sets the output file to use"),
            )
            .arg(Arg::new("ms")
                .short('t')
                .long("time")
                .required(false)
                .default_value("100")
                .value_parser(value_parser!(u64).range(0..=10000))
                .help("Sets the time in ms to wait between writing records"),
            )
            .arg(Arg::new("progress_bar")
                .short('p')
                .long("progress")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Displays a status bar while processing"),
            ),
        )
        .subcommand(Command::new("is")
            .about("Checks various things on the STDF file.")
            .subcommand(Command::new("ws")
                .about("Checks if the STDF file comes from Wafer Sort.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            )
            .subcommand(Command::new("ft")
                .about("Checks if the STDF file comes from Final Test.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            )
            .subcommand(Command::new("be")
                .about("Checks if the STDF file is in Big Endian format.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            )
            .subcommand(Command::new("le")
                .about("Checks if the STDF file is in Little Endian format.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            )
            .subcommand(Command::new("clean")
                .about("Checks if the STDF file is clean (ends on an MRR).")
                .alias("finished")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            )
            .subcommand(Command::new("retest")
                .about("Checks if the STDF file holds retest data.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            )
            .subcommand(Command::new("concatenable")
                .about("Checks if the STDF files are concatenable.")
                .alias("concat")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input")
                    .required(true)
                    .num_args(2)
                    .help("Sets the input files to use"),
                )
            )
        )
        .subcommand(Command::new("count")
            .about("Counts various thing in the STDF file.")
            .subcommand(Command::new("records")
                .about("Counts the records in the STDF file.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("verbose")
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
                    .help("Limits the records to counted to the given ones"),
                ),
            )
            .subcommand(Command::new("parts")
                .about("Counts the parts in the STDF file.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            )
            .subcommand(Command::new("yield")
                .about("Calculates the yield in the parts of the STDF file.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .help("Sets the verbosity to high"),
                )
            )
            .subcommand(Command::new("tests")
                .about("Counts the number of unique tests in the STDF file.")
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
                .about("Counts the number of sites in the STDF file.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            )
            .subcommand(Command::new("heads")
                .about("Counts the number of heads in the STDF file.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            )
        ) 
        .subcommand(Command::new("dump")
            .about("Dumps various things of the STDF file in a more readable form to the console.")
            .subcommand(Command::new("record")
                .about("Dumps the record at a postion of the STDF file.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("offset")
                    .long("offset")
                    .value_parser(clap::value_parser!(u64))
                    .required(true)
                    .help("Sets the offset in the STDF of the record to dump"),
                ),
            )
            .subcommand(Command::new("records")
                .about("Dumps the records of the STDF file.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("records")
                    .short('r')
                    .long("records")
                    .required(false)
                    .num_args(1..)
                    .help("Sets the list of records to dump\n`stdf list records` for a list of valid records\nInvalid records will be ignored"),
                ),
            )
            .subcommand(Command::new("parts")
                .about("Dumps the parts of the STDF file.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            )
            .subcommand(Command::new("index")
                .about("Dumps the index of the STDF file.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input_file")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("records")
                    .short('r')
                    .long("records")
                    .required(false)
                    .num_args(1..)
                    .help("Sets the list of records to dump. (see `stdf records` for a valid list of records)`"),
                ),
            )
            .subcommand(Command::new("length")
                .about("Dumps the length of the STDF file.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input")
                    .required(true)
                    .help("Sets the input file to use"),
                )
            )
        )
        .subcommand(Command::new("to")
            .about("Converts the STDF file into another format.")
            .subcommand(Command::new("csv")
                .about("Converts the STDF file to CSV format.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("output_file")
                    .short('o')
                    .long("output")
                    .required(false)
                    .help("Sets the output file to use"),
                )
                .arg(Arg::new("progress_bar")
                    .short('p')
                    .long("progress_bar")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .help("Displays a status bar while processing"),
                ),
            )
            .subcommand(Command::new("xlsx")
                .about("Converts the STDF file to XLSX format.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("output_file")
                    .short('o')
                    .long("output")
                    .required(false)
                    .help("Sets the output file to use"),
                )
                .arg(Arg::new("progress_bar")
                    .short('p')
                    .long("progress")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .help("Displays a status bar while processing"),
                ),
            )
            .subcommand(Command::new("be")
                .about("Converts the STDF file to Big Endian format.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("output_file")
                    .short('o')
                    .long("output")
                    .required(false)
                    .help("Sets the output file to use"),
                )
                .arg(Arg::new("progress_bar")
                    .short('p')
                    .long("progress")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .help("Displays a status bar while processing"),
                ),
            )
            .subcommand(Command::new("le")
                .about("Converts the STDF file to Little Endian format.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("output_file")
                    .short('o')
                    .long("output")
                    .required(false)
                    .help("Sets the output file to use"),
                )
                .arg(Arg::new("progress_bar")
                    .short('p')
                    .long("progress")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .help("Displays a status bar while processing"),
                ),
            )
            .subcommand(Command::new("npy")
                .about("Converts the STDF file to Numpy format.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("output_file")
                    .short('o')
                    .long("output")
                    .required(false)
                    .help("Sets the output file to use"),
                )
                .arg(Arg::new("progress_bar")
                    .short('p')
                    .long("progress")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .help("Displays a status bar while processing"),
                ),
            )
            .subcommand(Command::new("hdf5")
                .about("Converts the STDF file to HDF5 format.")
                .arg(Arg::new("input_file")
                    .short('i')
                    .long("input")
                    .required(true)
                    .help("Sets the input file to use"),
                )
                .arg(Arg::new("output_file")
                    .short('o')
                    .long("output")
                    .required(false)
                    .help("Sets the output file to use"),
                )
                .arg(Arg::new("progress_bar")
                    .short('p')
                    .long("progress")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .help("Displays a status bar while processing"),
                ),
            ),
        )
        .get_matches();

    match matches.subcommand() {
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
                    eprintln!("Error: NO STDF file!");
                    process::exit(1);
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        Some(("play", sub_m)) => {
            let file_name = sub_m.get_one::<String>("input_file").unwrap().to_string();
            let mut file = File::open(file_name).unwrap();
            match count_records(&mut file, true) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        Some(("list", sub_m)) => {
            match sub_m.subcommand()    {
                Some(("records", _)) => {
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
                Some(("types", _)) => {
                    println!("types ...");
                }
                _ => eprintln!("No valid subcommand was used for list"),
            }
        }
        Some(("is", sub_m)) => {
            match sub_m.subcommand() {
                Some(("ws", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut file = File::open(input_file).unwrap();
                    let endian = get_endian_from_file(&mut file).unwrap();
                    if endian.is_none() {
                        panic!("Endianess not detected");
                    }
                }
                Some(("ft", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut file = File::open(input_file).unwrap();
                    let endian = get_endian_from_file(&mut file).unwrap();
                    if endian.is_none() {
                        panic!("Endianess not detected");
                    }
                }
                Some(("be", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut file = File::open(input_file).unwrap();
                    let endian = get_endian_from_file(&mut file).unwrap();
                    if endian.is_none() {
                        panic!("Endianess not detected");
                    }
                }
                Some(("le", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut file = File::open(input_file).unwrap();
                    let endian = get_endian_from_file(&mut file).unwrap();
                    if endian.is_none() {
                        panic!("Endianess not detected");
                    }
                }
                Some(("clean", sub_sub_m)) => {
                    let file_name = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut file = match File::open(file_name) {
                        Ok(file) => file,
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            process::exit(1);
                        }
                    };
                    file.seek(SeekFrom::Start(0)).unwrap();
                    let retval = true;
                    //TODO: implement has_mrr_at_end
                    // let retval = match has_mrr_at_end(&mut file){
                    //     Ok(retval) => retval,
                    //     Err(e) => {
                    //         eprintln!("Error: {}", e);
                    //         process::exit(1);
                    //     }
                    // };
                    if retval {
                        println!("OK");
                    } else {
                        println!("NO MRR");
                        process::exit(1);
                    }
                }
                Some(("retest", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut file = File::open(input_file).unwrap();
                    let endian = get_endian_from_file(&mut file).unwrap();
                    if endian.is_none() {
                        panic!("Endianess not detected");
                    }
                }
                Some(("concatenable", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut file = File::open(input_file).unwrap();
                    let endian = get_endian_from_file(&mut file).unwrap();
                    if endian.is_none() {
                        panic!("Endianess not detected");
                    }
                }
                _ => eprintln!("No valid subcommand was used for is"),
            }
        }
        Some(("count", sub_m)) => {
            match sub_m.subcommand() {
                Some(("records", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut file = File::open(input_file).unwrap();
                    let index = get_index_from_file(&mut file).unwrap();
                    
                    let mut record_count: u64 = 0;
                    for (key, value) in index.iter() {
                        match typ_sub_to_name(key.0, key.1).as_str() {
                            // "MRR" => {
                            //     if sub_sub_m.get_flag("verbose") {
                            //         println!("{} : {:>10} -> {:?}", typ_sub_to_name(key.0, key.1), value.len(), value);
                            //     }
                            //     record_count += value.len() as u64;
                            // },
                            "???" => {
                                if sub_sub_m.get_flag("verbose") {
                                    println!("{} : ({:>9})", typ_sub_to_name(key.0, key.1), value.len());
                                }
                            },
                            _ => {
                                if sub_sub_m.get_flag("verbose") {
                                    println!("{} : {:>10}", typ_sub_to_name(key.0, key.1), value.len());
                                }
                                record_count += value.len() as u64;
                            },
                        }
                    }
                    if sub_sub_m.get_flag("verbose") {
                        println!("    + -----------");
                        println!("TTL : {:>10}", record_count);
                    } else {
                        println!("{}", record_count);
                    }
                }
                Some(("parts", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut file = File::open(input_file).unwrap();
                    let index = get_index_from_file(&mut file).unwrap();

                    let empty_vec: Vec<u64> = Vec::new();
                    let part_count_pir = index.get(&(5, 10)).unwrap_or(&empty_vec).len() as u64;
                    let part_count_prr = index.get(&(5, 20)).unwrap_or(&empty_vec).len() as u64;
                    if part_count_pir == part_count_prr {
                        println!("{}", part_count_pir);
                    } else {
                        eprintln!("Error: PIR and PRR part count mismatch");
                        process::exit(1);
                    }
                }
                Some(("yield", sub_sub_m)) => {
                    let input_file = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut file = File::open(input_file).unwrap();
                    let endian = get_endian_from_file(&mut file).unwrap().unwrap();
                    let index = get_index_from_file(&mut file).unwrap();
                    
                    let m = unsafe { MmapOptions::new().map(&file).unwrap() };
                    let bytes = &m[..];

                    let offset = &mut 0;

                    let mut pass_count: u64 = 0;
                    let mut fail_count: u64 = 0;
                    for (key, value) in index.iter() {
                        if (key.0, key.1) == (5, 20) { // PRR
                            for pos in value.iter() {
                                *offset = *pos as usize;
                                match bytes.read_with::<PRR>(offset, endian) {
                                    Ok(prr) =>{
                                        if prr.part_flg.0 & 0b00011000 == 0 {
                                            pass_count += 1;
                                        } else {
                                            fail_count += 1;
                                        }
                                    },
                                    _ => break,
                                };
                            }
                        }
                    }
                    let yeild = (pass_count as f64 / (pass_count + fail_count) as f64) * 100.0;
                    let total = pass_count + fail_count;
                    if sub_sub_m.get_flag("verbose") {
                        println!("{}/{}={:.2}%", pass_count, total, yeild);
                    } else {
                        println!("{:.2}%", yeild);
                    }
                }
                Some(("tests", sub_sub_m)) => {
                    let input_file_name = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut input_file = File::open(input_file_name).unwrap();
                    let endian = get_endian_from_file(&mut input_file).unwrap();
                    if endian.is_none() {
                        panic!("Endianess not detected");
                    }
                }
                Some(("sites", sub_sub_m)) => {
                    let input_file_name = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut input_file = File::open(input_file_name).unwrap();
                    let endian = get_endian_from_file(&mut input_file).unwrap();
                    if endian.is_none() {
                        panic!("Endianess not detected");
                    }
                }
                Some(("heads", sub_sub_m)) => {
                    let input_file_name = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut input_file = File::open(input_file_name).unwrap();
                    let endian = get_endian_from_file(&mut input_file).unwrap();
                    if endian.is_none() {
                        panic!("Endianess not detected");
                    }
                }
                _ => eprintln!("No valid subcommand was used for convert_to"),
            }
        }
        Some(("dump", sub_m)) => {
            match sub_m.subcommand() {
                Some(("record", sub_sub_m)) => {
                    let input_file_name = sub_sub_m.get_one::<String>("input_file").unwrap();
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
                    let current_position = input_file.seek(SeekFrom::Current(0)).unwrap();
                    let m = unsafe { MmapOptions::new().map(&input_file).unwrap() };
                    let bytes = &m[..];
                    let offset_u64 = sub_sub_m.get_one::<u64>("offset").unwrap();
                    let mut offset = *offset_u64 as usize;
                    match bytes.read_with::<V4>(&mut offset, endian) {
                        Ok(v4) => println!("{:?}", v4),
                        _ => println!("Error: Bad offset"),
                    };
                    input_file.seek(SeekFrom::Start(current_position)).unwrap();
                }
                Some(("records", sub_sub_m)) => {
                    let input_file_name = sub_sub_m.get_one::<String>("input_file").unwrap();
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

                    let all_records = is_supported_records();
                    println!("All records = {:?}", all_records);

                    let records_asked_to_dump=sub_sub_m
                        .get_many::<String>("records")
                        .map(|vals| vals.map(|s| s.to_string()).collect())
                        .unwrap_or_else(|| all_records.clone());
                    println!("Records asked to dump = {:?}", records_asked_to_dump);

                    let records_to_dump: Vec<String> = records_asked_to_dump
                        .into_iter()
                        .filter(|record| all_records.contains(record))
                        .collect();
                    println!("Records to dump = {:?}", records_to_dump);

                    // TODO: let the user know if there are records that are not supported (and removed form the list)
                    // let non_existing_records: Vec<String> = records_asked_to_dump
                    //     .into_iter()
                    //     .filter(|record| !all_records.contains(record))
                    //     .collect();
                    // println!("Non existing records = {:?}", records_asked_to_dump);

                    let m = unsafe { MmapOptions::new().map(&input_file).unwrap() };
                    let bytes = &m[..];
                    let offset = &mut 0;
                    loop {
                        match bytes.read_with::<V4>(offset, endian) {
                            Ok(v4) => {
                                if records_to_dump.contains(&v4.name()) {
                                    match v4 {
                                        V4::FAR(record) => println!("{}", record),
                                        V4::ATR(record) => println!("{}", record),
                                        V4::MIR(record) => println!("{}", record),
                                        V4::MRR(record) => println!("{}", record),
                                        V4::PCR(record) => println!("{}", record),
                                        V4::HBR(record) => println!("{}", record),
                                        V4::SBR(record) => println!("{}", record),
                                        V4::PMR(record) => println!("{}", record),
                                        V4::PGR(record) => println!("{}", record),
                                        V4::PLR(record) => println!("{}", record),
                                        V4::RDR(record) => println!("{}", record),
                                        V4::SDR(record) => println!("{}", record),
                                        V4::WIR(record) => println!("{}", record),
                                        V4::WRR(record) => println!("{}", record),
                                        V4::WCR(record) => println!("{}", record),
                                        V4::PIR(record) => println!("{}", record),
                                        V4::PRR(record) => println!("{}", record),
                                        V4::TSR(record) => println!("{}", record),
                                        V4::PTR(record) => println!("{}", record),
                                        V4::MPR(record) => println!("{}", record),
                                        V4::FTR(record) => println!("{}", record),
                                        V4::BPS(record) => println!("{}", record),
                                        V4::EPS(record) => println!("{}", record),
                                        V4::GDR(record) => println!("{}", record),
                                        V4::DTR(record) => println!("{}", record),
                                        _ => println!("???"),
                                    }
                                }
                            },
                            // Err(byte::Error::BadOffset(x)) => println!("Error : bad offset {} before EOF", x),
                            // Err(e) => println!("Error : {:?}", e),
                            _ => break,
                        };
                    }
                }
                Some(("parts", sub_sub_m)) => {
                    let input_file_name = sub_sub_m.get_one::<String>("input_file").unwrap();
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
                    println!("TO BE IMPLEMENTED {:?}", endian);
                }
                Some(("index", sub_sub_m)) => {
                    let file_name = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut file = File::open(file_name).unwrap();

                    let all_records = is_supported_records();
                    // println!("All records = {:?}", all_records);

                    let records_asked_to_dump=sub_sub_m
                        .get_many::<String>("records")
                        .map(|vals| vals.map(|s| s.to_string()).collect())
                        .unwrap_or_else(|| all_records.clone());
                    // println!("Records asked to dump = {:?}", records_asked_to_dump);

                    let records_to_dump: Vec<String> = records_asked_to_dump
                        .into_iter()
                        .filter(|record| all_records.contains(record))
                        .collect();
                    // println!("Records to dump = {:?}", records_to_dump);

                    // TODO: let the user know if there are records that are not supported (and removed form the list)
                    // let non_existing_records: Vec<String> = records_asked_to_dump
                    //     .into_iter()
                    //     .filter(|record| !all_records.contains(record))
                    //     .collect();
                    // println!("Non existing records = {:?}", records_asked_to_dump);

                    let index = get_index_from_file(&mut file).unwrap();

                    for (key, value) in index.iter() {
                        let record_name  = typ_sub_to_name(key.0, key.1);
                        if records_to_dump.contains(&record_name) {
                            println!("{} : {:?}", typ_sub_to_name(key.0, key.1).replace("\"", ""), value);
                        }
                    }
                }
                Some(("length", sub_sub_m)) => {
                    let input_file_name = sub_sub_m.get_one::<String>("input_file").unwrap();
                    let mut input_file = File::open(input_file_name).unwrap();
                    let file_length = match input_file.seek(SeekFrom::End(0)) {
                        Ok(file_length) => file_length,
                        Err(e) => {
                            println!("Error: {}", e);
                            process::exit(1);
                        }
                    };
                    println!("{}", file_length);
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
                    let a = dummy_function();
                }
                _ => eprintln!("No valid subcommand was used for convert_to"),
            }
        }
        _ => eprintln!("No valid subcommand was used"),
    }
}