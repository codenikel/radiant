/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_radiantkitappcontroller_free(a: number): void;
export function radiantkitappcontroller_new(a: number, b: number, c: number, d: number, e: number, f: number, g: number): number;
export function radiantkitappcontroller_handleMessage(a: number, b: number): void;
export function __wbg_colorcomponent_free(a: number): void;
export function __wbg_transformcomponent_free(a: number): void;
export function transformcomponent_transform_xy(a: number, b: number): void;
export function transformcomponent_transform_scale(a: number, b: number): void;
export function transformcomponent_set_position(a: number, b: number): void;
export function transformcomponent_set_scale(a: number, b: number): void;
export function transformcomponent_set_rotation(a: number, b: number): void;
export function transformcomponent_position(a: number): number;
export function transformcomponent_scale(a: number): number;
export function transformcomponent_get_rotation(a: number): number;
export function __wbg_vec3_free(a: number): void;
export function __wbg_get_vec3_x(a: number): number;
export function __wbg_set_vec3_x(a: number, b: number): void;
export function __wbg_get_vec3_y(a: number): number;
export function __wbg_set_vec3_y(a: number, b: number): void;
export function __wbg_get_vec3_z(a: number): number;
export function __wbg_set_vec3_z(a: number, b: number): void;
export function vec3_zero(): number;
export function vec3_new(a: number, b: number, c: number): number;
export function vec3_new_with_min(a: number): number;
export function vec3_new_with_added(a: number, b: number): number;
export function vec3_add(a: number, b: number): void;
export function vec3_add_with_min(a: number, b: number, c: number): void;
export function vec3_add_scalar(a: number, b: number): void;
export function vec3_set_with_min(a: number, b: number, c: number): void;
export function __wbg_selectioncomponent_free(a: number): void;
export function wgpu_compute_pass_set_pipeline(a: number, b: number): void;
export function wgpu_compute_pass_set_bind_group(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_compute_pass_set_push_constant(a: number, b: number, c: number, d: number): void;
export function wgpu_compute_pass_insert_debug_marker(a: number, b: number, c: number): void;
export function wgpu_compute_pass_push_debug_group(a: number, b: number, c: number): void;
export function wgpu_compute_pass_pop_debug_group(a: number): void;
export function wgpu_compute_pass_write_timestamp(a: number, b: number, c: number): void;
export function wgpu_compute_pass_begin_pipeline_statistics_query(a: number, b: number, c: number): void;
export function wgpu_compute_pass_end_pipeline_statistics_query(a: number): void;
export function wgpu_compute_pass_dispatch_workgroups(a: number, b: number, c: number, d: number): void;
export function wgpu_compute_pass_dispatch_workgroups_indirect(a: number, b: number, c: number): void;
export function wgpu_render_bundle_set_pipeline(a: number, b: number): void;
export function wgpu_render_bundle_set_bind_group(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_set_vertex_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_set_push_constants(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_draw(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_draw_indexed(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_bundle_draw_indirect(a: number, b: number, c: number): void;
export function wgpu_render_bundle_draw_indexed_indirect(a: number, b: number, c: number): void;
export function wgpu_render_pass_set_pipeline(a: number, b: number): void;
export function wgpu_render_pass_set_bind_group(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_vertex_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_push_constants(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_draw(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_draw_indexed(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_pass_draw_indirect(a: number, b: number, c: number): void;
export function wgpu_render_pass_draw_indexed_indirect(a: number, b: number, c: number): void;
export function wgpu_render_pass_multi_draw_indirect(a: number, b: number, c: number, d: number): void;
export function wgpu_render_pass_multi_draw_indexed_indirect(a: number, b: number, c: number, d: number): void;
export function wgpu_render_pass_multi_draw_indirect_count(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_pass_multi_draw_indexed_indirect_count(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_pass_set_blend_constant(a: number, b: number): void;
export function wgpu_render_pass_set_scissor_rect(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_viewport(a: number, b: number, c: number, d: number, e: number, f: number, g: number): void;
export function wgpu_render_pass_set_stencil_reference(a: number, b: number): void;
export function wgpu_render_pass_insert_debug_marker(a: number, b: number, c: number): void;
export function wgpu_render_pass_push_debug_group(a: number, b: number, c: number): void;
export function wgpu_render_pass_pop_debug_group(a: number): void;
export function wgpu_render_pass_write_timestamp(a: number, b: number, c: number): void;
export function wgpu_render_pass_begin_pipeline_statistics_query(a: number, b: number, c: number): void;
export function wgpu_render_pass_end_pipeline_statistics_query(a: number): void;
export function wgpu_render_pass_execute_bundles(a: number, b: number, c: number): void;
export function wgpu_render_bundle_set_index_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_pop_debug_group(a: number): void;
export function wgpu_render_bundle_insert_debug_marker(a: number, b: number): void;
export function wgpu_render_bundle_push_debug_group(a: number, b: number): void;
export function wgpu_render_pass_set_index_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function __wbindgen_malloc(a: number, b: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number, d: number): number;
export const __wbindgen_export_2: WebAssembly.Table;
export function _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4fb0e95bb6198ae7(a: number, b: number, c: number): void;
export function _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3245379ba746818e(a: number, b: number): void;
export function _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5234d6fbfa4fe934(a: number, b: number, c: number): void;
export function wasm_bindgen__convert__closures__invoke1_mut__h08d52ce9fda41a9d(a: number, b: number, c: number): void;
export function wasm_bindgen__convert__closures__invoke0_mut__h63b5a1009b247d3f(a: number, b: number): void;
export function _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7f0e0b13653e5123(a: number, b: number, c: number): void;
export function __wbindgen_free(a: number, b: number, c: number): void;
export function __wbindgen_exn_store(a: number): void;
export function wasm_bindgen__convert__closures__invoke2_mut__h27d1f5ca9c4fdc62(a: number, b: number, c: number, d: number): void;
