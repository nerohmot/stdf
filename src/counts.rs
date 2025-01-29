use std::fs::File;
use std::io::Result;
use std::collections::HashMap;

use crate::get_index_from_file;
use crate::records::typ_sub_to_name;
use std::io::{Error, ErrorKind};

/// Counts the records in an STDF (Standard Test Data Format) file and optionally prints detailed information.
///
/// This function retrieves the index of records using `get_index_from_file`,
/// and counts the number of records. If the `verbose` flag is set to `true`, it prints detailed information
/// about each record type and subtype, as well as the total count of records.
///
/// # Arguments
///
/// * `file` - A mutable reference to an open `File` handle representing the STDF file.
/// * `verbose` - A `bool` flag indicating whether to print detailed information.
///
/// # Returns
///
/// * `Result<Option<u32>>` - The total count of records wrapped in a `Result` and `Option`.
///   - `Ok(Some(record_count))` if the operation is successful.
///   - `Err` if an I/O error occurs.
///
/// # Errors
///
/// This function will return an error if there are issues reading from the file.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::Result;
/// 
/// use stdf::counts::count_records;
///
/// fn main() -> Result<()> {
///     let mut file = File::open("tests/files/test.stdf")?;
///     let record_count = count_records(&mut file, false)?;
///     println!("Total record count: {:?}", record_count);
///     Ok(())
/// }
/// ```
pub fn count_records(file: &mut File, verbose: bool) -> Result<HashMap<String, u32>> {
    let index = get_index_from_file(file)?;
    let mut retval: HashMap<String, u32> = HashMap::new();
    for (key, value) in index.iter(){
        let new_key = typ_sub_to_name(key.0, key.1).clone();
        let new_val = value.len() as u32;
        retval.insert(new_key, new_val);
    }
    let mut record_count: u32 = 0;
    for (key, value) in retval.iter(){
        if key == "???" {
            if verbose {
                println!("{} : ({:>9})", key, value);
            }
        } else {
            if verbose {
                println!("{} : {:>10}", key, value);
            }
            record_count += value;
        }
    }   
    if verbose {
        println!("    + -----------");
        println!("TTL : {:>10}", record_count);
    } 
    Ok(retval)
}

pub fn count_parts(file: &mut File) -> Result<u32> {
    let index = get_index_from_file(file)?;
    let mut records_count: HashMap<String, u32> = HashMap::new();
    for (key, value) in index.iter(){
        let new_key = typ_sub_to_name(key.0, key.1).clone();
        let new_val = value.len() as u32;
        records_count.insert(new_key, new_val);
    }
    let pir_count = records_count.get("PIR").unwrap_or(&0);
    let prr_count = records_count.get("PRR").unwrap_or(&0);
    if *pir_count == 0 || *prr_count == 0 {
        return Err(Error::new(ErrorKind::InvalidData, "Missing PIR or PRR records"));
    }
    if pir_count != prr_count {
        return Err(Error::new(ErrorKind::InvalidData, "Mismatched PIR and PRR records"));
    }
    Ok(*pir_count)
}

