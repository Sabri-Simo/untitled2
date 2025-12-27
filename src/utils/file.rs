use std::{fs::File, io::Write};

pub fn write_version(path: impl AsRef<std::path::Path>, version: &str) -> std::io::Result<()> {
    let path = path.as_ref();
    let mut file = File::create(path).map_err(|e| {
        std::io::Error::new(
            e.kind(),
            format!("Failed to create {}: {}", path.display(), e),
        )
    })?;

    file.write_all(version.as_bytes()).map_err(|e| {
        std::io::Error::new(
            e.kind(),
            format!("Failed to write to {}: {}", path.display(), e),
        )
    })?;

    Ok(())
}
