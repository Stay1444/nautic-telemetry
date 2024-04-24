#pragma once

#include <Arduino.h>
#include <stdint.h>

namespace utils::arrays {
void copy(uint8_t *source, uint8_t *destination, size_t length);
}
