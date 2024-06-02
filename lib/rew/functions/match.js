const SerializableData = ["string", "number", "boolean"];

const isRegExp = (obj) =>
  Object.prototype.toString.call(obj) === "[object RegExp]";

module.exports.match = function match(value, templates, props) {
  const matchProps = (pattern, value) => {
    if (!props) return false;
    if (typeof props === "object") {
      let t = true;
      for (let i in props) {
        t = t && pattern[i] == value[i];
        if (!t) break;
      }
      return t;
    } else if (typeof props === "function") {
      return props(pattern, value);
    } else {
      return false;
    }
  };

  const matchRegex = (pattern, value) => pattern.test(value);

  const entries =
    templates instanceof Map
      ? templates.entries()
      : Array.isArray(templates)
        ? templates
        : Object.entries(templates);

  for (const [pattern, callback] of entries) {
    if (
      isRegExp(pattern)
        ? matchRegex(pattern, value)
        : SerializableData.includes(typeof value)
          ? pattern == value
          : isRegExp(pattern)
            ? matchRegex(pattern, value)
            : typeof pattern === "function"
              ? value instanceof pattern ||
                value?.__proto__?.["@instance"] === pattern
              : matchProps(pattern, value)
    ) {
      return callback(...(isRegExp(pattern) ? pattern.exec(value) : [value]));
    }
  }

  return null;
};
