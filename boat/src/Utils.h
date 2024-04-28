#pragma once

#include <Arduino.h>
#include <stdint.h>

namespace utils {
namespace arrays {
void copy(const uint8_t *source, uint8_t *destination, size_t length);
}
namespace conversions {
size_t bytesToSizeT(const uint8_t *bytes);
uint32_t bytesToUint32(const uint8_t *bytes);
float bytesToFloat(const uint8_t *bytes);
} // namespace conversions
} // namespace utils