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
            generated_code: "X, Y = map(int, input().split())\nprint(X + Y * 10)".to_string(),
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
            generated_code: r#"def factorial(n):
    if n == 0:
        return 1
    return n * factorial(n-1)

n = int(input())
print(factorial(n))
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
            generated_code: r#"import math

def is_prime(n):
    if n < 2:
        return False
    for i in range(2, int(math.sqrt(n)) + 1):
        if n % i == 0:
            return False
    return True

n = int(input())
print("Prime" if is_prime(n) else "Not prime")
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
            generated_code: r#"N = int(input())
A = [list(map(int, input().split())) for _ in range(N)]
B = [list(map(int, input().split())) for _ in range(N)]

result = [[sum(A[i][k] * B[k][j] for k in range(N)) for j in range(N)] for i in range(N)]

for row in result:
    print(*row)
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
            generated_code: r#"import heapq
import sys

input = sys.stdin.read
data = input().splitlines()
N, M = map(int, data[0].split())
graph = {i: [] for i in range(1, N+1)}

for i in range(1, M+1):
    U, V, W = map(int, data[i].split())
    graph[U].append((W, V))

source = int(data[M+1])
dist = {i: float('inf') for i in range(1, N+1)}
dist[source] = 0
pq = [(0, source)]

while pq:
    d, node = heapq.heappop(pq)
    if d > dist[node]:
        continue
    for weight, neighbor in graph[node]:
        new_dist = d + weight
        if new_dist < dist[neighbor]:
            dist[neighbor] = new_dist
            heapq.heappush(pq, (new_dist, neighbor))

for i in range(1, N+1):
    print(dist[i] if dist[i] != float('inf') else -1)
"#.to_string(),
        },
        // Example 6: Balanced Parentheses Checker
        SolutionTurn {
            statement: r#"
Write a program to check if a given string of parentheses is balanced.

Output Format:
Print "Balanced" if the string is balanced, otherwise print "Not Balanced".
            "#.to_string(),
            input_format: "A single string containing only characters '(', ')', '{', '}', '[' and ']'.".to_string(),
            constraints: "1 ≤ Length of string ≤ 10^5".to_string(),
            generated_code: r#"def is_balanced(s):
    stack = []
    mapping = {')': '(', '}': '{', ']': '['}
    
    for char in s:
        if char in mapping:
            top_element = stack.pop() if stack else '#'
            if mapping[char] != top_element:
                return "Not Balanced"
        else:
            stack.append(char)
    
    return "Balanced" if not stack else "Not Balanced"

s = input().strip()
print(is_balanced(s))
"#.to_string(),
        },
    ]
}
