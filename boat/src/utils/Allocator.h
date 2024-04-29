#pragma once

#include <Arduino.h>

class Allocator {
public:
  static void *Malloc(size_t size) {
    Serial.print(size);
    return malloc(size);
  }

  static void Free(void *buffer) { free(buffer); }
};
