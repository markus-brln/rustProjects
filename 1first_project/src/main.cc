#include <chrono>
#include <iostream>

using namespace std;


bool is_odd(long n) 
{
    return n % 2 == 1;
}


int main() 
{
    long upper = 100000000000;

    auto start = chrono::high_resolution_clock::now();

    long acc = 0;
    long idx = 0;
    while (1) {
        long n_squared = idx * idx;
        idx += 1;

        if (n_squared >= upper) {
            break;
        } else if (is_odd(n_squared)) {
            acc += n_squared;
        }
    }
    auto stop = chrono::high_resolution_clock::now();
    auto duration = chrono::duration_cast<chrono::microseconds>(stop - start);
    cout << "Time taken by function: "
         << duration.count() << " microseconds" << endl << acc;
}