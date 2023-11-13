#include <iostream>
#include <vector>
#include <algorithm>
#include <chrono>
#include <random>

int main() {
    // Seed for random number generation
    std::random_device rd;
    std::mt19937 gen(rd());

    // Vector initialization
    std::vector<int> myVector(1000000);

    // Fill the vector with random integers
    for (int& num : myVector) {
        num = gen();
    }

    // Measure the time taken for sorting
    for (int i = 0; i < 30; i++ ){ 
        std::sort(myVector.begin(), myVector.end());
    }
    auto start = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < 50; i++ ){ 
        std::sort(myVector.begin(), myVector.end());
    }

    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);

    // Output the sorted vector and the time taken for sorting
    std::cout << "Vector sorted in " << duration.count() / 50 << " milliseconds." << std::endl;

    return 0;
}
