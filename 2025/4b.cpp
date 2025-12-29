#include <iostream>
#include <utility>
#include <vector>

constexpr int dy[] = {-1, -1, -1, 0, 0, 1, 1, 1};
constexpr int dx[] = {-1, 0, 1, -1, 1, -1, 0, 1};

int main()
{
    std::vector<std::string> lines;
    std::string line;
    while (std::getline(std::cin, line)) {
        lines.push_back(line);
    }
    int accessible = 0;
    int maxy = lines.size();
    int maxx = lines[0].size();
    bool converged = false;
    while (!converged) {
        converged = true;
        std::vector<std::pair<int, int>> removable;
        for (int i = 0; i < maxy; i++) {
            for (int j=0; j < maxx; j++) {
                char node = lines[i][j];
                switch (node) {
                    case '@':
                        int rolls = 0;
                        for (int k=0; k < 8; k++) {
                            int x = j + dx[k];
                            int y = i + dy[k];
                            
                            if (x >= 0 && x < maxx && y >= 0 && y < maxy) {
                                if (lines[y][x] == '@') {
                                    rolls++;
                                }
                            }
                        }
                        if (rolls < 4) {
                            accessible++;
                            removable.push_back(std::make_pair(i, j));
                        }
                        break;
                }
            }
        }
        for (auto&& [i, j] : removable) {
            converged = false;
            lines[i][j] = '.';
        }
    }
    std::cout << "removable: " << accessible << std::endl;
}