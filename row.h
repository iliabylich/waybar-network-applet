#ifndef WAYBAR_NETWORK_APPLET_ROW_H
#define WAYBAR_NETWORK_APPLET_ROW_H

#include <gtk/gtk.h>
#include "network-list.h"

typedef struct
{
    GtkLabel *label;
    GtkImage *icon;
    void *data;
} row_t;

#endif // WAYBAR_NETWORK_APPLET_ROW_H
