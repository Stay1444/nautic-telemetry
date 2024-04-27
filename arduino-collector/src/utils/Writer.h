#include <Arduino.h>

class Writer {
public:
  Writer();
  ~Writer();

  size_t length();
  size_t capacity();

  void write(uint8_t value);
  void write(uint32_t value);
  void write(float value);
  void write(int32_t value);
  void write(bool value);
  void write(uint8_t *buffer, size_t length);
  uint8_t *raw();

private:
  uint8_t *m_Buffer;
  size_t m_Position = 0;
  size_t m_Capacity = 0;
  void reserve(size_t size);
};
