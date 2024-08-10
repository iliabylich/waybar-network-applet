#ifndef WAYBAR_NETWORK_APPLET_ACTIONS_H
#define WAYBAR_NETWORK_APPLET_ACTIONS_H

#include <gtk/gtk.h>
#include "row.h"

typedef struct
{
    row_t *row;
    char *text;
} restore_point_t;

void copy_ip(
    GtkGesture *self,
    GdkEventSequence *sequence,
    gpointer data);

void open_settings(
    GtkGesture *self,
    GdkEventSequence *sequence,
    gpointer data);

void close_widget(
    GtkGesture *self,
    GdkEventSequence *sequence,
    gpointer data);

#endif // WAYBAR_NETWORK_APPLET_ACTIONS_H
