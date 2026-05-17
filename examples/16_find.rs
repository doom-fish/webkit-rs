mod common;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    common::load_html(
        &view,
        "<main>Needle in a haystack</main>",
        "https://find.test/",
    )?;

    let result = view.find_string_with_configuration(
        "Needle",
        &FindConfiguration {
            case_sensitive: true,
            ..FindConfiguration::default()
        },
    )?;
    assert!(result.match_found);

    println!("find-in-page succeeded");
    Ok(())
}
