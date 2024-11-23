

class RewCurrent {
  filename: string;
  constructor(rew: Rew, filename: string) {
    this.filename = filename;
  }
}

class RewFS {
  constructor(rew: Rew, filename: string) {}
}

class RewIO {
  constructor(rew: Rew, filename: string) {}

  print(...string: any[]){
    console.log(...string);
  }
}

class RewProto {
  current: RewCurrent;
  fs: RewFS;
  io: RewIO;


  static create(rew: Rew, filename: string){
    const p = new RewProto;
    p.current = new RewCurrent(rew, filename);
    p.fs = new RewFS(rew, filename);
    p.io = new RewIO(rew, filename);
    return p;
  }
}

export default class Rew {

  prototype: RewProto;
  static create(
    filename: string,
  ){
    const rew = new Rew();
    rew.prototype = RewProto.create(rew, filename);
    return rew;
  }



}