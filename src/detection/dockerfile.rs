use crate::{detection::project_type::should_skip_dir, utils::file};
use std::env;
use std::{ffi::OsStr, fs, path::Path};
use walkdir::WalkDir;
pub fn dockerfile_go(path: &str) -> Option<String> {
    for entry in WalkDir::new(&path)
        .max_depth(4)
        .into_iter()
        .filter_entry(|e| !should_skip_dir(e.file_name()))
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.file_name() == Some(OsStr::new("go.mod")) {
            let content = fs::read_to_string(path).ok()?;
            for line in content.lines() {
                let line = line.trim();
                if let Some(version) = line.strip_prefix("go ") {
                    let short_version = version.split('.').take(2).collect::<Vec<_>>().join(".");
                    return Some(short_version.to_string());
                }
            }
        }
    }
    None
}

pub fn generate_docker_content(go_version: &str) -> String {
    let docker = r#"FROM golang:{GO_VERSION}-alpine AS builder

    WORKDIR /app

    # Install git (needed for go mod)
    RUN apk add --no-cache git

    # Copy go mod files
    COPY go.mod go.sum ./
    RUN go mod download

    # Copy source code
    COPY . .

    # Build binary
    RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o app .

    # ===== Run stage =====
    FROM alpine:latest

    WORKDIR /app

    # Add CA certificates
    RUN apk add --no-cache ca-certificates

    # Copy binary from builder
    COPY --from=builder /app/app .

    # Copy uploads folder if exists
    COPY uploads ./uploads

    EXPOSE 8084

    CMD ["./app"]"#;
    docker.replace("{GO_VERSION}", go_version)
}

pub fn create_dockerfile_go(path: &str, file_content: &str) -> Result<(), std::io::Error> {
    let current_path = Path::new(path).join("Dockerfile");
    std::fs::write(current_path, file_content);
    Ok(())
}

pub fn run_dockerit() {
    let cwd = env::current_dir().expect("failed to get current directory");
    let cwd_str = cwd.to_str().expect("invalid UTF-8 path");
    let go_version = dockerfile_go(cwd_str);
    let version = match go_version {
        Some(v) => v,
        None => {
            eprintln!("❌ could not find go.mod or Go version");
            return;
        }
    };
    println!("✅ Go version detected: {}", version);
    let content = generate_docker_content(&version);
    if let Err(e) = create_dockerfile_go(cwd_str, &content) {
        eprintln!("❌ failed to create Dockerfile: {}", e);
        return;
    }
    println!("🐳 Dockerfile created successfully");
}
