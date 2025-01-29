extern crate clap;

use std::{collections::HashMap, fs::{self, File}};
use std::process;

use stdf::records::V4;
use stdf::get_endian_from_file;
use stdf::tally::{count_parts, count_records};

use memmap::MmapOptions;
use byte::BytesExt;
use rust_xlsxwriter::{Format, FormatAlign, Workbook};
use clap::{Arg, Command, ArgAction};
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let matches = Command::new("hvcl_tc")
        .version("0.1.0")
        .author("Tom Hören <tom.hoeren@tdk.com>")
        .about("Converts an STDF to an XLSX file for the HVCL temperature check TP.")
        .arg(Arg::new("input_file")
            .short('i')
            .long("input")
            .required(true)
            .help("Sets the input file to use"),
        )
        .arg(Arg::new("progress_bar")
            .short('p')
            .long("progress")
            .required(false)
            .action(ArgAction::SetTrue)
            .help("Displays a status bar while processing"),
        )
        .arg(Arg::new("force")
            .short('f')
            .long("force")
            .required(false)
            .action(ArgAction::SetTrue)
            .help("Forces the overwriting of existig file"),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input_file").unwrap();
    let use_progress_bar = *matches.get_one::<bool>("progress_bar").unwrap_or(&false);

    let mut file = match File::open(input_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file {}: {}", input_file, err);
            process::exit(1);
        }
    };

    let output_file = input_file.clone() + ".xlsx";

    if fs::metadata(output_file.clone()).is_ok() {
        if !*matches.get_one::<bool>("force").unwrap_or(&false) {
            eprintln!("Error: Output file {} already exists. Use -f to force overwrite.", output_file);
            process::exit(1);
        }
    }

    let records = match count_records(&mut file, false) {
        Ok(records) => records,
        Err(err) => {
            eprintln!("Error counting records: {}", err);
            process::exit(1);
        }
    };
    
    let part_count = match count_parts(&mut file){
        Ok(part_count) => part_count,
        Err(err) => {
            eprintln!("Error counting parts: {}", err);
            process::exit(1);
        }
    };

    let endian = match get_endian_from_file(&mut file) {
        Ok(endian) => endian.unwrap(),
        Err(err) => {
            eprintln!("Error determining endianess: {}", err);
            process::exit(1);
        }
    };

    let mmap = unsafe {
        match MmapOptions::new().map(&file) {
            Ok(mmap) => mmap,
            Err(err) => {
                eprintln!("Error memory-mapping file {}: {}", input_file, err);
                process::exit(1);
            }
        }
    };
        
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.write_string(1, 0, "Time [s]").unwrap();
    for (i, test_num) in (18606..=18905).enumerate() {
        let time = ((test_num-18606) as f64 / 10_f64) + 0.1_f64;
        let row = i as u32 + 2;
        worksheet.write_number(row, 0, time).unwrap();
    }

    let bytes = &mmap[..];
    let offset = &mut 0;
 
    let pb = ProgressBar::new(part_count as u64 * 301_u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap());

    let mut loop_map: HashMap<u8, u16> = (1..=8).map(|key| (key, 0)).collect();
    loop {
        match bytes.read_with::<V4>(offset, endian) {
            Ok(v4) => {
                match v4 {
                    V4::MRR(_) => {break},
                    V4::PIR(pir) => {
                        let site_num:u8 = pir.site_num.into();
                        let site_loops = *loop_map.get(&site_num).unwrap();
                        let col: u16 = (8 * site_loops) + site_num as u16;
                        worksheet.write_number(0, col as u16, col as u16).unwrap();
                        worksheet.write_string(1, col, format!("Site{}", site_num)).unwrap();
                    },
                    V4::PRR(prr) => {
                        let site_num:u8 = prr.site_num.into();
                        if let Some(value) = loop_map.get_mut(&site_num) {
                            *value += 1;
                        }
                        if use_progress_bar {
                            pb.inc(1);
                        }
                    },
                    V4::PTR(ptr) => {
                        let test_num: u32 = ptr.test_num.into();
                        if 18606 <= test_num && test_num <= 18905 {
                            let row = test_num - 18606 + 2;
                            let site_num:u8 = ptr.site_num.into();
                            let site_loops = loop_map.get(&site_num).unwrap();
                            let col: u16 = (8 * site_loops) + site_num as u16;
                            let value: f32 = ptr.result.into();
                            worksheet.write_number(row, col, value as f64).unwrap();
                        }
                    },
                    _ => {},
                }
            },
            _ => break,
        };
    }

    match workbook.save(output_file.clone()) {
        Ok(_) => println!("File saved to {}", output_file),
        Err(err) => {
            eprintln!("Error saving file {}: {}", output_file, err);
            process::exit(1);
        }
    }
    if use_progress_bar {
        pb.inc(1);
        pb.finish_and_clear();
    }
}    