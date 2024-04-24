#include "Utils.h"

void utils::arrays::copy(const uint8_t *source, uint8_t *destination,
                         size_t length) {
  for (size_t i = 0; i < length; i++) {
    destination[i] = source[i];
  }
}

size_t utils::conversions::convertToSizeT(const uint8_t *bytes) {
  size_t result = 0;
  for (size_t i = 0; i < sizeof(size_t); ++i) {
    result |= (static_cast<size_t>(bytes[i]) << (i * 8));
  }
  return result;
}
