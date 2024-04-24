#include "PacketDeserializer.h"
#include "Utils.h"
#include "radio/Protocol.h"

#include <Arduino.h>
#include <stdint.h>

#include "CRC8.h"

using namespace radio::PacketDeserializer;

DeserializeResult deserialize(const uint8_t *buffer, size_t bufferLength) {
  DeserializeResult result;

  if (bufferLength < 3) // packet head, id, data length
  {
    result.status = PacketStatus::Incomplete;
    return result;
  }

  if (buffer[0] != PACKET_HEAD_BYTE) {
    result.status = PacketStatus::Invalid;
    return result;
  }

  uint8_t packetId = buffer[1];
  uint8_t dataLength = buffer[2];
  uint8_t expectedPacketLength =
      dataLength + sizeof(uint8_t) * 3 + sizeof(size_t) +
      sizeof(uint8_t); // head | id | data length | ... | index | crc

  if (bufferLength < expectedPacketLength) {

    result.status = PacketStatus::Incomplete;
    return result;
  }

  size_t packetIndex = utils::conversions::convertToSizeT(
      &buffer[expectedPacketLength - 1 - sizeof(size_t)]);

  uint8_t crcValue = buffer[expectedPacketLength];
  CRC8 crc;
  crc.add(&buffer[3], dataLength);

  if (crc.calc() != crcValue) {
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
