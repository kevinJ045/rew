"no-compile"
if(!rew.extensions.has('path')) rew.extensions.add('path', (Deno, module) => {
  const isWindows = Deno.core.ops.op_os_info_os() === 'windows';
  const pathSep = isWindows ? '\\' : '/';
  const pathSepRegex = isWindows ? /[/\\]/g : /\//g;
  
  function normalizePath(path) {
    return path.replace(pathSepRegex, pathSep);
  }
  
  function splitPath(path) {
    return path.split(pathSepRegex);
  }
  
  function joinPath(...parts) {
    return parts.join(pathSep);
  }
  
  function isAbsolutePath(path) {
    return isWindows ? /^[A-Za-z]:[/\\]/.test(path) : path.startsWith('/');
  }
  
  return rew.extensions.createClass({
    _namespace(){
      return 'path';
    },
    inapp(path){
      return this.join(module.app.path || ".", path);
    },
    resolveFrom(base, relative) {
      return rew.prototype._path.prototype.resolveFrom(base, relative);
    },
    resolve(...paths){
      const path = this.join(...paths);
      return this.resolveFrom(module.filename, path);
    },
    choose(...paths) {
      return paths
        .map(p => isAbsolutePath(p) ? p : this.resolve(p))
        .find(p => rew.ops.op_fs_exists(module.filename, p)) || null;
    },
    join(...segments) {
      let segment_root = segments.find(segment => isAbsolutePath(segment));
      return segments.indexOf(segment_root) > 0 ? segment_root : segments
        .map((segment) => segment.replace(new RegExp(pathSep.replace(/\\/g, '\\\\') + '+$'), '')) // Remove trailing separators
        .join(pathSep)
        .replace(new RegExp(pathSep.replace(/\\/g, '\\\\') + '+', 'g'), pathSep); // Normalize multiple separators
    },
    normalize(path) {
      const parts = splitPath(path);
      const normalized = [];

      for (const part of parts) {
        if (part === '.' || part === '') continue;
        if (part === '..') normalized.pop();
        else normalized.push(part);
      }

      let np = joinPath(...normalized);
      return isAbsolutePath(path) ? np : (isWindows ? np : pathSep + np);
    },
    dirname(path) {
      const parts = splitPath(path);
      parts.pop();
      return joinPath(...parts) || (isWindows ? path.match(/^[A-Za-z]:/)?.[0] || '.' : '/');
    },
    basename(path) {
      const parts = splitPath(path);
      return parts.pop() || '';
    },
    extname(path) {
      const base = this.basename(path);
      const index = base.lastIndexOf('.');
      return index > 0 ? base.slice(index) : '';
    },
    isAbsolute(path) {
      return isAbsolutePath(path);
    },
    relative(from, to) {
      const fromParts = splitPath(this.resolve(isWindows ? 'C:\\' : '/', from));
      const toParts = splitPath(this.resolve(isWindows ? 'C:\\' : '/', to));
      while (fromParts.length && toParts.length && fromParts[0] === toParts[0]) {
        fromParts.shift();
        toParts.shift();
      }
      const upDirs = fromParts.length > 0 ? fromParts.length - 1 : 0;
      const relativeParts = upDirs > 0 ? new Array(upDirs).fill('..') : [];
      return joinPath(...relativeParts, ...toParts);
    },
    sep: pathSep,
    delimiter: isWindows ? ';' : ':'
  });
});
