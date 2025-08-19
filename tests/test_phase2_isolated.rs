/// Isolated Phase 2 Tests
/// These tests verify the phone lookup functionality without requiring Phase 1 (case scraping)
/// They use mock data and can run independently

#[cfg(test)]
mod phase2_isolated_tests {
    use foreclose_scrape::case::Case;
    use foreclose_scrape::phone_lookup::PhoneResult;

    /// Create mock cases as if they came from Phase 1
    fn create_mock_cases() -> Vec<Case> {
        vec![
            Case {
                name: "Wells Fargo Bank vs John Doe".to_string(),
                docket: "CV-2024-001234".to_string(),
                defendant: "John Doe".to_string(),
                property_address: "123 Main Street, Middletown, CT 06457".to_string(),
                phone_numbers: vec![],
            },
            Case {
                name: "Bank of America vs Jane Smith".to_string(),
                docket: "CV-2024-005678".to_string(),
                defendant: "Jane Smith".to_string(),
                property_address: "456 Oak Avenue, Hartford, CT 06103".to_string(),
                phone_numbers: vec![],
            },
            Case {
                name: "Chase Bank vs Robert Johnson".to_string(),
                docket: "CV-2024-009012".to_string(),
                defendant: "Robert Johnson".to_string(),
                property_address: "789 Elm Street, New Haven, CT 06511".to_string(),
                phone_numbers: vec![],
            },
        ]
    }

    /// Create mock phone lookup results as if they came from TruePeopleSearch
    fn create_mock_phone_results(defendant: &str) -> Vec<PhoneResult> {
        match defendant {
            "John Doe" => vec![
                PhoneResult {
                    name: "John Doe".to_string(),
                    address: "123 Main Street, Middletown, CT".to_string(),
                    phone_numbers: vec!["(860) 555-1234".to_string(), "(860) 555-5678".to_string()],
                    confidence_score: 0.95,
                },
                PhoneResult {
                    name: "John M Doe".to_string(),
                    address: "125 Main Street, Middletown, CT".to_string(),
                    phone_numbers: vec!["(860) 555-9999".to_string()],
                    confidence_score: 0.65,
                },
            ],
            "Jane Smith" => vec![
                PhoneResult {
                    name: "Jane Smith".to_string(),
                    address: "456 Oak Ave, Hartford, CT".to_string(),
                    phone_numbers: vec!["(203) 555-2222".to_string()],
                    confidence_score: 0.90,
                },
            ],
            "Robert Johnson" => vec![
                PhoneResult {
                    name: "Robert Johnson".to_string(),
                    address: "789 Elm St, New Haven, CT".to_string(),
                    phone_numbers: vec!["(475) 555-3333".to_string(), "(203) 555-4444".to_string()],
                    confidence_score: 0.88,
                },
                PhoneResult {
                    name: "Bob Johnson".to_string(),
                    address: "791 Elm Street, New Haven, CT".to_string(),
                    phone_numbers: vec!["(203) 555-7777".to_string()],
                    confidence_score: 0.55,
                },
                PhoneResult {
                    name: "R Johnson".to_string(),
                    address: "Unknown".to_string(),
                    phone_numbers: vec!["(860) 555-8888".to_string()],
                    confidence_score: 0.30,
                },
            ],
            _ => vec![],
        }
    }

    #[test]
    fn test_phase2_workflow_simulation() {
        // Step 1: Get mock cases (simulating Phase 1 output)
        let mut cases = create_mock_cases();
        assert_eq!(cases.len(), 3);

        // Step 2: For each case, simulate phone lookup
        for case in &mut cases {
            println!("Processing: {} - {}", case.defendant, case.property_address);
            
            // Get mock phone results
            let phone_results = create_mock_phone_results(&case.defendant);
            
            if !phone_results.is_empty() {
                // Select best match (highest confidence)
                if let Some(best_match) = phone_results
                    .iter()
                    .max_by(|a, b| a.confidence_score.partial_cmp(&b.confidence_score).unwrap())
                {
                    case.phone_numbers = best_match.phone_numbers.clone();
                    println!("  Found {} phone number(s) with confidence {:.2}", 
                             case.phone_numbers.len(), best_match.confidence_score);
                }
            }
        }

        // Step 3: Verify results
        assert_eq!(cases[0].phone_numbers.len(), 2); // John Doe should have 2 numbers
        assert_eq!(cases[1].phone_numbers.len(), 1); // Jane Smith should have 1 number
        assert_eq!(cases[2].phone_numbers.len(), 2); // Robert Johnson should have 2 numbers

        // Verify specific phone numbers
        assert!(cases[0].phone_numbers.contains(&"(860) 555-1234".to_string()));
        assert!(cases[1].phone_numbers.contains(&"(203) 555-2222".to_string()));
        assert!(cases[2].phone_numbers.contains(&"(475) 555-3333".to_string()));
    }

    #[test]
    fn test_confidence_scoring_logic() {
        let results = create_mock_phone_results("Robert Johnson");
        
        // Should have 3 results with different confidence scores
        assert_eq!(results.len(), 3);
        
        // Check confidence scores are in expected ranges
        assert!(results[0].confidence_score > 0.8); // Good match
        assert!(results[1].confidence_score > 0.5 && results[1].confidence_score < 0.8); // Partial match
        assert!(results[2].confidence_score < 0.5); // Poor match
        
        // Best match should be the first one
        let best_match = results
            .iter()
            .max_by(|a, b| a.confidence_score.partial_cmp(&b.confidence_score).unwrap())
            .unwrap();
        
        assert_eq!(best_match.name, "Robert Johnson");
        assert_eq!(best_match.confidence_score, 0.88);
    }

    #[test]
    fn test_csv_output_with_phone_numbers() {
        let mut cases = create_mock_cases();
        
        // Add phone numbers to first case
        cases[0].phone_numbers = vec![
            "(860) 555-1234".to_string(),
            "(860) 555-5678".to_string(),
        ];
        
        // Test CSV record generation
        let csv_record = cases[0].to_csv_record();
        assert_eq!(csv_record.len(), 5);
        assert_eq!(csv_record[4], "(860) 555-1234; (860) 555-5678");
        
        // Test case without phone numbers
        let csv_record_empty = cases[1].to_csv_record();
        assert_eq!(csv_record_empty[4], "");
    }

    #[test]
    fn test_handling_no_results() {
        // Test with a name that returns no results
        let phone_results = create_mock_phone_results("Unknown Person");
        assert!(phone_results.is_empty());
        
        // Case should keep empty phone numbers
        let mut case = Case {
            name: "Bank vs Unknown Person".to_string(),
            docket: "CV-2024-UNKNOWN".to_string(),
            defendant: "Unknown Person".to_string(),
            property_address: "Unknown Address".to_string(),
            phone_numbers: vec![],
        };
        
        // Simulate processing with no results
        if !phone_results.is_empty() {
            if let Some(best_match) = phone_results
                .iter()
                .max_by(|a, b| a.confidence_score.partial_cmp(&b.confidence_score).unwrap())
            {
                case.phone_numbers = best_match.phone_numbers.clone();
            }
        }
        
        assert!(case.phone_numbers.is_empty());
    }

    #[test]
    fn test_multiple_matches_selection() {
        let results = create_mock_phone_results("John Doe");
        
        // Should have 2 results
        assert_eq!(results.len(), 2);
        
        // The first result should have higher confidence
        assert!(results[0].confidence_score > results[1].confidence_score);
        
        // Simulate selection of best match
        let best_match = results
            .iter()
            .max_by(|a, b| a.confidence_score.partial_cmp(&b.confidence_score).unwrap())
            .unwrap();
        
        assert_eq!(best_match.confidence_score, 0.95);
        assert_eq!(best_match.phone_numbers.len(), 2);
    }
}