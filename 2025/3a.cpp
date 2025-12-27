#include <iostream>

int main()
{
    size_t tot = 0;
    std::string line;
    while (std::getline(std::cin, line)) {
        int left = 0;
        int right = 0;
        for (size_t i=0; i < line.size(); i++) {
            int val = line[i] - '0';

            if (val > left && i != line.size()-1) {
                right = 0;
                left = val;
            } else if (val > right) {
                right = val;
            }         
        }
        tot += std::stoul(std::to_string(left) + std::to_string(right));
    }
    std::cout << "tot: " << tot << std::endl;
}