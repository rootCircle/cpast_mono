use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SolutionTurn {
    pub(crate) statement: String,
    pub(crate) input_format: String,
    pub(crate) constraints: String,
    pub(crate) generated_code: String,
}

pub(crate) fn get_examples() -> Vec<SolutionTurn> {
    vec![
        // Example 1: CodeChef Time Penalty Problem
        SolutionTurn {
            statement: r#"
You are participating in CodeChef Starters 173, which has a time penalty of 10 minutes for every incorrect submission you make.
That is, the total penalty for a problem equals the number of minutes from the start of the contest till your submission receives the AC verdict, plus 10 minutes for every incorrect submission made before that.

You solved a problem X minutes after the start of the contest, and made Y incorrect submissions while doing so.
What's the total time penalty for this problem?

Output Format:
Output a single integer: the total time penalty for the problem.
            "#.to_string(),
            input_format: "The first and only line of input will contain two space-separated integers X and Y — the number of minutes after which you solved the problem, and the number of wrong submissions you made.".to_string(),
            constraints: "1 ≤ X ≤ 150\n0 ≤ Y ≤ 10".to_string(),
            generated_code: r#" 
#include <iostream>
using namespace std;

int main() {
    int X, Y;
    cin >> X >> Y;
    cout << (X + Y * 10) << endl;
    return 0;
}
"#.to_string(),
        },
        // Example 2: Factorial Calculation
        SolutionTurn {
            statement: r#"
Write a program to calculate the factorial of a given non-negative integer N.
Recall that the factorial of N, denoted as N!, is defined as:
    N! = N × (N-1) × (N-2) × ... × 2 × 1, for N > 0, and
    0! = 1

Output Format:
Print the factorial of the number N.
            "#.to_string(),
            input_format: "The input consists of a single integer N.".to_string(),
            constraints: "0 ≤ N ≤ 20".to_string(),
            generated_code: r#" 
#include <iostream>
using namespace std;

long long factorial(int n) {
    return (n == 0) ? 1 : n * factorial(n - 1);
}

int main() {
    int N;
    cin >> N;
    cout << factorial(N) << endl;
    return 0;
}
"#.to_string(),
        },
        // Example 3: Prime Number Checker
        SolutionTurn {
            statement: r#"
Create a program to determine if a given number N is prime.
A prime number is a natural number greater than 1 that is not a product of two smaller natural numbers.

Output Format:
Output "Prime" if N is a prime number, or "Not prime" otherwise.
            "#.to_string(),
            input_format: "A single integer N.".to_string(),
            constraints: "2 ≤ N ≤ 10^5".to_string(),
            generated_code: r#" 
#include <iostream>
#include <cmath>
using namespace std;

bool is_prime(int n) {
    if (n < 2) return false;
    for (int i = 2; i <= sqrt(n); i++) {
        if (n % i == 0) return false;
    }
    return true;
}

int main() {
    int N;
    cin >> N;
    cout << (is_prime(N) ? "Prime" : "Not prime") << endl;
    return 0;
}
"#.to_string(),
        },
        // Example 4: Matrix Multiplication
        SolutionTurn {
            statement: r#"
Given two square matrices of size N x N, write a program to multiply them.

Output Format:
Print the resultant N x N matrix, with each row in a new line, and elements separated by spaces.
            "#.to_string(),
            input_format: "The first line contains an integer N. The next N lines contain N space-separated integers representing the first matrix. The next N lines contain N space-separated integers representing the second matrix.".to_string(),
            constraints: "1 ≤ N ≤ 100, Matrix elements are integers between -1000 and 1000".to_string(),
            generated_code: r#" 
#include <iostream>
using namespace std;

int main() {
    int N;
    cin >> N;
    int A[N][N], B[N][N], result[N][N] = {0};

    for (int i = 0; i < N; i++)
        for (int j = 0; j < N; j++)
            cin >> A[i][j];

    for (int i = 0; i < N; i++)
        for (int j = 0; j < N; j++)
            cin >> B[i][j];

    for (int i = 0; i < N; i++)
        for (int j = 0; j < N; j++)
            for (int k = 0; k < N; k++)
                result[i][j] += A[i][k] * B[k][j];

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++)
            cout << result[i][j] << " ";
        cout << endl;
    }
    return 0;
}
"#.to_string(),
        },
        // Example 5: Dijkstra's Algorithm (Shortest Path)
        SolutionTurn {
            statement: r#"
Implement Dijkstra's Algorithm to find the shortest path from a given source node in a weighted graph.

Output Format:
For each node, print the shortest distance from the source node.
            "#.to_string(),
            input_format: "The first line contains two integers N (nodes) and M (edges). The next M lines contain three integers U, V, W (representing an edge from U to V with weight W). The last line contains the source node.".to_string(),
            constraints: "1 ≤ N ≤ 10^5, 1 ≤ M ≤ 2×10^5, 1 ≤ U, V ≤ N, 1 ≤ W ≤ 10^9".to_string(),
            generated_code: r#" 
#include <iostream>
#include <vector>
#include <queue>
#include <limits>

using namespace std;
typedef pair<int, int> pii;

int main() {
    int N, M;
    cin >> N >> M;

    vector<vector<pii>> graph(N + 1);
    for (int i = 0; i < M; i++) {
        int U, V, W;
        cin >> U >> V >> W;
        graph[U].push_back({W, V});
    }

    int source;
    cin >> source;
    
    vector<long long> dist(N + 1, LLONG_MAX);
    priority_queue<pii, vector<pii>, greater<pii>> pq;

    dist[source] = 0;
    pq.push({0, source});

    while (!pq.empty()) {
        auto [d, node] = pq.top();
        pq.pop();

        if (d > dist[node]) continue;

        for (auto [weight, neighbor] : graph[node]) {
            long long new_dist = d + weight;
            if (new_dist < dist[neighbor]) {
                dist[neighbor] = new_dist;
                pq.push({new_dist, neighbor});
            }
        }
    }

    for (int i = 1; i <= N; i++) {
        cout << (dist[i] == LLONG_MAX ? -1 : dist[i]) << endl;
    }

    return 0;
}
"#.to_string(),
        },
    ]
}
