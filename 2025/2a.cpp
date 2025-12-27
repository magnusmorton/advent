#include <iostream>
#include <sstream>
#include <cmath>

int main()
{
    size_t invalid = 0;
    std::string line;
    std::getline(std::cin, line);

    std::stringstream ss(line);
    std::string tok;
    while (std::getline(ss, tok, ',')) {
        size_t i = tok.find("-");
        std::string start = tok.substr(0, i);
        std::string end = tok.substr(i+1);
        size_t start_i = std::stoul(start);
        size_t end_i = std::stoul(end);
        size_t end_len = end.size()/2;
        size_t max = std::pow(10l, end_len) - 1;
        size_t min = std::pow(10l, end_len-1);

        for (int dig = min; dig <= max; dig++) {
            size_t candidate = std::stoul(std::to_string(dig) + std::to_string(dig));
            if (candidate >= start_i && candidate <= end_i) {
                invalid+=candidate;
            }
        }
    }
    std::cout << "invalid: " << invalid <<  std::endl;
}