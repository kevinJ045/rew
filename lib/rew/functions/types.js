
const _defaultConstructors = {
  string: String,
  array: Array,
  number: Number,
  bigint: BigInt,
  boolean: Boolean,
  symbol: Symbol,
  undefined: Object,
  object: Object,
  function: Function
}

function getType(value){
  return typeof value === 'object' ? Array.isArray(value) ? 'array' : typeof value : typeof value;
}

function typedef(value) {
  return {
    defaultValue: value,
    class: typeof value == 'function' ? value : typeof value === 'object' && value !== null && !Array.isArray(value) ? value.constructor : _defaultConstructors[getType(value)],
    type: getType(value),
    isConstucted: typeof value === 'object' && value !== null && !Array.isArray(value),
    isEmpty: typeof value == "object" ? !Object.keys(value).length : typeof value == "string" ? value == "" : true
  };
}

function typeis(obj, typeDef) {
  if (typeDef.isConstucted && typeDef.class && !(obj instanceof typeDef.class)) {
    return false;
  }

  if(getType(obj) == "object" && typeDef.type == 'function') {
    return (obj instanceof typeDef.class);
  }

  if(getType(obj) !== typeDef.type){
    return false;
  }

  if(!typeDef.isEmpty) {
    if(typeDef.type == 'object'){
      for (const key in typeDef.defaultValue) {
        const propTypeDef = typeDef.defaultValue[key];
    
        if (typeof propTypeDef === 'object') {
          if (!typeis(obj[key], propTypeDef)) {
            return false;
          }
        } else if (typeof obj[key] !== propTypeDef) {
          return false;
        }
      }
    } else if(typeDef.type == 'string'){
      return typeDef.defaultValue == obj;
    }
  }

  return true;
}

function typex(child, parent) {
  return child.prototype instanceof parent || child === parent;
}

function typei(child, parent) {
  return child instanceof parent || child.constructor === parent;
}

function int(str){
  return parseInt(str);
}

function float(str){
  return parseFloat(str);
}

function num(str){
  return Number(str);
}

function str(str){
  return str ? str.toString() : "";
}

function bool(value){
  return typeof value == 'string' ? (
    value == 'true' ? true : false
  ) : value !== null && value !== undefined;
}

module.exports = {
  typex,
  typei,
  typeis,
  typedef,
  
  int,
  float,
  num,
  str,
  bool
}


// const f = typedef('');
// const fg = typedef({ g: f });

// const g = { g: 'sss' };

// class L {}

// class N {}

// class M extends N{}

// let n = new N
// let m = new M
// let l = new L

// let tn = typedef(n)

// // console.log(typeis(g, fg), typeis(1, f), typei('', String));
// console.log(typeis(l, tn));