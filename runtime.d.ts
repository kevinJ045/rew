

interface ImportOptions {
  /**
   * Determines how to import the given module
   */
  type: "js" | "coffee" | "yaml" | "json" | "qrew" | "text";
  [key: string]: any;
}

interface ModuleConfOptionCenter {
  /**
   * Get a config key
   * @param key The key of the config to get
   * @param defaultValue The default value ig null
   * @returns The value of the key or the defaultValue if it's null.
   */
  get: <T = any>(key: string, defaultValue?: T) => T;
  /**
   * Set a config key
   * @param key The key of the config to set
   * @param value The value to set it to
   * @returns true if it was a success
   */
  set: <T = any>(key: string, value: T) => boolean;
  /**
   * Removes a key from the config
   * @param key The key of the config to remove
   * @returns true if it was a success
   */
  remove: (key: string) => boolean;
  /**
   * Resets the entire config option center to it's default value
   */
  reset: () => boolean;
  /**
   * Get all values in an option center
   * @param str
   * @returns
   */
  getAll: (() => string) | ((str?: false) => Record<string, any>);
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
  staticFile: (
    name: string,
    defaults?: any
  ) => {
    write: (value: any, ifExists?: boolean) => any;
    // @ts-ignore
    read: (to?: string | object) => string | object | Buffer;
    fileRoot: string;
    exists: boolean;
  };
}

interface ModuleEnv {
  has: (key: string) => boolean;
  get: (key: string) => string;
  set: (key: string, value: string) => boolean;
  rm: (key: string) => boolean;
  is: (key: string, value: string) => boolean;
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
  collection: (name: string) => ModuleRuneDBCollcetion;
  map: (name: string) => ModuleRuneDBMap;
  findRef: (ref: string) => any;
  setData: (data: Record<string, any>) => void;
  getData: () => Record<string, any>;
  makeRef(value: object, props?: string): string | null;
}

interface ModuleRune {
  db(dbname: string, data?: object, encryptionKey?: string): ModuleRuneDB;
  genKey(secret: string): string;
  push(...values: any[]): any;
  pop(...values: any[]): any;
}

interface ModuleThreads {
  thread: (cb: Function) => {
    stopAll: () => void;
    start: (context: Record<string, any>) => {
      on: (event: string, callback: (data) => void) => void;
      off: (event: string, callback: (data) => void) => void;
      emit: (event: string, data: any) => void;
      get: () => Promise<any>;
      stop: () => void;
    };
  };
}


type _Request = typeof globalThis extends { onmessage: any } ? {} : import("undici-types").Request;
type _Response = typeof globalThis extends { onmessage: any } ? {} : import("undici-types").Response;
type _FormData = typeof globalThis extends { onmessage: any } ? {} : import("undici-types").FormData;
type _Headers = typeof globalThis extends { onmessage: any } ? {} : import("undici-types").Headers;
type _RequestInit = typeof globalThis extends { onmessage: any } ? {}
    : import("undici-types").RequestInit;
type _ResponseInit = typeof globalThis extends { onmessage: any } ? {}
    : import("undici-types").ResponseInit;
// @ts-ignore
type _File = typeof globalThis extends { onmessage: any } ? {} : import("node:buffer").File;

interface Request {}
declare var Request: typeof globalThis extends {
    onmessage: any;
    Request: infer T;
} ? T
    : typeof import("undici-types").Request;

interface ResponseInit extends _ResponseInit {}

interface Response extends _Response {}
declare var Response: typeof globalThis extends {
    onmessage: any;
    Response: infer T;
} ? T
    : typeof import("undici-types").Response;

type GenericTraps = Record<string, any>;

type IRequestStrict = {
  route: string;
  params: {
    [key: string]: string;
  };
  query: {
    [key: string]: string | string[] | undefined;
  };
  proxy?: any;
} & Request;

type IRequest = IRequestStrict & GenericTraps;

type RequestHandler<RequestType = IRequest, Args extends Array<any> = any[]> = (
  request: RequestType,
  ...args: Args
) => any;

type ResponseHandler<
  ResponseType = any,
  RequestType = IRequest,
  Args extends any[] = any[]
> = (response: ResponseType, request: RequestType, ...args: Args) => any;

type StatusErrorObject = {
  error?: string;
  [key: string]: any;
};

interface StatusError extends Error {
  status: number;
  [key: string]: any;
  constructor(status?: number, body?: StatusErrorObject | string);
}

type ErrorHandler<
  ErrorType extends Error = StatusError,
  RequestType = IRequest,
  Args extends any[] = any[]
> = (error: ErrorType, request: RequestType, ...args: Args) => any;

type RouteEntry<RequestType = IRequest, Args extends any[] = any[]> = [
  httpMethod: string,
  match: RegExp,
  handlers: RequestHandler<RequestType, Args>[],
  path?: string
];

type IttyRouterOptions = {
  base?: string;
  routes?: RouteEntry[];
} & GenericTraps;

type RouterOptions<RequestType = IRequest, Args extends any[] = []> = {
  before?: RequestHandler<RequestType, Args>[];
  catch?: ErrorHandler<StatusError, RequestType, Args>;
  finally?: ResponseHandler<any, RequestType, Args>[];
} & IttyRouterOptions;

type AutoRouterOptions<RequestType, Args extends any[]> = {
  missing?: RequestHandler<RequestType, Args>;
  format?: ResponseHandler;
} & RouterOptions<RequestType, Args>;

type RequestLike = {
  method: string;
  url: string;
} & GenericTraps;

type Route<R = IRequest, A extends Array<any> = any[]> = <
  RequestType = R,
  Args extends Array<any> = A
>(
  path: string,
  ...handlers: RequestHandler<RequestType, Args>[]
) => IttyRouterType<R, A>;

type CustomRoutes<R = Route> = {
  [key: string]: R;
};

type IttyRouterType<
  RequestType = IRequest,
  Args extends any[] = any[],
  ResponseType = any
> = {
  __proto__: IttyRouterType<RequestType, Args, ResponseType>;
  routes: RouteEntry[];
  fetch: <A extends any[] = Args>(
    request: RequestLike,
    ...extra: A
  ) => Promise<ResponseType>;
  all: Route<RequestType, Args>;
  delete: Route<RequestType, Args>;
  get: Route<RequestType, Args>;
  head: Route<RequestType, Args>;
  options: Route<RequestType, Args>;
  patch: Route<RequestType, Args>;
  post: Route<RequestType, Args>;
  put: Route<RequestType, Args>;
} & CustomRoutes<Route<RequestType, Args>> &
  GenericTraps;

type RouterType<
  RequestType = IRequest,
  Args extends any[] = any[],
  ResponseType = any
> = {
  before?: RequestHandler<RequestType, Args>[];
  catch?: ErrorHandler<StatusError, RequestType, Args>;
  finally?: ResponseHandler<any, RequestType, Args>[];
} & IttyRouterType<RequestType, Args, ResponseType>;

type AutoRouterType<
  RequestType = IRequest,
  Args extends any[] = any[],
  ResponseType = any
> = {
  missing?: RequestHandler<RequestType, Args>;
  format?: ResponseHandler;
} & RouterType<RequestType, Args, ResponseType>;

type HasContent<ContentType> = {
  content: ContentType;
} & IRequestStrict;

type ResponseFormatter = (body?: any, options?: ResponseInit) => Response;

interface ErrorLike extends Error {
  status?: number;
  [any: string]: any;
}
type ErrorBody = string | object;
interface ErrorFormatter {
  (statusCode?: number, body?: ErrorBody): Response;
}

type IttyRouter = <
  RequestType extends IRequest = IRequest,
  Args extends any[] = any[],
  ResponseType = any
>({
  base,
  routes,
  ...other
}?: IttyRouterOptions) => IttyRouterType<RequestType, Args, ResponseType>;

type Router = <
  RequestType = IRequest,
  Args extends any[] = any[],
  ResponseType = any
>({
  base,
  routes,
  ...other
}?: RouterOptions<RequestType, Args>) => RouterType<
  RequestType,
  Args,
  ResponseType
>;

type AutoRouter = <
  RequestType extends IRequest = IRequest,
  Args extends any[] = any[],
  ResponseType = any
>({
  format,
  missing,
  finally: f,
  before,
  ...options
}?: AutoRouterOptions<RequestType, Args>) => AutoRouterType<
  RequestType,
  Args,
  ResponseType
>;

type createResponse = (
  format?: string,
  transform?: ((body: any) => any) | undefined
) => ResponseFormatter;

type statusR = (status: number, options?: ResponseInit) => Response;

type withContent = (request: IRequest) => Promise<void>;

type withCookies = (r: IRequest) => void;

type withParams = (request: IRequest) => void;

type CorsOptions = {
  credentials?: true;
  origin?:
    | boolean
    | string
    | string[]
    | RegExp
    | ((origin: string) => string | void);
  maxAge?: number;
  allowMethods?: string | string[];
  allowHeaders?: any;
  exposeHeaders?: string | string[];
};
type Preflight = (request: IRequest) => Response | void;
type Corsify = (response: Response, request?: IRequest) => Response;
type CorsPair = {
  preflight: Preflight;
  corsify: Corsify;
};
type cors = (options?: CorsOptions) => {
  corsify: (response: Response, request?: Request) => Response;
  preflight: (request: Request) => Response | undefined;
};

interface ModuleServeRouter extends RouterType {
  to(server: ModuleServeServer): any;
}

interface ResponseType{}

declare class ModuleServeServerOptions {
  handler?: (req: RequestLike, res: ResponseType) => any;
  routers?: ModuleServeRouter[];
  fetch?: (req: RequestLike) => ResponseType | Promise<ResponseType>;
}

interface ModuleServeServer {
  _server: any;
  routers: Record<string, ModuleServeRouter>;

  handleRequest: typeof ModuleServeServerOptions.prototype.handler;
  listen: this;
  port: (port: number) => this;
  log: (string: string) => this;
}

interface ModuleServeFileRouterOptions {
  root: string;
  basePath?: string;
  resolveExtensions?: string[];
  bundlerOptions?: Record<string, any>;
  bundlerEntry?: (file: string, layouts?: string[]) => string;
  ssrBundlerEntry?: (file: string, layouts?: string[]) => string;
  onError?: () => any;
}

declare class ModuleServe {
  router({
    id,
    type,
  }: {
    id?: string;
    type?: "auto" | "normal" | "default";
    [key: string]: any;
  }): ModuleServeRouter;
  create(o: ModuleServeServerOptions): ModuleServeServer;

  createFileRouter(
    o: ModuleServeFileRouterOptions
  ): (req: RequestLike) => ResponseType | Promise<ResponseType>;

  cors: cors;
  json: ResponseFormatter;
  error: ResponseFormatter;
  png: ResponseFormatter;
  jpeg: ResponseFormatter;
  webp: ResponseFormatter;
  text: ResponseFormatter;
  html: ResponseFormatter;
  status: statusR;

  createResponse: createResponse;

  withContent: withContent;
  withCookies: withCookies;
  withParams: withParams;

  // @ts-ignore
  Request = Request;
  // @ts-ignore
  Response = Response;
}

// @ts-ignore
type nodable = Element | Node | any;
interface Node {
  type: string;
  props: {
    children: nodable[];
    [key: string]: any;
  };
}
interface ElementNode extends Node {}

interface ModuleWebPageOptions {
  viewportMeta?: string | boolean;
  title?: string | boolean;
}

interface ModuleWebPage {
  root: nodable;
  body: nodable;
  head: nodable;

  find(key: string, value?: string): Node;
  add(...children: nodable[]): typeof this.body;

  script(script: string): ReturnType<typeof this.add>;

  serializeState(): string;

  render(static?: boolean): string;

  clone(): ModuleWebPage;
}

interface ModuleWebState {
  value: any;
  _value: any;
  subscribe(callback: CallableFunction): this;
}

declare class ModuleWeb {
  create(options: ModuleWebPageOptions): ModuleWebPage;
  isNode(node: any): boolean;
  isTextNode(node: any): boolean;
  isElementNode(node: any): boolean;

  createText(text: string): Node;
  createElement(...args: any[]): ElementNode;

  state(value): ModuleWebState | any;
  // @ts-ignore
  invokeState(states: State[], callback: CallableFunction): any;

  bundle(filePath: string, options?: Record<string, any>): string;
}

declare class Stack<T = any> {
  constructor();
  push(item: T): void;
  pop(): T | undefined;
  isEmpty(): boolean;
  peek(): T | undefined;
  toArray(): T[];
}

declare class Queue<T = any> {
  constructor();
  enqueue(item: T): void;
  dequeue(): T | undefined;
  isEmpty(): boolean;
  peek(): T | undefined;
  toArray(): T[];
}

declare class LinkedList<T = any> {
  constructor();
  append(value: T): void;
  prepend(value: T): void;
  find(value: T): LinkedList.Node<T> | null;
  delete(value: T): void;
  toArray(): T[];
}

declare namespace LinkedList {
  class Node<T = any> {
    constructor(value: T);
    value: T;
    next: Node<T> | null;
  }
}

declare class BinaryTree<T = any> {
  constructor();
  insert(value: T): void;
  find(value: T): BinaryTree.Node<T> | null;
  toArray(): T[];
}

declare namespace BinaryTree {
  class Node<T = any> {
    constructor(value: T);
    value: T;
    left: Node<T> | null;
    right: Node<T> | null;
  }
}

declare class DoublyLinkedList<T = any> {
  constructor();
  append(value: T): void;
  prepend(value: T): void;
  find(value: T): DoublyLinkedList.Node<T> | null;
  delete(value: T): void;
  toArray(): T[];
}

declare namespace DoublyLinkedList {
  class Node<T = any> {
    constructor(value: T);
    value: T;
    next: Node<T> | null;
    prev: Node<T> | null;
  }
}

declare interface ModuleData {
  Stack: typeof Stack;
  Queue: typeof Queue;
  BinaryTree: typeof BinaryTree;
  DoublyLinkedList: typeof DoublyLinkedList;
  LinkedList: typeof LinkedList;
}

interface ModuleStream {
  // @ts-ignore
  Readable: Readable,
  // @ts-ignore
  Writable: Writable,
  // @ts-ignore
  Transform: Transform,
  // @ts-ignore
  Duplex: Duplex,
  // @ts-ignore
  pipeline: pipeline,
  // @ts-ignore
  finished: finished
}

declare function imp(path: "conf", options?: ImportOptions): ModuleConf;
declare function imp(path: "env", options?: ImportOptions): ModuleEnv;
declare function imp(path: "rune", options?: ImportOptions): ModuleRune;
declare function imp(path: "threads", options?: ImportOptions): ModuleThreads;
declare function imp(
  path: "serve",
  options?: ImportOptions
): typeof ModuleServe;
declare function imp(path: "web", options?: ImportOptions): typeof ModuleWeb;
declare function imp(path: "data", options?: ImportOptions): ModuleData;
declare function imp(path: "stream", options?: ImportOptions): ModuleStream;
declare function imp(path: string, options?: ImportOptions): any;

declare const inc: typeof imp;

// @ts-ignore
declare function require(moduleName: string): any;

interface Module {
  exports: any;
  filepath: string;
  main: boolean;
  impots: string[];
  compiled: string;
}

// @ts-ignore
declare const module: Module;

interface Imports {
  meta: {
    // @ts-ignore
    url: URL,
    main: boolean
  };
  assets: any;
}

declare const imports: Imports;

// @ts-ignore
declare const process: {
  argv: string[];
  target: ReturnType<typeof emitter>;
  __execFile: string;
  env: Record<string, any>;
  cwd: () => string;
  arch: string;
  exit: () => void;
};

interface AppConfig {
  manifest: {
    package: string;
    [key: string]: any;
  };
}

declare const app: {
  path: string;
  config: AppConfig;
};

declare function read(filepath: string, options?: { encoding: string }): string;

declare function realpath(
  filepath: string,
  options?: { encoding: string }
): string;

declare function write(filepath: string, content: any, options?: any): void;

declare function exists(filepath: string, options?: any): boolean;

declare function fstat(filepath: string, options?: any): any;

declare function rm(filepath: string, options?: any): void;

declare function chmod(filepath: string, mode: any, options?: any): void;

declare function mkdir(filepath: string, options?: any): void;

declare function ls(filepath: string, options?: any): string[];

declare function struct(template: {
  [key: string]: any;
}): (...args: any[]) => any;

declare function future(
  callback: (resolve: (data: any) => void, reject: (data: any) => void) => void,
  timeout?: number,
  defData?: any
): {
  pipe(callback: (data: any) => any): Promise<any>;
  last(callback: (data: any) => any): Promise<any>;
  catch(callback: (data: any) => any): Promise<any>;
  resolve(data: any): void;
  reject(data: any): void;
  wait(): Promise<any>;
};
declare namespace future {
  function promise(
    promse: Promise<any>,
    timeout?: number,
    defData?: any
  ): ReturnType<typeof future>;
}

declare function emitter(): {
  on(
    event: string | string[],
    callback: (...args: any[]) => void,
    props?: {}
  ): ReturnType<typeof emitter>;
  off(
    event: string | string[],
    callback: (...args: any[]) => void,
    removable?: (event: any) => void
  ): ReturnType<typeof emitter>;
  emit(event: string | string[], ...data: any[]): ReturnType<typeof emitter>;
};
declare function exec(command: string, options?: { output?: boolean }): any;
declare namespace exec {
  function background(
    command: string,
    options?: any,
    callback?: (...args: any[]) => void
  ): any;
}
declare function spawn(command: string, ...args: any[]): any;

declare function typedef(
  value: any,
  strict?: boolean
): {
  strict: boolean;
  defaultValue: any;
  class: Function;
  type: string;
  isConstucted: boolean;
  isEmpty: boolean;
};

declare function typeis(obj: any, typeDef: any): boolean;

declare function typex(child: any, parent: any): boolean;

declare function typei(child: any, parent: any): boolean;

declare function int(v: any): number;

declare namespace int {
  const type: {
    strict: boolean;
    defaultValue: number;
    class: Function;
    type: string;
    isConstucted: boolean;
    isEmpty: boolean;
  };
}
declare function float(v: any): number;
declare namespace float {
  const type: {
    strict: boolean;
    defaultValue: number;
    class: Function;
    type: string;
    isConstucted: boolean;
    isEmpty: boolean;
  };
}
declare function num(v: any): number;
declare namespace num {
  const type: {
    strict: boolean;
    defaultValue: number;
    class: Function;
    type: string;
    isConstucted: boolean;
    isEmpty: boolean;
  };
}
declare function str(str: any): string;
declare namespace str {
  const type: {
    strict: boolean;
    defaultValue: string;
    class: Function;
    type: string;
    isConstucted: boolean;
    isEmpty: boolean;
  };
}
declare function bool(value: any): boolean;
declare namespace bool {
  const type: {
    strict: boolean;
    defaultValue: boolean;
    class: Function;
    type: string;
    isConstucted: boolean;
    isEmpty: boolean;
  };
}
declare function isEmpty(value: any): boolean;
declare function clone(value: any): any;
declare function deepClone(value: any): any;
declare function merge(obj1: any, obj2: any): any;
declare const uniqueId: () => string;
declare function filter(arr: any[], fn: (value: any) => boolean): any[];
declare function reduce(
  arr: any[],
  fn: (acc: any, value: any) => any,
  initial: any
): any;
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
declare function curl(
  url: string,
  options: {
    /**
     * Indicates whether to return a promise.
     */
    a: true;
    /**
     * Indicates whether to return the response as plain text.
     */
    text: true;
    o?: string;
  }
): Promise<string>;
/**
 * Makes a HTTP request to the specified URL.
 * @param url The URL to request.
 * @param options The options for the request.
 * @returns A promise resolving to the response or other specified output based on the options.
 */
declare function curl(
  url: string,
  options: {
    /**
     * Indicates whether to return a promise.
     */
    a: true;
    /**
     * Indicates whether to return the response as JSON.
     */
    json: true;
    /**
     * The file path to output the response.
     */
    o?: string;
  }
): Promise<object>;
/**
 * Makes a HTTP request to the specified URL.
 * @param url The URL to request.
 * @param options The options for the request.
 * @returns A promise resolving to the response or other specified output based on the options.
 */
declare function curl(
  url: string,
  options: {
    /**
     * Indicates whether to return a promise.
     */
    a: true;
    /**
     * The file path to output the response.
     */
    o?: string;
  }
  // @ts-ignore
): Promise<Response>;

/**
 * Makes a HTTP request to the specified URL.
 * @param url The URL to request.
 * @param options The options for the request.
 * @returns A promise resolving to the response or other specified output based on the options.
 */
declare function curl(
  url: string,
  options?: {
    /**
     * Indicates whether to return a promise.
     */
    a?: boolean;
    /**
     * The file path to output the response.
     */
    o?: string;
    /**
     * Indicates whether to return the response as JSON.
     */
    json?: boolean;
    /**
     * Indicates whether to return the response as plain text.
     */
    text?: boolean;
  }
): ReturnType<typeof future>;

declare function print(...args: any[]): void;
declare namespace print {
  // @ts-ignore
  const stdout: WriteStream;
  // @ts-ignore
  const stdin: ReadStream;
}

declare function input(prompt: string): string;

declare const basename: (path: string) => string;
declare const dirname: (path: string) => string;
declare const extname: (path: string) => string;
declare const pjoin: (...paths: string[]) => string;
declare const presolve: (...paths: string[]) => string;

// @ts-ignore
declare function exports(value: any): any;


declare function pub(value: any): any;
declare function pub(name: string, value: any): any;

declare const opt: {
  set: (key: string, value: any) => void;
  get: (key: string) => any;
  push: (key: string, value: any) => any;
  pop: (key: string) => any;
};

declare const JSX: any;
declare const TYPES: any;
declare const DECORATORS: any;
declare function using(fn: any, ...args: any[]): any;

declare function wait(fn: CallableFunction, ...args: any[]): any;
declare function clear(): void;

declare class Usage<T = () => void> {
  name: string;
  trigger: T;
  constructor(name: string, trigger: T);
  create(name: string, trigger: T): Usage;
}