import CivetCompiler from "./civet";
import { Compiler } from "./compilet";

Compiler.register(
  ['civet', 'coffee'],
  CivetCompiler
)

export default Compiler;