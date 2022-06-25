using namespace std;

size_t simple_loop()
{
    size_t sum = 0;
    for (size_t idx = 0; idx < 1000000000; ++idx)
        sum += 1;

    return sum;
}

size_t loop_add_idx()
{
    size_t sum = 0;
    for (size_t idx = 0; idx < 10000000000; ++idx)
        sum += idx;

    return sum;
}

#include <vector>
size_t vec()
{
    vector<int> vec;
    for (size_t idx = 0; idx < 100000; ++idx)
        vec.push_back(idx % 3);

    return 0;
}


size_t vec_2d()
{
    vector<vector<int>> vec;
    for (size_t x = 0; x < 1000; ++x) {
        vector<int> v;
        for (size_t y = 0; y < 1000; ++y) {
            v.push_back(y);
        }
        vec.push_back(v);
    }

    return 0;
}