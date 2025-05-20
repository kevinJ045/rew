"no-compile"
if(!rew.extensions.has('path')) rew.extensions.add('path', (Deno, module) => rew.extensions.createClass({
  _namespace(){
    return 'shell';
  },
  resolveFrom(base, relative) {
    const baseParts = base.split('/');
    baseParts.pop();
    const relativeParts = relative.split('/');

    for (const part of relativeParts) {
      if (part === '.' || part === '') continue;
      if (part === '..') baseParts.pop();
      else baseParts.push(part);
    }

    return baseParts.join('/');
  },
  resolve(...paths){
    const path = this.join(...paths);
    return this.resolveFrom(module.filename, path);
  },
  choose(...paths) {
    return paths
      .map(p => this.resolve(p))
      .find(p => rew.ops.op_fs_exists(module.filename, p)) || null;
  },
  join(...segments) {
    return segments
      .map((segment) => segment.replace(/\/+$/, '')) // Remove trailing slashes
      .join('/')
      .replace(/\/+/g, '/'); // Normalize multiple slashes
  },
  normalize(path) {
    const parts = path.split('/');
    const normalized = [];

    for (const part of parts) {
      if (part === '.' || part === '') continue;
      if (part === '..') normalized.pop();
      else normalized.push(part);
    }

    return normalized.join('/');
  },
  dirname(path) {
    const parts = path.split('/');
    parts.pop();
    return parts.join('/') || '/';
  },
  basename(path) {
    const parts = path.split('/');
    return parts.pop() || '';
  },
  extname(path) {
    const base = this.basename(path);
    const index = base.lastIndexOf('.');
    return index > 0 ? base.slice(index) : '';
  },
  isAbsolute(path) {
    return path.startsWith('/');
  },
  relative(from, to) {
    const fromParts = this.resolve('/', from).split('/');
    const toParts = this.resolve('/', to).split('/');
    while (fromParts.length && toParts.length && fromParts[0] === toParts[0]) {
      fromParts.shift();
      toParts.shift();
    }
    return '../'.repeat(fromParts.length - 1) + toParts.join('/');
  },
}));