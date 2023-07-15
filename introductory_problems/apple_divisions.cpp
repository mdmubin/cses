#include <iostream>
#include <vector>
 
#define i32 int32_t
#define i64 int64_t
#define u32 uint32_t
 
#define ABS(X) ((((X) >> 31) | 1) * (X))
#define MIN(x, y) ((x < y) ? (x) : (y))


int main() {
    i32 n;
    std::cin >> n;
 
    i64 total_w = 0;
    std::vector<i64> apples;
    apples.reserve(20);
 
    i64 x;
    for (int i = 0; i < n; i++) {
        std::cin >> x;
        apples.push_back(x);
        total_w += x;
    }
 
    i64 mindiff = INT64_MAX;
 
    i64 g1;
 
    // (1 << n) == pow(2, n)
    // `i` iterates over all combinations of 1's and 0's within 0 ~ 2^n
    for (i64 i = 0; i < (1LL << n); i++) {
        g1 = 0;
 
        for (u32 j = 0; j < apples.size(); j++) {
            // only add apples to group 1.
            // done only if current bit of `i` is 1
            if ((i >> j) & 1) {
                g1 += apples[j];
            }
        }
 
        mindiff = MIN(mindiff, ABS(g1 - (total_w - g1)));
    }
 
    std::cout << mindiff;
}