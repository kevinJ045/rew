using namespace rew::ns()

print "Starting event listener..."

# Listen for events from Rust
addEventListener "tick", (data) ->
  print "Received tick event:", data.counter, "at", data.timestamp

# You can also do other work here
i = 0
while i < 1000
  i += 1
  
print "Setup complete, waiting for events..."