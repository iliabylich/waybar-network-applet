#include "utils.h"
#include <stdlib.h>
#include <stddef.h>
#include <string.h>

char *copystr(const char *s)
{
    size_t len = strlen(s);
    char *out = malloc(len + 1);
    strncpy(out, s, len);
    out[len] = 0;
    return out;
}
