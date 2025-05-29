"no-compile"
if(!rew.extensions.has('os')) rew.extensions.add('os', (Deno) => rew.extensions.createClass({
  slug: Deno.core.ops.op_os_info_os(),
  arch: Deno.core.ops.op_os_info_arch(),
  family: Deno.core.ops.op_os_info_family(),
  release: Deno.os.osRelease(),
  get loadavg(){
    return Deno.os.loadavg()
  },
  get uptime(){
    return Deno.os.osUptime()
  },
  get hostname(){
    return Deno.os.hostname()
  },
  mem: () => Deno.os.systemMemoryInfo(),
  networkInterfaces: () => Deno.os.networkInterfaces(),
  get homeDir(){
    return rew.prototype.env.prototype.get("HOME") || rew.prototype.env.prototype.get("USERPROFILE")
  },
  get tempDir(){
    return rew.prototype.env.prototype.get("TMPDIR") || rew.prototype.env.prototype.get("TEMP")
  },
  userInfo: () => ({
    username: rew.prototype.env.prototype.get("USER") || rew.prototype.env.prototype.get("USERNAME"),
    uid: Deno.os.uid(),
    gid: Deno.os.gid(),
  })
}));