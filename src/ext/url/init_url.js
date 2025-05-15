import * as url from 'ext:deno_url/00_url.js';
import * as urlPattern from 'ext:deno_url/01_urlpattern.js';

globalThis.URL = url.URL;
globalThis.URLPattern = urlPattern.URLPattern;
globalThis.URLSearchParams = url.URLSearchParams;
