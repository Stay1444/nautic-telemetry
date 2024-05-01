#include "Writer.h"
#include "Utils.h"
#include "utils/Allocator.h"
#include <Arduino.h>

Writer Writer::create() {
  Writer writer;
  writer.m_Buffer = (uint8_t *)Allocator::Malloc(8);
  writer.m_Capacity = 8;
  writer.m_Initialized = true;

  return writer;
}
void Writer::free() {
  if (this->m_Initialized)
    Allocator::Free(this->m_Buffer);
}
size_t Writer::length() { return this->m_Position; }
size_t Writer::capacity() { return this->m_Capacity; }
bool Writer::initialized() { return this->m_Initialized; }

void Writer::reserve(size_t size) {
  if (!this->m_Initialized) {
    Serial.println(
        "ERROR: TRIED TO USE A WRITE BUFFER WITHOUT CALING .create()");
    return;
  }
  while (this->m_Position + size > this->m_Capacity) {
    uint8_t *buffer = this->m_Buffer;

    this->m_Capacity *= 2;
    this->m_Buffer = (uint8_t *)Allocator::Malloc(this->m_Capacity);

    utils::arrays::copy(buffer, this->m_Buffer, this->m_Position);
    Allocator::Free((void *)buffer);
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

void Writer::write(uint64_t value) {
  write((uint8_t)(value >> 56));
  write((uint8_t)(value >> 48));
  write((uint8_t)(value >> 40));
  write((uint8_t)(value >> 32));
  write((uint8_t)(value >> 24));
  write((uint8_t)(value >> 16));
  write((uint8_t)(value >> 8));
  write((uint8_t)value);
}

void Writer::write(float value) {
  uint32_t uintValue;
  memcpy(&uintValue, &value, sizeof(float));
  this->write(uintValue);
}

void Writer::write(double value) {
  uint64_t temp = *reinterpret_cast<uint64_t *>(&value);
  write(temp);
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

uint8_t *Writer::raw() {
  if (!this->m_Initialized)
    return NULL;
  return this->m_Buffer;
}
