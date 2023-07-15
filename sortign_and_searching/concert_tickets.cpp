#include <algorithm>
#include <iostream>
#include <set>
#include <vector>
 
using pii = std::pair<int, int>;
 
auto cmp = [](const pii &l, const pii &r) {
    return (l.first == r.first) ? (l.second < r.second) : (l.first < r.first);
};
 
using VecI   = std::vector<int>;
using SetPii = std::set<pii, decltype(cmp)>;
 
 
int main() {
    int n, m;
    std::cin >> n >> m;
 
    SetPii tickets{cmp};
    int price;
    for (int i = 0; i < n; i++) {
        std::cin >> price;
        tickets.insert({price, i});
    }

    VecI customers(m);
    for (int i = 0; i < m; i++) {
        std::cin >> customers[i];
    }

    for (int i : customers) {
        auto searched = tickets.upper_bound({i + 1, -1});
        if (searched == tickets.begin()) {
            std::cout << "-1\n";
        } else {
            searched--;
            std::cout << (*searched).first << '\n';
            tickets.erase(searched);
        }
    }
}