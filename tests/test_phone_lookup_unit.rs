#[cfg(test)]
mod phone_lookup_unit_tests {
    use foreclose_scrape::phone_lookup::PhoneResult;
    use foreclose_scrape::case::Case;

    #[test]
    fn test_phone_result_creation() {
        let phone_result = PhoneResult {
            name: "John Doe".to_string(),
            address: "123 Main St, Middletown, CT 06457".to_string(),
            phone_numbers: vec!["(860) 555-1234".to_string(), "(203) 555-5678".to_string()],
            confidence_score: 0.85,
        };

        assert_eq!(phone_result.name, "John Doe");
        assert_eq!(phone_result.address, "123 Main St, Middletown, CT 06457");
        assert_eq!(phone_result.phone_numbers.len(), 2);
        assert_eq!(phone_result.phone_numbers[0], "(860) 555-1234");
        assert_eq!(phone_result.confidence_score, 0.85);
    }

    #[test]
    fn test_case_with_phone_numbers() {
        let mut case = Case {
            name: "Bank vs John Doe".to_string(),
            docket: "CV-2024-001234".to_string(),
            defendant: "John Doe".to_string(),
            property_address: "123 Main St, Middletown, CT 06457".to_string(),
            phone_numbers: vec![],
        };

        // Initially no phone numbers
        assert!(case.phone_numbers.is_empty());

        // Add phone numbers
        case.phone_numbers.push("(860) 555-1234".to_string());
        case.phone_numbers.push("(203) 555-5678".to_string());

        assert_eq!(case.phone_numbers.len(), 2);
        
        // Test CSV record generation
        let csv_record = case.to_csv_record();
        assert_eq!(csv_record.len(), 5);
        assert_eq!(csv_record[0], "Bank vs John Doe");
        assert_eq!(csv_record[1], "CV-2024-001234");
        assert_eq!(csv_record[2], "John Doe");
        assert_eq!(csv_record[3], "123 Main St, Middletown, CT 06457");
        assert_eq!(csv_record[4], "(860) 555-1234; (203) 555-5678");
    }

    #[test]
    fn test_case_without_phone_numbers() {
        let case = Case {
            name: "Bank vs Jane Smith".to_string(),
            docket: "CV-2024-005678".to_string(),
            defendant: "Jane Smith".to_string(),
            property_address: "456 Oak Ave, Hartford, CT 06103".to_string(),
            phone_numbers: vec![],
        };

        let csv_record = case.to_csv_record();
        assert_eq!(csv_record[4], ""); // Empty string when no phone numbers
    }

    #[test]
    fn test_confidence_score_boundaries() {
        let results = vec![
            PhoneResult {
                name: "Exact Match".to_string(),
                address: "123 Main St".to_string(),
                phone_numbers: vec!["555-1111".to_string()],
                confidence_score: 1.0,
            },
            PhoneResult {
                name: "Partial Match".to_string(),
                address: "123 Main".to_string(),
                phone_numbers: vec!["555-2222".to_string()],
                confidence_score: 0.5,
            },
            PhoneResult {
                name: "Poor Match".to_string(),
                address: "Unknown".to_string(),
                phone_numbers: vec!["555-3333".to_string()],
                confidence_score: 0.1,
            },
        ];

        // Find best match
        let best_match = results
            .iter()
            .max_by(|a, b| a.confidence_score.partial_cmp(&b.confidence_score).unwrap())
            .unwrap();

        assert_eq!(best_match.name, "Exact Match");
        assert_eq!(best_match.confidence_score, 1.0);

        // All scores should be between 0.1 and 1.0
        for result in &results {
            assert!(result.confidence_score >= 0.1);
            assert!(result.confidence_score <= 1.0);
        }
    }

    #[test]
    fn test_multiple_phone_numbers_formatting() {
        let case = Case {
            name: "Test Case".to_string(),
            docket: "CV-2024-TEST".to_string(),
            defendant: "Test Defendant".to_string(),
            property_address: "Test Address".to_string(),
            phone_numbers: vec![
                "(860) 555-1111".to_string(),
                "(203) 555-2222".to_string(),
                "(475) 555-3333".to_string(),
            ],
        };

        let csv_record = case.to_csv_record();
        assert_eq!(csv_record[4], "(860) 555-1111; (203) 555-2222; (475) 555-3333");
    }

    #[test]
    fn test_phone_result_serialization() {
        let phone_result = PhoneResult {
            name: "Test Name".to_string(),
            address: "Test Address".to_string(),
            phone_numbers: vec!["555-0000".to_string()],
            confidence_score: 0.75,
        };

        // Test that the struct can be serialized to JSON
        let json = serde_json::to_string(&phone_result).unwrap();
        assert!(json.contains("\"name\":\"Test Name\""));
        assert!(json.contains("\"confidence_score\":0.75"));
        
        // Test deserialization
        let deserialized: PhoneResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, phone_result.name);
        assert_eq!(deserialized.confidence_score, phone_result.confidence_score);
    }
}