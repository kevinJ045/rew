import Rew from "../stdlib/rew";
import { BastardObject } from "../types/misc";


export default class Context {

  bastard: BastardObject;

  static create(filename: string, object?: BastardObject){
    const context = new Context();
    context.bastard = object || {};

    context.define("rew", Rew.create(filename));

    return context;
  }

  define(
    name: string,
    value: any,
    types?: (new () => any)[]
  ){
    const keys = name.split('.');
    if(keys.length == 1){
      this.bastard[name] = value;
    } else {
      let last = this.bastard;
      keys.forEach((key, index) => {
        if(!last[key]){
          last[key] = types?.[index] ? new types[index]() : {};
        }
        last = last[key];
      });
    }
    return value;
  }

}