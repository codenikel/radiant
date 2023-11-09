"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.RadiantController = void 0;
const tslib_1 = require("tslib");
const radiantkit_1 = require("radiantkit");
class RadiantController {
    constructor(controller) {
        this._controller = controller;
    }
    static createController(f) {
        return tslib_1.__awaiter(this, void 0, void 0, function* () {
            return new RadiantController(yield new radiantkit_1.RadiantAppController(f));
        });
    }
    /**
     * Activates the provided tool.
     *
     * @param tool the tool to activate.
     */
    activateTool(toolId) {
        this._controller.handleMessage({
            SceneMessage: {
                SelectTool: {
                    id: toolId,
                },
            },
        });
    }
    addRectangle(position, scale) {
        this._controller.handleMessage({
            AddRectangle: {
                position,
                scale,
            },
        });
    }
    addImage(path, name = "", position = [100, 100], scale = [100, 100]) {
        this._controller.handleMessage({
            AddImage: {
                name,
                path,
            },
        });
    }
    addText(text, position = [100, 100], scale = [100, 100]) {
        this._controller.handleMessage({
            AddText: {
                text,
                position,
            },
        });
    }
    setTransform(nodeId, position, scale) {
        this._controller.handleMessage({
            SceneMessage: {
                SetTransform: {
                    id: nodeId,
                    position,
                    scale,
                },
            },
        });
    }
    setFillColor(nodeId, color) {
        this._controller.handleMessage({
            SceneMessage: {
                SetFillColor: {
                    id: nodeId,
                    fill_color: color,
                },
            },
        });
    }
    setStrokeColor(nodeId, color) {
        this._controller.handleMessage({
            SceneMessage: {
                SetStrokeColor: {
                    id: nodeId,
                    stroke_color: color,
                },
            },
        });
    }
}
exports.RadiantController = RadiantController;
//# sourceMappingURL=index.js.map