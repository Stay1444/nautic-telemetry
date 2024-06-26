#include "Utils.h"
#include <Arduino.h>
#include <HardwareSerial.h>

#include "Connection.h"
#include "PacketDeserializer.h"
#include "Protocol.h"
#include "packets/Master.h"
#include "packets/Slave.h"
#include "radio/Packet.h"
#include "utils/Allocator.h"

using namespace radio;

#define READ_CHUNK_SIZE 16

Packet *Connection::recv() {
  while (this->m_BufferLength + READ_CHUNK_SIZE <= this->BUFFER_SIZE &&
         RADIO_PORT.available()) {

    size_t read =
        RADIO_PORT.readBytes(&this->m_Buffer[this->m_BufferLength],
                             min(READ_CHUNK_SIZE, RADIO_PORT.available()));
    this->m_BufferLength += read;
    this->m_Rx += read;
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

      if (this->m_BufferLength <= i + result.packetLength) {
        this->m_BufferLength = 0;
      } else {
        utils::arrays::trimStart(this->m_Buffer, i + result.packetLength,
                                 this->m_BufferLength);
        this->m_BufferLength -= i + result.packetLength;
      }

      Packet *packet =
          Packet::deserialize(packetBuffer, result.dataLength, result.packetId);
      return packet;
    } else if (result.status == PacketDeserializer::PacketStatus::Invalid) {
      this->m_Logger.info("received invalid packet.");
      this->m_BufferLength = 0;
    } else if (result.status == PacketDeserializer::PacketStatus::Incomplete) {
      break;
    }
  }

  return NULL;
}

void Connection::write(Packet *packet) {
  PacketFrame frame = packet->serialize();
  free(packet);

  Writer writer = Writer::create();

  writer.write((uint8_t)PACKET_HEAD_BYTE);
  writer.write(frame.id);
  if (frame.writer.initialized()) {
    writer.write((uint32_t)frame.writer.length());
    writer.write(frame.writer.raw(), frame.writer.length());

    frame.writer.free();
  } else {
    writer.write((uint32_t)0);
  }

  this->m_Tx += writer.length();

  RADIO_PORT.write((char *)writer.raw(), writer.length());

  writer.free();
}

void Connection::tick() {
  if (this->m_InSendWindow) {
    this->write(new packets::Slave::EndSendWindow());
    this->m_Logger.info("Send window ended");
    RADIO_PORT.flush();
    this->m_InSendWindow = false;
    return;
  }

  Packet *incoming = NULL;
  while ((incoming = this->recv()) != NULL) {
    handle(incoming);
  }
}

void Connection::handle(Packet *packet) {
  if (packet == NULL)
    return;

  if (packet->id() == MASTER_START_SEND_WINDOW_PACKET) {
    this->m_Logger.info("Send window started");
    Allocator::Free(packet);
    this->m_InSendWindow = true;
    if (this->m_StatisticsTimer.fire()) {
      auto packet = new packets::Slave::RadioReport();

      packet->channel = this->m_Channel;
      packet->rx = this->m_Rx;
      packet->tx = this->m_Tx;

      this->m_Rx = 0;
      this->m_Tx = 0;

      this->write(packet);
    }
    return;
  }

  if (this->m_Handler != NULL) {
    this->m_Handler(packet);
  } else {
    Allocator::Free(packet);
  }
}

void Connection::handler(PacketCallbackFunction handler) {
  this->m_Handler = handler;
}

String *Connection::at(String &command) { return this->at(command.c_str()); }

String *Connection::at(const char *command) {
  digitalWrite(RADIO_MODE_PORT, LOW);

  delay(120);

  RADIO_PORT.print(command);
  RADIO_PORT.println();
  RADIO_PORT.flush();

  String *response = new String();

  uint32_t start = millis();

  while (!response->endsWith("\r\n")) {
    if (!RADIO_PORT.available())
      continue;

    if (millis() - start > 500) {
      free(response);
      digitalWrite(RADIO_MODE_PORT, HIGH);
      this->m_Logger.error("Timed out while waiting for AT command response.");
      return NULL;
    }

    char read = RADIO_PORT.read();
    response->concat(read);
  }

  digitalWrite(RADIO_MODE_PORT, HIGH);

  delay(120);

  return response;
}

bool Connection::isSending() { return this->m_InSendWindow; }
