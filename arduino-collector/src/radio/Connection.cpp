#include "Utils.h"
#include <Arduino.h>
#include <HardwareSerial.h>

#include "Connection.h"
#include "PacketDeserializer.h"
#include "Protocol.h"

using namespace radio;

void Connection::tick() {
  uint8_t buffer[16];
  size_t available = 0;

  if (Serial1.available()) {
    available = min(Serial1.available(), sizeof(buffer));
    Serial1.readBytes(buffer, available);
  }

  if (this->bufferLength + available > this->BUFFER_SIZE) {
    this->logger.warn(
        "Incoming packet won't fit in the available buffer size. Discarding "
        "entire packet buffer and hoping for the best.");
    this->bufferLength = 0;
    return;
  }

  for (size_t i = 0; i < available; i++) {
    this->buffer[this->bufferLength++] = buffer[i];
  }

  for (size_t i = 0; i < this->bufferLength; i++) {
    if (this->buffer[i] != PACKET_HEAD_BYTE) {
      continue;
    }

    auto result = PacketDeserializer::deserialize(&this->buffer[i],
                                                  this->bufferLength - i);

    if (result.status == PacketDeserializer::PacketStatus::Ok) {
      uint8_t *packet = (uint8_t *)malloc(result.dataLength);
      utils::arrays::copy(result.dataStart, packet, result.dataLength);

      this->logger.info("received packet");
      Serial.print(result.dataLength);
      Serial.print(" ");
      Serial.print(result.packetId);
      Serial.print(" ");
      Serial.println(result.packetIndex);

      this->bufferLength = 0;
      free(packet); // TODO: Handle packet
    } else if (result.status == PacketDeserializer::PacketStatus::FailedCRC) {
      this->logger.info("received packet with failed crc check.");
      this->bufferLength = 0;
    } else if (result.status == PacketDeserializer::PacketStatus::Invalid) {
      this->logger.info("received invalid packet.");
      this->bufferLength = 0;
    } else if (result.status == PacketDeserializer::PacketStatus::Incomplete) {
      break;
    }
  }
}
