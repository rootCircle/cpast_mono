import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        double number = scanner.nextDouble();
        double square = number * number;
        System.out.println(square);
        scanner.close();
    }
}
