#include "Utils.h"
#include <Arduino.h>
#include <CRC8.h>
#include <HardwareSerial.h>

#include "Connection.h"
#include "PacketDeserializer.h"
#include "Protocol.h"
#include "radio/Packet.h"

using namespace radio;

Packet *Connection::recv() {
  uint8_t buffer[16];
  size_t available = 0;

  if (Serial1.available()) {
    available = min((size_t)Serial1.available(), sizeof(buffer));
    Serial1.readBytes(buffer, available);
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
      uint8_t *packet_buffer = (uint8_t *)malloc(result.dataLength);
      utils::arrays::copy(result.dataStart, packet_buffer, result.dataLength);
      this->m_BufferLength = 0;
      Cursor cursor(packet_buffer, result.dataLength);
      Packet *packet = Packet::deserialize(cursor, result.packetId);
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

  Writer writer;

  writer.write((uint8_t)PACKET_HEAD_BYTE);
  writer.write(frame.id);
  writer.write((uint32_t)frame.writer.length());
  writer.write(frame.writer.raw(), frame.writer.length());

  CRC8 crc;
  crc.add(frame.writer.raw(), frame.writer.length());

  writer.write(crc.calc());

  Serial1.write((char *)writer.raw(), writer.length());
}
