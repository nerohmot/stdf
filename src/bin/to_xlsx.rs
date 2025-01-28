use ndarray::Array2;
use xlsxwriter::*;
use std::collections::HashMap;

#[derive(Debug)]
enum Data {
    Bool(bool),
    Float(f64),
    Text(String),
}

pub fn create_2d_array(index: HashMap<(u8, u8), Vec<u64>>) -> Array2<Data> {
    // Determine the size of the array based on the index
    let rows = index.len();
    let cols = 3; // Example number of columns

    // Create a 2D array filled with a default value
    let mut array = Array2::from_elem((rows, cols), Data::Bool(false));

    // Fill the array with different types of data
    for ((i, j), elem) in array.indexed_iter_mut() {
        *elem = match (i, j) {
            (0, 0) => Data::Bool(true),
            (0, 1) => Data::Float(3.14),
            (0, 2) => Data::Text("Hello".to_string()),
            _ => Data::Bool(false),
        };
    }

    array
}

pub fn save_to_xlsx(array: &Array2<Data>, file_path: &str) -> Result<(), XlsxError> {
    let workbook = Workbook::new(file_path);
    let mut sheet = workbook.add_worksheet(None)?;

    for ((i, j), elem) in array.indexed_iter() {
        match elem {
            Data::Bool(value) => {
                sheet.write_number(i as u32, j as u16, value, None)?;
            }
            Data::Float(value) => {
                sheet.write_number(i as u32, j as u16, value, None)?;
            }
            Data::Text(value) => {
                sheet.write_string(i as u32, j as u16, value, None)?;
            }
        }
    }

    workbook.close()
}

fn main() {
    let index = HashMap::new(); // Example index
    let array = create_2d_array(index);
    println!("Mixed 2D array:\n{:?}", array);

    if let Err(e) = save_to_xlsx(&array, "output.xlsx") {
        eprintln!("Error saving to xlsx: {}", e);
    }
}