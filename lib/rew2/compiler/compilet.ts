import FIO from "../fio/fio";
import { CompileOptions } from "../types/options";

export class CompilerType {
  compile: (
    filename: string,
    options: CompileOptions,
    content: string
  ) => string;


  static create({
    compile,
  } : {
    compile: (
      filename: string,
      options: CompileOptions,
      content: string
    ) => string
  }){
    return { compile };
  }
}

export class Compiler {

  static compilers = new Map<string, CompilerType>()

  static compile(
    type: string,
    filename: string,
    options: CompileOptions,
    content?: string,
  ){
    const compiler = this.compilers.get(type);
    if(!content) FIO.read(filename);

    if(compiler){
      return compiler.compile(
        filename,
        options || {},
        content
      );
    }

    return "";
  }

  static find(name: string){
    return this.compilers.has(name);
  }

  static register(name: string | string[], compiler: CompilerType){
    if(Array.isArray(name)){
      name.forEach(name => {
        this.compilers.set(name, compiler);
      });
    } else if(typeof name === "string"){
      this.compilers.set(name, compiler);
    }
    return this;
  }

}