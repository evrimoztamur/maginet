//! Export the exact campaign graph for reports and browser checks.
fn main() {
    let entries = shared::campaign_catalogue(false);
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "catalogue": entries,
            "connections": shared::campaign_connections(&entries),
            "main_route": shared::MAIN_ROUTE,
            "optional_routes": shared::OPTIONAL_ROUTES,
        }))
        .unwrap()
    );
}
