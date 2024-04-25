#include "PacketDeserializer.h"
#include "Utils.h"
#include "radio/Protocol.h"

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

  uint8_t packetId = buffer[1];
  uint8_t dataLength = utils::conversions::convertToUint32(&buffer[2]);
  uint8_t expectedPacketLength =
      dataLength + sizeof(uint8_t) * 2 + sizeof(uint32_t) * 2 +
      sizeof(uint8_t); // head | id | data length | ... | index | crc

  if (bufferLength < expectedPacketLength) {

    result.status = PacketStatus::Incomplete;
    return result;
  }

  uint32_t packetIndex = utils::conversions::convertToUint32(
      &buffer[expectedPacketLength - 1 - sizeof(uint32_t)]);

  uint8_t crcValue = buffer[expectedPacketLength - 1];
  CRC8 crc;
  crc.add(&buffer[6], dataLength);
  for (size_t i = 0; i < dataLength; i++) {
    Serial.print("Feeding ");
    Serial.print(buffer[6 + i]);
    Serial.println(" to CRC check");
  }

  uint8_t crcExpected = crc.calc();

  if (crcExpected != crcValue) {
    Serial.print("Expected CRC: ");
    Serial.print(crcExpected);
    Serial.print(" but got ");
    Serial.print(crcValue);
    Serial.print(", data length ");
    Serial.print(dataLength);
    Serial.print(", packet id was ");
    Serial.print(packetId);
    Serial.print(", packet index was ");
    Serial.println(packetIndex);
    result.status = PacketStatus::FailedCRC;
    return result;
  }

  result.status = PacketStatus::Ok;
  result.dataStart = &buffer[2];
  result.dataLength = dataLength;
  result.packetId = packetId;
  result.packetIndex = packetIndex;

  return result;
}
