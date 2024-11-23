import { dirname } from "path";

export const BASE_DIR = dirname(dirname(import.meta.url.split('file://')[1]));
