use crate::installation_path;
use crate::progress_bar;
use crate::releases;
use std::io::Cursor;
use tar::Archive;
use xz::read::XzDecoder;

//https://storage.googleapis.com/flutter_infra/stable/linux/flutter_linux_2.0.4-stable.tar.xz
//https://storage.googleapis.com/flutter_infra/stable/linux/flutter_linux_2.0.4-stable.tar.xz
//https://storage.googleapis.com/flutter_infra/releases/stable/linux/flutter_linux_2.0.4-stable.tar.xz
pub fn download_progress_bar(release: &releases::Release) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cursor = rt.block_on(download_release(&release));
    let _ = rt.block_on(unpack_flutter_tar(&release, cursor));
}

async fn download_release(release: &releases::Release) -> Cursor<Vec<u8>> {
    let mut res = reqwest::get(&release.archive).await.unwrap();
    let bytes_len = res.headers()["content-length"]
        .to_str()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    while let Some(chunk) = res.chunk().await.unwrap() {
        bytes.extend(chunk.to_vec());
        progress_bar::print_progress_bar(80, bytes.len() as f64 / bytes_len as f64);
    }
    println!("\nDownload finished");
    let cursor: Cursor<Vec<u8>> = Cursor::new(bytes);
    return cursor;
}

async fn unpack_flutter_tar(release: &releases::Release, cursor: Cursor<Vec<u8>>) {
    let progress_circle = progress_bar::AsyncProgressBar::start(
        vec![
            "|".to_string(),
            "/".to_string(),
            "-".to_string(),
            "\\".to_string(),
        ],
        std::time::Duration::from_millis(100),
        "Unpacking files".to_string(),
    );
    let tar = XzDecoder::new(cursor);
    let mut archive = Archive::new(tar);
    let mut installation_path = installation_path::get_installation_path();
    installation_path.push(format!("{}", release));
    match archive.unpack(installation_path) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
    match progress_circle.stop() {
        Ok(_) => {
            println!("done");
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
