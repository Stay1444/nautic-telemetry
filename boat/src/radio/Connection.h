#pragma once

#include "FireTimer.h"
#include "Packet.h"
#include "utils/Allocator.h"
#include "utils/Vector.h"
#include <Arduino.h>
#include <log/Logger.h>
#include <stdint.h>

namespace radio {

#define RADIO_PORT Serial1
#define RADIO_MODE_PORT 4

class Connection {
public:
  typedef void (*PacketCallbackFunction)(Packet *);

  Connection() {
    RADIO_PORT.begin(9600);
    pinMode(RADIO_MODE_PORT, OUTPUT);
    digitalWrite(RADIO_MODE_PORT, HIGH);
    m_Buffer = (uint8_t *)Allocator::Malloc(BUFFER_SIZE);

    m_StatisticsTimer.begin(1000);
  }
  ~Connection() { Allocator::Free(m_Buffer); }

  static const size_t BUFFER_SIZE = 512;
  void queue(Packet *packet, bool optional = true);
  void handler(PacketCallbackFunction handler);
  void tick();

private:
  Packet *recv();
  void handle(Packet *packet);
  void write(Packet *packet);
  void flush();

  uint8_t *m_Buffer;
  size_t m_BufferLength = 0;
  Logger m_Logger = Logger("Radio");

  uint32_t m_Tx = 0;
  uint32_t m_Rx = 0;
  uint8_t m_Channel = 0;

  FireTimer m_StatisticsTimer;

  Vector m_PendingPackets;

  PacketCallbackFunction m_Handler;
};

} // namespace radio
