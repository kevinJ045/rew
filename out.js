rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/external.coffee", {
"/home/makano/workspace/rew-rust/test/external.coffee"(globalThis){
with (globalThis) {
  const j = rew.prototype.mod.prototype.find(module,  "gaga.daga!")
rew.prototype.io.prototype.out.print(j)
rew.prototype.io.prototype.out.print('hi')
}
return globalThis.module.exports;
}          
}, ["app://test.app/external"]);
// entry "/home/makano/.rew/apps/gaga.daga/main.coffee" 

rew.prototype.mod.prototype.defineNew("/home/makano/.rew/apps/gaga.daga/main.coffee", {
"/home/makano/.rew/apps/gaga.daga/main.coffee"(globalThis){
with (globalThis) {
  
const submodule = rew.prototype.mod.prototype.find(module,  "./help.coffee")
const sn = rew.prototype.mod.prototype.find(module,  "./src/main.coffee")

//declare* "sayhello" = rew::io::out.print('hi');

module.exports = {
	isMain: true,
	submodule,
	sn
}

}
return globalThis.module.exports;
}          
}, ["app://gaga.daga/main"]);rew.prototype.mod.prototype.defineNew("/home/makano/.rew/apps/gaga.daga/help.coffee", {
"/home/makano/.rew/apps/gaga.daga/help.coffee"(globalThis){
with (globalThis) {
  module.exports = {
	submodule: true
}

}
return globalThis.module.exports;
}          
}, ["app://gaga.daga/help"]);rew.prototype.mod.prototype.defineNew("/home/makano/.rew/apps/gaga.daga/src/main.coffee", {
"/home/makano/.rew/apps/gaga.daga/src/main.coffee"(globalThis){
with (globalThis) {
  let fgh
module.exports.fgh =  fgh = "hello"

}
return globalThis.module.exports;
}          
}, ["app://gaga.daga/src/main"]);

rew.prototype.mod.prototype.get('/home/makano/.rew/apps/gaga.daga/main.coffee');
rew.prototype.mod.prototype.get('/home/makano/workspace/rew-rust/test/external.coffee');