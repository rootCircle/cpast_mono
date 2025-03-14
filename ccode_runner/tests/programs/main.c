#include <stdio.h>

int main() {
    double number;
    scanf("%lf", &number);
    double square = number * number;
    printf("%.2f\n", square);
    return 0;
}
