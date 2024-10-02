## Clex Frequently Asked Questions (FAQs)

This document is a compilation of frequently asked questions (FAQs) about the Clex language and its grammar. The questions and answers are based on a conversation between a large language model and another entity knowledgeable about Clex.

**General:**

* **What is Clex?**
    Clex is a language for generating random text. It uses a simplified grammar similar to regular expressions but focuses on generating random values rather than matching text patterns.

* **What are the strengths of Clex?**
    Clex is known for its simplicity, efficiency, and ease of use. Its limited feature set makes it intuitive to learn and use, particularly for users familiar with basic regular expressions.

* **What are some areas for potential improvement in Clex?**
    While simplicity is valuable, some users might benefit from features that enhance expressiveness without compromising efficiency. Additionally, user customization options and a potential community around Clex could encourage exploration and expansion of the language.

**Grammar:**

* **What are capturing groups in Clex?**
    Capturing groups capture single values during random text generation and can be referenced later in the expression using backreferences. They are numbered based on their order of appearance, starting from 1.

* **Can capturing groups be nested?**
    No, nesting of capturing groups is not supported in Clex.

* **How does Clex handle overlapping capturing groups?**
    Clex does not directly support overlapping capturing groups. Overlapping matches are either disallowed or require special syntax depending on future development decisions.

* **What are quantifiers in Clex?**
    Quantifiers specify how many times a preceding element can be repeated during random text generation. Quantifiers can be applied to non-capturing groups but not directly to capturing groups.

* **Can quantifiers be nested?**
    No.

* **Can ranges in data types specify non-numeric values?**
    No, ranges within data types are currently limited to numeric values. Supporting character ranges or string length ranges is considered for future development, but potential complexity is a concern.

* **Does Clex support advanced features like conditional branching or lookarounds?**
    No, Clex does not currently support features like conditional branching or lookarounds, as these would significantly increase complexity without clear benefits for its core use cases.

* **Does Clex offer user-defined functions or macros for customization?**
    No, the current design of Clex does not include user-defined functions or macros. However, future versions might explore alternative mechanisms for customization without compromising simplicity.

* **What happens if a generated integer falls outside the specified range?**
    The random generated number adheres to the set bound of min and max value of the specified range, ensuring it never falls off the range.

* **How are potential precision issues dealt with for floating-point numbers?**
    Clex uses the standard `double` type for floating-point numbers, providing a familiar level of precision similar to languages like C and C++.

* **Does Clex have built-in mechanisms for error handling or validation?**
    Yes, Clex has error handlers at various levels, from the lexer to the parser to the generator. When an error occurs, the program throws an error, which can be handled or not handled by the user/developer.

* **What are common use cases or applications for Clex?**
    - Debugging programming problems (like testing CP/DSA questions)
    - Live hacking during/after coding contests
    - Generating test cases for problem setters

**Specific Questions:**

* **When specifying a character set for string generation, can it include individual characters (e.g., 'a', 'z') alongside character classes (e.g., 'N', 'L')?**
    No, including individual characters within the character set for string generation is not currently supported in Clex. This design choice prioritizes random text generation, where character classes offer more flexibility.

* **How is the ambiguity between reference to a group number and a literal numeric value resolved during parsing?**
    For backreferences, a backslash (\) prefix denotes a reference to a captured group. So, "1" always refers to a literal value, while "\1" refers to the first captured group.

* **Can quantifiers be nested? For example, is (N{2}){3} a valid expression?**
    No, while nesting quantifiers within non-capturing groups is allowed, applying quantifiers directly to capturing groups is not supported in Clex.

**Comparison to Regular Expressions:**

* **How does Clex compare to traditional regex engines like PCRE or RE2?**
    Clex shares some similarities with Regex in syntax and concept, but their focuses differ. Regex is used for matching text patterns, while Clex specializes in generating random text. This leads to differences in expressiveness and limitations. Clex prioritizes simplicity and efficiency for random generation, while Regex offers more complex features for pattern matching.

**Additional Notes:**

* This FAQ is based on the current understanding of Clex and its limitations. Future development might introduce changes or enhancements to the language.
* If you have further questions or suggestions regarding Clex, feel free to explore the language and its potential or engage with potential communities that might form around it.
