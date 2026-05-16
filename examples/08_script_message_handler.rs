mod common;

use std::sync::mpsc;
use std::time::{Duration, Instant};

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    config.add_message_handler("bridge");
    let mut view = WebView::with_config(&config)?;
    let (tx, rx) = mpsc::channel();
    view.set_message_handler(move |name, body| {
        let _ = tx.send((name.to_owned(), body.to_owned()));
    });

    common::load_html(
        &view,
        r"<script>window.webkit.messageHandlers.bridge.postMessage('hello from script handler');</script>",
        "https://script-message.test/",
    )?;

    let deadline = Instant::now() + Duration::from_secs(2);
    let mut received = None;
    while Instant::now() < deadline {
        if let Ok(message) = rx.try_recv() {
            received = Some(message);
            break;
        }
        webkit::pump_run_loop(0.05);
    }
    let received = received.expect("message callback should fire");
    assert_eq!(received.0, "bridge");
    assert_eq!(received.1, "hello from script handler");
    let _ = view.drain_script_messages();

    println!("received script message from {}", received.0);
    Ok(())
}
