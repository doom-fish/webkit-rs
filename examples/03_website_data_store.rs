mod common;

use std::time::SystemTime;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let store = config.website_data_store().expect("website data store");
    let view = WebView::with_config(&config)?;

    common::load_html(
        &view,
        r"<script>
        document.cookie = 'area=website-data; path=/';
        localStorage.setItem('webkit-rs', 'present');
        </script>",
        "https://example.test/",
    )?;

    let data_types = [WebsiteDataType::cookies(), WebsiteDataType::local_storage()];
    let records = store.data_records(&data_types)?;
    assert!(!records.is_empty(), "expected at least one website data record");
    store.remove_data_for_records(&data_types, &records)?;
    store.remove_data_modified_since(&data_types, SystemTime::UNIX_EPOCH)?;

    println!("fetched {} website data records", records.len());
    Ok(())
}
