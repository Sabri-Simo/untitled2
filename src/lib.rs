mod detection;
mod utils;
use detection::project_type::*;
use detection::version::*;
use std::ffi::OsStr;
use utils::file::*;
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_project(files: &[&str]) -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        for file in files {
            let file_path = temp_dir.path().join(file);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::File::create(file_path).unwrap();
        }
        temp_dir
    }

    #[test]
    fn test_should_skip_common_dirs() {
        assert!(should_skip_dir(OsStr::new("node_modules")));
        assert!(should_skip_dir(OsStr::new(".git")));
        assert!(should_skip_dir(OsStr::new("target")));
        assert!(should_skip_dir(OsStr::new(".venv")));
        assert!(should_skip_dir(OsStr::new("__pycache__")));
        assert!(!should_skip_dir(OsStr::new("src")));
        assert!(!should_skip_dir(OsStr::new("tests")));
    }

    #[test]
    fn test_detect_python_with_requirements() {
        let temp_dir = create_test_project(&["main.py", "utils.py", "requirements.txt"]);

        let project_type = detect_project_type(temp_dir.path().to_str().unwrap());
        assert_eq!(
            project_type,
            ProjectType::Python(PythonDependency::RequirementsTxt)
        );
    }

    #[test]
    fn test_detect_python_with_pyproject() {
        let temp_dir = create_test_project(&["src/main.py", "pyproject.toml"]);

        let project_type = detect_project_type(temp_dir.path().to_str().unwrap());
        assert_eq!(
            project_type,
            ProjectType::Python(PythonDependency::PyProjectToml)
        );
    }

    #[test]
    fn test_detect_python_with_setup_py() {
        let temp_dir = create_test_project(&["src/package/__init__.py", "setup.py"]);

        let project_type = detect_project_type(temp_dir.path().to_str().unwrap());
        assert_eq!(project_type, ProjectType::Python(PythonDependency::SetupPy));
    }

    #[test]
    fn test_detect_python_priority() {
        let temp_dir = create_test_project(&["main.py", "pyproject.toml"]);

        let project_type = detect_project_type(temp_dir.path().to_str().unwrap());
        assert_eq!(
            project_type,
            ProjectType::Python(PythonDependency::PyProjectToml)
        );
    }

    #[test]
    fn test_detect_go_project_with_mod() {
        let temp_dir = create_test_project(&["main.go", "utils.go", "go.mod", "go.sum"]);

        let project_type = detect_project_type(temp_dir.path().to_str().unwrap());
        assert_eq!(project_type, ProjectType::Go);
    }

    #[test]
    fn test_detect_go_project_without_mod() {
        let temp_dir = create_test_project(&["main.go", "handler.go"]);

        let project_type = detect_project_type(temp_dir.path().to_str().unwrap());
        assert_eq!(project_type, ProjectType::Go);
    }

    #[test]
    fn test_detect_csharp_project() {
        let temp_dir = create_test_project(&["Program.cs", "MyApp.csproj"]);

        let project_type = detect_project_type(temp_dir.path().to_str().unwrap());
        assert_eq!(project_type, ProjectType::CSharp);
    }

    #[test]
    fn test_detect_unknown_project() {
        let temp_dir = create_test_project(&["README.md", "data.json"]);

        let project_type = detect_project_type(temp_dir.path().to_str().unwrap());
        assert_eq!(project_type, ProjectType::Unknown);
    }

    #[test]
    fn test_python_priority_over_other_languages() {
        let temp_dir = create_test_project(&["main.py", "requirements.txt", "main.go", "go.mod"]);

        let project_type = detect_project_type(temp_dir.path().to_str().unwrap());
        match project_type {
            ProjectType::Python(_) => assert!(true),
            _ => panic!("Expected Python project type"),
        }
    }

    #[test]
    fn test_skip_venv_directories() {
        let temp_dir = create_test_project(&[
            "main.py",
            "requirements.txt",
            ".venv/lib/python3.9/site-packages/package.py",
            "venv/lib/python3.9/site-packages/another.py",
        ]);

        let project_type = detect_project_type(temp_dir.path().to_str().unwrap());
        assert_eq!(
            project_type,
            ProjectType::Python(PythonDependency::RequirementsTxt)
        );
    }

    #[test]
    fn test_nested_project_structure() {
        let temp_dir = create_test_project(&[
            "src/main.py",
            "src/utils/helper.py",
            "tests/test_main.py",
            "requirements.txt",
        ]);

        let project_type = detect_project_type(temp_dir.path().to_str().unwrap());
        assert_eq!(
            project_type,
            ProjectType::Python(PythonDependency::RequirementsTxt)
        );
    }

    #[test]
    fn test_write_version_creates_file() {
        let temp_dir = TempDir::new().unwrap();
        let version_file = temp_dir.path().join("version.txt");

        let result = write_version(&version_file, "1.2.3");
        assert!(result.is_ok());

        let content = fs::read_to_string(&version_file).unwrap();
        assert_eq!(content, "1.2.3");
    }

    #[test]
    fn test_write_version_overwrites_existing() {
        let temp_dir = TempDir::new().unwrap();
        let version_file = temp_dir.path().join("version.txt");

        write_version(&version_file, "1.0.0").unwrap();
        write_version(&version_file, "2.0.0").unwrap();

        let content = fs::read_to_string(&version_file).unwrap();
        assert_eq!(content, "2.0.0");
    }

    #[test]
    fn test_get_python_version_format() {
        if let Some(version) = get_python_version() {
            assert!(version.contains("Python") || version.contains("python"));
            assert!(version.chars().any(|c| c.is_ascii_digit()));
        }
    }

    #[test]
    fn test_get_go_version_format() {
        if let Some(version) = go_version_command() {
            assert!(version.contains("go version"));
            assert!(version.chars().any(|c| c.is_ascii_digit()));
        }
    }

    #[test]
    fn test_get_project_version_python() {
        let temp_dir = create_test_project(&["main.py", "requirements.txt"]);
        let path = temp_dir.path().to_str().unwrap();

        if let Some(version) = select_command(path) {
            assert!(version.contains("Python") || version.contains("python"));
        }
    }

    #[test]
    fn test_get_project_version_go() {
        let temp_dir = create_test_project(&["main.go", "go.mod"]);
        let path = temp_dir.path().to_str().unwrap();

        if let Some(version) = select_command(path) {
            assert!(version.contains("go version"));
        }
    }

    #[test]
    fn test_get_project_version_unknown() {
        let temp_dir = create_test_project(&["README.md"]);
        let path = temp_dir.path().to_str().unwrap();

        assert_eq!(select_command(path), None);
    }
}
