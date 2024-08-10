#include "args.h"
#include "string.h"
#include "stdio.h"
#include "stdlib.h"

int width = 500;
int offset_right = 0;

int get_width()
{
    return width;
}

int get_offset_right()
{
    return offset_right;
}

void print_help()
{
    printf("Usage: waybar-network-applet [--width N] [--offset-right N]\n");
}

int parse_int(const char *s)
{
    char *end;
    long out = strtol(s, &end, 10);
    if (end - s > 0 && (size_t)(end - s) == strlen(s))
    {
        return (int)out;
    }
    fprintf(stderr, "Not a number '%s'\n", s);
    print_help();
    exit(EXIT_FAILURE);
}

void parse_args(int argc, char **argv)
{
    for (int i = 1; i < argc; i++)
    {
        if (strcmp(argv[i], "--width") == 0 && i + 1 < argc)
        {
            width = parse_int(argv[i + 1]);
            i += 1;
        }
        else if (strcmp(argv[i], "--offset-right") == 0 && i + 1 < argc)
        {
            offset_right = parse_int(argv[i + 1]);
            i += 1;
        }
        else if (strcmp(argv[i], "--help") == 0)
        {
            print_help();
            exit(EXIT_SUCCESS);
        }
        else
        {
            fprintf(stderr, "Unsupported argument %s\n", argv[i]);
            print_help();
            exit(EXIT_FAILURE);
        }
    }
}
