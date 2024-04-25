#include "PacketDeserializer.h"
#include "radio/Protocol.h"
#include "utils/Cursor.h"

#include <Arduino.h>
#include <stdint.h>

#include "CRC8.h"

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

  uint8_t packetId = cursor.next();

  uint32_t dataLength = cursor.next_u32();
  size_t dataStart = cursor.position();

  CRC8 crc;
  crc.add(&buffer[dataStart], dataLength);
  cursor.skip(dataLength);

  uint32_t packetIndex = cursor.next_u32();

  uint8_t crcValue = cursor.next();
  uint8_t crcExpected = crc.calc();

  if (crcExpected != crcValue) {
    result.status = PacketStatus::FailedCRC;
    return result;
  }

  result.status = PacketStatus::Ok;
  result.dataStart = &buffer[dataStart];
  result.dataLength = dataLength;
  result.packetId = packetId;
  result.packetIndex = packetIndex;

  return result;
}
