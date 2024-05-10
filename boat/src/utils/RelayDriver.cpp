#include "RelayDriver.h"
#include "Arduino.h"
#include "metrics/Voltimeter.h"
#include "radio/Connection.h"

RelayDriver::RelayDriver(uint8_t relayPin, uint8_t thermistorPin,
                         float openThreshold, float closeThreshold,
                         bool invert) {
  this->m_Timer.begin(100);
  this->m_RelayPin = relayPin;
  this->m_VoltimeterPin = thermistorPin;
  this->m_OpenThreshold = openThreshold;
  this->m_CloseThreshold = closeThreshold;
  this->m_Invert = invert;
  this->m_Open = true;
  pinMode(relayPin, OUTPUT);
  digitalWrite(relayPin, LOW);
}

void RelayDriver::tick(radio::Connection &radio) {
  if (!this->m_Timer.fire())
    return;

  float voltage = Voltimeter::read(this->m_VoltimeterPin);

  if (this->m_Open && voltage < this->m_CloseThreshold) {
    this->m_Open = false;
    if (!this->m_Invert)
      digitalWrite(this->m_RelayPin, HIGH);
    else
      digitalWrite(this->m_RelayPin, LOW);
  }

  if (!this->m_Open && voltage > this->m_OpenThreshold) {
    this->m_Open = true;
    if (!this->m_Invert)
      digitalWrite(this->m_RelayPin, LOW);
    else
      digitalWrite(this->m_RelayPin, HIGH);
  }
}
