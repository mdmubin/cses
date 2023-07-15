#include <algorithm>
#include <iostream>
#include <vector>
 
using u64 = uint64_t;
 
int main() {
    int n; std::cin >> n;
 
    std::vector<u64> coins(n);
    for (int i = 0; i < n; i++) {
        std::cin >> coins[i];
    }
 
    std::sort(coins.begin(), coins.end());
    
    u64 curSum = 0;
    
    for (u64 i: coins) {
        if (curSum + 1 < i) {
            break;
        }
        curSum += i;
    }
    
    std::cout << curSum + 1;
}