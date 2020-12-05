use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use regex::Regex;
use std::convert::TryFrom;


/// Formats the sum of two numbers as string.
#[pyfunction]
fn verify_id(id: String) -> PyResult<bool> {
    let re = Regex::new(r"^[0-9]{13}$").unwrap();

    let is_13_digit = re.is_match(&id);
    let mut is_valid = true;

    if !is_13_digit {
        is_valid = false;
    } else {

        let digits: Vec<u32> = id.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let mut sum = 0;

        for (i, d) in digits[..12].iter().enumerate(){
            sum += d * (13 - u32::try_from(i).unwrap());
        }

        let calculated_checksum = (11 - sum % 11) % 10;
        let expected_checksum = digits[12];

        if calculated_checksum != expected_checksum {
            is_valid = false;
        }
    }

    Ok(is_valid)
}

/// A Python module implemented in Rust.
#[pymodule]
fn konjingjing(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(verify_id, m)?)?;
    Ok(())
}