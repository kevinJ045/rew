
interface ImportOptions {
  /**
   * Determines how to import the given module
   */
  type: 'js' | 'coffee' | 'yaml' | 'json' | 'qrew';
  [key: string]: any;
}

interface ModuleConfOptionCenter {
  /**
   * Get a config key
   * @param key The key of the config to get
   * @param defaultValue The default value ig null
   * @returns The value of the key or the defaultValue if it's null.
   */
  get: <T = any>(key: string, defaultValue?: T) => T
  /**
   * Set a config key
   * @param key The key of the config to set
   * @param value The value to set it to
   * @returns true if it was a success
   */
  set: <T = any>(key: string, value: T) => boolean
  /**
   * Removes a key from the config
   * @param key The key of the config to remove
   * @returns true if it was a success
   */
  remove: (key: string) => boolean
  /**
   * Resets the entire config option center to it's default value
   */
  reset: () => boolean
  /**
   * Get all values in an option center
   * @param str 
   * @returns 
   */
  getAll: (() => string) | ((str?: false) => Record<string, any>)
  root: string;
}

interface ModuleConf extends ModuleConfOptionCenter {
  /**
   * A separate options file for a related set of options
   * @param name The option center full path
   * @param defaults The default values
   * 
   * @example 
   * conf = imp 'conf'
   * 
   * animations = conf.optionCenter 'animations', enable: false, speed: '1x'
   * 
   * if animations.get 'enable'
   *  animate animations.get 'speed'
   */
  optionCenter: (name: string, defaults?: any) => ModuleConfOptionCenter;
  /**
   * Manage Static files
   */
  staticFile: (name: string, defaults?: any) => {
    write: (value: any, ifExists?: boolean) => this,
    read: (to?: string | object) => string | object | Buffer,
    fileRoot: string,
    exists: boolean
  };
}

interface ModuleEnv {
  has: (key: string) => boolean,
  get: (key: string) => string,
  set: (key: string, value: string) => boolean,
  rm: (key: string) => boolean,
  is: (key: string, value: string) => boolean,
}

interface ModuleRuneDBCollcetion {
  insert(record: object): any; 
  read(id: string | object, evaluate?: boolean): any; 
  update(caseRecord: string | object, newRecord: object): any; 
  remove(id: string | object): boolean;
  find(criteria: string | object): any; 
  map(cb: (data: any[]) => any[], mutate?: boolean): any[]; 
  transform(cb: (data: any[]) => any[], mutate?: boolean): any[]; 
  filter(cb: (data: any[]) => boolean, mutate?: boolean): any[]; 
  sort(cb: (a: any, b: any) => number, mutate?: boolean): any[]; 
  list(): any[];
}

interface ModuleRuneDBMap {
  set(key: string, value: any): void;
  get(key: string): any | null;
  remove(key: string): boolean;
  transform(cb: (data: any) => any, mutate?: boolean): any;
  list(): { [key: string]: any };
}

interface ModuleRuneDB {
  collection: (name: string) => ModuleRuneDBCollcetion
  map: (name: string) => ModuleRuneDBMap
  findRef: (ref: string) => any
  setData: (data: Record<string, any>) => void
  getData: () => Record<string, any>
  makeRef(value: object, props?: string): string | null;
}

interface ModuleRune {
  db(dbname: string, data?: object, encryptionKey?: string): ModuleRuneDB;
  genKey(secret: string): string;
  push(...values: any[]): PushChange;
  pop(...values: any[]): PopChange;
}

interface ModuleThreads {
  thread: (cb: Function) => {
    stopAll: () => void
    start: (context: Record<string, any>) => {
      on: (event: string, callback: (data) => void) => void;
      off: (event: string, callback: (data) => void) => void;
      emit: (event: string, data: any) => void;
      get: () => Promise,
      stop: () => void 
    }
  }
}

declare function imp(path: "conf", options?: ImportOptions): ModuleConf;
declare function imp(path: "env", options?: ImportOptions): ModuleEnv;
declare function imp(path: "rune", options?: ImportOptions): ModuleRune;
declare function imp(path: "threads", options?: ImportOptions): ModuleThreads;
declare function imp(path: string, options?: ImportOptions): any;

declare const inc = imp;

declare function require(moduleName: string): any;

interface Module {
  exports: any;
  filepath: string;
  main: boolean;
  impots: string[];
  compiled: string
}

declare const module: Module;

interface Imports {
  meta: {},
  assets: any
}

declare const imports: Imports;

declare const process: {
  argv: string[],
  target: ReturnType<typeof emitter>,
  __execFile: string,
  env: Record<string, any>,
  cwd: () => string,
  arch: string,
  exit: () => void
};

interface AppConfig {
  manifest: {
    package: string
  }
}

declare const app: {
  path: string,
  config: AppConfig
}


declare function read(filepath: string, options?: { encoding: string }): string;

declare function realpath(filepath: string, options?: { encoding: string }): string;

declare function write(filepath: string, content: any, options?: any): void;

declare function exists(filepath: string, options?: any): boolean;

declare function fstat(filepath: string, options?: any): any; 

declare function rm(filepath: string, options?: any): void;

declare function chmod(filepath: string, mode: any, options?: any): void;

declare function mkdir(filepath: string, options?: any): void;

declare function ls(filepath: string, options?: any): string[];

declare function struct(template: { [key: string]: any }): (...args: any[]) => any; 

declare function future(callback: (resolve: (data: any) => void, reject: (data: any) => void) => void, timeout?: number, defData?: any): {
  pipe(callback: (data: any) => any): Promise<any>;
  last(callback: (data: any) => any): Promise<any>;
  catch(callback: (data: any) => any): Promise<any>;
  resolve(data: any): void;
  reject(data: any): void;
  wait(): Promise<any>;
};
declare namespace future {
  function promise(promse: Promise<any>, timeout?: number, defData?: any): ReturnType<typeof future>;
}

declare function emitter(): {
  on(event: string | string[], callback: (...args: any[]) => void, props?: {}): ReturnType<typeof emitter>;
  off(event: string | string[], callback: (...args: any[]) => void, removable?: (event: any) => void): ReturnType<typeof emitter>;
  emit(event: string | string[], ...data: any[]): ReturnType<typeof emitter>;
};
declare function exec(command: string, options?: { output?: boolean }): any; 
declare namespace exec {
  function background(command: string, options?: any, callback?: (...args: any[]) => void): any; 
}
declare function spawn(command: string, ...args: any[]): any; 

declare function typedef(value: any, strict?: boolean): { strict: boolean; defaultValue: any; class: Function; type: string; isConstucted: boolean; isEmpty: boolean };

declare function typeis(obj: any, typeDef: any): boolean;

declare function typex(child: any, parent: any): boolean;

declare function typei(child: any, parent: any): boolean;

declare function int(str: string): number;

declare namespace int {
  const type: { strict: boolean; defaultValue: number; class: Function; type: string; isConstucted: boolean; isEmpty: boolean };
}
declare function float(str: string): number;
declare namespace float {
  const type: { strict: boolean; defaultValue: number; class: Function; type: string; isConstucted: boolean; isEmpty: boolean };
}
declare function num(str: string): number;
declare namespace num {
  const type: { strict: boolean; defaultValue: number; class: Function; type: string; isConstucted: boolean; isEmpty: boolean };
}
declare function str(str: any): string;
declare namespace str {
  const type: { strict: boolean; defaultValue: string; class: Function; type: string; isConstucted: boolean; isEmpty: boolean };
}
declare function bool(value: any): boolean;
declare namespace bool {
  const type: { strict: boolean; defaultValue: boolean; class: Function; type: string; isConstucted: boolean; isEmpty: boolean };
}
declare function isEmpty(value: any): boolean;
declare function clone(value: any): any;
declare function deepClone(value: any): any;
declare function merge(obj1: any, obj2: any): any;
declare const uniqueId: () => string;
declare function filter(arr: any[], fn: (value: any) => boolean): any[];
declare function reduce(arr: any[], fn: (acc: any, value: any) => any, initial: any): any;
declare function compose(...fns: Function[]): (initialValue: any) => any;
declare function curry(fn: Function): (...args: any[]) => any;
declare function json(thing: string): any;
declare function jsons(thing: any): string;
declare function yaml(thing: any): any;
declare function yamls(thing: any): string;


/**
 * Makes a HTTP request to the specified URL.
 * @param url The URL to request.
 * @param options The options for the request.
 * @returns A promise resolving to the response or other specified output based on the options.
 */
declare function curl(url: string, options: {
  /**
   * Indicates whether to return a promise.
   */
  a: true,
  /**
   * Indicates whether to return the response as plain text.
   */
  text: true,
  o?: string
}): Promise<string>;
/**
 * Makes a HTTP request to the specified URL.
 * @param url The URL to request.
 * @param options The options for the request.
 * @returns A promise resolving to the response or other specified output based on the options.
 */
declare function curl(url: string, options: {
  /**
   * Indicates whether to return a promise.
   */
  a: true,
  /**
   * Indicates whether to return the response as JSON.
   */
  json: true,
  /**
   * The file path to output the response.
   */
  o?: string
}): Promise<object>;
/**
 * Makes a HTTP request to the specified URL.
 * @param url The URL to request.
 * @param options The options for the request.
 * @returns A promise resolving to the response or other specified output based on the options.
 */
declare function curl(url: string, options: {
  /**
   * Indicates whether to return a promise.
   */
  a: true,
  /**
   * The file path to output the response.
   */
  o?: string
}): Promise<Response>;

/**
 * Makes a HTTP request to the specified URL.
 * @param url The URL to request.
 * @param options The options for the request.
 * @returns A promise resolving to the response or other specified output based on the options.
 */
declare function curl(url: string, options?: {
  /**
   * Indicates whether to return a promise.
   */
  a?: boolean,
  /**
   * The file path to output the response.
   */
  o?: string,
  /**
   * Indicates whether to return the response as JSON.
   */
  json?: boolean,
  /**
   * Indicates whether to return the response as plain text.
   */
  text?: boolean
}): ReturnType<typeof future>;




declare function print(...args: any[]): void;
declare namespace print {
  const stdout: WriteStream;
  const stdin: ReadStream;
};

declare function input(prompt: string): string;

declare function genID(len: number = 15, charachters?: string): string;

declare const basename: (path: string) => string;
declare const dirname: (path: string) => string;
declare const extname: (path: string) => string;
declare const pjoin: (...paths: string[]) => string;
declare const presolve: (...paths: string[]) => string;

declare function exports(value: any) : any;

declare function clear() : void;


declare function pub(value: any) : any;
declare function pub(name: string, value: any) : any;

declare const opt: {
  set: (key: string, value: any) => void;
  get: (key: string) => any,
  push: (key: string, value: any) => any,
  pop: (key: string) => any,
}

