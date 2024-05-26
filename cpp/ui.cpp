#include <gtk/gtk.h>
#include <webkit2/webkit2.h>
#include <json/json.h>
#include <iostream>
#include <sstream>
#include <string>
#include <fstream>
#include <thread>
#include <sys/inotify.h>
#include <unistd.h>
#include <vector>

struct AppData
{
  GtkWidget *window;
  WebKitWebView *web_view;
  WebKitUserContentManager *content_manager;
};

static void handle_json_message(const Json::Value &json, AppData *user_data)
{
  if (json.isMember("action") && json.isMember("data"))
  {
    std::string action = json["action"].asString();
    std::string data = json["data"].asString();

    // std::cout << action << std::endl;

    if (action == "setTitle")
    {
      gtk_window_set_title(GTK_WINDOW(user_data->window), data.c_str());
    }
    else if (action == "log")
    {
      g_print("%s\n", data.c_str());
    }
    else if (action == "HTML")
    {
      webkit_web_view_load_html(user_data->web_view, data.c_str(), NULL);
      g_print("SETUP::HTML");
    }
    else if (action == "JS")
    {
      webkit_web_view_run_javascript(user_data->web_view, data.c_str(), NULL, NULL, NULL);
    }
    else if (action == "JS2")
    {
      const char *js_code = g_strdup_printf("%s;", data.c_str());
      WebKitUserScript *user_script = webkit_user_script_new(js_code,
                                                             WEBKIT_USER_CONTENT_INJECT_ALL_FRAMES,
                                                             WEBKIT_USER_SCRIPT_INJECT_AT_DOCUMENT_START,
                                                             NULL, NULL);
      webkit_user_content_manager_add_script(user_data->content_manager, user_script);
    }
  }
  else
  {
    g_print("Invalid JSON format: 'action' and 'data' fields are required\n");
  }
}

static void js_callback(WebKitUserContentManager *manager,
                        WebKitJavascriptResult *js_result,
                        gpointer user_data)
{
  JSCValue *value = webkit_javascript_result_get_js_value(js_result);
  if (jsc_value_is_string(value))
  {
    char *str_value = jsc_value_to_string(value);

    Json::Value root;
    Json::CharReaderBuilder builder;
    std::string errors;
    std::istringstream json_stream(str_value);
    bool success = Json::parseFromStream(builder, json_stream, &root, &errors);

    if (success)
    {
      handle_json_message(root, static_cast<AppData *>(user_data));
    }
    else
    {
      g_print("%s", str_value);
    }

    g_free(str_value);
  }
}

void watch_file(const std::string &file_path, AppData *app_data)
{
  int inotify_fd = inotify_init();
  if (inotify_fd < 0)
  {
    perror("inotify_init");
    return;
  }

  int watch_fd = inotify_add_watch(inotify_fd, file_path.c_str(), IN_MODIFY);
  if (watch_fd < 0)
  {
    perror("inotify_add_watch");
    close(inotify_fd);
    return;
  }

  const size_t event_size = sizeof(struct inotify_event);
  const size_t buf_len = 1024 * (event_size + 16);
  std::vector<char> buffer(buf_len);

  while (true)
  {
    int length = read(inotify_fd, buffer.data(), buf_len);
    if (length < 0)
    {
      perror("read");
      break;
    }

    for (int i = 0; i < length;)
    {
      struct inotify_event *event = reinterpret_cast<struct inotify_event *>(&buffer[i]);
      if (event->mask & IN_MODIFY)
      {
        std::ifstream file(file_path);
        std::stringstream buffer;
        buffer << file.rdbuf();
        std::string content = buffer.str();
        file.close();

        if (!content.empty())
        {
          Json::Value root;
          Json::CharReaderBuilder builder;
          std::string errors;
          std::istringstream json_stream(content);
          bool success = Json::parseFromStream(builder, json_stream, &root, &errors);

          // std::cout << "CONTENT DETECTED" << std::endl;

          if (success)
          {
            handle_json_message(root, app_data);
          }
          else
          {
            g_print("Failed to parse JSON string: %s\n", errors.c_str());
          }

          std::ofstream clear_file(file_path, std::ofstream::out | std::ofstream::trunc);
          clear_file.close();
        }
      }
      i += event_size + event->len;
    }
  }

  close(watch_fd);
  close(inotify_fd);
}

int main(int argc, char **argv)
{
  if (argc != 2)
  {
    g_print("Usage: %s <RUNID>\n", argv[0]);
    return 1;
  }

  const char *rid = argv[1];
  std::string file_path = "/tmp/" + std::string(rid) + ".ruw.ui.socket";

  AppData app_data;

  gtk_init(&argc, &argv);

  GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_title(GTK_WINDOW(window), "WebKit Example");
  gtk_window_set_default_size(GTK_WINDOW(window), 800, 600);

  app_data.window = window;

  WebKitUserContentManager *content_manager = webkit_user_content_manager_new();
  webkit_user_content_manager_register_script_message_handler(content_manager, "external");
  g_signal_connect(content_manager, "script-message-received::external", G_CALLBACK(js_callback), &app_data);
  app_data.content_manager = content_manager;

  const char *js_code = g_strdup_printf("window.RUNID = \"%s\";", rid);
  WebKitUserScript *user_script = webkit_user_script_new(js_code,
                                                         WEBKIT_USER_CONTENT_INJECT_ALL_FRAMES,
                                                         WEBKIT_USER_SCRIPT_INJECT_AT_DOCUMENT_START,
                                                         NULL, NULL);
  webkit_user_content_manager_add_script(content_manager, user_script);

  WebKitWebView *web_view = WEBKIT_WEB_VIEW(webkit_web_view_new_with_user_content_manager(content_manager));
  gtk_container_add(GTK_CONTAINER(window), GTK_WIDGET(web_view));
  app_data.web_view = web_view;

  webkit_web_view_load_html(web_view, "Loading...", NULL);

  GIOChannel *channel = g_io_channel_unix_new(fileno(stdin));
  GIOFlags flags = static_cast<GIOFlags>(g_io_channel_get_flags(channel) | G_IO_FLAG_NONBLOCK);
  g_io_channel_set_flags(channel, flags, NULL);

  g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), NULL);

  std::thread watch_thread(watch_file, file_path, &app_data);

  g_print("INIT::READY");

  gtk_widget_show_all(window);
  gtk_main();

  watch_thread.detach();

  return 0;
}
