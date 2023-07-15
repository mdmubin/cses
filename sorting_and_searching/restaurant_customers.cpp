#include <algorithm>
#include <iostream>
#include <vector>
#include <utility>
 
using pib = std::pair<int, bool>;
using vpib = std::vector<pib>;
 
auto cmp = [](const pib &p1, const pib &p2) {
    return p1.first < p2.first;
};
 
int main() {
    std::ios::sync_with_stdio(false);
 
    int n;
    std::cin >> n;
 
    vpib times;
    times.reserve(n * 2);
 
    int arrival, depart;
 
    for (int i = 0; i < n; i++) {
        std::cin >> arrival >> depart;
        times.push_back({arrival, true});
        times.push_back({depart, false});
    }
 
    std::sort(times.begin(), times.end(), cmp);
 
    int count = 0;
    int maxCount = -1;
    for (auto &t : times) {
        if (t.second) {
            count++;
            maxCount = std::max(count, maxCount);
        }
        else count--;
    }
 
    std::cout << maxCount << std::endl;
}