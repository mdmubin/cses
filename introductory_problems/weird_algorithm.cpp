#include <iostream>

void solve(uint64_t n)
{
    while (n > 1)
    {
        std::cout << n << " ";
        if (n & 1)
        {
            n = n * 3 + 1;
        }
        else
        {
            n >>= 1;
        }
    }
    std::cout << n;
}

int main()
{
    std::ios::sync_with_stdio(false);
    uint64_t n;
    std::cin >> n;
    solve(n);
}