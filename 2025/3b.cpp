#include <iostream>
#include <vector>
#include <machine/limits.h>


size_t join(const std::vector<size_t>& deq)
{
    size_t acc =0;
    for (auto&& el : deq) {
        acc = acc*10 +el;
    }
    std::cout << acc << std::endl;
    return acc;
}

int main()
{
    size_t tot = 0;
    std::string line;
    while (std::getline(std::cin, line)) {
        std::vector<size_t> vec;
        size_t start = 0;
        size_t rem = 12;
        while (rem > 0) {
            size_t finish = line.size() - rem + 1;
            size_t max = 0;
            size_t max_i = 0;

            for (size_t i = start; i < finish; i++) {
                size_t val = line[i] - '0';

                if (val > max) {
                    max = val;
                    max_i = i;
                }
                
            }
            vec.push_back(max);
            start = max_i + 1;
            rem--;
        }
        tot += join(vec);
    }
    std::cout << "tot: " << tot << std::endl;
}