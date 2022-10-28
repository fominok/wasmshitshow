#define export __attribute__((visibility("default")))

#include <vector>
#include <cstdint>

extern "C" export uint32_t add_to_vector_and_sum(uint32_t element) {
  std::vector<uint32_t> vec = {1, 2, 3, 4};
  vec.push_back(element);

  uint32_t sum = 0;

  for (auto& item : vec) sum += item;
  
  return sum;
}
