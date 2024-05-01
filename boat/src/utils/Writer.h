#pragma once
#include <Arduino.h>

class Writer {
public:
  Writer() = default;
  size_t length();
  size_t capacity();

  static Writer create();
  void free();
  bool initialized();

  void write(uint8_t value);
  void write(uint32_t value);
  void write(uint64_t value);
  void write(float value);
  void write(double value);
  void write(int32_t value);
  void write(bool value);
  void write(uint8_t *buffer, size_t length);
  uint8_t *raw();

private:
  bool m_Initialized = false;
  uint8_t *m_Buffer;
  size_t m_Position = 0;
  size_t m_Capacity = 0;
  void reserve(size_t size);
};
