#include <iostream>
#include <vector>
#include <algorithm>

template<class T>
struct Arr {
public:
    std::vector<std::pair<int, std::vector<T>>> a;
    size_t a_size = 0;
    
    static std::vector<T> merge(std::vector<T> a1, std::vector<T> a2) {
        int i = 0;
        int j = 0;
        
        std::vector<T> res;
        while (i < a1.size() && j < a2.size()) {
            if (a1[i] <= a2[j]) {
                res.push_back(a1[i]);
                i++;
            }
            else {
                res.push_back(a2[j]);
                j++;
            }
        }
        
        while (i < a1.size()) {
            res.push_back(a1[i]);
            i++;
        }
        
        while (j < a2.size()) {
            res.push_back(a2[j]);
            j++;
        }
        
        return res;
    }
    
    void push(T n) {
        std::vector<T> to_add {n};
        a_size++;
        
        int i = 0;
        while (true) {
            if (a.size() == i) {
                a.push_back({0, {}});
            }
            
            if (a[i].first == 0) {
                a[i].first = 1;
                a[i].second = std::move(to_add);
                return;
            }
            
            to_add = Arr<int>::merge(std::move(to_add), std::move(a[i].second));
            a[i].first = 0;
            i += 1;
        }
    }
    
    T pop() {
        if (a_size == 0) {
            throw std::runtime_error("Error");
        }
        a_size--;
        std::vector<std::pair<T, size_t>> max_arr;
        
        for (size_t i = 0; i < a.size(); i++) {
            if (a[i].first == 1 && !a[i].second.empty()) {
                max_arr.push_back({a[i].second.back(), i});
            }
        }
    
        std::pair<T, size_t> max {max_arr[0]};
        for (auto e : max_arr) {
            if (e.first > max.first) {
                max = e;
            }
        }
        
        a[max.second].second.pop_back();
        return max.first;
    }
    
    size_t size() {
        return a_size;
    }
};

template<class T>
void print_levels(Arr<T> &a) {
    std::cout << "=== *** ===" << std::endl;
    
    for (size_t i = 0; i != a.a.size(); ++i) {
        std::cout << i << ": " << a.a[i].first << "; ";
        for (auto k : a.a[i].second) {
            std::cout << k << " ";
        }
        std::cout << std::endl;
    }
    
    std::cout << "===========\n" << std::endl;
}

int main() {
    Arr<int> a;
    a.push(2);
    a.push(3);
    a.push(4);
    a.pop();
    a.push(1);
    a.pop();
    
    print_levels(a);
    
    return 0;
}