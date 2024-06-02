const fs = require("fs");
const path = require("path");

module.exports.customRequire = function customRequire(modulePath, filePath) {
  const resolvedPath = resolveModulePath(modulePath, filePath);
  return require(resolvedPath);
};

function resolveModulePath(modulePath, filePath) {
  if (
    modulePath.startsWith("./") ||
    modulePath.startsWith("../") ||
    path.isAbsolute(modulePath)
  ) {
    return path.resolve(modulePath);
  }

  const paths = module.constructor._nodeModulePaths(path.dirname(filePath));
  for (const basePath of paths) {
    const fullPath = path.join(basePath, modulePath);
    if (fs.existsSync(fullPath + ".js")) {
      return fullPath + ".js";
    }
    if (fs.existsSync(fullPath + ".json")) {
      return fullPath + ".json";
    }
    if (fs.existsSync(fullPath) && fs.statSync(fullPath).isDirectory()) {
      const packageJsonPath = path.join(fullPath, "package.json");
      if (fs.existsSync(packageJsonPath)) {
        const main = require(packageJsonPath).main || "index.js";
        const mainPath = path.join(fullPath, main);
        if (fs.existsSync(mainPath)) {
          return mainPath;
        }
      }
      const indexPath = path.join(fullPath, "index.js");
      if (fs.existsSync(indexPath)) {
        return indexPath;
      }
    }
  }

  throw new Error(`Cannot find module '${modulePath}'`);
}
