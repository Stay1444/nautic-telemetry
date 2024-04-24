#pragma once

#include <Arduino.h>

namespace radio::PacketVerifier {
enum class Result { Ok, Incomplete, Corrupted };
Result verify(const uint8_t *buffer, size_t bufferLength, size_t *end);
} // namespace radio::PacketVerifier
