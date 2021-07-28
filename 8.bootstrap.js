(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[8],{

/***/ "./components/VariableRegisters.js":
/*!*****************************************!*\
  !*** ./components/VariableRegisters.js ***!
  \*****************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("class VariableRegisters extends HTMLElement {\n    constructor() {\n      super();\n\n      this.values = [];\n    }\n  \n    connectedCallback() {\n      this.render();\n    }\n\n    update(new_values) {\n        if (this.values === new_values) {\n            return;\n        }\n        \n        if (!(new_values instanceof Uint8Array)) {\n            console.error(\"Unexpected variable registers values type: \", typeof new_values);\n            return;\n        }\n        \n        this.values = new_values;\n\n        // TODO: cache\n        this.values.forEach((value, index) => {\n            const element = document.getElementById(`variableRegister${index}`)\n            if (!element) {\n                // TODO: report\n                return;\n            }\n            if (!element.update) {\n                // TODO: report\n                return;\n            }\n            element.update(value)\n        })\n    }\n  \n    render() {\n        const contents = [...new Array(16)].map((value, index) => `<variable-register id=\"variableRegister${index}\" index=\"${index}\"></variable-register>`)\n        this.innerHTML = `\n            <div class=\"contents\">\n                ${contents.join(\"\")}\n            </div>\n            `;\n    }\n  }\n  \n  customElements.define(\"variable-registers\", VariableRegisters);\n\n//# sourceURL=webpack:///./components/VariableRegisters.js?");

/***/ })

}]);