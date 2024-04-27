#include "Writer.h"
#include "Utils.h"
#include <Arduino.h>

Writer::Writer() {
  this->m_Buffer = (uint8_t *)malloc(8);
  this->m_Capacity = 8;
}

Writer::~Writer() { free((void *)this->m_Buffer); }

size_t Writer::length() { return this->m_Position; }
size_t Writer::capacity() { return this->m_Capacity; }

void Writer::reserve(size_t size) {
  while (this->m_Position + size > this->m_Capacity) {
    uint8_t *buffer = this->m_Buffer;

    this->m_Capacity *= 2;
    this->m_Buffer = (uint8_t *)malloc(this->m_Capacity);

    utils::arrays::copy(buffer, this->m_Buffer, this->m_Position);
    free((void *)buffer);
  }
}

void Writer::write(uint8_t value) {
  this->reserve(1);
  this->m_Buffer[this->m_Position++] = value;
}

void Writer::write(uint32_t value) {
  this->reserve(4);
  this->m_Buffer[this->m_Position++] = (value >> 24) & 0xFF;
  this->m_Buffer[this->m_Position++] = (value >> 16) & 0xFF;
  this->m_Buffer[this->m_Position++] = (value >> 8) & 0xFF;
  this->m_Buffer[this->m_Position++] = value & 0xFF;
}

void Writer::write(float value) {
  uint32_t uintValue;
  memcpy(&uintValue, &value, sizeof(float));
  this->write(uintValue);
}

void Writer::write(int32_t value) {
  this->write(
      static_cast<uint32_t>(value)); // Just treat it as an unsigned integer
}

void Writer::write(bool value) {
  this->reserve(1);
  this->m_Buffer[this->m_Position++] = value ? 1 : 0;
}

void Writer::write(uint8_t *buffer, size_t length) {
  this->reserve(length);

  for (size_t i = 0; i < length; i++) {
    this->m_Buffer[m_Position++] = buffer[i];
  }
}

uint8_t *Writer::raw() { return this->m_Buffer; }
