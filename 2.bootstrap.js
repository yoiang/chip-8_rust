(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[2],{

/***/ "./components/DisassemblerLine.js":
/*!****************************************!*\
  !*** ./components/DisassemblerLine.js ***!
  \****************************************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _Utility__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Utility */ \"./components/Utility.js\");\n\n\nclass DisassemblerLine extends HTMLElement {\n    constructor() {\n      super();\n\n      this.isCurrentLine = false;\n      this.location = 0;\n      this.value = [0];\n      this.disassembly = \"\";\n    }\n  \n    connectedCallback() {\n      this.isCurrentLine = this.getAttribute(\"isCurrentLine\") || false;\n      this.location = this.getAttribute(\"location\") || 0;\n      this.value = this.getAttribute(\"value\") || [0];\n      this.disassembly = this.getAttribute(\"disassembly\") || \"\";\n\n      this.render();\n    }\n\n    update(new_is_current_line, new_location, new_value, new_disassembly) {\n      if (typeof new_is_current_line !== 'boolean') {\n        console.error(\"Unexpected disassembler line is current line type: \", typeof new_is_current_line);\n        return;\n      }\n\n      if (typeof new_location !== 'number') {\n        console.error(\"Unexpected disassembler line location type: \", typeof new_location);\n        return;\n      }\n      \n      if (!Array.isArray(new_value)) {\n        console.error(\"Unexpected disassembler line value type: \", typeof new_value);\n        return;\n      }\n\n      if (typeof new_disassembly !== 'string') {\n        console.error(\"Unexpected disassembler line disassembly type: \", typeof new_disassembly);\n        return;\n      }\n      \n      if (this.isCurrentLine === new_is_current_line && \n          this.location === new_location && \n          Object(_Utility__WEBPACK_IMPORTED_MODULE_0__[\"isArrayContentsEqual\"])(this.value, new_value) &&\n          this.disassembly === new_disassembly) {\n          return;\n      }\n\n      this.isCurrentLine = new_is_current_line;\n      this.location = new_location;\n      this.value = new_value;\n      this.disassembly = new_disassembly;\n\n      this.render();\n    }\n  \n    render() {\n      this.innerHTML = `\n        <div class=\"disassembler_line${this.isCurrentLine ? \" current_disassembler_line\" : \"\"}\">\n            ${Object(_Utility__WEBPACK_IMPORTED_MODULE_0__[\"renderHexValue\"])(this.location, 4)}: ${Object(_Utility__WEBPACK_IMPORTED_MODULE_0__[\"renderHexValue\"])(this.value[0]<< 8 | this.value[1], 4, false)} - <span class=\"disassembly\">${this.disassembly}</span>\n        </div>\n        `;\n    }\n  }\n  \n  customElements.define(\"disassembler-line\", DisassemblerLine);\n\n//# sourceURL=webpack:///./components/DisassemblerLine.js?");

/***/ }),

/***/ "./components/Utility.js":
/*!*******************************!*\
  !*** ./components/Utility.js ***!
  \*******************************/
/*! exports provided: renderNumberValue, renderHexValue, isArrayContentsEqual */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"renderNumberValue\", function() { return renderNumberValue; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"renderHexValue\", function() { return renderHexValue; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"isArrayContentsEqual\", function() { return isArrayContentsEqual; });\nconst renderNumberValue = (name, value, hexLength) => {\n    return `\n        <span class=\"name\">${name}</span>: \n            ${renderHexValue(value, hexLength)} / \n            <span class=\"decimal_value\">${value}</span>\n        `\n}\n\nconst renderHexValue = (value, hexLength, with0x = true) => {\n    return `<span class=\"hex_value\">${with0x ? \"0x\" : \"\"}${value.toString(16).padStart(hexLength, \"0\")}</span>`\n}\n\nconst isArrayContentsEqual = (left, right) => {\n    if (!Array.isArray(left) || !Array.isArray(right)) {\n        // TODO: or throw?\n        return false;\n    }\n\n    if (left.length !== right.length) {\n        return false;\n    }\n\n    for(let index = 0; index < left.length; index++) {\n        if (left[index] !== right[index]) {\n            return false;\n        }\n    }\n\n    return true;\n}\n\n//# sourceURL=webpack:///./components/Utility.js?");

/***/ })

}]);