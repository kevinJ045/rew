module.exports = function emitter() {
  const events = [];
  const on = (event, callback) => {
    const addEvent = (event) => events.push({ event, callback });
    if (Array.isArray(event)) {
      event.forEach(addEvent);
    } else {
      addEvent(event);
    }
    return listener;
  };
  const off = (event, callback) => {
    const rmEvent = (event) => {
      if (callback) {
        const _events = events.filter(({ callback: c }) => c == callback);
        _events.forEach((e) => {
          if (Array.isArray(event)) {
            if (event.includes(e.event)) events.splice(events.indexOf(e), 1);
          } else {
            if (e.event == event) events.splice(events.indexOf(e), 1);
          }
        });
      } else {
        const _events = events.filter(({ event: e }) => e == event);
        _events.forEach((e) => {
          events.splice(events.indexOf(e), 1);
        });
      }
    };
    if (Array.isArray(event)) {
      event.forEach(rmEvent);
    } else {
      rmEvent(event);
    }
    return listener;
  };
  const emit = (event, ...data) => {
    const emitEvent = (event) =>
      events
        .filter(({ event: e }) => e == event)
        .forEach(({ callback }) => callback(...data));
    if (Array.isArray(event)) {
      event.forEach(emitEvent);
    } else {
      emitEvent(event);
    }
    return listener;
  };
  const listener = { on, off, emit };
  return listener;
};
