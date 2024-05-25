const { struct } = require("../models/struct");
const fs = require("fs");

const file = (module.exports.file = struct({
  path: "",
  content: "",
}));

const readFile = (module.exports.readFile = function readFile(file) {
  return (file.content = fs.readFileSync(file.path, { encoding: "utf-8" }));
});

module.exports.getFile = function (filepath) {
  const f = file({
    path: filepath,
  });
  readFile(f);
  return f;
};
