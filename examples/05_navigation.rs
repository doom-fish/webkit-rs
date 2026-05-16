mod common;

use std::fs;
use std::time::Duration;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    let directory = common::artifact_dir("example-navigation")?;
    let first_path = directory.join("first.html");
    let second_path = directory.join("second.html");
    fs::write(&first_path, "<!doctype html><html><head><title>first</title></head><body>first</body></html>")?;
    fs::write(&second_path, "<!doctype html><html><head><title>second</title></head><body>second</body></html>")?;

    let first = view.load_file_url(&first_path, &directory)?;
    let second = view.load_file_url(&second_path, &directory)?;

    assert_ne!(first.id(), 0);
    assert_ne!(second.id(), 0);
    assert!(view.can_go_back());
    let _ = view.go_back().expect("back navigation");
    assert!(common::wait_for(Duration::from_secs(2), || view.url().contains("first.html")));
    assert!(common::wait_for(Duration::from_secs(2), || view.can_go_forward()));
    let _ = view.go_forward().expect("forward navigation");
    assert!(common::wait_for(Duration::from_secs(2), || view.url().contains("second.html")));

    println!("navigation round-trip completed");
    Ok(())
}
