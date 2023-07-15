#include <algorithm>
#include <iostream>
#include <vector>

using vi = std::vector<int>;

int minGondolaCount(const vi &weights, int limit)
{
    size_t l = 0, r = weights.size() - 1;
    int count = 0;
    while (l < r)
    {
        if ((weights[r] + weights[l]) <= limit)
        {
            l++;
        }
        r--;
        count++;
    }
    if (l == r)
    {
        count++;
    }
    return count;
}

int main()
{
    int count, limit;
    std::cin >> count >> limit;

    vi children(count);

    for (int i = 0; i < count; i++)
        std::cin >> children[i];

    std::sort(children.begin(), children.end());

    std::cout << minGondolaCount(children, limit);
}