#include "Vector.h"
#include "Utils.h"
#include "utils/Allocator.h"

Vector::Vector() {
  this->m_Capacity = 4;
  this->m_Slice = (void **)Allocator::Malloc(this->m_Capacity);
  this->m_Length = 0;
}

Vector::~Vector() {
  if (this->m_Slice == NULL)
    return;
  this->m_Capacity = 0;
  Allocator::Free(this->m_Slice);
  this->m_Slice = NULL;
}

size_t Vector::length() { return this->m_Length; }
size_t Vector::capacity() { return this->m_Capacity; }

void Vector::clear() {
  if (this->m_Slice == NULL)
    return;
  Allocator::Free(this->m_Slice);
  this->m_Length = 0;
  this->m_Capacity = 4;
  this->m_Slice = static_cast<void **>(
      Allocator::Malloc(this->m_Capacity * sizeof(void *)));
}

void Vector::extend() {
  void **old = this->m_Slice;
  this->m_Capacity *= 2;
  this->m_Slice = static_cast<void **>(
      Allocator::Malloc(this->m_Capacity * sizeof(void *)));
  utils::arrays::copy(old, this->m_Slice, this->m_Length);
  Allocator::Free(old);
}

void Vector::push(void *entry) {
  if (this->m_Length >= this->m_Capacity) {
    extend();
  }

  this->m_Slice[this->m_Length++] = entry;
}

void *Vector::pop() {
  if (this->m_Length == 0) {
    return nullptr;
  }
  void *lastEntry = this->m_Slice[--this->m_Length];
  return lastEntry;
}
