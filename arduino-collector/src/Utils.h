#pragma once

#include <Arduino.h>
#include <stdint.h>

namespace utils {
namespace arrays {
void copy(const uint8_t *source, uint8_t *destination, size_t length);
}
namespace conversions {
size_t convertToSizeT(const uint8_t *bytes);
uint32_t convertToUint32(const uint8_t *bytes);
} // namespace conversions
} // namespace utils
