use foreclose_scrape::phone_lookup::{PhoneLookup, PhoneResult};
use thirtyfour::prelude::*;

#[cfg(test)]
mod phone_lookup_tests {
    use super::*;

    async fn create_test_driver() -> WebDriver {
        let caps = DesiredCapabilities::chrome();
        let port = "46107";
        let driver_path = format!("http://localhost:{}", port);
        WebDriver::new(driver_path, caps).await.expect("Failed to create WebDriver")
    }

    #[tokio::test]
    async fn test_build_search_url() {
        let driver = create_test_driver().await;
        let phone_lookup = PhoneLookup::new(driver).await.unwrap();

        // Test with full name and address
        let url = phone_lookup.build_search_url(
            "John Doe",
            "123 Main St, Middletown, CT 06457"
        );
        assert!(url.contains("John"));
        assert!(url.contains("Doe"));
        assert!(url.contains("Middletown"));
        assert!(url.contains("CT"));
        assert!(url.starts_with("https://www.truepeoplesearch.com/results"));

        // Test with single name
        let url = phone_lookup.build_search_url(
            "Smith",
            "456 Oak Ave, Hartford, CT"
        );
        assert!(url.contains("Smith"));
        assert!(url.contains("Hartford"));

        // Test with complex name
        let url = phone_lookup.build_search_url(
            "Mary Jane Smith-Johnson",
            "789 Elm St, New Haven, CT 06511"
        );
        assert!(url.contains("Mary"));
        assert!(url.contains("Smith-Johnson"));
        assert!(url.contains("New Haven"));
    }

    #[tokio::test]
    async fn test_phone_result_structure() {
        // Test PhoneResult creation and serialization
        let result = PhoneResult {
            name: "John Doe".to_string(),
            address: "123 Main St, Middletown, CT".to_string(),
            phone_numbers: vec!["555-1234".to_string(), "555-5678".to_string()],
        };

        assert_eq!(result.name, "John Doe");
        assert_eq!(result.phone_numbers.len(), 2);

        // Test serialization
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("John Doe"));
        assert!(json.contains("555-1234"));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    #[ignore] // This test requires a running WebDriver and internet connection
    async fn test_actual_phone_lookup() {
        let driver = create_test_driver().await;
        let phone_lookup = PhoneLookup::new(driver).await.unwrap();

        // Test with a generic common name (less likely to change)
        let results = phone_lookup
            .search_phone_number("John Smith", "Hartford, CT")
            .await
            .unwrap();

        // We should get some results for such a common name
        assert!(!results.is_empty(), "Should find at least one result for a common name");

        // Check that results have expected structure
        for result in &results {
            assert!(!result.name.is_empty());
        }
    }

    async fn create_test_driver() -> WebDriver {
        let mut caps = DesiredCapabilities::chrome();
        caps.add_arg("--headless=new").ok(); // Run in headless mode for tests
        let port = "46107";
        let driver_path = format!("http://localhost:{}", port);
        WebDriver::new(driver_path, caps).await.expect("Failed to create WebDriver")
    }
}