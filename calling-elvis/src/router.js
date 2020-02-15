"use strict";
exports.__esModule = true;
var Router = /** @class */ (function () {
    function Router(routes) {
        this.routes = routes;
    }
    Router.back = function () {
        window.history.back();
    };
    Router.push = function (path, pushProps) {
        if (pushProps === void 0) { pushProps = { props: {}, title: document.title }; }
        if (window.location.pathname.slice(1) === path) {
            return;
        }
        window.history.pushState(pushProps.props, pushProps.title, path);
        window.route();
    };
    return Router;
}());
exports["default"] = Router;
