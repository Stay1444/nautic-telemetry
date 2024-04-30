#pragma once

#include <Arduino.h>

class Allocator {
public:
  static void *Malloc(size_t size) { return malloc(size); }

  static void Free(void *buffer) { free(buffer); }
};
