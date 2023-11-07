# Grammar Rules for Clex Generator

Written similar to regex, just so that regex doesn't support Repeating Things through Back-references and Regex is unnecessary complex.

## Rules

### Meta-characters
`()?:\{}[],`

### Character Sets

- SPACE = WHITESPACE | e
- N = Integer (-infinity to infinity)
- F = Float (-infinity.sth to infinity.sth)
- S = Non-whitespace String
- C = Non-whitespace Character

### Misc character
- Literal non-negative numbers (to denote ranges/no. of occurrences/back-referencing etc.)

### Special Functions

- () => Capturing Group Indexed by 1
- (?:) => Non-capturing Group
- \1 => Back-reference
- (?:.....){} => Specify number of occurrence of group
- N|F[m, n] => Specifying min and max values of N or F (Skip one of the values means MIN and MAX respectively), check for the string if it is within the range or not

## Rule
- Capturing group can only have **single** **non-negative** **INTEGER** element. If not specified its min value is set to 0.

## Language

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

### Newer Designs
- PROGRAM := Vector<PRIMARY_DATA_TYPE | CAPTURING_GROUP | NON_CAPTURING_GROUP>
- PRIMARY_DATA_TYPE(REPETITION_STORE) := NUMERAL_TYPE(MIN_VALUE, MAX_VALUE) | CHARACTER | STRING
- NUMERAL_TYPE(MIN_VALUE, MAX_VALUE) := INTEGER | FLOAT
- CAPTURING_GROUP := PRIMARY_DATA_TYPE(1)::NUMERAL_TYPE(0|POSITIVE_NUMBER, MAX_VALUE)::INTEGER
- NON_CAPTURING_GROUP(REPETITION_STORE) := Vector<PRIMARY_DATA_TYPE | NON_CAPTURING_GROUP>
- REPETITION_STORE := BY_GROUP(GROUP_NO) | BY_COUNT(POSITIVE_NUMBER) | NONE

- MIN_VALUE, MAX_VALUE is of type Integer 
- POSITIVE_NUMBER is non-negative Integer
- Vector -> Similar to Variable Sized Array

## Example Usage

- `(N) N[,1000] (?:N F S){\1}` : Accepts: "2 2 2 2.2 ABC2 3 4.5 ASD"
                          Description: Two integers(say k1 and k2 with k2<=1000) followed by triplets of Integer Float and String occurring k1(first capturing group) times.
- `(N[,1000]){\2}` => Valid
- `(?:N[,1000]{\2})` => Valid
- `(?:N{\2}[,1000])` => Invalid
- `(N F)` => Invalid, Capturing Group can only contain a single non-negative number only, else put it in non-capturing one

## TODO Later

- [ ] Floating Limit support in Range Bounds for Numeral Data Type for Float

## References
- [Back-references in repetition construct regex](https://stackoverflow.com/questions/3407696/using-a-regex-back-reference-in-a-repetition-construct-n)
- [Back-references S.O.](https://stackoverflow.com/questions/29728622/regex-with-backreference-as-repetition-count)
- [Possible solution using Code Call-out](https://stackoverflow.com/questions/29728622/regex-with-backreference-as-repetition-count/61898415#61898415)