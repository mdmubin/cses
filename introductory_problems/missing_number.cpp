#include <iostream>
#include <vector>
 
#define int uint64_t
 
using vi = std::vector<int>;
 
void solve(vi &nums, int n) {
    int sum = n * (n + 1) / 2;
    for (int i : nums) sum -= i;
    std::cout << sum;
}
 
 
int main() {
    int n; std::cin >> n;
    vi nums(n); for (int i = 0; i < n - 1; i++) std::cin >> nums[i];
    solve(nums, n);
}