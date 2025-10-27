use clex_gen::{generator, generator_iter};

#[test]
fn test_iterator_with_simple_integer_expression() {
    let language = "(N[3,3]) (?:N[3,3]){\\1}";

    // Generate using iterator
    let mut iter_output = String::new();
    for chunk in generator_iter(language.to_string()).unwrap() {
        match chunk {
            Ok(data) => iter_output.push_str(&data),
            Err(e) => panic!("Iterator error: {}", e),
        }
    }
    // Remove trailing space
    if iter_output.ends_with(' ') {
        iter_output.pop();
    }

    // Compare with non-iterator version
    let direct_output = generator(language.to_string()).unwrap();
    assert_eq!(iter_output, direct_output);
    assert_eq!(iter_output, "3 3 3 3");
}

#[test]
fn test_iterator_with_nested_expression() {
    let language = "(N[3,3]) (?:(N[1,1]) N[2,2]{\\2}) N[1,1]{\\2}";

    // Generate using iterator
    let mut iter_output = String::new();
    for chunk in generator_iter(language.to_string()).unwrap() {
        match chunk {
            Ok(data) => iter_output.push_str(&data),
            Err(e) => panic!("Iterator error: {}", e),
        }
    }
    // Remove trailing space
    if iter_output.ends_with(' ') {
        iter_output.pop();
    }

    // Compare with non-iterator version
    let direct_output = generator(language.to_string()).unwrap();
    assert_eq!(iter_output, direct_output);
    assert_eq!(iter_output, "3 1 2 1");
}

#[test]
fn test_iterator_with_float_expression() {
    let language = "F[1, 1]";

    // Generate using iterator
    let mut iter_output = String::new();
    for chunk in generator_iter(language.to_string()).unwrap() {
        match chunk {
            Ok(data) => iter_output.push_str(&data),
            Err(e) => panic!("Iterator error: {}", e),
        }
    }
    // Remove trailing space
    if iter_output.ends_with(' ') {
        iter_output.pop();
    }

    // Compare with non-iterator version
    let direct_output = generator(language.to_string()).unwrap();
    assert_eq!(iter_output, direct_output);
    assert_eq!(iter_output, "1");
}

#[test]
fn test_iterator_with_string_expression() {
    let language = "S[5,5,'A']";

    // Generate using iterator
    let mut iter_output = String::new();
    for chunk in generator_iter(language.to_string()).unwrap() {
        match chunk {
            Ok(data) => iter_output.push_str(&data),
            Err(e) => panic!("Iterator error: {}", e),
        }
    }
    // Remove trailing space
    if iter_output.ends_with(' ') {
        iter_output.pop();
    }

    // Compare with non-iterator version
    let direct_output = generator(language.to_string()).unwrap();
    assert_eq!(iter_output, direct_output);
    assert!(!iter_output.is_empty());
    assert!(iter_output.chars().all(|c| c == 'A'));
}

#[test]
fn test_iterator_with_multiple_primitives() {
    let language = "N[1,1] N[2,2] N[3,3]";

    // Generate using iterator
    let mut iter_output = String::new();
    for chunk in generator_iter(language.to_string()).unwrap() {
        match chunk {
            Ok(data) => iter_output.push_str(&data),
            Err(e) => panic!("Iterator error: {}", e),
        }
    }
    // Remove trailing space
    if iter_output.ends_with(' ') {
        iter_output.pop();
    }

    // Compare with non-iterator version
    let direct_output = generator(language.to_string()).unwrap();
    assert_eq!(iter_output, direct_output);
    assert_eq!(iter_output, "1 2 3");
}

#[test]
fn test_iterator_error_handling() {
    let language = "(?:(N)){\\1}"; // This should cause an error (group not found)

    let iter_result = generator_iter(language.to_string());
    assert!(
        iter_result.is_err() || {
            // If the iterator is created, it should produce an error on iteration
            let mut error_found = false;
            if let Ok(iter) = iter_result {
                for chunk in iter {
                    if chunk.is_err() {
                        error_found = true;
                        break;
                    }
                }
            }
            error_found
        }
    );
}

#[test]
fn test_iterator_yields_chunks() {
    let language = "N[1,1] N[2,2] N[3,3]";

    // Count the number of chunks yielded
    let chunk_count = generator_iter(language.to_string())
        .unwrap()
        .filter(|r| r.is_ok())
        .count();

    // We expect 3 chunks (one for each N expression)
    assert_eq!(chunk_count, 3);
}

#[test]
fn test_iterator_streaming_behavior() {
    let language = "N[1,1] N[2,2] N[3,3] N[4,4]";

    let mut iter = generator_iter(language.to_string()).unwrap();

    // First chunk
    let chunk1 = iter.next().unwrap().unwrap();
    assert_eq!(chunk1, "1 ");

    // Second chunk
    let chunk2 = iter.next().unwrap().unwrap();
    assert_eq!(chunk2, "2 ");

    // Third chunk
    let chunk3 = iter.next().unwrap().unwrap();
    assert_eq!(chunk3, "3 ");

    // Fourth chunk
    let chunk4 = iter.next().unwrap().unwrap();
    assert_eq!(chunk4, "4 ");

    // No more chunks
    assert!(iter.next().is_none());
}

#[test]
fn test_iterator_with_capturing_groups() {
    let language = "(N[5,5]) N{\\1}";

    // Generate using iterator
    let mut iter_output = String::new();
    for chunk in generator_iter(language.to_string()).unwrap() {
        match chunk {
            Ok(data) => iter_output.push_str(&data),
            Err(e) => panic!("Iterator error: {}", e),
        }
    }
    // Remove trailing space
    if iter_output.ends_with(' ') {
        iter_output.pop();
    }

    // Parse the output to verify correctness
    let parts: Vec<&str> = iter_output.split_whitespace().collect();
    assert_eq!(parts.len(), 6); // First number + 5 repeated numbers
    let first = parts[0];
    assert_eq!(first, "5");
    // All subsequent should be individual numbers (N generates random numbers)
    for part in parts.iter().skip(1) {
        assert!(!part.is_empty());
        // Verify each part can be parsed as a number
        part.parse::<i64>().expect("Should be a valid integer");
    }
}
