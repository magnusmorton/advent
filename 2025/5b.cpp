#include <iostream>

int main()
{
    std::vector<std::pair<size_t, size_t>> ranges;
    std::string line;
    while (std::getline(std::cin, line)) {
        if (line == "") {
            break;
        } else {
            size_t i = line.find("-");
            std::string start = line.substr(0, i);
            std::string end = line.substr(i+1);
            size_t start_i = std::stoul(start);
            size_t end_i = std::stoul(end);
            ranges.push_back(std::make_pair(start_i, end_i));
        }
    }
    std::sort(ranges.begin(), ranges.end());
    std::vector<std::pair<size_t,size_t>> merged;
    merged.push_back(ranges[0]);

    for (int i =1; i < ranges.size(); i ++) {
        auto& [lastl, lastr] = merged.back();
        auto& [newl, newr] = ranges[i];

        if (newl <= lastr && newr >= lastr) {
            lastr = newr;
        } else if (newl >= lastl && newr <= lastr) {
            continue;
        } else {
            merged.push_back(ranges[i]);
        }
        
    }

    size_t fresh = 0;
    for (auto&& [l, r]: merged) {
        fresh += r-l+1;
    }
    std::cout << "fresh: " << fresh << std::endl;
}