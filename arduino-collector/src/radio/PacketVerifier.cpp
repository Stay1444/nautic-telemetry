#include "PacketVerifier.h"

#include <Arduino.h>
#include <stdint.h>

using namespace radio::PacketVerifier;

Result verify(const uint8_t *buffer, size_t bufferLength, size_t *end) {
  return Result::Ok;
}
