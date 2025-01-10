

module.exports = class RuneDB {
  constructor(attrs){
    for(let i in attrs){
      this[i] = attrs[i];
    }
  }
};