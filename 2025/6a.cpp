#include <iostream>
#include <sstream>

int main()
{
    std::vector<std::vector<long>> tokens;
    std::vector<std::string> lines;
    std::string line;
    while (std::getline(std::cin, line)) {
        lines.push_back(line);
    }

    for (int i = 0; i < lines.size() - 1; i++ ) {
        std::istringstream istream(lines[i]);
        tokens.push_back(std::vector<long>(std::istream_iterator<long>(istream), std::istream_iterator<long>()));
    }

    std::istringstream istream(lines.back());
    
    std::string op;
    int i = 0;
    long tot = 0;
    while (istream >> op) {
        long acc;
        switch (op[0]) {
            case '*':
                acc = 1;
                for (auto&& vec : tokens) {
                    acc *= vec[i];
                }
                break;
            case '+':
                acc = 0;
                for (auto&& vec : tokens) {
                    acc += vec[i];
                }
                break;
        }
        tot += acc;
        i++;
    }
    std::cout << tot << std::endl;
}
