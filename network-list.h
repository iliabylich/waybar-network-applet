#ifndef WAYBAR_NETWORK_APPLET_NETWORK_LIST_H
#define WAYBAR_NETWORK_APPLET_NETWORK_LIST_H

typedef struct
{
    char *name;
    char *ip;
} network_t;

typedef struct
{
    size_t size;
    network_t *list[10];
} network_list_t;

network_list_t new_network_list();

#endif // WAYBAR_NETWORK_APPLET_NETWORK_LIST_H
