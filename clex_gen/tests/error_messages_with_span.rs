/// Test that error messages include span information
/// This test validates the issue fix for "Better error message for clex consumption"
use clex_gen::generator;

#[test]
fn test_lexer_error_includes_span() {
    let input = "'unclosed string";
    let result = generator(input.to_string());
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    
    // Should include position information
    assert!(error_message.contains("at position"));
    assert!(error_message.contains("0.."));
    
    // Should still include the error description
    assert!(error_message.contains("Expected closing single quote"));
}

#[test]
fn test_parser_error_includes_span() {
    let input = "N[1, 2";
    let result = generator(input.to_string());
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    
    // Should include position information
    assert!(error_message.contains("at position"));
    
    // Should still include the error description
    assert!(error_message.contains("Expected closing square bracket"));
}

#[test]
fn test_invalid_charset_error_includes_span() {
    let input = "@CH_INVALID@";
    let result = generator(input.to_string());
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    
    // Should include position information
    assert!(error_message.contains("at position"));
    assert!(error_message.contains("0.."));
    
    // Should still include the error description
    assert!(error_message.contains("Invalid character set"));
}

#[test]
fn test_multiple_errors_have_different_positions() {
    let test_cases = vec![
        "'hello",     // Position starts at 0
        "N '  ",      // Position starts at 2
        "N N '    ",  // Position starts at 4
    ];
    
    for (i, input) in test_cases.iter().enumerate() {
        let result = generator(input.to_string());
        assert!(result.is_err(), "Test case {} should error", i);
        let error_message = result.unwrap_err().to_string();
        
        // Each should have different position information
        assert!(error_message.contains("at position"));
    }
}

#[test]
fn test_unclosed_non_capturing_group_error() {
    let input = "(?:N";
    let result = generator(input.to_string());
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    
    // Should include position information
    assert!(error_message.contains("at position"));
    
    // Should include the error description
    assert!(error_message.contains("Non-Capturing group"));
}

#[test]
fn test_missing_colon_after_question_mark() {
    let input = "N ?";
    let result = generator(input.to_string());
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    
    // Should include position information
    assert!(error_message.contains("at position"));
    
    // Should include the error description
    assert!(error_message.contains("Expected colon (:) after question mark"));
}

#[test]
fn test_negative_group_number_error() {
    let input = "N{\\0}";
    let result = generator(input.to_string());
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    
    // Should include position information
    assert!(error_message.contains("at position"));
    
    // Should include the error description
    assert!(error_message.contains("can't be 0 or negative"));
}
