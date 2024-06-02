function exportsThe(item, name, context) {
  if (name) {
    if (!context.module.exports) context.module.exports = {};
    context.module.exports[name] = item;
  } else {
    if (context.module.exports) context.module.exports.default = item;
    else context.module.exports = item;
  }
}

module.exports.pubFunction = function (context) {
  return function (name, item) {
    if (name && !item) {
      item = name;
      name = null;
    }
    exportsThe(item, name, context);
  };
};

module.exports.exportsFunction = function (context) {
  return function (item, name) {
    exportsThe(item, name, context);
  };
};
