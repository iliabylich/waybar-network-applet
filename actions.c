#include "actions.h"
#include "utils.h"

void after_copying_ip(gpointer data)
{
    restore_point_t *restore_point = (restore_point_t *)data;
    gtk_label_set_text(restore_point->row->label, restore_point->text);
}

restore_point_t *make_restore_point(row_t *row)
{
    restore_point_t *out = malloc(sizeof(restore_point_t));
    out->row = row;
    out->text = copystr(gtk_label_get_text(row->label));
    return out;
}

void copy_ip(
    GtkGesture *self,
    GdkEventSequence *sequence,
    gpointer data)
{
    gtk_gesture_set_state(self, GTK_EVENT_SEQUENCE_CLAIMED);

    row_t *row = (row_t *)data;
    network_t *network = (network_t *)row->data;

    GdkDisplay *display = gdk_display_get_default();
    GdkClipboard *clipboard = gdk_display_get_clipboard(display);
    gdk_clipboard_set_text(clipboard, network->ip);

    restore_point_t *restore_point = make_restore_point(row);
    gtk_label_set_text(row->label, "Copied!");

    g_timeout_add_once(1000, after_copying_ip, restore_point);
}

void open_settings(
    GtkGesture *self,
    GdkEventSequence *sequence,
    gpointer data)
{
    gtk_gesture_set_state(self, GTK_EVENT_SEQUENCE_CLAIMED);
    g_spawn_command_line_async("kitty --name nmtui nmtui", NULL);
    GtkWindow *window = (GtkWindow *)data;
    gtk_window_close(window);
}

void close_widget(
    GtkGesture *self,
    GdkEventSequence *sequence,
    gpointer data)
{
    gtk_gesture_set_state(self, GTK_EVENT_SEQUENCE_CLAIMED);
    GtkWindow *window = (GtkWindow *)data;
    gtk_window_close(window);
}
