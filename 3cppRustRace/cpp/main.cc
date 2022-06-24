#include <chrono>
#include <iostream>
#include "functions.h"

using namespace std;
// A floating point milliseconds type
using FpMilliseconds = 
    std::chrono::duration<float, std::chrono::milliseconds::period>;
using FpSeconds = 
    std::chrono::duration<float, std::chrono::seconds::period>;


int main() 
{
    auto start = chrono::high_resolution_clock::now();

    // size_t res = simple_loop(); // 70-100ns
    // size_t res = loop_add_idx(); // 9.87s
    // size_t res = vec(); // 350 µs
    size_t res = vec_2d(); // 350 µs


    auto stop = chrono::high_resolution_clock::now();
    auto duration_sec = FpSeconds(stop - start);
    auto duration_micro = chrono::duration_cast<chrono::microseconds>(stop - start);
    auto duration_nano = chrono::duration_cast<chrono::nanoseconds>(stop - start);
    cout << duration_sec.count() << " s" << endl;
    cout << duration_micro.count() << " µs" << endl;
    cout << duration_nano.count() << " ns" << endl;

    cout << "Result: " << res << endl;
}

