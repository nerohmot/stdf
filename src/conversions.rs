// use std::{collections::HashMap, fs::{self, File}};
// use std::process;

// use crate::get_endian_from_file;
// use crate::tally::{count_parts, count_records};
// use crate::records::V4;

// use memmap::MmapOptions;
// use byte::BytesExt;
// // use rust_xlsxwriter::{Format, FormatAlign, Workbook};
// use clap::{Arg, Command, ArgAction};
// use indicatif::{ProgressBar, ProgressStyle};

// pub fn stdf_to_xls(input_file: &str, use_progress_bar: bool, force: bool) -> Result<()> {
//     let mut file = File::open(input_file)?;
//     let output_file = input_file.to_string() + ".xlsx";
//     if fs::metadata(output_file.clone()).is_ok() {
//         if !force {
//             eprintln!("Error: Output file {} already exists. Use -f to force overwrite.", output_file);
//             process::exit(1);
//         }
//     }
//     let records = count_records(&mut file, false)?;
//     let part_count = count_parts(&mut file)?;
//     let endian = get_endian_from_file(&mut file)?;
//     let mmap = unsafe {
//         MmapOptions::new().map(&file)?
//     };
//     let mut workbook = Workbook::new();
//     let worksheet = workbook.add_worksheet();
//     //TODO: implement the rest of the function
//     Ok(())
// }
