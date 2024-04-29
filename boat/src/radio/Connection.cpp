#include "Utils.h"
#include <Arduino.h>
#include <HardwareSerial.h>

#include "Connection.h"
#include "PacketDeserializer.h"
#include "Protocol.h"
#include "radio/Packet.h"
#include "utils/Allocator.h"

using namespace radio;

Packet *Connection::recv() {
  uint8_t buffer[16];
  size_t available = 0;

  if (RADIO_PORT.available()) {
    available = min((size_t)RADIO_PORT.available(), sizeof(buffer));
    RADIO_PORT.readBytes(buffer, available);
  }

  if (this->m_BufferLength + available > this->BUFFER_SIZE) {
    this->m_Logger.warn(
        "Incoming packet won't fit in the available buffer size. Discarding "
        "entire packet buffer and hoping for the best.");
    this->m_BufferLength = 0;
    return NULL;
  }

  for (size_t i = 0; i < available; i++) {
    this->m_Buffer[this->m_BufferLength++] = buffer[i];
  }

  for (size_t i = 0; i < this->m_BufferLength; i++) {
    if (this->m_Buffer[i] != PACKET_HEAD_BYTE) {
      continue;
    }

    auto result = PacketDeserializer::deserialize(&this->m_Buffer[i],
                                                  this->m_BufferLength - i);

    if (result.status == PacketDeserializer::PacketStatus::Ok) {
      uint8_t *packetBuffer = (uint8_t *)Allocator::Malloc(result.dataLength);
      if (packetBuffer == NULL) {
        this->m_Logger.error("Could not allocate memory for packet buffer");
        return NULL;
      }

      utils::arrays::copy(result.dataStart, packetBuffer, result.dataLength);
      this->m_BufferLength = 0;
      Packet *packet =
          Packet::deserialize(packetBuffer, result.dataLength, result.packetId);
      return packet;
    } else if (result.status == PacketDeserializer::PacketStatus::FailedCRC) {
      this->m_Logger.info("received packet with failed crc check.");
      this->m_BufferLength = 0;
    } else if (result.status == PacketDeserializer::PacketStatus::Invalid) {
      this->m_Logger.info("received invalid packet.");
      this->m_BufferLength = 0;
    } else if (result.status == PacketDeserializer::PacketStatus::Incomplete) {
      break;
    }
  }

  return NULL;
}

void Connection::send(Packet *packet) {
  PacketFrame frame = packet->serialize();

  free(packet);

  Writer writer = Writer::create();

  writer.write((uint8_t)PACKET_HEAD_BYTE);
  writer.write(frame.id);
  writer.write((uint32_t)frame.writer.length());
  writer.write(frame.writer.raw(), frame.writer.length());

  frame.writer.free();

  RADIO_PORT.write((char *)writer.raw(), writer.length());

  writer.free();
}
