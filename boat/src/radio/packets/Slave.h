#include "radio/Packet.h"
#include <Arduino.h>
#include <stdint.h>

namespace radio::packets::Slave {

#define SLAVE_END_SEND_WINDOW_PACKET 0
#define SLAVE_GPS_PACKET 1
#define SLAVE_TEMPERATURE_PACKET 2
#define SLAVE_VOLTAGE_PACKET 3
#define SLAVE_RADIO_REPORT_PACKET 4
#define SLAVE_AMPS_PACKET 5

class EndSendWindow : public Packet {
public:
  uint8_t id() override { return SLAVE_END_SEND_WINDOW_PACKET; }

  PacketFrame serialize() override {
    PacketFrame frame = {0};
    frame.id = this->id();
    Writer writer = Writer::create();
    writer.write((uint32_t)millis());
    frame.writer = writer;
    return frame;
  }
};

class GPS : public Packet {
public:
  uint8_t id() override { return SLAVE_GPS_PACKET; }

  uint32_t timestamp = millis();
  uint8_t satellites = 0;
  float mps = 0.0;
  double lat = 0.0;
  double lon = 0.0;
  double altitude = 0.0;

  PacketFrame serialize() override {
    PacketFrame frame = {0};
    frame.id = this->id();

    Writer writer = Writer::create();

    writer.write(this->timestamp);
    writer.write(this->satellites);
    writer.write(this->mps);
    writer.write(this->lat);
    writer.write(this->lon);
    writer.write(this->altitude);

    frame.writer = writer;
    return frame;
  }
};

class Temperature : public Packet {
public:
  uint8_t id() override { return SLAVE_TEMPERATURE_PACKET; }

  uint32_t timestamp = millis();
  uint8_t tag;
  float temperature = 0.0;

  PacketFrame serialize() override {
    PacketFrame frame = {0};
    frame.id = this->id();

    Writer writer = Writer::create();

    writer.write(this->timestamp);
    writer.write(this->tag);
    writer.write(this->temperature);

    frame.writer = writer;
    return frame;
  }
};

class Voltage : public Packet {
public:
  uint8_t id() override { return SLAVE_VOLTAGE_PACKET; }

  uint32_t timestamp = millis();
  uint8_t tag;
  float voltage = 0.0;

  PacketFrame serialize() override {
    PacketFrame frame = {0};
    frame.id = this->id();

    Writer writer = Writer::create();

    writer.write(this->timestamp);
    writer.write(this->tag);
    writer.write(this->voltage);

    frame.writer = writer;
    return frame;
  }
};

class Amperimeter : public Packet {
public:
  uint8_t id() override { return SLAVE_AMPS_PACKET; }
  uint32_t timestamp = millis();
  uint8_t tag;
  float value = 0.0;

  PacketFrame serialize() override {
    PacketFrame frame = {0};
    frame.id = this->id();

    Writer writer = Writer::create();
    writer.write(this->timestamp);
    writer.write(this->tag);
    writer.write(this->value);

    frame.writer = writer;
    return frame;
  }
};

class RadioReport : public Packet {
public:
  uint8_t id() override { return SLAVE_RADIO_REPORT_PACKET; }

  uint32_t timestamp = millis();
  uint8_t channel = 0;
  uint32_t rx = 0;
  uint32_t tx = 0;

  PacketFrame serialize() override {
    PacketFrame frame = {0};
    frame.id = this->id();

    Writer writer = Writer::create();

    writer.write(this->timestamp);
    writer.write(this->channel);
    writer.write(this->rx);
    writer.write(this->tx);

    frame.writer = writer;

    return frame;
  }
};

} // namespace radio::packets::Slave
