use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Example {
    pub input_format: String,
    pub constraints: String,
    pub generated_language: String,
}

pub fn get_examples() -> Vec<Example> {
    vec![
            Example {
                input_format: "The first line contains an integer T (number of test cases). Each of the next T lines contains two integers N and M.".to_string(),
                constraints: "1 ≤ T ≤ 10\n1 ≤ N, M ≤ 10^5".to_string(),
                generated_language: "(N[1,10]) (?:N[1,100000] N[1,100000]){\\1}".to_string(),
            },
            Example {
                input_format: "The first line contains the length of the string. The second line contains a string S consisting of lowercase English letters.".to_string(),
                constraints: "1 ≤ |S| ≤ 100".to_string(),
                generated_language: "(N[1,100]) S[\\1, \\1, @CH_LOWER@]".to_string(),
            },
            Example {
                input_format: "The first line contains a float X between 0 and 1. The next line contains 5 integers, each between 1 and 100.".to_string(),
                constraints: "0 ≤ X ≤ 1\n1 ≤ each integer ≤ 100".to_string(),
                generated_language: "(F[0,1]) (?:N[1,100]){5}".to_string(),
            },
            Example {
                input_format: "The first line contains an integer N. The second line contains N uppercase letters. The third line contains N random characters.".to_string(),
                constraints: "1 ≤ N ≤ 26".to_string(),
                generated_language: "(N[1,26]) S[\\1, \\1, @CH_UPPER@] S[\\1,@CH_ALL@]".to_string(),
            },
            Example {
                input_format: "The first line contains an integer T. The next T lines each contain a float between -10 and 10, followed by a string of 5 alphanumeric characters.".to_string(),
                constraints: "1 ≤ T ≤ 5".to_string(),
                generated_language: "(N[1,5]) (?:F[-10,10] S[5, 5, @CH_ALNUM@]){\\1}".to_string(),
            },
            Example {
                input_format: "The first line contains an integer N, followed by N lines each containing a string S.".to_string(),
                constraints: "1 ≤ N ≤ 100\n|S| = 50\nS consists of lowercase English letters".to_string(),
                generated_language: "(N[1,100]) S[50, 50, @CH_LOWER@]{\\1}".to_string(),
            },
            Example {
                input_format: "The first line contains two integers N and M. The next N lines contain M integers each.".to_string(),
                constraints: "1 ≤ N, M ≤ 100\n1 ≤ each integer ≤ 1000".to_string(),
                generated_language: "(N[1,100]) (N[1,100]) (?:(?:N[1,1000]){\\2}){\\1}".to_string(), 
            },
            Example {
                input_format: "The first line contains an integer N. The next line conatins a string S of alphanumeric characters.".to_string(),
                constraints: "1 ≤ N ≤ 100\n1 ≤ |S| ≤ 50".to_string(),
                generated_language: "N[1,100] S[1, 50, @CH_ALNUM@]".to_string(),
            },
            Example {
                input_format: "The first line contains an integer N. The next line contains N string containing the letters 'ABCD$#?'.".to_string(),
                constraints: "1 ≤ N ≤ 100\n1≤|S|≤4".to_string(),
                generated_language: "(N[1,100]) (?:S[1,4,'ABCD$#?']){\\1}".to_string(),
            },
        ]
}
