using namespace rew::ns;

e = rew::channel::emitter()

e
  .on 'hi', (i) -> print 'hi', i
  .on 'hello', -> print 'hello'
  .emit ['hi', 'hello'], 'Yo'