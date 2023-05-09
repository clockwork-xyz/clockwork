use {
    super::*,
    anyhow::{
        Context,
        Result,
    },
    bzip2::read::BzDecoder,
    clap::crate_version,
    indicatif::{
        ProgressBar,
        ProgressStyle,
    },
    reqwest::{
        blocking::get,
        Url,
    },
    std::{
        ffi::OsStr,
        fs::{
            self,
            copy,
            File,
        },
        io::{self,},
        path::{
            Path,
            PathBuf,
        },
    },
    tar::Archive,
};

pub fn download_deps(
    runtime_dir: &Path,
    force_init: bool,
    solana_archive: Option<String>,
    clockwork_archive: Option<String>,
    dev: bool,
) -> Result<()> {
    let solana_tag = env!("GEYSER_INTERFACE_VERSION").to_owned().to_tag_version();
    let clockwork_tag = crate_version!().to_owned().to_tag_version();

    // Create the version directory if it does not exist
    let active_runtime = &runtime_dir.join(&clockwork_tag);

    download_and_extract(
        &active_runtime,
        &solana_archive.unwrap_or(CliConfig::solana_release_url(&solana_tag)),
        &active_runtime.join(CliConfig::solana_release_archive()),
        config::SOLANA_DEPS,
        force_init,
    )?;
    if !dev {
        download_and_extract(
            &active_runtime,
            &clockwork_archive.unwrap_or(CliConfig::clockwork_release_url(&clockwork_tag)),
            &active_runtime.join(CliConfig::clockwork_release_archive()),
            config::CLOCKWORK_DEPS,
            force_init,
        )?;
    }
    Ok(())
}

fn all_target_files_exist(directory: &Path, target_files: &[&str]) -> bool {
    let target_file_names: Vec<&OsStr> = target_files.iter().map(OsStr::new).collect();
    target_file_names
        .iter()
        .all(|file_name| directory.join(file_name).exists())
}

pub fn download_and_extract(
    runtime_dir: &Path,
    src_url: &str,
    dest_path: &Path,
    files_to_extract: &[&str],
    force_init: bool,
) -> Result<()> {
    if !force_init && all_target_files_exist(runtime_dir, files_to_extract) {
        print_note!("Using {:#?} from cache", files_to_extract.join(", "));
        return Ok(());
    }
    // create runtime dir if necessary
    fs::create_dir_all(runtime_dir)
        .context(format!("Unable to create dirs for {:#?}", runtime_dir))?;
    download_file(src_url, &dest_path)?;
    extract_archive(&dest_path, runtime_dir, files_to_extract)?;
    println!();
    Ok(())
}
fn download_file(url: &str, dest: &Path) -> Result<()> {
    // Check if the input is a URL or a local path
    match Url::parse(url) {
        Ok(url) => {
            // Download the file from the internet
            print_status!("Downloading", "{}", &url.to_string());
            let response =
                get(url.clone()).context(format!("Failed to download file from {}", url))?;
            if response.status() != reqwest::StatusCode::OK {
                return Err(anyhow::anyhow!("File not found at {}", &url));
            }

            let pb = ProgressBar::new(response.content_length().unwrap_or(0));
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} 🚚 [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .progress_chars("#>-"));

            let mut source = pb.wrap_read(response);

            let mut dest =
                File::create(&dest).context(format!("Failed to create file {:#?}", dest))?;
            io::copy(&mut source, &mut dest)?;
            pb.finish_with_message("Download complete.");
        }
        Err(_) => {
            // Copy the local file to the destination
            let source_path = Path::new(url);
            copy(source_path, dest).context(format!(
                "failed to copy file from {:#?} to {:#?}",
                source_path, dest
            ))?;
        }
    }
    Ok(())
}

fn extract_archive(
    archive_path: &Path,
    runtime_dir: &Path,
    files_to_extract: &[&str],
) -> Result<()> {
    let file =
        File::open(&archive_path).context(format!("Failed to open file {:#?}", archive_path))?;
    let target_file_names: Vec<&OsStr> = files_to_extract.iter().map(OsStr::new).collect();
    let mut archive = Archive::new(BzDecoder::new(file));

    print_status!("Extracting", "{:?}", archive_path);
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .filter(|entry| {
            entry
                .path()
                .map(|path| {
                    target_file_names.contains(&path.file_name().unwrap_or_else(|| OsStr::new("")))
                })
                .unwrap_or(false)
        })
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?;
            let file_name = path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Failed to extract file name from {:#?}", path))?
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Failed to convert file name to string"))?
                .to_owned();
            let target_path = runtime_dir.join(&file_name);
            entry.unpack(&target_path).context(format!(
                "Failed to unpack {:#?} into {:#?}",
                file_name, target_path
            ))?;
            Ok(target_path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| {
            print_status!(">", "{}", x.display());
        });
    Ok(())
}

pub trait ToTagVersion {
    fn to_tag_version(&self) -> String;
}

impl ToTagVersion for String {
    fn to_tag_version(&self) -> String {
        if !self.starts_with("v") {
            format!("v{}", self)
        } else {
            self.to_owned()
        }
    }
}
