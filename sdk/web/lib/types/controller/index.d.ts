import { RadiantAppController } from "radiant-wasm";
export declare class RadiantController {
    _controller: RadiantAppController;
    constructor(controller: RadiantAppController);
    static createController(f: Function): Promise<RadiantController>;
    /**
     * Activates the provided tool.
     *
     * @param tool the tool to activate.
     */
    activateTool(tool: string): void;
    setTransform(nodeId: number, position: number[], scale: number[]): void;
    setFillColor(nodeId: number, color: number[]): void;
    setStrokeColor(nodeId: number, color: number[]): void;
}
//# sourceMappingURL=index.d.ts.map