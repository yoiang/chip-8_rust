(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/chip8_wasm.js":
/*!****************************!*\
  !*** ../pkg/chip8_wasm.js ***!
  \****************************/
/*! exports provided: Index, InterpreterSnapshot, __wbindgen_json_parse, __wbindgen_object_drop_ref, __wbindgen_string_new, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbg_log_9a99fb1af846153b, __wbg_buffer_9e184d6f785de5ed, __wbg_newwithbyteoffsetandlength_e57ad1f2ce812c03, __wbg_new_e8101319e4cf95fc, __wbg_random_29218b0f217b2697, __wbindgen_number_get, __wbindgen_throw, __wbindgen_memory */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./chip8_wasm_bg.wasm */ \"../pkg/chip8_wasm_bg.wasm\");\n/* harmony import */ var _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./chip8_wasm_bg.js */ \"../pkg/chip8_wasm_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"Index\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"Index\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"InterpreterSnapshot\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"InterpreterSnapshot\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_parse\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_json_parse\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_string_new\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_59cb74e423758ede\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_stack_558ba5917b466edd\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_4bb6c2a97407129a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_9a99fb1af846153b\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_log_9a99fb1af846153b\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_buffer_9e184d6f785de5ed\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_buffer_9e184d6f785de5ed\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newwithbyteoffsetandlength_e57ad1f2ce812c03\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_newwithbyteoffsetandlength_e57ad1f2ce812c03\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_e8101319e4cf95fc\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_e8101319e4cf95fc\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_random_29218b0f217b2697\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_random_29218b0f217b2697\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_number_get\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_number_get\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_memory\", function() { return _chip8_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_memory\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/chip8_wasm.js?");

/***/ }),

/***/ "../pkg/chip8_wasm_bg.js":
/*!*******************************!*\
  !*** ../pkg/chip8_wasm_bg.js ***!
  \*******************************/
/*! exports provided: Index, InterpreterSnapshot, __wbindgen_json_parse, __wbindgen_object_drop_ref, __wbindgen_string_new, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbg_log_9a99fb1af846153b, __wbg_buffer_9e184d6f785de5ed, __wbg_newwithbyteoffsetandlength_e57ad1f2ce812c03, __wbg_new_e8101319e4cf95fc, __wbg_random_29218b0f217b2697, __wbindgen_number_get, __wbindgen_throw, __wbindgen_memory */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Index\", function() { return Index; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"InterpreterSnapshot\", function() { return InterpreterSnapshot; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_parse\", function() { return __wbindgen_json_parse; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return __wbindgen_string_new; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return __wbg_new_59cb74e423758ede; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return __wbg_stack_558ba5917b466edd; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return __wbg_error_4bb6c2a97407129a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_9a99fb1af846153b\", function() { return __wbg_log_9a99fb1af846153b; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_buffer_9e184d6f785de5ed\", function() { return __wbg_buffer_9e184d6f785de5ed; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newwithbyteoffsetandlength_e57ad1f2ce812c03\", function() { return __wbg_newwithbyteoffsetandlength_e57ad1f2ce812c03; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_e8101319e4cf95fc\", function() { return __wbg_new_e8101319e4cf95fc; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_random_29218b0f217b2697\", function() { return __wbg_random_29218b0f217b2697; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_number_get\", function() { return __wbindgen_number_get; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_memory\", function() { return __wbindgen_memory; });\n/* harmony import */ var _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./chip8_wasm_bg.wasm */ \"../pkg/chip8_wasm_bg.wasm\");\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction getObject(idx) { return heap[idx]; }\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nfunction isLikeNone(x) {\n    return x === undefined || x === null;\n}\n\nlet cachegetFloat64Memory0 = null;\nfunction getFloat64Memory0() {\n    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetFloat64Memory0 = new Float64Array(_chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetFloat64Memory0;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nfunction passArray8ToWasm0(arg, malloc) {\n    const ptr = malloc(arg.length * 1);\n    getUint8Memory0().set(arg, ptr / 1);\n    WASM_VECTOR_LEN = arg.length;\n    return ptr;\n}\n\nlet cachegetUint32Memory0 = null;\nfunction getUint32Memory0() {\n    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint32Memory0 = new Uint32Array(_chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint32Memory0;\n}\n\nfunction getArrayJsValueFromWasm0(ptr, len) {\n    const mem = getUint32Memory0();\n    const slice = mem.subarray(ptr / 4, ptr / 4 + len);\n    const result = [];\n    for (let i = 0; i < slice.length; i++) {\n        result.push(takeObject(slice[i]));\n    }\n    return result;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nfunction notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }\n/**\n*/\nclass Index {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Index.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_index_free\"](ptr);\n    }\n    /**\n    * @returns {Index}\n    */\n    static new() {\n        var ret = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"index_new\"]();\n        return Index.__wrap(ret);\n    }\n    /**\n    * @param {Uint8Array} program\n    */\n    load(program) {\n        var ptr0 = passArray8ToWasm0(program, _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"index_load\"](this.ptr, ptr0, len0);\n    }\n    /**\n    */\n    update() {\n        _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"index_update\"](this.ptr);\n    }\n    /**\n    * @returns {string}\n    */\n    render_text() {\n        try {\n            const retptr = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n            _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"index_render_text\"](retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            return getStringFromWasm0(r0, r1);\n        } finally {\n            _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n            _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n        }\n    }\n    /**\n    * @param {any} js_index\n    * @returns {boolean}\n    */\n    keydown(js_index) {\n        var ret = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"index_keydown\"](this.ptr, addHeapObject(js_index));\n        return ret !== 0;\n    }\n    /**\n    * @param {any} js_index\n    * @returns {boolean}\n    */\n    keyup(js_index) {\n        var ret = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"index_keydown\"](this.ptr, addHeapObject(js_index));\n        return ret !== 0;\n    }\n    /**\n    * @returns {any[]}\n    */\n    key_state() {\n        try {\n            const retptr = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n            _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"index_key_state\"](retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            var v0 = getArrayJsValueFromWasm0(r0, r1).slice();\n            _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 4);\n            return v0;\n        } finally {\n            _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n        }\n    }\n    /**\n    * @returns {string}\n    */\n    dump_memory() {\n        try {\n            const retptr = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n            _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"index_dump_memory\"](retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            return getStringFromWasm0(r0, r1);\n        } finally {\n            _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n            _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n        }\n    }\n    /**\n    * @returns {string}\n    */\n    dump_program_counter() {\n        try {\n            const retptr = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n            _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"index_dump_program_counter\"](retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            return getStringFromWasm0(r0, r1);\n        } finally {\n            _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n            _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n        }\n    }\n    /**\n    * @param {number} count_before\n    * @param {number} count_after\n    * @returns {InterpreterSnapshot}\n    */\n    create_interpreter_snapshot(count_before, count_after) {\n        var ret = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"index_create_interpreter_snapshot\"](this.ptr, count_before, count_after);\n        return InterpreterSnapshot.__wrap(ret);\n    }\n}\n/**\n*/\nclass InterpreterSnapshot {\n\n    static __wrap(ptr) {\n        const obj = Object.create(InterpreterSnapshot.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_interpretersnapshot_free\"](ptr);\n    }\n    /**\n    * @returns {number}\n    */\n    get program_counter_position() {\n        var ret = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_interpretersnapshot_program_counter_position\"](this.ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @param {number} arg0\n    */\n    set program_counter_position(arg0) {\n        _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_interpretersnapshot_program_counter_position\"](this.ptr, arg0);\n    }\n    /**\n    * @returns {number}\n    */\n    get index_register_value() {\n        var ret = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_interpretersnapshot_index_register_value\"](this.ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @param {number} arg0\n    */\n    set index_register_value(arg0) {\n        _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_interpretersnapshot_index_register_value\"](this.ptr, arg0);\n    }\n    /**\n    * @returns {number}\n    */\n    get delay_timer_value() {\n        var ret = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_interpretersnapshot_delay_timer_value\"](this.ptr);\n        return ret;\n    }\n    /**\n    * @param {number} arg0\n    */\n    set delay_timer_value(arg0) {\n        _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_interpretersnapshot_delay_timer_value\"](this.ptr, arg0);\n    }\n    /**\n    * @returns {number}\n    */\n    get sound_timer_value() {\n        var ret = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_interpretersnapshot_sound_timer_value\"](this.ptr);\n        return ret;\n    }\n    /**\n    * @param {number} arg0\n    */\n    set sound_timer_value(arg0) {\n        _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_interpretersnapshot_sound_timer_value\"](this.ptr, arg0);\n    }\n    /**\n    * @returns {Uint8Array}\n    */\n    get variable_register_values() {\n        var ret = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"interpretersnapshot_variable_register_values\"](this.ptr);\n        return takeObject(ret);\n    }\n    /**\n    * @returns {any}\n    */\n    get partial_disassemble() {\n        var ret = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"interpretersnapshot_partial_disassemble\"](this.ptr);\n        return takeObject(ret);\n    }\n}\n\nfunction __wbindgen_json_parse(arg0, arg1) {\n    var ret = JSON.parse(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\nfunction __wbindgen_string_new(arg0, arg1) {\n    var ret = getStringFromWasm0(arg0, arg1);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_new_59cb74e423758ede() {\n    var ret = new Error();\n    return addHeapObject(ret);\n};\n\nfunction __wbg_stack_558ba5917b466edd(arg0, arg1) {\n    var ret = getObject(arg1).stack;\n    var ptr0 = passStringToWasm0(ret, _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbg_error_4bb6c2a97407129a(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](arg0, arg1);\n    }\n};\n\nfunction __wbg_log_9a99fb1af846153b(arg0) {\n    console.log(getObject(arg0));\n};\n\nfunction __wbg_buffer_9e184d6f785de5ed(arg0) {\n    var ret = getObject(arg0).buffer;\n    return addHeapObject(ret);\n};\n\nfunction __wbg_newwithbyteoffsetandlength_e57ad1f2ce812c03(arg0, arg1, arg2) {\n    var ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_new_e8101319e4cf95fc(arg0) {\n    var ret = new Uint8Array(getObject(arg0));\n    return addHeapObject(ret);\n};\n\nconst __wbg_random_29218b0f217b2697 = typeof Math.random == 'function' ? Math.random : notDefined('Math.random');\n\nfunction __wbindgen_number_get(arg0, arg1) {\n    const obj = getObject(arg1);\n    var ret = typeof(obj) === 'number' ? obj : undefined;\n    getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;\n    getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);\n};\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\nfunction __wbindgen_memory() {\n    var ret = _chip8_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"];\n    return addHeapObject(ret);\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/chip8_wasm_bg.js?");

/***/ }),

/***/ "../pkg/chip8_wasm_bg.wasm":
/*!*********************************!*\
  !*** ../pkg/chip8_wasm_bg.wasm ***!
  \*********************************/
/*! exports provided: memory, __wbg_index_free, index_new, index_load, index_update, index_render_text, index_keydown, index_key_state, index_dump_memory, index_dump_program_counter, __wbg_interpretersnapshot_free, __wbg_get_interpretersnapshot_program_counter_position, __wbg_set_interpretersnapshot_program_counter_position, __wbg_get_interpretersnapshot_index_register_value, __wbg_set_interpretersnapshot_index_register_value, __wbg_get_interpretersnapshot_delay_timer_value, __wbg_set_interpretersnapshot_delay_timer_value, __wbg_get_interpretersnapshot_sound_timer_value, __wbg_set_interpretersnapshot_sound_timer_value, interpretersnapshot_variable_register_values, interpretersnapshot_partial_disassemble, index_create_interpreter_snapshot, index_keyup, __wbindgen_malloc, __wbindgen_add_to_stack_pointer, __wbindgen_free, __wbindgen_realloc */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./chip8_wasm_bg.js */ \"../pkg/chip8_wasm_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/chip8_wasm_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _pkg_chip8_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../pkg/chip8_wasm */ \"../pkg/chip8_wasm.js\");\n/* harmony import */ var _utility__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./utility */ \"./utility.js\");\n\n\n\nconst pre = document.getElementById(\"chip8_render-canvas\");\n\nlet index = null;\nlet indexReady = false;\nlet programsList = null;\nlet programsListReady = false;\nlet isPaused = false;\n\nconst updateElementById = (id, valueOrFunction) => {\n    const element = document.getElementById(id);\n    if (!element) {\n        console.error(\"Unable to update element with id '\" + id + \"', cannot find it\");\n        return;\n    }\n    if (typeof element.update !== 'function') {\n        console.error(\"Unable to update element with id '\" + id + \"', does not contain an update function\")\n        return;\n    }\n\n    if (typeof valueOrFunction === 'function') {\n        valueOrFunction(element);\n    } else {\n        element.update(valueOrFunction);\n    }\n}\n\nconst updateSnapshot = () => {\n    let snapshot = index.create_interpreter_snapshot(6, 6);\n\n    updateElementById(\"program_counter\", snapshot.program_counter_position);\n    updateElementById(\"index_register\", snapshot.index_register_value);\n    updateElementById(\"variable_registers\", snapshot.variable_register_values);\n    updateElementById(\"delay_timer\", snapshot.delay_timer_value);\n    updateElementById(\"sound_timer\", snapshot.sound_timer_value);\n    updateElementById(\"partial_disassembler\", (element) => {\n        element.update(snapshot.program_counter_position, snapshot.partial_disassemble)\n    });\n}\n\nconst renderLoop = () => {\n    index.update();\n    pre.textContent = index.render_text();\n\n    updateSnapshot();\n  \n    if (!isPaused) {\n        requestAnimationFrame(renderLoop); \n    }\n};\n\nconst togglePause = () => {\n    isPaused = !isPaused;\n    if (!isPaused && indexReady) {\n        requestAnimationFrame(renderLoop);\n    }\n}\n\nconst step = () => {\n    if (!isPaused) {\n        isPaused = true;\n    } else {\n        renderLoop();\n    }\n}\n\nconst handleKeydownEvent = (event) => {\n    const key = event.key;\n    const key_index = Object(_utility__WEBPACK_IMPORTED_MODULE_1__[\"mapKeyEventCodeToKeypadIndex\"])(key);\n    const result = index.keydown(key_index);\n}\n\nconst handleKeyupEvent = (event) => {\n    const key = event.key;\n    const key_index = Object(_utility__WEBPACK_IMPORTED_MODULE_1__[\"mapKeyEventCodeToKeypadIndex\"])(key);\n    const result = index.keyup(key_index);\n}\n\nconst setIndex = (newIndex) => {\n    if (indexReady) {\n        console.error(\"setIndex called by index already set: \", newIndex);\n        return;\n    }\n\n    index = newIndex;\n    indexReady = true;\n\n    document.addEventListener('keydown', handleKeydownEvent);\n    document.addEventListener('keyup', handleKeyupEvent);\n\n    const pauseToggleElement = document.getElementById(\"play_pause\");\n    pauseToggleElement.onclick = togglePause;\n\n    const stepElement = document.getElementById(\"step\");\n    stepElement.onclick = step;\n\n    requestAnimationFrame(renderLoop);\n};\n\nconst loadProgram = (fileName) => {\n    let result = \"\";\n    return fetch('./' + fileName)\n        .then((response) => {\n            if (response.status !== 200) {\n                throw new Error('Looks like there was a problem. Status Code: ' + response.status);\n            }\n\n            let reader = response.body.getReader();\n            return reader.read()\n                .then(function processText({ done, value }) {\n                    if (done) {\n                        return;\n                    }\n                \n                    const chunk = value;\n                    result += chunk;\n                \n                    return reader.read().then(processText);\n                });\n        })\n        .then(() => {\n            let bytes = result.split(',');\n            index.load(bytes);\n        })\n        .catch((error) => {\n            console.error('While loading loading program: ', error);\n        });\n}\n\nconst getProgramsListElement = () => {\n    return document.querySelector('#app-programs');\n}\n\nconst createProgramsListOption = (programsListItem) => {\n    let result = document.createElement('option');\n    result.value = programsListItem.title;\n    result.innerHTML = programsListItem.title;\n    return result;\n}\n\nconst programsListElementSelectedIndex = (index) => {\n    // TODO: check out of bounds\n    const program = programsList[index]\n    loadProgram(`./${program.file}`);\n}\n\nconst programsListElementChanged = (event) => {\n    programsListElementSelectedIndex(event.target.selectedIndex);\n    document.activeElement.blur();\n}\n\nconst setProgramsList = (newProgramsList) => {\n    if (!Array.isArray(newProgramsList)) {\n        throw new Error(`Expected program list to be an array, received '${typeof newProgramsList}'`);\n    }\n    if (newProgramsList.length == 0) {\n        throw new Error(`Updated program list is empty`);\n    }\n\n    let programsListElement = getProgramsListElement();\n    while (programsListElement.options.length > 0) {                \n        programsListElement.remove(programsListElement.options.length - 1);\n    } \n\n    for (let index = 0; index < newProgramsList.length; index ++) {\n        let programsListItem = newProgramsList[index];\n        const option = createProgramsListOption(programsListItem);\n        programsListElement.appendChild(option);\n    }\n\n    programsListElement.onchange = programsListElementChanged;\n    programsListElement.selectedIndex = 0;\n\n    programsList = newProgramsList;\n    programsListReady = true;\n\n    programsListElementSelectedIndex(programsListElement.selectedIndex);\n}\n\nconst loadProgramList = () => {\n    return fetch('./programs.json')\n        .then((response) => {\n            if (response.status !== 200) {\n                console.log('Looks like there was a problem. Status Code: ' + response.status);\n                return;\n            }\n\n            return response.json();\n        })\n        .then((json) => {\n            setProgramsList(json);\n        })\n        .catch((error) => {\n            console.error(\"While loading program list: \", error);\n        });\n}\n\nloadProgramList();\nconst newValue = _pkg_chip8_wasm__WEBPACK_IMPORTED_MODULE_0__[\"Index\"].new();\nif (newValue && typeof newValue.then == 'function') {\n    newValue\n        .then(setIndex)\n        .catch((error) => {\n            console.error(\"While waiting for Index.new() -> Promise to finish: \", error);\n        });\n} else {\n    setIndex(newValue);\n}\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ }),

/***/ "./utility.js":
/*!********************!*\
  !*** ./utility.js ***!
  \********************/
/*! exports provided: mapKeyEventCodeToKeypadIndex */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"mapKeyEventCodeToKeypadIndex\", function() { return mapKeyEventCodeToKeypadIndex; });\n\n// TODO: flexible key mapping\nconst mapKeyEventCodeToKeypadIndex = (key) => {\n    switch(key) {\n        case \"1\":\n        return 0;\n\n        case \"2\":\n        return 1;\n\n        case \"3\":\n        return 2;\n\n        case \"4\":\n        return 3;\n\n\n        case \"q\":\n        return 4;\n\n        case \"w\":\n        return 5;\n\n        case \"e\":\n        return 6;\n\n        case \"r\":\n        return 7;\n\n\n        case \"a\":\n        return 8;\n\n        case \"s\":\n        return 9;\n\n        case \"d\":\n        return 10;\n\n        case \"f\":\n        return 11;\n\n\n        case \"z\":\n        return 12;\n\n        case \"x\":\n        return 13;\n\n        case \"c\":\n        return 14;\n\n        case \"v\":\n        return 15;\n    }\n}\n\n//# sourceURL=webpack:///./utility.js?");

/***/ })

}]);