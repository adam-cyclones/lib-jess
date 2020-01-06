"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const fs_1 = require("fs");
const constants_1 = require("../../../constants");
const encoding = 'utf8';
/**
 * Appends the Tess file extname if not existing in path
*/
function import_jess_sync(path) {
    return fs_1.readFileSync(path.endsWith(constants_1.JESS_EXTNAME, 0) ? path : `${path}${constants_1.JESS_EXTNAME}`, { encoding });
}
exports.import_jess_sync = import_jess_sync;
/**
 * Appends the Tess file extname if not existing in path
*/
function import_tess_sync(path) {
    return fs_1.readFileSync(path.endsWith(constants_1.TESS_EXTNAME, 0) ? path : `${path}${constants_1.TESS_EXTNAME}`, { encoding });
}
exports.import_tess_sync = import_tess_sync;
