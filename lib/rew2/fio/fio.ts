import { resolve } from "path";
import { readFileSync } from "fs";


export default class FIO {

  static read(file: string, string?: boolean, relative?: string){
    const filepath = relative ? resolve(file, relative) : file;
    return readFileSync(filepath, string ? { encoding: 'utf-8' } : null);
  }


}