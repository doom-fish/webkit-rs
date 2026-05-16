mod common;

use std::fs;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    let directory = common::artifact_dir("example-back-forward-list")?;
    let first_path = directory.join("first.html");
    let second_path = directory.join("second.html");
    fs::write(&first_path, "<!doctype html><html><head><title>first</title></head><body>first</body></html>")?;
    fs::write(&second_path, "<!doctype html><html><head><title>second</title></head><body>second</body></html>")?;

    view.load_file_url(&first_path, &directory)?;
    view.load_file_url(&second_path, &directory)?;

    let list = view.back_forward_list();
    assert!(view.can_go_back());
    if let Some(current_item) = list.current_item() {
        println!("current item: {}", current_item.url);
    } else {
        println!("back/forward list items unavailable for this offscreen navigation; can_go_back=true");
    }

    println!("back/forward list items: {}", list.items().len());
    Ok(())
}
