#include "Thermistor.h"
#include "radio/Connection.h"
#include "radio/packets/Slave.h"

float Thermistor::celsius(uint8_t pin) {
  int Vo;
  float R1 = 10000;
  float logR2, R2, T, Tc;
  float c1 = 1.009249522e-03, c2 = 2.378405444e-04, c3 = 2.019202697e-07;

  Vo = analogRead(pin);
  R2 = R1 * (1023.0 / (float)Vo - 1.0);
  logR2 = log(R2);
  T = (1.0 / (c1 + c2 * logR2 + c3 * logR2 * logR2 * logR2));
  Tc = T - 273.15;

  return Tc;
}

void Thermistor::flush(radio::Connection &radio) {
  auto packet = new radio::packets::Slave::Temperature();

  packet->tag = this->m_Tag;
  packet->temperature = Thermistor::celsius(this->m_Pin);

  radio.write(packet);
}
