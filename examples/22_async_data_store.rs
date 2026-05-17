//! Async `WebsiteDataStore` example.
#[cfg(feature = "async")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use webkit::async_api::AsyncWebsiteDataStore;
    use webkit::{WebsiteDataStore, WebsiteDataType};

    webkit::init_app();
    let store = WebsiteDataStore::default_data_store();

    pollster::block_on(async {
        let records_json =
            AsyncWebsiteDataStore::fetch_data_records(&store, &[WebsiteDataType::cookies()])
                .await?;
        println!("records: {records_json}");
        Ok::<(), Box<dyn std::error::Error>>(())
    })
}

#[cfg(not(feature = "async"))]
fn main() {
    println!("compile with --features async");
}
