#include <gtk/gtk.h>
#include <gtk4-layer-shell.h>
#include "ui.h"
#include "network-list.h"
#include "actions.h"
#include "row.h"
#include "args.h"

GtkWindow *make_window(GtkApplication *app);
GtkBox *make_wrapper(GtkWindow *window);

row_t *make_row(const char *text, const char *icon_name, const char *css_class, GtkBox *parent);

row_t *make_network_row(network_t *network, GtkBox *parent);
row_t *make_settings_row(GtkBox *parent);
row_t *make_exit_row(GtkBox *parent);

void set_on_click_callback(row_t *row, GCallback callback, gpointer data);

GtkWindow *make_ui(GtkApplication *app)
{
    GtkWindow *window = make_window(app);
    GtkBox *wrapper = make_wrapper(window);
    network_list_t networks = new_network_list();
    for (size_t i = 0; i < networks.size; i++)
    {
        row_t *row = make_network_row(networks.list[i], wrapper);

        set_on_click_callback(row, G_CALLBACK(copy_ip), row);
    }
    row_t *settings_row = make_settings_row(wrapper);
    set_on_click_callback(settings_row, G_CALLBACK(open_settings), window);

    row_t *exit_row = make_exit_row(wrapper);
    set_on_click_callback(exit_row, G_CALLBACK(close_widget), window);

    return window;
}

GtkWindow *make_window(GtkApplication *app)
{
    GtkWindow *window = GTK_WINDOW(gtk_application_window_new(app));
    gtk_window_set_default_size(window, get_width(), 0);

    gtk_layer_init_for_window(window);
    gtk_layer_set_layer(window, GTK_LAYER_SHELL_LAYER_OVERLAY);

    gtk_layer_auto_exclusive_zone_enable(window);
    gtk_layer_set_anchor(window, GTK_LAYER_SHELL_EDGE_TOP, true);
    gtk_layer_set_anchor(window, GTK_LAYER_SHELL_EDGE_RIGHT, true);
    gtk_layer_set_margin(window, GTK_LAYER_SHELL_EDGE_RIGHT, get_offset_right());

    return window;
}

GtkBox *make_wrapper(GtkWindow *window)
{
    GtkBox *wrapper = GTK_BOX(gtk_box_new(GTK_ORIENTATION_VERTICAL, 0));
    gtk_widget_add_css_class(GTK_WIDGET(wrapper), "row-list");
    gtk_window_set_child(window, GTK_WIDGET(wrapper));
    return wrapper;
}

row_t *make_row(const char *text, const char *icon_name, const char *css_class, GtkBox *parent)
{
    GtkLabel *label = GTK_LABEL(gtk_label_new(text));

    GtkImage *icon = GTK_IMAGE(gtk_image_new_from_icon_name(icon_name));
    gtk_image_set_icon_size(icon, GTK_ICON_SIZE_LARGE);

    GtkBox *icon_box = GTK_BOX(gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 0));
    gtk_box_set_baseline_position(icon_box, GTK_BASELINE_POSITION_CENTER);
    gtk_widget_set_halign(GTK_WIDGET(icon_box), GTK_ALIGN_START);
    gtk_widget_add_css_class(GTK_WIDGET(icon_box), "icons");
    gtk_box_append(icon_box, GTK_WIDGET(icon));

    GtkCenterBox *container = GTK_CENTER_BOX(gtk_center_box_new());
    gtk_orientable_set_orientation(GTK_ORIENTABLE(container), GTK_ORIENTATION_HORIZONTAL);
    // gtk_box_set_baseline_position(GTK_BOX(container), GTK_BASELINE_POSITION_CENTER);
    gtk_widget_set_halign(GTK_WIDGET(container), GTK_ALIGN_FILL);
    gtk_center_box_set_start_widget(GTK_CENTER_BOX(container), GTK_WIDGET(label));
    gtk_center_box_set_end_widget(GTK_CENTER_BOX(container), GTK_WIDGET(icon_box));
    gtk_widget_add_css_class(GTK_WIDGET(container), "row");
    gtk_widget_add_css_class(GTK_WIDGET(container), css_class);

    gtk_box_append(parent, GTK_WIDGET(container));

    row_t *row = malloc(sizeof(row_t));
    row->label = label;
    row->icon = icon;
    return row;
}

row_t *make_network_row(network_t *network, GtkBox *parent)
{
    char text[100];
    sprintf(text, "%s: %s", network->name, network->ip);
    row_t *row = make_row(text, "edit-copy", "ip-row", parent);

    row->data = network;
    return row;
}
row_t *make_settings_row(GtkBox *parent)
{
    return make_row("Settings (nmtui)", "preferences-system-network", "settings-row", parent);
}
row_t *make_exit_row(GtkBox *parent)
{
    return make_row("Close", "window-close", "exit-row", parent);
}

void set_on_click_callback(row_t *row, GCallback callback, gpointer data)
{
    GtkGesture *gesture = gtk_gesture_click_new();
    g_signal_connect(gesture, "end", callback, data);
    gtk_widget_add_controller(GTK_WIDGET(row->icon), GTK_EVENT_CONTROLLER(gesture));
}
