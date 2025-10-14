#pragma once

#include "_common.h"

void free(void *);
void* aligned_alloc( size_t alignment, size_t size );

void qsort(
    void* ptr, size_t count, size_t size,
    int (*comp)(const void*, const void*) );
