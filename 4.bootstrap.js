(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[4],{

/***/ "./components/ProgramCounter.js":
/*!**************************************!*\
  !*** ./components/ProgramCounter.js ***!
  \**************************************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _Utility__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Utility */ \"./components/Utility.js\");\n\n\nclass ProgramCounter extends HTMLElement {\n    constructor() {\n      super();\n\n      this.position = 0;\n    }\n  \n    connectedCallback() {\n      this.render();\n    }\n\n    update(new_position) {\n        if (this.position == new_position) {\n            return;\n        }\n        \n        if (typeof new_position != 'number') {\n            console.error(\"Unexpected position counter position type: \", typeof new_value);\n            return;\n        }\n\n        this.position = new_position;\n\n        this.render();\n    }\n  \n    render() {\n      this.innerHTML = `\n        <div class=\"contents\">\n          ${Object(_Utility__WEBPACK_IMPORTED_MODULE_0__[\"renderNumberValue\"])(\"PROGRAM\", this.position, 4)}\n        </div>\n        `;\n    }\n  }\n  \n  customElements.define(\"program-counter\", ProgramCounter);\n\n//# sourceURL=webpack:///./components/ProgramCounter.js?");

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