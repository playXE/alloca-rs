#include <stddef.h>
#include <inttypes.h>

void* c_with_alloca(size_t size,void* (*callback) (uint8_t*,void*),void* data) {
    uint8_t buffer[size];
    return callback(&buffer[0],data);
}