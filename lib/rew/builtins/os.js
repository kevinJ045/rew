"no-compile"
if(!rew.extensions.has('os')) rew.extensions.add('os', (Deno) => rew.extensions.createClass({
  slug: Deno.build.os,
  arch: Deno.build.arch,
  version: Deno.version.deno,
  v8: Deno.version.v8,
  typescript: Deno.version.typescript,
  hostname: Deno.hostname(),
  homeDir: rew.prototype.env.prototype.get("HOME") || rew.prototype.env.prototype.get("USERPROFILE"),
  tempDir: rew.prototype.env.prototype.get("TMPDIR") || rew.prototype.env.prototype.get("TEMP"),
  userInfo: () => ({
    username: rew.prototype.env.prototype.get("USER") || rew.prototype.env.prototype.get("USERNAME"),
    uid: Deno.uid(),
    gid: Deno.gid(),
  })
}));