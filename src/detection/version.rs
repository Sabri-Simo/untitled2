use crate::detection::project_type::*;
use std::process::Command;
/*pub fn get_python_version1() -> Option<String> {
    const CAND: &[&str; 3] = &["python", "python3", "py"];
    for cmd in CAND {
        if let Ok(output) = Command::new(cmd).arg("--version").output() {
            if output.status.success() {
                let version_bytes = if !output.stdout.is_empty() {
                    &output.stdout
                } else {
                    &output.stderr
                };
                let version = String::from_utf8_lossy(version_bytes).trim().to_string();
                if !version.is_empty() {
                    return Some(version.to_string());
                }
            }
        }
    }
    None
}*/

pub fn command_version(cmd: &str, arg: &str) -> Option<String> {
    Command::new(cmd)
        .arg(arg)
        .output()
        .ok()
        .filter(|out| out.status.success())
        .and_then(|out| {
            let bytes = if !out.stdout.is_empty() {
                &out.stdout
            } else {
                &out.stderr
            };
            let version = String::from_utf8_lossy(&bytes).trim().to_string();
            (!version.is_empty()).then_some(version)
        })
}
pub fn get_python_version() -> Option<String> {
    ["python", "python3", "py"]
        .iter()
        .filter_map(|cmd| command_version(cmd, "--version"))
        .next()
}
pub fn go_version_command() -> Option<String> {
    command_version("go", "version")
}

pub fn charp_version() -> Option<String> {
    command_version("dotnet", "--version")
}

pub fn select_command(path: &str) -> Option<String> {
    let project_type = detect_project_type(path);
    match project_type {
        ProjectType::Python(_dep) => get_python_version(),
        ProjectType::CSharp => charp_version(),
        ProjectType::Go => go_version_command(),
        ProjectType::Unknown => None,
    }
}
