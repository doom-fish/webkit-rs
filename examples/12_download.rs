mod common;

use std::fs;
use std::time::{Duration, Instant};

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let download_dir =
        common::artifact_dir("example-downloads")?.join(format!("run-{}", std::process::id()));
    if download_dir.exists() {
        fs::remove_dir_all(&download_dir)?;
    }
    fs::create_dir_all(&download_dir)?;

    let (url, server) = common::start_attachment_server(
        b"hello from download".to_vec(),
        "example.txt",
        "text/plain",
    )?;

    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    let download = view.start_download_using_request(&url, &download_dir)?;
    assert_eq!(download.original_request_url(), url);

    let deadline = Instant::now() + Duration::from_secs(5);
    let mut finished = false;
    while Instant::now() < deadline {
        for event in download.drain_events() {
            if event.kind == "finish" {
                finished = true;
                break;
            }
        }
        if finished {
            break;
        }
        webkit::pump_run_loop(0.05);
    }
    server.join().expect("download server thread");
    assert!(finished, "download did not finish in time");
    assert!(
        fs::read_dir(&download_dir)?.next().is_some(),
        "download directory should contain a file"
    );

    println!("download finished into {}", download_dir.display());
    Ok(())
}
