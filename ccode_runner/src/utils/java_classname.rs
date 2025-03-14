use regex::Regex;

pub fn get_java_public_classname_from_text(text: &str) -> Option<String> {
    // Regex explanation:
    // (?m) - Multiline mode
    // (?i) - Case-insensitive matching
    // \bpublic\s+ - Match "public" as a word boundary followed by whitespace
    // (?:(?:abstract|final|strictfp)\s+)? - Optional class modifiers
    // class\s+ - Match "class" followed by whitespace
    // ([A-Za-z_][A-Za-z0-9_$]*) - Capture group for class name: starts with letter/underscore, followed by letters/numbers/underscore/dollar
    let re = Regex::new(
        r"(?m)(?i)\bpublic\s+(?:(?:abstract|final|strictfp)\s+)?class\s+([A-Za-z_][A-Za-z0-9_$]*)",
    )
    .unwrap();
    re.captures(text).map(|cap| cap[1].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_class() {
        let input = r#"
            public class MyClass {
                public static void main(String[] args) {}
            }
        "#;
        assert_eq!(
            get_java_public_classname_from_text(input),
            Some("MyClass".to_string())
        );
    }

    #[test]
    fn test_public_abstract_class() {
        let input = r#"
            public abstract class MyAbstractClass {
                public static void main(String[] args) {}
            }
        "#;
        assert_eq!(
            get_java_public_classname_from_text(input),
            Some("MyAbstractClass".to_string())
        );
    }

    #[test]
    fn test_public_final_class() {
        let input = r#"
            public final class MyFinalClass {
                public static void main(String[] args) {}
            }
        "#;
        assert_eq!(
            get_java_public_classname_from_text(input),
            Some("MyFinalClass".to_string())
        );
    }

    #[test]
    fn test_public_strictfp_class() {
        let input = r#"
            public strictfp class MyStrictfpClass {
                public static void main(String[] args) {}
            }
        "#;
        assert_eq!(
            get_java_public_classname_from_text(input),
            Some("MyStrictfpClass".to_string())
        );
    }

    #[test]
    fn test_no_public_class() {
        let input = r#"
            class MyClass {
                public static void main(String[] args) {}
            }
        "#;
        assert_eq!(get_java_public_classname_from_text(input), None);
    }

    #[test]
    fn test_class_with_extends() {
        let input = r#"
            public class Dog extends Animal {
                public static void main(String[] args) {}
            }
        "#;
        assert_eq!(
            get_java_public_classname_from_text(input),
            Some("Dog".to_string())
        );
    }

    #[test]
    fn test_class_with_implements() {
        let input = r#"
            public class Dog implements Animal, Pet {
                public static void main(String[] args) {}
            }
        "#;
        assert_eq!(
            get_java_public_classname_from_text(input),
            Some("Dog".to_string())
        );
    }

    #[test]
    fn test_class_with_comments() {
        let input = r#"
            // This is a comment
            public class MyClass {
                public static void main(String[] args) {}
            }
        "#;
        assert_eq!(
            get_java_public_classname_from_text(input),
            Some("MyClass".to_string())
        );
    }
}
