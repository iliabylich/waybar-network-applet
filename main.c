#include <gtk/gtk.h>
#include "network-list.h"
#include "ui.h"
#include "css.h"
#include "args.h"

void on_activate(GtkApplication *app, gpointer data)
{
    GtkWindow *window = make_ui(app);
    gtk_window_present(window);

    GtkStyleContext *style_context = gtk_widget_get_style_context(GTK_WIDGET(window));
    printf(
        "[TREE] Widget tree:\n%s\n",
        gtk_style_context_to_string(
            style_context,
            GTK_STYLE_CONTEXT_PRINT_RECURSE | GTK_STYLE_CONTEXT_PRINT_SHOW_STYLE));
}

void on_startup()
{
    load_css();
}

int main(int argc, char **argv)
{
    parse_args(argc, argv);

    GtkApplication *app = gtk_application_new(
        "com.ilyabylich.waybar_network_applet",
        G_APPLICATION_HANDLES_OPEN);

    g_signal_connect(app, "activate", G_CALLBACK(on_activate), NULL);
    g_signal_connect(app, "startup", G_CALLBACK(on_startup), NULL);

    g_application_run(G_APPLICATION(app), 0, NULL);
    g_object_unref(app);

    return 0;
}
