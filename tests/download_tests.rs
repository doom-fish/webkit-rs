mod common;

use std::fs;
use std::time::{Duration, Instant};

use webkit::prelude::*;

#[test]
fn download_redirect_policy_is_typed() {
    assert_eq!(
        DownloadRedirectPolicy::from_raw(0),
        DownloadRedirectPolicy::Cancel
    );
    assert_eq!(
        DownloadRedirectPolicy::from_raw(1),
        DownloadRedirectPolicy::Allow
    );
    assert_eq!(DownloadRedirectPolicy::Allow.as_raw(), 1);

    let _: fn(&Download, DownloadRedirectPolicy) = Download::set_redirect_policy;
    let _: fn(&Download) -> DownloadRedirectPolicy = Download::redirect_policy;
}

#[test]
#[ignore = "WKDownload smoke tests must run on the process main thread; examples cover live validation"]
fn download_finishes_against_local_attachment_server() -> Result<(), Box<dyn std::error::Error>> {
    let download_dir =
        common::artifact_dir("test-downloads")?.join(format!("run-{}", std::process::id()));
    if download_dir.exists() {
        fs::remove_dir_all(&download_dir)?;
    }
    fs::create_dir_all(&download_dir)?;

    let (url, server) = common::start_attachment_server(
        b"download test body".to_vec(),
        "download.txt",
        "text/plain",
    )?;

    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    let download = view.start_download_using_request(&url, &download_dir)?;

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
    assert!(finished);
    assert!(fs::read_dir(&download_dir)?.next().is_some());
    Ok(())
}
