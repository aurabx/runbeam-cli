use serde_json::Value;
use std::fs;

#[test]
fn test_routes_json_structure() {
    // Load sample JSON data
    let json_str = fs::read_to_string("samples/harmony_routes.json")
        .expect("Failed to read harmony_routes.json sample");
    
    let json_value: Value = serde_json::from_str(&json_str)
        .expect("Failed to parse sample JSON");
    
    // Verify the JSON structure is as expected
    let routes_array = json_value.get("routes")
        .and_then(|v| v.as_array())
        .expect("JSON should have 'routes' array");
    
    assert_eq!(routes_array.len(), 4, "Sample should have 4 routes");
    
    // Test that first route has expected fields
    let first_route = &routes_array[0];
    assert!(first_route.get("path").is_some());
    assert!(first_route.get("methods").is_some());
    assert!(first_route.get("description").is_some());
    assert!(first_route.get("endpoint_name").is_some());
    assert!(first_route.get("service_type").is_some());
    assert!(first_route.get("pipeline").is_some());
}
