import Context from "./context/context";
import Runtime from "./runtime/runtime";
import { BastardObject } from "./types/misc";
import { RuntimeOptions } from "./types/options";



export function run(
  filepath: string,
  options: RuntimeOptions = {},
  custom_context?: BastardObject
){
  return Runtime.run(filepath, options, custom_context ? Context.create(filepath, custom_context) : undefined)
}