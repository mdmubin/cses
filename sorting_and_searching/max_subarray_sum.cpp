#include <algorithm>
#include <iostream>
 
 
int main() {
    int32_t n; std::cin >> n;
    auto val = new int64_t[n];
 
    for (int i = 0; i < n; i++) {
        std::cin >> val[i];
    }
 
    int64_t curSum = val[0];
    int64_t maxSum = val[0];
 
    for (auto i = 1; i < n; i++) {
        curSum = std::max(val[i], curSum + val[i]);
        maxSum = std::max(curSum, maxSum);
    }
 
    std::cout << maxSum;
    delete val;
}