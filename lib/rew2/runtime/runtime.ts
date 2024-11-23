import Compiler from "../compiler/all";
import Context from "../context/context";
import { extname } from "path";
import FIO from "../fio/fio";
import { RuntimeOptions } from "../types/options";
import * as vm from 'vm';


export default class Runtime {

  context: Context;

  runInVm(
    filename: string,
    content: string,
  ){
    return vm.runInContext(content, vm.createContext(
      this.context.bastard
    ), {
      filename
    });
  }

  static run(
    filename: string,
    options?: RuntimeOptions,
    context?: Context,
  ){
    const runtime = new Runtime();
    if(context){
      runtime.context = context;
    } else {
      runtime.context = Context.create(filename);
    }

    const type = extname(filename).slice(1);
    let content = FIO.read(filename, true).toString();

    if(Compiler.find(type)){
      content = Compiler.compile(
        type,
        filename,
        options?.compileOptions || {},
        content
      );
    }
    
    return runtime.runInVm(
      filename,
      content
    );
  }


}