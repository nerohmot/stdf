extern crate byte;

#[macro_use]
extern crate stdf_record_derive;

pub mod records;
pub mod types;
pub mod conversions;
pub mod tally;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Result};
use byte::ctx::Endian;

pub fn mrr_offset_in_file(file: &mut File) -> Option<u64> {
    let endian = match get_endian_from_file(file).unwrap() {
        Some(endian) => endian,
        None => panic!("Endianess not detected"),
    };
    let saved_position = file.seek(SeekFrom::Current(0)).unwrap();
    let file_length = file.seek(SeekFrom::End(0)).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

    let mut rec_len = [0_u8; 2];
    let mut rec_typ = [0_u8; 1];
    let mut rec_sub = [0_u8; 1];
    let mut pos:u64 = 0;
    
    loop {
        if file_length - pos < 4 { break; }
        file.read_exact(&mut rec_len).unwrap();
        file.read_exact(&mut rec_typ).unwrap();
        file.read_exact(&mut rec_sub).unwrap();
        let rec_size = match endian {
            Endian::Little => u16::from_le_bytes(rec_len) as i64,
            Endian::Big => u16::from_be_bytes(rec_len) as i64,
        };
        if file_length - (pos+4) < rec_size as u64 { break; }
        match (rec_typ[0], rec_sub[0]) {
            (1, 20) => {
                file.seek(SeekFrom::Start(saved_position)).unwrap();
                return Some(pos);
            },
            _ => {
                pos += 4 + rec_size as u64;
            },
        }
        file.seek(SeekFrom::Current(rec_size)).unwrap(); // skip the record data
    }
    file.seek(SeekFrom::Start(saved_position)).unwrap();
    None
}

/// Indexes the records in an STDF (Standard Test Data Format) file.
///
/// This function reads through an STDF file and creates an index of the records
/// based on their type and subtype. The index is a `HashMap` where the keys are
/// tuples of `(record_type, record_subtype)` and the values are vectors of file
/// positions where the records are located.
///
/// # Arguments
///
/// * `file` - A mutable reference to the STDF file to be indexed.
///
/// # Returns
///
/// A `Result` containing a `HashMap` with the index of the records if successful,
/// or an `io::Error` if an error occurs during file operations.
///
/// # Errors
///
/// This function will return an error if there are issues reading from the file
/// or seeking within the file.
///
/// # Panics
///
/// This function will panic if the endianness of the file cannot be determined.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::Result;
/// use std::collections::HashMap;
/// use stdf::get_index_from_stdf_file;
/// 
/// fn main() -> Result<()> {
///     let mut file = File::open("tests/fixtures/test.stdf")?;
///     let index = get_index_from_stdf_file(&mut file)?;
///     println!("{:?}", index);
///     Ok(())
/// }
/// ```
pub fn get_index_from_stdf_file(file: &mut File) -> Result<HashMap<(u8, u8), Vec<u64>>> {
    let mut index = HashMap::new();
    let endian = get_endian_from_file(file)?;
    if endian.is_none() {
        panic!("Endianess not detected");
    } 

    let saved_position = file.seek(SeekFrom::Current(0))?;
    let file_length = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(0))?;

    let mut rec_len = [0_u8; 2];
    let mut rec_typ = [0_u8; 1];
    let mut rec_sub = [0_u8; 1];
    let mut pos:u64 = 0;
    
    loop {
        // FIXME: What if the file grows while we process it?
        if file_length - pos < 4 { break; }
        file.read_exact(&mut rec_len)?;
        file.read_exact(&mut rec_typ)?;
        file.read_exact(&mut rec_sub)?;
        let rec_size = match endian {
            Some(Endian::Little) => u16::from_le_bytes(rec_len) as i64,
            Some(Endian::Big) => u16::from_be_bytes(rec_len) as i64,
            None => panic!("Endianess not detected"),
        };
        if file_length - pos < rec_size as u64 { break; }
        if index.contains_key(&(rec_typ[0], rec_sub[0])) {
            let vec:&mut Vec<u64>  = index.get_mut(&(rec_typ[0], rec_sub[0])).unwrap();
            vec.push(pos.clone());
        } else {
            index.insert((rec_typ[0], rec_sub[0]), vec![pos.clone()]);
        }
        //FIXME: do not continue after a MRR record is found
        pos += 4 + rec_size as u64;
        file.seek(SeekFrom::Current(rec_size))?; // skip the record data
    }
    file.seek(SeekFrom::Start(saved_position))?;
    Ok(index)
}

/// Determines the endianness of a file based on its content.
///
/// This function reads the FAR record of the file to determine its endianness.
/// It checks specific byte patterns to identify whether the file is an STDF
/// file or not and if so it is in little-endian or big-endian format.
///
/// # Arguments
///
/// * `file` - A mutable reference to an open file handle.
///
/// # Returns
///
/// * `Ok(Some(Endian::Little))` if the file is determined to be an STDF file in little-endian format.
/// * `Ok(Some(Endian::Big))` if the file is determined to be an STDF file in big-endian format.
/// * `Ok(None)` if the file is determined not to be an STDF file.
/// * `Err` if an I/O error occurs.
///
/// # Errors
///
/// This function will return an error if any I/O operation fails.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::Result;
/// use byte::ctx::Endian;
/// use stdf::get_endian_from_file;
/// 
/// fn main() -> Result<()> {
///     let mut file = File::open("tests/fixtures/test.stdf")?;
///     match get_endian_from_file(&mut file)? {
///         Some(Endian::Little) => println!("File is little-endian"),
///         Some(Endian::Big) => println!("File is big-endian"),
///         None => println!("File is not an STDF file"),
///     }
///     Ok(())
/// }
/// ```
pub fn get_endian_from_file(file: &mut File) -> Result<Option<Endian>> {
    let saved_position = file.seek(SeekFrom::Current(0))?;
    let end_pos = file.seek(SeekFrom::End(0))?;
    if end_pos < 6 {
        return Ok(None);
    }
    file.seek(SeekFrom::Start(0))?;
    let mut rec_len = [0; 2];
    file.read_exact(&mut rec_len)?;
    let mut rec_typ = [0; 1];
    file.read_exact(&mut rec_typ)?;
    let mut rec_sub = [0; 1];
    file.read_exact(&mut rec_sub)?;
    file.seek(SeekFrom::Start(saved_position))?;

    if rec_typ[0] == 0_u8 && rec_sub[0] == 10_u8 {
        if u16::from_le_bytes(rec_len) == 2 {
            Ok(Some(Endian::Little))
        } else {
            if u16::from_be_bytes(rec_len) == 2 {
                Ok(Some(Endian::Big))
            } else {
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempfile;

    #[test]
    fn test_get_endian_from_file_none_file_too_short() {
        let mut file = tempfile().unwrap();
        let test_data: &[u8] = &[0x00, 0x02, 0x00];
        file.write_all(test_data).unwrap();
        let endian = get_endian_from_file(&mut file).unwrap();
        drop(file);
        assert_eq!(endian, None);
    }

    #[test]
    fn test_get_endian_from_file_none_typ_or_sub_fail() {
        let mut file = tempfile().unwrap();
        let test_data: &[u8] = &[0x00, 0x02, 0xAA, 0x55];
        file.write_all(test_data).unwrap();
        let endian = get_endian_from_file(&mut file).unwrap();
        drop(file);
        assert_eq!(endian, None);
    }

    #[test]
    fn test_get_endian_from_file_none_rec_len_fail() {
        let mut file = tempfile().unwrap();
        let test_data: &[u8] = &[0x00, 0x03, 0x00, 0x0A];
        file.write_all(test_data).unwrap();
        let endian = get_endian_from_file(&mut file).unwrap();
        assert_eq!(endian, None);
    }

    // #[test]
    // fn test_get_endian_from_file_be() {
    //     let mut file = tempfile().unwrap();
    //     let test_data: &[u8] = &[0x02, 0x00, 0x00, 0x0A];
    //     file.write_all(test_data).unwrap();
    //     let endian = get_endian_from_file(&mut file).unwrap();
    //     assert_eq!(endian, Some(Endian::Big));
    //     drop(file);
    // }

    // #[test]
    // fn test_get_endian_from_file_le() {
    //     let mut file = tempfile().unwrap();
    //     let test_data: &[u8] = &[0x02, 0x00, 0x00, 0x0A];
    //     file.write_all(test_data).unwrap();
    //     let endian = get_endian_from_file(&mut file).unwrap();
    //     assert_eq!(endian, Some(Endian::Little));
    //     drop(file);
    // }
}

