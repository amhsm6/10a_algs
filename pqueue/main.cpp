#include <iostream>
#include <vector>
#include <tuple>
#include <stdlib.h>
#include <time.h>
#include <initializer_list>
#include <algorithm>

template<class K, class V>
struct PQueue {
public:
    std::vector<std::pair<K, V>> arr;
    
    PQueue() {}
    
    PQueue(std::initializer_list<std::pair<K, V>> a) {
        std::pair<K, V> *iter = a.begin();
        while (iter != a.end()) {
            push(iter->first, iter->second);
            iter++;
        }
    }
    
    void push(K a, V b) {
        arr.push_back({a, b});
        
        size_t c = arr.size() - 1;
        while (c != 0) {
            auto &current = arr[c];
            c = (c - 1) / 2;
            auto &parent = arr[c];
            
            if (current.first >= parent.first) {
                break;
            }
            
            std::swap(current, parent);
        }
    }
    
    std::pair<K, V> pop() {
        auto ret = arr.at(0);
        
        std::swap(arr[0], arr[arr.size() - 1]);
        arr.pop_back();
        
        size_t c = 0;
        while (true) {
            auto left_child_index = 2*c+1;
            auto right_child_index = 2*c+2;
            
            if (left_child_index >= arr.size()) { return ret; }
            auto &current = arr[c];
            
            if (right_child_index >= arr.size()) {
                auto &child = arr[left_child_index];
                
                if (child.first < current.first) { std::swap(child, current); }
                return ret;
            }
            
            auto &left_child  = arr[left_child_index];
            auto &right_child = arr[right_child_index];
            
            auto next_index = left_child.first < right_child.first ? left_child_index : right_child_index;
            auto &next_child = arr[next_index];
            
            if (current.first > next_child.first) { std::swap(current, next_child); }
            c = next_index;
        }
    }
    
    size_t size() {
        return arr.size();
    }
};

void test_pqueue(size_t n) {
    PQueue<int, std::tuple<>> q;
    std::vector<int> array;
    
    for (size_t i = 0; i != n; ++i) {
        int x = rand() % 1000;
        array.push_back(x);
        q.push(x, {});
    }
    
    std::sort(array.begin(), array.end());
    
    for (auto x : array) {
        if (x != q.pop().first) { std::cout << "Test " << n << " failed" << std::endl; return; }
    }
}


int main() {
    srand(time(nullptr));
    
    for (int i = 0; i < 100; ++i) {
        test_pqueue(i);
    }
    
    return 0;
}
