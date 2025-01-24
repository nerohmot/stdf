extern crate byte;

#[macro_use]
extern crate stdf_record_derive;

pub mod records;
pub mod types;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Result};
use byte::ctx::Endian;

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
pub fn get_endian_from_file(file: &mut File) -> Result<Option<Endian>> {
    let saved_position = file.seek(SeekFrom::Current(0))?;
    if file.seek(SeekFrom::End(0))? < 4 {
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
mod lib {
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

    #[test]
    fn test_get_endian_from_file_be() {
        let mut file = tempfile().unwrap();
        let test_data: &[u8] = &[0x00, 0x02, 0x00, 0x0A];
        file.write_all(test_data).unwrap();
        let endian = get_endian_from_file(&mut file).unwrap();
        drop(file);
        assert_eq!(endian, Some(Endian::Big));
    }

    #[test]
    fn test_get_endian_from_file_le() {
        let mut file = tempfile().unwrap();
        let test_data: &[u8] = &[0x02, 0x00, 0x00, 0x0A];
        file.write_all(test_data).unwrap();
        let endian = get_endian_from_file(&mut file).unwrap();
        drop(file);
        assert_eq!(endian, Some(Endian::Little));
    }
}

