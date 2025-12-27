use std::ffi::OsStr;

use walkdir::WalkDir;
#[derive(Debug, PartialEq)]
pub enum ProjectType {
    Python(PythonDependency),
    Go,
    CSharp,
    Unknown,
}
#[derive(Debug, PartialEq)]
pub enum PythonDependency {
    RequirementsTxt,
    PyProjectToml,
    SetupPy,
    None,
}

pub fn detect_project_type(path: &str) -> ProjectType {
    let mut python_dep = PythonDependency::None;
    let mut has_go = false;
    let mut has_csharp = false;
    let mut has_rust = false;

    for entry in WalkDir::new(path)
        .max_depth(5)
        .into_iter()
        .filter_entry(|e| !should_skip_dir(e.file_name()))
        .filter_map(Result::ok)
    {
        let path = entry.path();

        if let Some(ext) = path.extension().and_then(OsStr::to_str) {
            match ext {
                "py" if python_dep == PythonDependency::None => {
                    python_dep = PythonDependency::SetupPy;
                }
                "go" => has_go = true,
                "cs" | "csproj" => has_csharp = true,
                "rs" => has_rust = true,
                _ => {}
            }
        }

        if let Some(name) = path.file_name().and_then(OsStr::to_str) {
            match name {
                "requirements.txt" => python_dep = PythonDependency::RequirementsTxt,
                "pyproject.toml" => python_dep = PythonDependency::PyProjectToml,
                "setup.py" if python_dep == PythonDependency::None => {
                    python_dep = PythonDependency::SetupPy;
                }
                "go.mod" | "go.sum" => has_go = true,
                "Cargo.toml" => has_rust = true,
                _ => {}
            }
        }

        if python_dep != PythonDependency::None && has_go && has_csharp && has_rust {
            break;
        }
    }
    match python_dep {
        PythonDependency::None => {
            if has_go {
                ProjectType::Go
            } else if has_csharp {
                ProjectType::CSharp
            } else {
                ProjectType::Unknown
            }
        }
        dep => ProjectType::Python(dep),
    }
}
pub fn should_skip_dir(name: &OsStr) -> bool {
    name.to_str().map_or(false, |s| {
        matches!(
            s,
            "node_modules"
                | ".git"
                | ".venv"
                | "venv"
                | "__pycache__"
                | "target"
                | "build"
                | "dist"
                | ".idea"
                | ".vscode"
                | ".next"
                | "out"
        )
    })
}
