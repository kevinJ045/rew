import { compile as compileCivet } from "../../civet/main";
import { CompilerType } from "./compilet";
import { CompileOptions } from "../types/options";

export default CompilerType.create({
  compile(
    filename: string,
    options: CompileOptions,
    content: string
  ): string {
    return compileCivet(
      content,
      {
        ...options,
        sync: true,
        filename,
        parseOptions: {
          coffeeCompat: filename.endsWith('.coffee')
        },
      }
    )  
  }
});