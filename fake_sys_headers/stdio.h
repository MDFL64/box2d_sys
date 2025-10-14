#pragma once

#include "_common.h"

typedef int FILE;

FILE *fopen(const char * path, const char * mode);
int fclose(FILE *stream);

int printf( const char * format, ... );
int snprintf( char * s, size_t n, const char * format, ... );
int fprintf( FILE * stream, const char * format, ... );
