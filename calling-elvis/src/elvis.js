"use strict";
exports.__esModule = true;
var elvis_web_1 = require("elvis-web");
var Elvis = /** @class */ (function () {
    function Elvis(props) {
        var _this = this;
        // init global route
        window.route = function () {
            var ptr = window.location.pathname.slice(1);
            var widget = _this.router.routes[ptr];
            _this.proto = new elvis_web_1.Elvis(widget);
            _this.calling();
        };
        // setters
        this.router = props.router;
        this.home = props.home;
        if (window.location.pathname === "/") {
            this.proto = new elvis_web_1.Elvis(this.home);
        }
        else {
            window.route();
        }
    }
    Elvis.call = function (widget) {
        new elvis_web_1.Elvis(widget).calling();
    };
    Elvis.prototype.calling = function () {
        this.proto.calling();
    };
    return Elvis;
}());
exports["default"] = Elvis;
