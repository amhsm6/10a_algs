#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <map>

struct mystr {
    std::string a = "";
    
    mystr combine(mystr b) {
        return {
            a + b.a
        };
    }
};

template<class K, class V>
struct Object {
    K left;
    K right;
    V v;
};

template<class K, class V, int GROUP_SIZE = 2>
struct Tree {
public:
    std::vector<std::vector<Object<K, V>>> data;
    
    Tree(std::map<K, V> input) {
        std::vector<Object<K, V>> start;
        for (auto e : input) {
            start.push_back({e.first, e.first, e.second});
        }
        sort(start);
        data.emplace_back();
        data[0] = std::move(start);
        int k = 0;
        while (data.back().size() != 1) {
            update(data[k]);
            k++;
        }
    }
    
    void sort(std::vector<Object<K, V>> &v) {
        std::sort(v.begin(), v.end(), [](Object<K, V> a, Object<K, V> b) {
            return a.left < b.left;
        });
    }
    
    void update(std::vector<Object<K, V>> current) {
        data.emplace_back();
        for (int i = 0; i < current.size(); i += GROUP_SIZE) {
            std::vector<int> keys_left;
            std::vector<int> keys_right;
            V value;
            
            for (int j = i; j < i + GROUP_SIZE; j++) {
                if (j >= current.size()) {
                    break;
                }
                keys_left.push_back(current[j].left);
                keys_right.push_back(current[j].right);
                value = value.combine(current[j].v);
            }
            
            data.back().push_back({
                *std::min_element(keys_left.begin(), keys_left.end()),
                *std::max_element(keys_right.begin(), keys_right.end()),
                value
            });
        }
    }
    
    void set(K a, V b) {
        set(a, b, data.size() - 1, 0);
    }
    
    void set(K a, V b, int level, int index) {
        for (int i = index; i < index + GROUP_SIZE; i++) {
            if (i == data[level].size()) {
                break;
            }
            
            if (data[level][i].left <= a && data[level][i].right >= a) {
                if (level == 0) {
                    data[level][i].v = b;
                    return;
                }
                set(a, b, level - 1, i * GROUP_SIZE);
                data[level][i].v = get(data[level][i].left, data[level][i].right, level - 1, i * GROUP_SIZE);
            }
        }
    }
    
    V get(int left, int right) {
        return get(left, right, data.size() - 1, 0);
    }
    
    V get(int left, int right, int level, int index) {
        if (level == -1) {
            return data[0][index / GROUP_SIZE].v;
        }
        V result;
        for (int i = index; i < index + GROUP_SIZE; i++) {
            if (i == data[level].size()) {
                break;
            }
            
            if (data[level][i].right < left) {
                continue;
            }
            if (data[level][i].left > right) {
                break;
            }
            if (data[level][i].left >= left && data[level][i].right <= right) {
                result = result.combine(data[level][i].v);
                continue;
            }
            result = result.combine(get(left, right, level - 1, i * GROUP_SIZE));
        }
        return result;
    }
};

int main() {
    Tree<int, mystr> a({
        {1, {"a"}},
        {7, {"d"}},
        {4, {"c"}},
        {9, {"e"}},
        {3, {"b"}},
        {10, {"f"}}
    });
    
    a.set(1, {"i"});
    std::cout << a.get(1, 10).a << std::endl;
    
    return 0;
}
