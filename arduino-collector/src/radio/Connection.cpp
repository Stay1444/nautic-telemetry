#include "Utils.h"
#include <Arduino.h>
#include <HardwareSerial.h>

#include "Connection.h"
#include "PacketVerifier.h"
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
    if (this->buffer[i] != PACKET_START_BYTE) {
      continue;
    }

    size_t end = 0;
    auto result = radio::PacketVerifier::verify(&this->buffer[i],
                                                this->bufferLength - i, &end);

    if (result == radio::PacketVerifier::Result::Ok) {
      size_t packetSize = end - i;
      uint8_t *packet = (uint8_t *)malloc(packetSize);
      utils::arrays::copy(&this->buffer[i], packet, packetSize);

      this->logger.info("received packet with length %d", packetSize);
    } else if (result == radio::PacketVerifier::Result::Corrupted) {
      this->bufferLength = 0;
      this->logger.warn("Packet buffer contains corrupted data. Resetting");
      break;
    }
  }
}
