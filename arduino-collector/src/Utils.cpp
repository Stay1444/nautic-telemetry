#include "Utils.h"
#include <stdint.h>

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

uint32_t utils::conversions::convertToUint32(const uint8_t *bytes) {
  uint32_t result = ((uint32_t)bytes[0] << 24) | ((uint32_t)bytes[1] << 16) |
                    ((uint32_t)bytes[2] << 8) | bytes[3];
  return result;
}
