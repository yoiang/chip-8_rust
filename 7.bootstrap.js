(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[7],{

/***/ "./components/PartialDisassembler.js":
/*!*******************************************!*\
  !*** ./components/PartialDisassembler.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("class PartialDisassembler extends HTMLElement {\n    constructor() {\n      super();\n\n      this.values = [];\n    }\n  \n    connectedCallback() {\n      this.render();\n    }\n\n    update(new_program_counter_location, new_values) {\n        if (this.values === new_values) {\n            return;\n        }\n        \n        if (!Array.isArray(new_values)) {\n            console.error(\"Unexpected variable registers values type: \", typeof new_values);\n            return;\n        }\n        \n        let resized = this.values.length !== new_values.length;\n        \n        this.values = new_values;\n\n        if (resized) {\n            this.render();\n        }\n\n        // TODO: cache\n        this.values.forEach((value, index) => {\n            const element = document.getElementById(`disassemblerLine${index}`)\n            if (!element) {\n                // TODO: report\n                return;\n            }\n            if (!element.update) {\n                // TODO: report\n                return;\n            }\n            element.update(new_program_counter_location === value.location, value.location, value.value, value.disassembly)\n        })\n    }\n  \n    render() {\n        const contents = this.values.map((value, index) => `<disassembler-line id=\"disassemblerLine${index}\"></disassembler-line>`)\n        this.innerHTML = `\n            <div class=\"partial-disassembler\">\n                ${contents.join(\"\")}\n            </div>\n            `;\n    }\n  }\n  \n  customElements.define(\"partial-disassembler\", PartialDisassembler);\n\n//# sourceURL=webpack:///./components/PartialDisassembler.js?");

/***/ })

}]);