#include <iostream>

int main() {
    std::string line;
    int dial = 50;
    int pw = 0;
    while (std::getline(std::cin, line)) {
        char dir = line[0];
        int l = dir == 'L' ? -1 : 1;
        int num = l*std::stoi(line.substr(1));
        while (num < 0) {
            if (dial == 0) {
                dial = 100;
            }
            dial--;
            num++;
            if (dial == 0) {
                pw++;
            }
        }
        while (num > 0) {
            dial++;
            num--;
            if (dial == 100) {
                pw++;
                dial = 0;
            }
        }
    }
    std::cout << pw << std::endl;
}
