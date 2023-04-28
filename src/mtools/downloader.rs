use reqwest::blocking::Client;
use std::fs::File;
use std::io::{self};
use indicatif::{ProgressBar, ProgressStyle};

pub fn download_package(url: &str) -> io::Result<()> {
    let path = {
        let value = url.split('/').collect::<Vec<_>>();
        value[value.len() - 1]

    };
    let client = Client::new();
    let mut response = client.get(url).send().unwrap();
    let total_length = response.content_length().unwrap_or(0);

    // Create a new progress bar
    let pb = ProgressBar::new(total_length);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap()
        .progress_chars("#>-"));

    let mut file = File::create(path)?;

    let mut downloaded_length = 0;
    loop {
        let bytes_read = response.copy_to(&mut file).unwrap();
        if bytes_read == 0 {
            break;
        }
        downloaded_length += bytes_read as u64;
        pb.set_position(downloaded_length);
    }

    pb.finish_with_message("Downloaded");

    Ok(())
}
