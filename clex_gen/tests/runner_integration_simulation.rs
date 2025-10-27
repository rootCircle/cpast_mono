// Example test demonstrating that the iterator can be used with ccode_runner
// This would be a realistic integration test

#[cfg(test)]
mod integration_with_runner {
    use clex_gen::generator_iter;

    #[test]
    fn test_iterator_produces_valid_output_for_runner() {
        // This demonstrates that the iterator produces output
        // that could be fed to a code runner
        let clex = "N[1,10] N[1,10] N[1,10]";

        let mut complete_output = String::new();
        for chunk_result in generator_iter(clex.to_string()).unwrap() {
            match chunk_result {
                Ok(chunk) => complete_output.push_str(&chunk),
                Err(e) => panic!("Error during generation: {}", e),
            }
        }

        // Clean up trailing space
        if complete_output.ends_with(' ') {
            complete_output.pop();
        }

        // Verify the output is well-formed
        let parts: Vec<&str> = complete_output.split_whitespace().collect();
        assert_eq!(parts.len(), 3, "Should have 3 numbers");

        for part in parts {
            let num: i64 = part.parse().expect("Should be a valid integer");
            assert!(
                (1..=10).contains(&num),
                "Number should be in range [1,10]"
            );
        }
    }

    #[test]
    fn test_iterator_memory_efficient_simulation() {
        // Simulate processing a large test case in chunks
        // without accumulating all data in memory
        let clex = "N N N N N N N N N N";

        let mut chunk_count = 0;
        let mut total_bytes = 0;

        for chunk_result in generator_iter(clex.to_string()).unwrap() {
            match chunk_result {
                Ok(chunk) => {
                    chunk_count += 1;
                    total_bytes += chunk.len();
                    // In a real scenario, we'd write to file or pipe to runner here
                    // without accumulating in memory
                }
                Err(e) => panic!("Error during generation: {}", e),
            }
        }

        assert_eq!(chunk_count, 10, "Should process 10 chunks");
        assert!(total_bytes > 0, "Should have generated some data");
    }

    #[test]
    fn test_streaming_to_simulated_runner() {
        // Simulate streaming output to a runner process
        let clex = "(N[5,5]) (?:N){\\1}";

        let mut lines_received = Vec::new();

        for chunk_result in generator_iter(clex.to_string()).unwrap() {
            match chunk_result {
                Ok(chunk) => {
                    // Simulate sending to runner (here we just collect for verification)
                    lines_received.push(chunk);
                }
                Err(e) => panic!("Error during generation: {}", e),
            }
        }

        // Should have received chunks
        assert!(
            !lines_received.is_empty(),
            "Should receive at least one chunk"
        );

        // First chunk should be the capturing group value
        let first_chunk = &lines_received[0];
        assert!(
            first_chunk.trim().parse::<u64>().is_ok(),
            "First chunk should be a number"
        );
    }
}
