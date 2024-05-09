> [!IMPORTANT]
> This specification is deprecated since v0.4.0 and is kept solely for archival purposes! See latest specs at [clex specs](./../../clex.specs.md)

# Grammar Rules for Clex Generator

Written similar to regex, just so that regex doesn't support Repeating Things through Back-references and Regex is unnecessary complex.

Refer [clex specs](../../clex.specs.md)for more detailed explanation.

## Rules

### Meta-characters

- `()?:\{}[],'`

### Character Sets

- `SPACE = WHITESPACE | e`
- `N = Integer (-infinity to infinity)`
- `F = Float (-infinity.sth to infinity.sth)`
- `S = Non-whitespace String`
- `C = Non-whitespace Character`

### Misc character
- Literal non-negative numbers (to denote ranges/no. of occurrences/back-referencing etc.)
- 'A' := Literal Characters, currently being in used only in string modifier expression

### Special Functions

- `() => Capturing Group Indexed by 1`
- `(?:) => Non-capturing Group`
- `\1 => Back-reference`
- `(?:.....){} => Specify the number of occurrences of the group`
- `N|F[m, n] => Specifying min and max values of N or F (Skip one of the values means MIN and MAX respectively), check for the string if it is within the range or not`
- `S[l,c] => Specifying the length(l) and character set of String i.e. Alphabets, Alphanumeric, Numeric, Uppercase, Lowercase, Newline, All. If c is not specified then defaults to ALphanumeric.`

## Rule
- Capturing group can only have **single** **non-negative** **INTEGER** element. If not specified its min value is set to 0.

## Language

### Current Design

- `PROGRAM := Vector<UNIT_EXPRESSION>`

- `UNIT_EXPRESSION := PRIMARY_DATA_TYPE(PRIMARY_DATA_TYPE, REFERENCE_TYPE) | CAPTURING_GROUP(POSITIVE NUMBER, PRIMARY_DATA_TYPE) | NON_CAPTURING_GROUP(Vec<UNIT_EXPRESSION>, REFERENCE_TYPE) | EOF`

- `PRIMARY_DATA_TYPE(REFERENCE_TYPE) := INTEGER(REFERENCE_TYPE, REFERENCE_TYPE) | FLOAT(REFERENCE_TYPE, REFERENCE_TYPE) | CHARACTER | STRING(REFERENCE_TYPE, CHARACTER_SET)`

- `CAPTURING_GROUP := PRIMARY_DATA_TYPE(1)::NUMERAL_TYPE(0|POSITIVE_NUMBER, MAX_VALUE)::INTEGER`

- `NON_CAPTURING_GROUP(REFERENCE_TYPE) := Vector<PRIMARY_DATA_TYPE | NON_CAPTURING_GROUP | CAPTURING_GROUP>`

- `REFERENCE_TYPE := BY_GROUP(GROUP_NO) | BY_COUNT(POSITIVE_NUMBER) | NONE`

- `CHARACTER_SET := 'U' | 'L' | 'n' | '0'..='9' | 'D' | 'N' | CHARACTERS` 

- MIN_VALUE, MAX_VALUE is of type Integer 
- POSITIVE_NUMBER is non-negative Integer
- Vector -> Similar to Variable Sized Array

### Old Designs
- NCG = (:?EXPRESSION)
- CG = (N[0,])
- RE = EXPRESSION{\NATURAL_LITERAL_NUMBER}|{NATURAL_LITERAL_NUMBER}
- RBP = N|F[LITERAL_NUMBER|e,LITERAL_NUMBER|e]
- EXPRESSION = CG|NCG|PRIMITIVE_TYPES|RBP|RE
- PRIMITIVE_TYPES = N|F|S|C

- LITERAL_NUMBER = Any Integral numbers like -2, 0, 5 etc
- NATURAL_LITERAL_NUMBER = Only Positive Integers like 1, 5 etc
- e stands for epsilon or null

Acronyms
- NCG = Non Capturing Group
- CG = Capturing Group
- RBP = Range Bound Primitive
- RE = Repeating Expressions

## Example Usage

Here are some example usages of the `clex` language:

- `(N) N[,1000] (?:N F S){\1}` : Accepts input like "2 2 2 2.2 ABC2 3 4.5 ASD".
                          Description: It expects two integers (say k1 and k2 with k2<=1000) followed by triplets of Integer, Float and String, occurring as many times as specified by the first capturing group(k1).

- `(N[,1000]){\2}`: Valid usage.

- `(?:N[,1000]{\2})`: Valid usage.

- `(?:N{\2}[,1000])`: Invalid usage.

- `(N F)` => Invalid, Capturing Group can only contain a single non-negative number only, else put it in non-capturing one

## References

For more details on the `clex` language and advanced usage, you can refer to the following references:

- [Back-references in repetition construct regex](https://stackoverflow.com/questions/3407696/using-a-regex-back-reference-in-a-repetition-construct-n)
- [Back-references S.O.](https://stackoverflow.com/questions/29728622/regex-with-backreference-as-repetition-count)
- [Possible solution using Code Call-out](https://stackoverflow.com/questions/29728622/regex-with-backreference-as-repetition-count/61898415#61898415)
