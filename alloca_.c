#include <stddef.h>
#include <inttypes.h>

void* c_with_alloca(size_t size,void* (*callback) (size_t,uint8_t*,void*),void* data) {
    uint8_t buffer[size];
    return callback(size,&buffer[0],data);
}