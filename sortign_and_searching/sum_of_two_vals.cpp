#include <iostream>
#include <unordered_map>
 
 
int main() {
    int n, sum;
    std::cin >> n >> sum;
    
    std::unordered_map<int, int> indexMap;
    indexMap.reserve(n);
 
    int num;
    bool found = false;
    for (int i = 0; i < n; i++) {
        std::cin >> num;
        auto search = indexMap.find(sum - num);
 
        if (search != indexMap.end()) {
            std::cout << (search->second + 1) << " " << (i + 1);
            found = true; break;
        } else {
            indexMap.insert({num, i});
        }
    }
 
    if (!found) {
        std::cout << "IMPOSSIBLE\n";   
    }
}