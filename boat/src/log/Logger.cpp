#include "Logger.h"
#include <Arduino.h>

void logImplementation(const char *level, const char *name,
                       const char *message) {

  Serial.print(millis());
  Serial.print(" - ");
  Serial.print("[");
  Serial.print(level);
  Serial.print("] [");
  Serial.print(name);
  Serial.print("]: ");
  Serial.print(message);

  Serial.println();
}

void Logger::info(const char *message) {
  logImplementation("INFO", this->loggerName, message);
}

void Logger::warn(const char *message) {
  logImplementation("WARN", this->loggerName, message);
}

void Logger::error(const char *message) {
  logImplementation("ERROR", this->loggerName, message);
}
