#include "Utils.h"
#include <stdint.h>

void utils::arrays::copy(const uint8_t *source, uint8_t *destination,
                         size_t length) {
  for (size_t i = 0; i < length; i++) {
    destination[i] = source[i];
  }
}

void utils::arrays::trimStart(uint8_t *source, size_t trimLength,
                              size_t totalLength) {
  if (trimLength >= totalLength) {
    return;
  }

  size_t newSize = totalLength - trimLength;

  if (newSize == 0) {
    return;
  }

  for (size_t i = 0; i < newSize; ++i) {
    source[i] = source[i + trimLength];
  }
}

size_t utils::conversions::bytesToSizeT(const uint8_t *bytes) {
  size_t result = 0;
  for (size_t i = 0; i < sizeof(size_t); ++i) {
    result |= (static_cast<size_t>(bytes[i]) << (i * 8));
  }
  return result;
}

uint32_t utils::conversions::bytesToUint32(const uint8_t *bytes) {
  uint32_t result = ((uint32_t)bytes[0] << 24) | ((uint32_t)bytes[1] << 16) |
                    ((uint32_t)bytes[2] << 8) | bytes[3];
  return result;
}

float utils::conversions::bytesToFloat(const uint8_t *bytes) {
  static_assert(sizeof(float) == 4, "float size must be 4 bytes");

  // Create a temporary uint32_t to hold the bytes
  uint32_t temp = ((uint32_t)bytes[0] << 24) | ((uint32_t)bytes[1] << 16) |
                  ((uint32_t)bytes[2] << 8) | bytes[3];

  // Create a float from the memory layout of the uint32_t
  float result;
  memcpy(&result, &temp, sizeof(float));

  return result;
}
