(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[6],{

/***/ "./components/Utility.js":
/*!*******************************!*\
  !*** ./components/Utility.js ***!
  \*******************************/
/*! exports provided: renderNumberValue, renderHexValue, isArrayContentsEqual */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"renderNumberValue\", function() { return renderNumberValue; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"renderHexValue\", function() { return renderHexValue; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"isArrayContentsEqual\", function() { return isArrayContentsEqual; });\nconst renderNumberValue = (name, value, hexLength) => {\n    return `\n        <span class=\"name\">${name}</span>: \n            ${renderHexValue(value, hexLength)} / \n            <span class=\"decimal_value\">${value}</span>\n        `\n}\n\nconst renderHexValue = (value, hexLength, with0x = true) => {\n    return `<span class=\"hex_value\">${with0x ? \"0x\" : \"\"}${value.toString(16).padStart(hexLength, \"0\")}</span>`\n}\n\nconst isArrayContentsEqual = (left, right) => {\n    if (!Array.isArray(left) || !Array.isArray(right)) {\n        // TODO: or throw?\n        return false;\n    }\n\n    if (left.length !== right.length) {\n        return false;\n    }\n\n    for(let index = 0; index < left.length; index++) {\n        if (left[index] !== right[index]) {\n            return false;\n        }\n    }\n\n    return true;\n}\n\n//# sourceURL=webpack:///./components/Utility.js?");

/***/ }),

/***/ "./components/VariableRegister.js":
/*!****************************************!*\
  !*** ./components/VariableRegister.js ***!
  \****************************************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _Utility__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Utility */ \"./components/Utility.js\");\n\n\nclass VariableRegister extends HTMLElement {\n    constructor() {\n      super();\n\n      this.index = 0;\n      this.value = 0;\n    }\n  \n    connectedCallback() {\n      this.index = this.getAttribute(\"index\");\n\n      this.render();\n    }\n\n    update(new_value) {\n        if (this.value == new_value) {\n            return;\n        }\n        \n        if (typeof new_value != 'number') {\n            console.error(\"Unexpected delay timer value type: \", typeof new_value);\n            return;\n        }\n        \n        this.value = new_value;\n\n        this.render();\n    }\n  \n    render() {\n      this.innerHTML = `\n        <div class=\"contents\">\n            ${Object(_Utility__WEBPACK_IMPORTED_MODULE_0__[\"renderNumberValue\"])(`V${this.index}`, this.value, 2)}\n        </div>\n        `;\n    }\n  }\n  \n  customElements.define(\"variable-register\", VariableRegister);\n\n//# sourceURL=webpack:///./components/VariableRegister.js?");

/***/ })

}]);