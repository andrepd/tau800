/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	var __webpack_modules__ = ({

/***/ "./src/live/live.js":
/*!**************************!*\
  !*** ./src/live/live.js ***!
  \**************************/
/***/ ((__unused_webpack_module, __unused_webpack_exports, __webpack_require__) => {

eval("const renderer = __webpack_require__(/*! ./renderer */ \"./src/live/renderer.js\")\nconst { sleep } = __webpack_require__(/*! ./../sleep */ \"./src/sleep.js\")\nconst { REGISTER_A } = __webpack_require__(/*! ./registers */ \"./src/live/registers.js\")\n\nwindow.addEventListener('DOMContentLoaded', () => {\n    renderer.initialize()\n    main()\n})\n\nasync function main() {\n    let timer = 0\n    let bits\n    let bit_remainder\n    while (true) {\n        renderer.write_clock(\n            String(Math.floor(timer/60)).padStart(2, '0'),\n            String(timer%60).padStart(2, '0'))\n\n        bits = Array(6)\n        bit_remainder = timer\n        for (let i = 5; i >= 0; i--) {\n            if (2**i <= bit_remainder) {\n                bits[i] = true\n                bit_remainder -= 2**i\n            } else {\n                bits[i] = false\n            }\n        }\n        renderer.write_register(REGISTER_A, bits)\n\n        timer++\n        await sleep(1000)\n    }\n}\n\n//# sourceURL=webpack://anachronic/./src/live/live.js?");

/***/ }),

/***/ "./src/live/registers.js":
/*!*******************************!*\
  !*** ./src/live/registers.js ***!
  \*******************************/
/***/ ((module) => {

eval("const REGISTER_A = 0\nconst REGISTER_F = 1\nconst REGISTER_BH = 2\nconst REGISTER_BL = 3\nconst REGISTER_CH = 4\nconst REGISTER_CL = 5\nconst REGISTER_X = 6\nconst REGISTER_SP = 7\nconst REGISTER_PC = 8\n\nmodule.exports = {\n    REGISTER_A,\n    REGISTER_F,\n    REGISTER_BH,\n    REGISTER_BL,\n    REGISTER_CH,\n    REGISTER_CL,\n    REGISTER_X,\n    REGISTER_SP,\n    REGISTER_PC,\n}\n\n//# sourceURL=webpack://anachronic/./src/live/registers.js?");

/***/ }),

/***/ "./src/live/renderer.js":
/*!******************************!*\
  !*** ./src/live/renderer.js ***!
  \******************************/
/***/ ((module) => {

eval("let initialized = false\nlet clock_left\nlet clock_right\nlet stack_boxes\nlet register_labels\nlet register_boxes\nlet cmd_displays\n\nfunction initialize() {\n    if (document == undefined) {\n        throw Error('Tried to initialize renderer before DOM was loaded.')\n    }\n\n    clock_left = document.getElementById('clock-left')\n    clock_right = document.getElementById('clock-right')\n\n    stack_boxes = document.querySelectorAll('.stack-box > g > rect')\n    register_labels = document.querySelectorAll('.register > .label')\n    register_boxes = Array.from(document.getElementsByClassName('register'))\n        .map((register_group) =>\n            Array.from(register_group.getElementsByClassName('register-box')).map(\n                (element) => element.querySelector('rect'))\n        )\n    console.log(register_boxes)\n\n    cmd_displays = document.getElementsByClassName('command-display')\n\n    initialized = true\n}\n\nfunction write_clock(hours, minutes) {\n    if (!initialized) {\n        throw Error('Need to initialize the renderer first.')\n    }\n\n    clock_left.innerHTML = hours\n    clock_right.innerHTML = minutes\n}\n\nfunction report_command_history(commands) {\n    commands.forEach((value, index) => { cmd_displays[index] = value })\n}\n\nfunction write_register(register, values) {\n    let register_group = register_boxes[register]\n\n    register_group.forEach((box, index) => {\n        if (values[index]) {\n            box.style.fill = '#ffffff'\n        } else {\n            box.style.fill = 'none'\n        }\n    })\n}\n\nmodule.exports = {\n    initialize,\n    write_clock,\n    report_command_history,\n    write_register,\n}\n\n//# sourceURL=webpack://anachronic/./src/live/renderer.js?");

/***/ }),

/***/ "./src/sleep.js":
/*!**********************!*\
  !*** ./src/sleep.js ***!
  \**********************/
/***/ ((module) => {

eval("module.exports = {\n    sleep: (ms) => new Promise(resolve => setTimeout(resolve, ms))\n}\n\n//# sourceURL=webpack://anachronic/./src/sleep.js?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = __webpack_require__("./src/live/live.js");
/******/ 	
/******/ })()
;