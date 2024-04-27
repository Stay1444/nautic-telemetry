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
  cursor.skip(1);

  uint8_t packetId = 0;
  if (!cursor.next(packetId)) {
    result.status = PacketStatus::Incomplete;
    return result;
  }

  uint32_t dataLength = 0;

  if (!cursor.next_u32(dataLength)) {
    result.status = PacketStatus::Incomplete;
    return result;
  }

  size_t dataStart = cursor.position();

  CRC8 crc;
  crc.add(&buffer[dataStart], dataLength);
  cursor.skip(dataLength);

  uint8_t crcValue = 0;
  if (!cursor.next(crcValue)) {
    result.status = PacketStatus::Incomplete;
    return result;
  }

  uint8_t crcExpected = crc.calc();

  if (crcExpected != crcValue) {
    result.status = PacketStatus::FailedCRC;
    Serial.print("Expected CRC ");
    Serial.print(crcExpected);
    Serial.print(" but got ");
    Serial.println(crcValue);
    return result;
  }

  result.status = PacketStatus::Ok;
  result.dataStart = &buffer[dataStart];
  result.dataLength = dataLength;
  result.packetId = packetId;

  return result;
}
