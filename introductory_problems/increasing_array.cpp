#include <iostream>
#include <vector>
 
using vi = std::vector<int>;
 
void solve(vi& v, int n) {
    uint64_t minMoves = 0;
    for (size_t i = 1; i < v.size(); i++) {
        if (v[i] < v[i - 1]) {
            minMoves += (v[i - 1] - v[i]);
            v[i] += (v[i - 1] - v[i]);
        }
    }
    std::cout << minMoves;
}
 
int main( ) {
    std::ios::sync_with_stdio(false);
    int n; std::cin >> n;
    vi v(n); for (int i = 0; i < n; i++) std::cin >> v[i];
    solve(v, n);    
}