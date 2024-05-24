#include <gtk/gtk.h>
#include <webkit2/webkit2.h>
#include <json/json.h>

struct AppData {
    const char* url;
    GtkWidget* window;
};

static void handle_json_message(const Json::Value& json, AppData* user_data) {
    // Check if the JSON object has the "action" and "data" fields
    if (json.isMember("action") && json.isMember("data")) {
        std::string action = json["action"].asString();
        std::string data = json["data"].asString();

        // Perform actions based on the received data
        if (action == "setTitle") {
            // Set the GTK window's title
            gtk_window_set_title(GTK_WINDOW(user_data->window), data.c_str());
        } else if (action == "log") {
            // Set the GTK window's title
            g_print("%s\n", data.c_str());
        } else {
            // Handle other actions as needed
        }
    } else {
        // Invalid JSON format
        g_print("Invalid JSON format: 'action' and 'data' fields are required\n");
    }
}

static void js_callback(WebKitUserContentManager* manager,
                        WebKitJavascriptResult* js_result,
                        gpointer user_data) {
    JSCValue* value = webkit_javascript_result_get_js_value(js_result);
    if (jsc_value_is_string(value)) {
        char* str_value = jsc_value_to_string(value);
        
        Json::Value root;
        Json::CharReaderBuilder builder;
        std::string errors;
        std::istringstream json_stream(str_value);
        bool success = Json::parseFromStream(builder, json_stream, &root, &errors);

        if (success) {
            // Handle the incoming JSON message
            handle_json_message(root, static_cast<AppData*>(user_data));
        } else {
            g_print("Failed to parse JSON string: %s\n", errors.c_str());
        }

        g_free(str_value);
    }
}


int main(int argc, char** argv) {
    if (argc != 2) {
        g_print("Usage: %s <URL>\n", argv[0]);
        return 1;
    }

    const char* url = argv[1];

    AppData app_data;
    app_data.url = url;

    gtk_init(&argc, &argv);

    GtkWidget* window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(window), "WebKit Example");
    gtk_window_set_default_size(GTK_WINDOW(window), 800, 600);

    app_data.window = window;

    WebKitUserContentManager* content_manager = webkit_user_content_manager_new();
    webkit_user_content_manager_register_script_message_handler(content_manager, "external");
    g_signal_connect(content_manager, "script-message-received::external", G_CALLBACK(js_callback), &app_data);

    WebKitWebView* web_view = WEBKIT_WEB_VIEW(webkit_web_view_new_with_user_content_manager(content_manager));
    gtk_container_add(GTK_CONTAINER(window), GTK_WIDGET(web_view));

    webkit_web_view_load_uri(web_view, url);

    g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(window);

    gtk_main();

    return 0;
}
