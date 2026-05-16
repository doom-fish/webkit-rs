mod common;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    config.add_user_script(
        &UserScript::new("window.__webkitInjected = 'from user script';")
            .with_injection_time(InjectionTime::AtDocumentStart),
    );
    let view = WebView::with_config(&config)?;

    common::load_html(&view, "<main>user script</main>", "https://user-script.test/")?;
    let value = view.evaluate_javascript("window.__webkitInjected")?;
    assert_eq!(value, "from user script");

    println!("user script injected value: {value}");
    Ok(())
}
