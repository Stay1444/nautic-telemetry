#include "RelayDriver.h"
#include "Arduino.h"
#include "metrics/Thermistor.h"
#include "radio/Connection.h"

RelayDriver::RelayDriver(uint8_t relayPin, uint8_t thermistorPin,
                         float threshold, bool invert) {
  this->m_Timer.begin(100);
  this->m_RelayPin = relayPin;
  this->m_ThermistorPin = thermistorPin;
  this->m_Threshold = threshold;
  this->m_Invert = invert;
  pinMode(relayPin, OUTPUT);
}

void RelayDriver::tick(radio::Connection &radio) {
  if (!this->m_Timer.fire())
    return;

  float temperature = Thermistor::celsius(this->m_ThermistorPin);

  bool wantedState = temperature >= this->m_Threshold;

  if (this->m_Invert)
    wantedState = !wantedState;

  if (wantedState)
    digitalWrite(this->m_RelayPin, HIGH);
  else
    digitalWrite(this->m_RelayPin, LOW);
}
