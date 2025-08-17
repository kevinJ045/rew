import "#std.http";

rew::http::withOptions(port: 3000) (req) ->
  rew::http::Response::new("Hello, World!!")