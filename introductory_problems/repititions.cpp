#include <algorithm>
#include <iostream>
#include <string>

void solve(std::string &sequence) {
    int maxLen = 1, curLen = 1;
    for (auto i = 1; i < sequence.length(); i++) {
        if (sequence[i - 1] != sequence[i]) {
            maxLen = std::max(curLen, maxLen);
            curLen = 1;
        } else {
            curLen++;
        }
    }
    std::cout << std::max(curLen, maxLen);
}

int main() {
    std::ios::sync_with_stdio(false);
    std::string sequence; std::cin >> sequence;
    solve(sequence);
}