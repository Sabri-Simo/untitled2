/*#[cfg(windows)]
use winreg::RegKey;
#[cfg(windows)]
use winreg::enums::*;

use std::{
    env,
    io::{self, Write},
    path::Path,
};

pub fn add_to_path_windows(exe_dir: &str) -> std::io::Result<()> {
    // Open HKEY_CURRENT_USER\Environment for writing PATH
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env_key = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

    // Get current PATH
    let current_path: String = env_key.get_value("Path").unwrap_or_default();

    // Add exe_dir if not already present
    if !current_path.contains(exe_dir) {
        let new_path = format!("{};{}", current_path, exe_dir);
        env_key.set_value("Path", &new_path)?;
        println!("✅ Dockerit added to PATH. Restart terminal to use 'dockerit' globally.");
    } else {
        println!("Dockerit is already in PATH.");
    }

    Ok(())
}
#[cfg(windows)]
pub fn prompt_add_to_path() {
    let exe_dir = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    println!("Dockerit Windows installer");
    print!("Do you want to add Dockerit to your PATH? (y/N): ");
    io::stdout().flush().unwrap();

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();

    if answer.trim().eq_ignore_ascii_case("y") {
        if let Err(e) = add_to_path_windows(&exe_dir) {
            eprintln!("❌ Failed to add to PATH: {}", e);
        }
    } else {
        println!("Skipping PATH modification.");
    }
}
*/
