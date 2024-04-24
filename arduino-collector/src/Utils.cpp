#include "Utils.h"

void utils::arrays::copy(uint8_t *source, uint8_t *destination, size_t length) {
  for (size_t i = 0; i < length; i++) {
    destination[i] = source[i];
  }
}
