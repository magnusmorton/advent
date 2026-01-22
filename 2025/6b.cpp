#include <iostream>
#include <functional>

int main()
{
    std::vector<std::vector<long>> tokens;
    std::vector<std::string> lines;
    std::string line;
    while (std::getline(std::cin, line)) {
        lines.push_back(line);
    }

    int i = 0;
    long tot = 0;
    line = lines.back();
    while (i < line.size()){
        char op = line[i];
        size_t next = line.find_first_of("+*", i+1);
        if (next == std::string::npos) {
            next = line.size()+1;
        }
        long acc;
        std::function<long(long, long)> func;
        switch (op) {
            case '+':
                acc = 0;
                func = std::plus<long>();
                break;
            case '*':
                acc = 1;
                func = std::multiplies<long>();
                break;
        }
        for (int col = next-2; col >= i; col--) {
            std::string num;
            for (size_t j = 0; j < lines.size()-1; j++) {
                num.push_back(lines[j][col]);
            }
            unsigned long parsed = std::stoul(num);
            acc = func(acc, parsed);
        }
        i=next;
        tot += acc;    
    }
    std::cout << tot << std::endl;
}
