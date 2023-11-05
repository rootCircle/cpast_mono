# Grammar Rules for TestCase Generator

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
- Literal non-negative numbers (to denote ranges/no. of occurrences/back-referencing etc)

### Special Functions

- () => Capturing Group Indexed by 1
- (?:) => Non-capturing Group
- \1 => Back-reference
- (?:.....){} => Specify number of occurrence of group
- N|F[m, n] => Specifying min and max values of N or F (Skip one of the values means MIN and MAX respectively), check for the string if it is within the range or not

## Example Usage

- `(N) N[,1000] (?:N F S){\1}` : Accepts: "2 2 2 2.2 ABC2 3 4.5 ASD"
                          Description: Two integers(say k1 and k2 with k2<=1000) followed by triplets of Integer Float and String occurring k1(first capturing group) times.
- `(N[,1000]){\2}` => Valid
- `(?:N[,1000]{\2})` => Valid
- `(?:N{\2}[,1000])` => Invalid
- `(N F)` => Invalid, Capturing Group can only contain a single non-negative number only, else put it in non-capturing one

## References
- [Back-references in repetition construct regex](https://stackoverflow.com/questions/3407696/using-a-regex-back-reference-in-a-repetition-construct-n)
- [Back-references S.O.](https://stackoverflow.com/questions/29728622/regex-with-backreference-as-repetition-count)
- [Possible solution using Code Callout](https://stackoverflow.com/questions/29728622/regex-with-backreference-as-repetition-count/61898415#61898415)