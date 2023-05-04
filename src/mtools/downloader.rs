use std::path::PathBuf;
use trauma::{download::Download, downloader::DownloaderBuilder, Error};

pub async fn download_package(url: &str) -> Result<(), Error> {
    let downloads = vec![Download::try_from(url).unwrap()];
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from("packages"))
        .build();
    downloader.download(&downloads).await;
    Ok(())
}
