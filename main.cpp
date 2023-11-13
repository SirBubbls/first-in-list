#include <algorithm>
#include <chrono>
#include <iostream>
#include <random>
#include <vector>

int main() {
  // Seed for random number generation
  std::random_device rd;
  std::mt19937 gen(rd());

  // Vector initialization
  std::vector<int> myVector(1000000 * 50);

  // Fill the vector with random integers
  for (int &num : myVector) {
    num = gen() % 1000000;
  }

  // Measure the time taken for sorting
  for (int i = 0; i < 30; i++) {
    std::sort(myVector.begin(), myVector.begin() + 1000000);
  }
  auto start = std::chrono::high_resolution_clock::now();

  for (int i = 1; i < 50; i++) {
    std::sort(myVector.begin() + i * 1000000,
              myVector.begin() + i * 1000000 + 1000000);
  }

  auto end = std::chrono::high_resolution_clock::now();
  auto duration =
      std::chrono::duration_cast<std::chrono::milliseconds>(end - start);

  // Output the sorted vector and the time taken for sorting
  std::cout << "Vector sorted in " << duration.count() / 49 << " milliseconds."
            << std::endl;

  return 0;
}
