/// Provides functions for interacting with the `gsettings` command-line utility.
///
/// This module contains two functions: `get` and `set`, which allow you to retrieve and update
/// fields in the GSettings configuration database using the `gsettings` command.
use std::process::Command;

/// Retrieves the field of a given key from the GSettings configuration database.
///
/// # Arguments
///
/// * `key` - The key for the `gsettings` command.
/// * `field` - The field for the `gsettings` command.
///
/// # Returns
///
/// The output of the `gsettings` command as a `Result` containing a `String` if successful,
/// or an error message as a `String` if unsuccessful.
pub fn get(key: &str, field: &str) -> String {
    let output = Command::new("gsettings")
        .arg("get")
        .arg(key)
        .arg(field)
        .output()
        .expect("Failed to get data");

    if output.status.success() {
        format!("{}", String::from_utf8_lossy(&output.stdout))
    } else {
        "Error retrieving field".into()
    }
}

/// Sets the field of a given key in the GSettings configuration database.
///
/// # Arguments
///
/// * `key` - The key for the `gsettings` command
/// * `field` - The field to look for of the provided `key`
/// * `value` - The value to be set for the specified `field`
///
/// # Returns
///
/// The output of the `gsettings` command as a `Result` containing a success message as a `String`
/// if the key field is updated successfully, or an error message as a `String` if unsuccessful.
pub fn set(key: &str, field: &str, value: &str) -> Result<String, String> {
    let output = Command::new("gsettings")
        .arg("set")
        .arg(key)
        .arg(field)
        .arg(value)
        .output()
        .expect("Failed to get data");

    if output.status.success() {
        Ok("Value updated successfully!".into())
    } else {
        Err("Error setting field value".into())
    }
}