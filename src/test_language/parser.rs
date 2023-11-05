

struct Language {
    tokens: Vec<LangTypes>
}

enum LangType {
    CapturingGroup {
        group_number: u32,
        value: CharacterSet
    },
    NonCapturingGroup {
        value: CharacterSet
    },

}

struct CharacterSets {
    charset: Vec<CharacterSet>
}

enum CharacterSet {
    Float,
    Integer(i64, i64), // Max and Min values
    String,
    Char
}