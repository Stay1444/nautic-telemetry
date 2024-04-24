#include "Logger.h"
#include <Arduino.h>

void logImplementation(const char *level, const char *name, const char *format,
                       ...) {
  char buffer[256];

  va_list args;
  va_start(args, format);

  vsnprintf(buffer, sizeof(buffer), format, args);

  va_end(args);

  Serial.print("[");
  Serial.print(level);
  Serial.print("] [");
  Serial.print(name);
  Serial.print("]: ");
  Serial.print(buffer);

  Serial.println();
}

void Logger::info(const char *format, ...) {
  logImplementation("INFO", this->loggerName, format);
}

void Logger::warn(const char *format, ...) {
  logImplementation("WARN", this->loggerName, format);
}

void Logger::error(const char *format, ...) {
  logImplementation("ERROR", this->loggerName, format);
}
