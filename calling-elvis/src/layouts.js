"use strict";
exports.__esModule = true;
var elvis_web_1 = require("elvis-web");
function Align(widget, style) {
    return elvis_web_1.Align(widget, new elvis_web_1.AlignStyle(style.align));
}
exports.Align = Align;
function Col(widgets) {
    var col = new elvis_web_1.Col();
    for (var i in widgets) {
        if (widgets[i] !== undefined) {
            col.push(widgets[i]);
        }
    }
    return col.widget();
}
exports.Col = Col;
function Container(widget, style) {
    if (style === undefined) {
        style = {};
    }
    return elvis_web_1.Container(widget, new elvis_web_1.ContainerStyle(style.align, style.color, style.height, style.margin, style.padding, style.width));
}
exports.Container = Container;
function Flex(widget, style) {
    if (style === undefined) {
        style = {};
    }
    return elvis_web_1.Flex(widget, new elvis_web_1.FlexStyle(style.align, style.basis, style.direction, style.grow, style.order, style.wrap));
}
exports.Flex = Flex;
function Grid(widgets, style) {
    if (style === undefined) {
        style = {};
    }
    var grid = new elvis_web_1.Grid(new elvis_web_1.GridStyle(style.col, style.col_gap, style.flow, style.row, style.row_gap, style.template_col, style.template_row));
    for (var i in widgets) {
        if (widgets[i] !== undefined) {
            grid.push(widgets[i]);
        }
    }
    return grid.widget();
}
exports.Grid = Grid;
function List(widgets) {
    var list = new elvis_web_1.List();
    for (var i in widgets) {
        if (widgets[i] !== undefined) {
            list.push(widgets[i]);
        }
    }
    return list.widget();
}
exports.List = List;
function MultiColumn(widgets, style) {
    if (style === undefined) {
        style = {};
    }
    var mc = new elvis_web_1.MultiColumn(new elvis_web_1.MultiColumnStyle(style.color, style.count, style.gap, style.style));
    for (var i in widgets) {
        if (widgets[i] !== undefined) {
            mc.push(widgets[i]);
        }
    }
    return mc.widget();
}
exports.MultiColumn = MultiColumn;
function Row(widgets) {
    var row = new elvis_web_1.Row();
    for (var i in widgets) {
        if (widgets[i] !== undefined) {
            row.push(widgets[i]);
        }
    }
    return row.widget();
}
exports.Row = Row;
function SizedBox(widget, style) {
    return elvis_web_1.SizedBox(widget, new elvis_web_1.SizedBoxStyle(style.height, style.width));
}
exports.SizedBox = SizedBox;
