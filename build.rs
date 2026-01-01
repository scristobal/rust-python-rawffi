use std::env;
use std::process::Command;

fn main() {
    if cfg!(target_os = "windows") {
        // Windows: Find Python installation and link against python3x.lib
        if let Ok(python_exe) = env::var("PYTHON_SYS_EXECUTABLE") {
            // Use provided Python executable
            configure_windows_python(Some(&python_exe));
        } else {
            // Try common Python executables
            for python_cmd in &["python", "python3", "py"] {
                if Command::new(python_cmd).arg("--version").output().is_ok() {
                    configure_windows_python(Some(python_cmd));
                    break;
                }
            }
        }
    } else if cfg!(target_os = "macos") {
        // macOS: Use python3-config to find framework
        if let Ok(output) = Command::new("python3-config").arg("--ldflags").output() {
            let flags = String::from_utf8_lossy(&output.stdout);
            for flag in flags.split_whitespace() {
                if let Some(lib_path) = flag.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={}", lib_path);
                }
            }
        }

        // Get Python prefix to find framework
        if let Ok(output) = Command::new("python3-config").arg("--prefix").output() {
            let prefix = String::from_utf8_lossy(&output.stdout).trim().to_string();
            // Extract framework directory (e.g., /opt/homebrew/opt/python@3.14/Frameworks)
            if let Some(framework_base) = prefix
                .strip_suffix("/Versions/3.14")
                .and_then(|dir| dir.strip_suffix("/Python.framework"))
            {
                println!("cargo:rustc-link-search=framework={}", framework_base);
            }
        }
        // Link against Python framework
        println!("cargo:rustc-link-lib=framework=Python");
    } else {
        // Linux and other Unix-like systems: Use python3-config
        if let Ok(output) = Command::new("python3-config").arg("--ldflags").output() {
            let flags = String::from_utf8_lossy(&output.stdout);
            for flag in flags.split_whitespace() {
                if let Some(lib_path) = flag.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={}", lib_path);
                }
            }
        }
        // Link against python3 library
        println!("cargo:rustc-link-lib=python3");
    }
}

fn configure_windows_python(python_exe: Option<&str>) {
    let python = python_exe.unwrap_or("python");

    // Get Python version to determine library name (e.g., python39, python310)
    if let Ok(output) = Command::new(python)
        .args([
            "-c",
            "import sys; print(f'{sys.version_info.major}{sys.version_info.minor}')",
        ])
        .output()
    {
        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !version.is_empty() {
            println!("cargo:rustc-link-lib=python{}", version);
        }
    }

    // Get Python library directory
    if let Ok(output) = Command::new(python)
        .args([
            "-c",
            "import sysconfig; print(sysconfig.get_config_var('LIBDIR') or '')",
        ])
        .output()
    {
        let libdir = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !libdir.is_empty() {
            println!("cargo:rustc-link-search=native={}", libdir);
        }
    }

    // Also try to get base prefix for additional search paths
    if let Ok(output) = Command::new(python)
        .args(["-c", "import sys; print(sys.base_prefix)"])
        .output()
    {
        let base_prefix = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !base_prefix.is_empty() {
            // Add libs subdirectory
            println!("cargo:rustc-link-search=native={}\\libs", base_prefix);
        }
    }
}
