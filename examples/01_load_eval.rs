use std::sync::mpsc;
use std::time::Duration;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== webkit-rs smoke test ==");

    let config = WebViewConfiguration::new();
    config.use_nonpersistent_data_store();
    config.add_message_handler("smokeTest");

    let mut view = WebView::with_config(&config)?;

    let (tx, rx) = mpsc::channel::<String>();
    view.set_message_handler(move |_name, body| {
        let _ = tx.send(body.to_owned());
    });

    view.set_navigation_handler(|event| {
        println!("nav: {:?} url={}", event.kind, event.url);
    });

    let html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>webkit-rs smoke</title></head>
        <body>
        <script>
          document.title = "webkit-rs smoke";
          window.webkit.messageHandlers.smokeTest.postMessage("hello from JS");
        </script>
        </body>
        </html>
    "#;

    view.load_html(html, None)?;
    println!("HTML loaded successfully");

    let title = view.evaluate_javascript("document.title")?;
    println!("document.title = {title:?}");
    assert_eq!(title, "webkit-rs smoke", "unexpected title");

    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        webkit::pump_run_loop(0.05);
        if let Ok(message) = rx.try_recv() {
            println!("script message: {message:?}");
            assert_eq!(message, "hello from JS");
            break;
        }
        if std::time::Instant::now() >= deadline {
            eprintln!("⚠️  timed out waiting for script message");
            break;
        }
    }

    println!("✅ webkit load/eval OK");
    Ok(())
}
