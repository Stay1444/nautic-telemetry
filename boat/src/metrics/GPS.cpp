#include "GPS.h"
#include "radio/Connection.h"
#include "radio/packets/Slave.h"

void GPS::tick(radio::Connection &radio) {
  while (this->m_Serial.available()) {
    this->m_Gps.encode(this->m_Serial.read());
  }

  bool locationUpdated =
      this->m_Gps.location.isUpdated() && this->m_Gps.location.isValid();
  bool speedUpdated =
      this->m_Gps.speed.isUpdated() && this->m_Gps.speed.isValid();

  if (locationUpdated || speedUpdated) {
    auto packet = new radio::packets::Slave::GPS();

    packet->satellites = (uint8_t)this->m_Gps.satellites.value();
    packet->lat = this->m_Gps.location.lat();
    packet->lon = this->m_Gps.location.lng();
    packet->mps = (float)this->m_Gps.speed.mps();

    radio.queue(packet);

    this->m_Logger.info("GPS sent information");
  }
}
