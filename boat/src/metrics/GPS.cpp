#include "GPS.h"
#include "radio/Connection.h"
#include "radio/packets/Slave.h"

void GPS::tick() {
  while (this->m_Serial.available()) {
    this->m_Gps.encode(this->m_Serial.read());
  }
}
void GPS::flush(radio::Connection &radio) {

  bool locationUpdated =
      this->m_Gps.location.isUpdated() && this->m_Gps.location.isValid();
  bool speedUpdated =
      this->m_Gps.speed.isUpdated() && this->m_Gps.speed.isValid();

  bool satellitesUpdated =
      this->m_Gps.satellites.isUpdated() && this->m_Gps.satellites.isValid();

  if (locationUpdated || speedUpdated || satellitesUpdated) {
    if (this->m_Gps.location.lat() <= 0.01 &&
        this->m_Gps.location.lng() <= 0.01)
      return;

    if (this->m_Gps.satellites.value() == 0)
      return;

    auto packet = new radio::packets::Slave::GPS();

    packet->satellites = (uint8_t)this->m_Gps.satellites.value();
    packet->lat = (float)this->m_Gps.location.lat();
    packet->lon = (float)this->m_Gps.location.lng();
    packet->mps = (float)this->m_Gps.speed.mps();
    packet->altitude = this->m_Gps.altitude.meters();

    radio.write(packet);

    this->m_Logger.info("GPS sent information");
  }
}
