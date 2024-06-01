import * as wasm from "./wasmBasics_bg.wasm";
import { __wbg_set_wasm } from "./wasmBasics_bg.js";
__wbg_set_wasm(wasm);
export * from "./wasmBasics_bg.js";
