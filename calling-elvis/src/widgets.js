"use strict";
exports.__esModule = true;
var elvis_web_1 = require("elvis-web");
function Image(cfg) {
    return elvis_web_1.Image(cfg.src, cfg.child);
}
exports.Image = Image;
// Text Wrapper
function Text(text, style) {
    return elvis_web_1.Text(text, new elvis_web_1.TextStyle(style.bold, style.color, style.italic, style.size, style.weight, style.height, style.stretch));
}
exports.Text = Text;
