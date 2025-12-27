#include <iostream>
#include <sstream>
#include <regex>

int main()
{
    const std::regex rex("^(.*)\\1+$");
    size_t invalid = 0;
    size_t count = 0;
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

        for (size_t num = start_i; num <= end_i; num++) {
            if (std::regex_match(std::to_string(num), rex)) {
                invalid += num;
                count++;
            }
        }        
    }
    std::cout << "invalid: " << invalid << " count: " << count <<  std::endl;
}