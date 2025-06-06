rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/rand.coffee", {
"/home/makano/workspace/rew-rust/test/rand.coffee"(globalThis){
with (globalThis) {
  using(namespace(rew.prototype.ns));

print(pickRandom(genUid()))
print(pickRandom(genUid(24)))
print(pickRandom(genUid(24, "kkiiggllmmffx")))
print(pickRandom(1))
print(pickRandom("ss", "sss", "ssss", "sssss"))
print(randFrom(1, 10))
print(randFrom(1, 10, "sss"))

}
return globalThis.module.exports;
}          
}, ["app://test.app/rand"]);
rew.prototype.mod.prototype.get('/home/makano/workspace/rew-rust/test/rand.coffee');