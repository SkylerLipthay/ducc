#include "wrapper.h"

#pragma push_macro("DUK_INVALID_INDEX")
#undef DUK_INVALID_INDEX
const duk_idx_t DUK_INVALID_INDEX =
#pragma pop_macro("DUK_INVALID_INDEX")
  DUK_INVALID_INDEX;

#pragma push_macro("DUK_VARARGS")
#undef DUK_VARARGS
const duk_int_t DUK_VARARGS =
#pragma pop_macro("DUK_VARARGS")
  DUK_VARARGS;

#pragma push_macro("duk_create_heap_default")
#undef duk_create_heap_default
duk_context *duk_create_heap_default() {
#pragma pop_macro("duk_create_heap_default")
  return duk_create_heap_default();
}

#pragma push_macro("duk_xmove_top")
#undef duk_xmove_top
void duk_xmove_top(duk_context *to_ctx, duk_context *from_ctx, duk_idx_t count) {
#pragma pop_macro("duk_xmove_top")
  duk_xmove_top(to_ctx, from_ctx, count);
}

#pragma push_macro("duk_xcopy_top")
#undef duk_xcopy_top
void duk_xcopy_top(duk_context *to_ctx, duk_context *from_ctx, duk_idx_t count) {
#pragma pop_macro("duk_xcopy_top")
  duk_xcopy_top(to_ctx, from_ctx, count);
}

#pragma push_macro("duk_push_string_file")
#undef duk_push_string_file
const char *duk_push_string_file(duk_context *ctx, const char *path) {
#pragma pop_macro("duk_push_string_file")
  return duk_push_string_file(ctx, path);
}

#pragma push_macro("duk_push_thread")
#undef duk_push_thread
duk_idx_t duk_push_thread(duk_context *ctx) {
#pragma pop_macro("duk_push_thread")
  return duk_push_thread(ctx);
}

#pragma push_macro("duk_push_thread_new_globalenv")
#undef duk_push_thread_new_globalenv
duk_idx_t duk_push_thread_new_globalenv(duk_context *ctx) {
#pragma pop_macro("duk_push_thread_new_globalenv")
  return duk_push_thread_new_globalenv(ctx);
}

#pragma push_macro("duk_push_error_object")
#undef duk_push_error_object
duk_idx_t duk_push_error_object(duk_context *ctx, duk_errcode_t err_code, const char *fmt) {
#pragma pop_macro("duk_push_error_object")
  return duk_push_error_object(ctx, err_code, fmt);
}

#pragma push_macro("duk_push_buffer")
#undef duk_push_buffer
void *duk_push_buffer(duk_context *ctx, duk_size_t size, duk_bool_t dynamic) {
#pragma pop_macro("duk_push_buffer")
  return duk_push_buffer(ctx, size, dynamic);
}

#pragma push_macro("duk_push_fixed_buffer")
#undef duk_push_fixed_buffer
void *duk_push_fixed_buffer(duk_context *ctx, duk_size_t size) {
#pragma pop_macro("duk_push_fixed_buffer")
  return duk_push_fixed_buffer(ctx, size);
}

#pragma push_macro("duk_push_dynamic_buffer")
#undef duk_push_dynamic_buffer
void *duk_push_dynamic_buffer(duk_context *ctx, duk_size_t size) {
#pragma pop_macro("duk_push_dynamic_buffer")
  return duk_push_dynamic_buffer(ctx, size);
}

#pragma push_macro("duk_push_external_buffer")
#undef duk_push_external_buffer
void duk_push_external_buffer(duk_context *ctx) {
#pragma pop_macro("duk_push_external_buffer")
  duk_push_external_buffer(ctx);
}

#pragma push_macro("duk_is_callable")
#undef duk_is_callable
duk_bool_t duk_is_callable(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_callable")
  return duk_is_callable(ctx, index);
}

#pragma push_macro("duk_is_primitive")
#undef duk_is_primitive
duk_bool_t duk_is_primitive(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_primitive")
  return duk_is_primitive(ctx, index);
}

#pragma push_macro("duk_is_object_coercible")
#undef duk_is_object_coercible
duk_bool_t duk_is_object_coercible(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_object_coercible")
  return duk_is_object_coercible(ctx, index);
}

#pragma push_macro("duk_is_error")
#undef duk_is_error
duk_bool_t duk_is_error(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_error")
  return duk_is_error(ctx, index);
}

#pragma push_macro("duk_is_eval_error")
#undef duk_is_eval_error
duk_bool_t duk_is_eval_error(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_eval_error")
  return duk_is_eval_error(ctx, index);
}

#pragma push_macro("duk_is_range_error")
#undef duk_is_range_error
duk_bool_t duk_is_range_error(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_range_error")
  return duk_is_range_error(ctx, index);
}

#pragma push_macro("duk_is_reference_error")
#undef duk_is_reference_error
duk_bool_t duk_is_reference_error(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_reference_error")
  return duk_is_reference_error(ctx, index);
}

#pragma push_macro("duk_is_syntax_error")
#undef duk_is_syntax_error
duk_bool_t duk_is_syntax_error(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_syntax_error")
  return duk_is_syntax_error(ctx, index);
}

#pragma push_macro("duk_is_type_error")
#undef duk_is_type_error
duk_bool_t duk_is_type_error(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_type_error")
  return duk_is_type_error(ctx, index);
}

#pragma push_macro("duk_is_uri_error")
#undef duk_is_uri_error
duk_bool_t duk_is_uri_error(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_uri_error")
  return duk_is_uri_error(ctx, index);
}

#pragma push_macro("duk_require_type_mask")
#undef duk_require_type_mask
void duk_require_type_mask(duk_context *ctx, duk_idx_t index, duk_uint_t mask) {
#pragma pop_macro("duk_require_type_mask")
  duk_require_type_mask(ctx, index, mask);
}

#pragma push_macro("duk_require_callable")
#undef duk_require_callable
void duk_require_callable(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_require_callable")
  duk_require_callable(ctx, index);
}

#pragma push_macro("duk_require_object_coercible")
#undef duk_require_object_coercible
void duk_require_object_coercible(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_require_object_coercible")
  duk_require_object_coercible(ctx, index);
}

#pragma push_macro("duk_to_buffer")
#undef duk_to_buffer
void *duk_to_buffer(duk_context *ctx, duk_idx_t index, duk_size_t *out_size) {
#pragma pop_macro("duk_to_buffer")
  return duk_to_buffer(ctx, index, out_size);
}

#pragma push_macro("duk_to_fixed_buffer")
#undef duk_to_fixed_buffer
void *duk_to_fixed_buffer(duk_context *ctx, duk_idx_t index, duk_size_t *out_size) {
#pragma pop_macro("duk_to_fixed_buffer")
  return duk_to_fixed_buffer(ctx, index, out_size);
}

#pragma push_macro("duk_to_dynamic_buffer")
#undef duk_to_dynamic_buffer
void *duk_to_dynamic_buffer(duk_context *ctx, duk_idx_t index, duk_size_t *out_size) {
#pragma pop_macro("duk_to_dynamic_buffer")
  return duk_to_dynamic_buffer(ctx, index, out_size);
}

#pragma push_macro("duk_safe_to_string")
#undef duk_safe_to_string
const char *duk_safe_to_string(duk_context *ctx, duk_idx_t index) {
#pragma pop_macro("duk_safe_to_string")
  return duk_safe_to_string(ctx, index);
}

#pragma push_macro("duk_eval")
#undef duk_eval
void duk_eval(duk_context *ctx) {
#pragma pop_macro("duk_eval")
  duk_eval(ctx);
}

#pragma push_macro("duk_eval_noresult")
#undef duk_eval_noresult
void duk_eval_noresult(duk_context *ctx) {
#pragma pop_macro("duk_eval_noresult")
  duk_eval_noresult(ctx);
}

#pragma push_macro("duk_peval")
#undef duk_peval
duk_int_t duk_peval(duk_context *ctx) {
#pragma pop_macro("duk_peval")
  return duk_peval(ctx);
}

#pragma push_macro("duk_peval_noresult")
#undef duk_peval_noresult
duk_int_t duk_peval_noresult(duk_context *ctx) {
#pragma pop_macro("duk_peval_noresult")
  return duk_peval_noresult(ctx);
}

#pragma push_macro("duk_compile")
#undef duk_compile
void duk_compile(duk_context *ctx, duk_uint_t flags) {
#pragma pop_macro("duk_compile")
  duk_compile(ctx, flags);
}

#pragma push_macro("duk_pcompile")
#undef duk_pcompile
duk_int_t duk_pcompile(duk_context *ctx, duk_uint_t flags) {
#pragma pop_macro("duk_pcompile")
  return duk_pcompile(ctx, flags);
}

#pragma push_macro("duk_eval_string")
#undef duk_eval_string
void duk_eval_string(duk_context *ctx, const char *src) {
#pragma pop_macro("duk_eval_string")
  duk_eval_string(ctx, src);
}

#pragma push_macro("duk_eval_string_noresult")
#undef duk_eval_string_noresult
void duk_eval_string_noresult(duk_context *ctx, const char *src) {
#pragma pop_macro("duk_eval_string_noresult")
  duk_eval_string_noresult(ctx, src);
}

#pragma push_macro("duk_peval_string")
#undef duk_peval_string
duk_int_t duk_peval_string(duk_context *ctx, const char *src) {
#pragma pop_macro("duk_peval_string")
  return duk_peval_string(ctx, src);
}

#pragma push_macro("duk_peval_string_noresult")
#undef duk_peval_string_noresult
duk_int_t duk_peval_string_noresult(duk_context *ctx, const char *src) {
#pragma pop_macro("duk_peval_string_noresult")
  return duk_peval_string_noresult(ctx, src);
}

#pragma push_macro("duk_compile_string")
#undef duk_compile_string
void duk_compile_string(duk_context *ctx, duk_uint_t flags, const char *src) {
#pragma pop_macro("duk_compile_string")
  duk_compile_string(ctx, flags, src);
}

#pragma push_macro("duk_compile_string_filename")
#undef duk_compile_string_filename
void duk_compile_string_filename(duk_context *ctx, duk_uint_t flags, const char *src) {
#pragma pop_macro("duk_compile_string_filename")
  duk_compile_string_filename(ctx, flags, src);
}

#pragma push_macro("duk_pcompile_string")
#undef duk_pcompile_string
duk_int_t duk_pcompile_string(duk_context *ctx, duk_uint_t flags, const char *src) {
#pragma pop_macro("duk_pcompile_string")
  return duk_pcompile_string(ctx, flags, src);
}

#pragma push_macro("duk_pcompile_string_filename")
#undef duk_pcompile_string_filename
duk_int_t duk_pcompile_string_filename(duk_context *ctx, duk_uint_t flags, const char *src) {
#pragma pop_macro("duk_pcompile_string_filename")
  return duk_pcompile_string_filename(ctx, flags, src);
}

#pragma push_macro("duk_eval_lstring")
#undef duk_eval_lstring
void duk_eval_lstring(duk_context *ctx, const char *buf, duk_size_t len) {
#pragma pop_macro("duk_eval_lstring")
  duk_eval_lstring(ctx, buf, len);
}

#pragma push_macro("duk_eval_lstring_noresult")
#undef duk_eval_lstring_noresult
void duk_eval_lstring_noresult(duk_context *ctx, const char *buf, duk_size_t len) {
#pragma pop_macro("duk_eval_lstring_noresult")
  duk_eval_lstring_noresult(ctx, buf, len);
}

#pragma push_macro("duk_peval_lstring")
#undef duk_peval_lstring
duk_int_t duk_peval_lstring(duk_context *ctx, const char *buf, duk_size_t len) {
#pragma pop_macro("duk_peval_lstring")
  return duk_peval_lstring(ctx, buf, len);
}

#pragma push_macro("duk_peval_lstring_noresult")
#undef duk_peval_lstring_noresult
duk_int_t duk_peval_lstring_noresult(duk_context *ctx, const char *buf, duk_size_t len) {
#pragma pop_macro("duk_peval_lstring_noresult")
  return duk_peval_lstring_noresult(ctx, buf, len);
}

#pragma push_macro("duk_compile_lstring")
#undef duk_compile_lstring
void duk_compile_lstring(duk_context *ctx, duk_uint_t flags, const char *buf, duk_size_t len) {
#pragma pop_macro("duk_compile_lstring")
  duk_compile_lstring(ctx, flags, buf, len);
}

#pragma push_macro("duk_compile_lstring_filename")
#undef duk_compile_lstring_filename
void duk_compile_lstring_filename(duk_context *ctx, duk_uint_t flags, const char *buf, duk_size_t len) {
#pragma pop_macro("duk_compile_lstring_filename")
  duk_compile_lstring_filename(ctx, flags, buf, len);
}

#pragma push_macro("duk_pcompile_lstring")
#undef duk_pcompile_lstring
duk_int_t duk_pcompile_lstring(duk_context *ctx, duk_uint_t flags, const char *buf, duk_size_t len) {
#pragma pop_macro("duk_pcompile_lstring")
  return duk_pcompile_lstring(ctx, flags, buf, len);
}

#pragma push_macro("duk_pcompile_lstring_filename")
#undef duk_pcompile_lstring_filename
duk_int_t duk_pcompile_lstring_filename(duk_context *ctx, duk_uint_t flags, const char *buf, duk_size_t len) {
#pragma pop_macro("duk_pcompile_lstring_filename")
  return duk_pcompile_lstring_filename(ctx, flags, buf, len);
}

#pragma push_macro("duk_eval_file")
#undef duk_eval_file
void duk_eval_file(duk_context *ctx, const char *path) {
#pragma pop_macro("duk_eval_file")
  duk_eval_file(ctx, path);
}

#pragma push_macro("duk_eval_file_noresult")
#undef duk_eval_file_noresult
void duk_eval_file_noresult(duk_context *ctx, const char *path) {
#pragma pop_macro("duk_eval_file_noresult")
  duk_eval_file_noresult(ctx, path);
}

#pragma push_macro("duk_peval_file")
#undef duk_peval_file
duk_int_t duk_peval_file(duk_context *ctx, const char *path) {
#pragma pop_macro("duk_peval_file")
  return duk_peval_file(ctx, path);
}

#pragma push_macro("duk_peval_file_noresult")
#undef duk_peval_file_noresult
duk_int_t duk_peval_file_noresult(duk_context *ctx, const char *path) {
#pragma pop_macro("duk_peval_file_noresult")
  return duk_peval_file_noresult(ctx, path);
}

#pragma push_macro("duk_compile_file")
#undef duk_compile_file
void duk_compile_file(duk_context *ctx, duk_uint_t flags, const char *path) {
#pragma pop_macro("duk_compile_file")
  duk_compile_file(ctx, flags, path);
}

#pragma push_macro("duk_pcompile_file")
#undef duk_pcompile_file
duk_int_t duk_pcompile_file(duk_context *ctx, duk_uint_t flags, const char *path) {
#pragma pop_macro("duk_pcompile_file")
  return duk_pcompile_file(ctx, flags, path);
}

#pragma push_macro("duk_dump_context_stdout")
#undef duk_dump_context_stdout
void duk_dump_context_stdout(duk_context *ctx) {
#pragma pop_macro("duk_dump_context_stdout")
  duk_dump_context_stdout(ctx);
}

#pragma push_macro("duk_dump_context_stderr")
#undef duk_dump_context_stderr
void duk_dump_context_stderr(duk_context *ctx) {
#pragma pop_macro("duk_dump_context_stderr")
  duk_dump_context_stderr(ctx);
}

duk_ret_t handle_func_nothrow(duk_context *ctx) {
  duk_require_stack(ctx, 2);
  duk_push_current_function(ctx);
  duk_get_prop_string(ctx, -1, DUK_HIDDEN_SYMBOL("__NOTHROWFUNC"));
  duk_c_function func = (duk_c_function)duk_get_pointer(ctx, -1);
  duk_pop_n(ctx, 2);

  duk_ret_t result = func(ctx);
  if (result >= 0) {
    return result;
  }

  return duk_throw(ctx);
}

duk_idx_t ducc_push_c_function_nothrow(duk_context *ctx,
    duk_c_function func, duk_idx_t nargs) {
  duk_require_stack(ctx, 2);
  duk_idx_t result = duk_push_c_function(ctx, handle_func_nothrow, nargs);
  duk_push_pointer(ctx, func);
  duk_put_prop_string(ctx, -2, DUK_HIDDEN_SYMBOL("__NOTHROWFUNC"));
  return result;
}

duk_bool_t default_exec_timeout_func(void *udata) {
  (void)(udata);
  return 0;
}

static ducc_exec_timeout_function EXEC_TIMEOUT_FUNC = default_exec_timeout_func;

void ducc_set_exec_timeout_function(ducc_exec_timeout_function func) {
  EXEC_TIMEOUT_FUNC = func;
}

ducc_exec_timeout_function ducc_get_exec_timeout_function() {
  return EXEC_TIMEOUT_FUNC;
}
