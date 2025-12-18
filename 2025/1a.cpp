#include <iostream>

int main() {
    std::string line;
    int dial = 50;
    int pw = 0;
    while (std::getline(std::cin, line)) {
        char dir = line[0];
        int l = dir == 'L' ? -1 : 1;
        int num = std::stoi(line.substr(1));
        dial = (dial + l*num) % 100;
        if (dial < 0) {
            dial = 100 + dial;
        } else if (dial == 0) {
            pw ++;
        }
    }
    std::cout << pw << std::endl;
}
