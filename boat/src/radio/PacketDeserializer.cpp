#include "PacketDeserializer.h"
#include "radio/Protocol.h"
#include "utils/Cursor.h"

#include <Arduino.h>
#include <stdint.h>

using namespace radio::PacketDeserializer;

DeserializeResult radio::PacketDeserializer::deserialize(const uint8_t *buffer,
                                                         size_t bufferLength) {
  DeserializeResult result;

  if (bufferLength <
      2 + sizeof(uint32_t)) // packet head, id, data length (uint32_t)
  {
    result.status = PacketStatus::Incomplete;
    return result;
  }

  if (buffer[0] != PACKET_HEAD_BYTE) {
    result.status = PacketStatus::Invalid;
    return result;
  }

  Cursor cursor(buffer, bufferLength);
  cursor.skip(1);

  uint8_t packetId = 0;
  if (!cursor.next(packetId)) {
    result.status = PacketStatus::Incomplete;
    return result;
  }

  uint32_t dataLength = 0;

  if (!cursor.next(dataLength)) {
    result.status = PacketStatus::Incomplete;
    return result;
  }

  if (cursor.remaining() != dataLength) {
    result.status = PacketStatus::Incomplete;
    return result;
  }

  size_t dataStart = cursor.position();

  result.status = PacketStatus::Ok;
  result.dataStart = &buffer[dataStart];
  result.dataLength = dataLength;
  result.packetId = packetId;

  return result;
}
