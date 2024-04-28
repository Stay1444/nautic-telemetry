#pragma once

#include <Arduino.h>
#include <stdint.h>

namespace radio::PacketDeserializer {

enum class PacketStatus { Ok, Incomplete, Invalid, FailedCRC };

struct DeserializeResult {
  PacketStatus status;
  const uint8_t *dataStart;
  size_t dataLength;
  uint8_t packetId;
};

DeserializeResult deserialize(const uint8_t *buffer, size_t bufferLength);

} // namespace radio::PacketDeserializer
