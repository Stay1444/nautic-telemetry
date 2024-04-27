#include "radio/Packet.h"
#include <Arduino.h>
#include <stdint.h>

namespace radio::packets::Slave {

#define SLAVE_PONG_PACKET 0
#define SLAVE_GPS_PACKET 1

class Pong : public Packet {
public:
  uint8_t id() override { return SLAVE_PONG_PACKET; }

  PacketFrame serialize() override {
    PacketFrame frame;
    frame.id = this->id();
    frame.writer = Writer();
    return frame;
  }
};

class GPS : public Packet {
public:
  uint8_t id() override { return SLAVE_GPS_PACKET; }

  uint8_t satellites = 0;
  float mps = 0.0;
  float lat = 0.0;
  float lon = 0.0;

  PacketFrame serialize() override {
    PacketFrame frame;
    frame.id = this->id();
    Writer writer;

    writer.write(this->satellites);
    writer.write(this->mps);
    writer.write(this->lat);
    writer.write(this->lon);

    frame.writer = writer;
    return frame;
  }
};

} // namespace radio::packets::Slave
