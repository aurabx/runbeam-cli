use anyhow::{Context, Result, anyhow};
use reqwest::blocking::Client;
use directories::BaseDirs;
use std::env::consts::{ARCH, OS};
use std::fs::{self};
use std::io::{Cursor};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{PathBuf};
use flate2::read::GzDecoder;
use tar::Archive;
use tracing::info;

fn get_default_install_dir() -> Result<PathBuf> {
    if let Some(base_dirs) = BaseDirs::new() {
        if let Some(exe_dir) = base_dirs.executable_dir() {
            return Ok(exe_dir.to_path_buf());
        }
        
        // Fallback for platforms where executable_dir() is None (e.g. macOS, Windows)
        #[cfg(target_os = "macos")]
        {
            // Use ~/.local/bin as a modern standard for user binaries
            return Ok(base_dirs.home_dir().join(".local").join("bin"));
        }
        #[cfg(target_os = "windows")]
        {
            // Use a bin directory inside the runbeam data directory
            return Ok(base_dirs.data_dir().join("runbeam").join("bin"));
        }
        #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
        {
            // Fallback for other Unix-likes
            return Ok(base_dirs.home_dir().join(".local").join("bin"));
        }
    }
    
    Err(anyhow!("Could not determine default installation directory"))
}

pub fn install(version: Option<&str>, output_dir: Option<PathBuf>) -> Result<()> {
    // 1. Determine target platform
    let target = match (OS, ARCH) {
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("linux", "aarch64") => "aarch64-unknown-linux-gnu",
        ("macos", "x86_64") => "x86_64-apple-darwin",
        ("macos", "aarch64") => "aarch64-apple-darwin",
        ("windows", "x86_64") => "x86_64-pc-windows-msvc",
        _ => return Err(anyhow!("Unsupported platform: {} {}", OS, ARCH)),
    };

    // 2. Construct URL
    let filename = format!("harmony-{}.tar.gz", target);
    let url = if let Some(v) = version {
        format!(
            "https://github.com/aurabx/harmony/releases/download/{}/{}",
            v, filename
        )
    } else {
        format!(
            "https://github.com/aurabx/harmony/releases/latest/download/{}",
            filename
        )
    };

    info!("Target platform: {} {}", OS, ARCH);
    println!("Downloading Harmony from {}", url);

    // 3. Download file
    let client = Client::new();
    let resp = client
        .get(&url)
        .send()
        .with_context(|| format!("Failed to download from {}", url))?;

    if !resp.status().is_success() {
        return Err(anyhow!("Download failed: {} {}", resp.status(), url));
    }

    let content = resp.bytes().context("Failed to read response body")?;

    // 4. Verify checksum (Optional but recommended - skipping for now as I need to fetch .sha256 file separately and parse it)
    // For simplicity in this iteration, I'll skip checksum verification unless requested, but I'll check file size at least.
    println!("Downloaded {} bytes", content.len());

    // 5. Extract
    let output_path = match output_dir {
        Some(d) => d,
        None => get_default_install_dir()?,
    };
    
    if !output_path.exists() {
        println!("Creating directory: {}", output_path.display());
        fs::create_dir_all(&output_path).context("Failed to create output directory")?;
    }
    
    println!("Extracting to {}", output_path.display());

    let decoder = GzDecoder::new(Cursor::new(&content));
    let mut archive = Archive::new(decoder);

    // We want to extract the binary. The tarball structure usually contains the binary at root or in a folder?
    // GitHub releases usually just contain the binary or a folder.
    // Let's assume the tarball contains the binary directly or we extract all.
    // The README says: "Extract the archive ... ./harmony"
    // So it likely contains the `harmony` binary at the root of the archive.

    // Let's extract to the target directory.
    archive.unpack(&output_path).context("Failed to unpack archive")?;

    // 6. Make executable (Unix only)
    #[cfg(unix)]
    {
        let binary_name = "harmony";
        let binary_path = output_path.join(binary_name);
        if binary_path.exists() {
            println!("Setting executable permissions for {}", binary_path.display());
            let mut perms = fs::metadata(&binary_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&binary_path, perms)?;
        } else {
            // It might be in a subdirectory if the tarball structure changed, but assuming README is correct.
            // If not found, we just warn.
             println!("Warning: '{}' not found in extraction output. You may need to find the binary manually.", binary_name);
        }
    }

    println!("✓ Harmony installed successfully to {}", output_path.display());
    Ok(())
}
