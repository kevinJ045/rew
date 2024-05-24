#include <gtk/gtk.h>
#include <libwebsockets.h>
#include <json/json.h>
#include <iostream>
#include <string>
#include <map>

std::map<std::string, GtkWidget *> widgets;

static void on_button_clicked(GtkWidget *widget, gpointer data)
{
  // Send event back to Node.js server
  std::string id = static_cast<char *>(data);
  std::string message = "{\"event\": \"buttonClicked\", \"id\": \"" + id + "\"}";
  lws_write(static_cast<lws *>(data), (unsigned char *)message.c_str(), message.length(), LWS_WRITE_TEXT);
}

static int callback_websocket(struct lws *wsi, enum lws_callback_reasons reason, void *user, void *in, size_t len)
{
  switch (reason)
  {
  case LWS_CALLBACK_CLIENT_ESTABLISHED:
    std::cout << "Connected to server" << std::endl;
    break;
  case LWS_CALLBACK_CLIENT_RECEIVE:
  {
    std::string message(static_cast<char *>(in), len);
    Json::CharReaderBuilder rbuilder;
    Json::Value root;
    std::string errors;

    std::istringstream s(message);
    std::string doc;
    std::getline(s, doc);
    std::istringstream ss(doc);
    if (Json::parseFromStream(rbuilder, ss, &root, &errors))
    {
      std::string action = root["action"].asString();

      if (action == "createButton")
      {
        GtkWidget *button = gtk_button_new_with_label(root["label"].asString().c_str());
        g_signal_connect(button, "clicked", G_CALLBACK(on_button_clicked), strdup(root["id"].asString().c_str()));
        widgets[root["id"].asString()] = button;
        gtk_container_add(GTK_CONTAINER(gtk_window_new(GTK_WINDOW_TOPLEVEL)), button);
        gtk_widget_show_all(button);
      }
    }
    break;
  }
  case LWS_CALLBACK_CLIENT_CLOSED:
    std::cout << "Disconnected from server" << std::endl;
    break;
  default:
    break;
  }
  return 0;
}

static const struct lws_protocols protocols[] = {
    {
        "example-protocol",
        callback_websocket,
        0,
        1024,
    },
    {NULL, NULL, 0, 0}};

int main(int argc, char **argv)
{
  std::cout << "Starting" << std::endl;
  if (argc != 2)
  {
    std::cerr << "Usage: " << argv[0] << " <WebSocket server URI>" << std::endl;
    return 1;
  }

  gtk_init(&argc, &argv);

  struct lws_context_creation_info info;
  memset(&info, 0, sizeof info);
  info.port = CONTEXT_PORT_NO_LISTEN;
  info.protocols = protocols;

  struct lws_context *context = lws_create_context(&info);

  struct lws_client_connect_info ccinfo = {0};
  ccinfo.context = context;
  ccinfo.address = argv[1];
  ccinfo.port = 8080;
  ccinfo.path = "/";
  ccinfo.host = lws_canonical_hostname(context);
  ccinfo.origin = "origin";
  ccinfo.protocol = protocols[0].name;

  struct lws *wsi = lws_client_connect_via_info(&ccinfo);

  while (lws_service(context, 1000) >= 0)
  {
    gtk_main_iteration_do(false);
  }

  lws_context_destroy(context);
  return 0;
}
