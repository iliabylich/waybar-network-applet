#include <gtk/gtk.h>
#include "css.h"

void on_parsing_error(GtkCssProvider *self,
                      GtkCssSection *section,
                      GError *error,
                      gpointer data)
{
    fprintf(stderr, "Failed to parse CSS:\n%s\n%s\n", gtk_css_section_to_string(section), error->message);
}

void load_css()
{
    GtkCssProvider *provider = gtk_css_provider_new();

    g_signal_connect(
        provider,
        "parsing-error",
        G_CALLBACK(on_parsing_error),
        NULL);

    char path[100];
    sprintf(path, "%s/.config/waybar/waybar-network-applet.css", getenv("HOME"));

    gtk_css_provider_load_from_file(
        provider,
        g_file_new_for_path(path));

    gtk_style_context_add_provider_for_display(
        gdk_display_get_default(),
        GTK_STYLE_PROVIDER(provider),
        GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);

    printf("[CSS] Using styles:\n%s\n", gtk_css_provider_to_string(provider));
}
