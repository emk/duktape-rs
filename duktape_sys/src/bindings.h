// This file is based on duktape.h, with manual cleanups for bindgen
// compatibility.

// We need to define this if we want thread-safe APIs.
#define DUK_API_VARIADIC_MACROS

// Glue to make things compile.  Remove the types from the generated output
// manually; we have generated.rs for that.
#include <stdint.h>
#include <stddef.h>
#include <stdarg.h>
#define DUK_API_NORETURN(DECL) DECL;
#define DUK_EXTERNAL_DECL
typedef size_t duk_size_t;
typedef ptrdiff_t duk_ptrdiff_t;
typedef int duk_int_t;
typedef unsigned int duk_uint_t;
typedef double duk_double_t;
typedef int duk_bool_t;
typedef duk_int_t duk_idx_t;
typedef duk_uint_t duk_uarridx_t;
typedef int duk_ret_t;
typedef duk_int_t duk_errcode_t;
typedef duk_int_t duk_codepoint_t;
typedef duk_uint_t duk_ucodepoint_t;
typedef uint16_t duk_uint16_t;
typedef int32_t duk_int32_t;
typedef uint32_t duk_uint32_t;

/*
 *  Duktape public API for Duktape 1.0.2.
 *  See the API reference for documentation on call semantics.
 *  The exposed API is inside the DUK_API_PUBLIC_H_INCLUDED
 *  include guard.  Other parts of the header are Duktape
 *  internal and related to platform/compiler/feature detection.
 *
 *  Git commit a476318bf025137c2847c808a8334d8b7db985f2 (v1.0.2).
 *
 *  See Duktape AUTHORS.rst and LICENSE.txt for copyright and
 *  licensing information.
 */

/* LICENSE.txt */
/*
 *  ===============
 *  Duktape license
 *  ===============
 *  
 *  (http://opensource.org/licenses/MIT)
 *  
 *  Copyright (c) 2013-2014 by Duktape authors (see AUTHORS.rst)
 *  
 *  Permission is hereby granted, free of charge, to any person obtaining a copy
 *  of this software and associated documentation files (the "Software"), to deal
 *  in the Software without restriction, including without limitation the rights
 *  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *  copies of the Software, and to permit persons to whom the Software is
 *  furnished to do so, subject to the following conditions:
 *  
 *  The above copyright notice and this permission notice shall be included in
 *  all copies or substantial portions of the Software.
 *  
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 *  THE SOFTWARE.
 *  
 */

/* AUTHORS.rst */
/*
 *  ===============
 *  Duktape authors
 *  ===============
 *  
 *  Copyright
 *  =========
 *  
 *  Duktape copyrights are held by its authors.  Each author has a copyright
 *  to their contribution, and agrees to irrevocably license the contribution
 *  under the Duktape ``LICENSE.txt``.
 *  
 *  Authors
 *  =======
 *  
 *  Please include an e-mail address, a link to your GitHub profile, or something
 *  similar to allow your contribution to be identified accurately.
 *  
 *  The following people have contributed code and agreed to irrevocably license
 *  their contributions under the Duktape ``LICENSE.txt`` (in order of appearance):
 *  
 *  * Sami Vaarala <sami.vaarala@iki.fi>
 *  * Niki Dobrev
 *  * Andreas \u00d6man <andreas@lonelycoder.com>
 *  
 *  Other contributions
 *  ===================
 *  
 *  The following people have contributed something other than code (e.g. reported
 *  bugs, provided ideas, etc; in order of appearance):
 *  
 *  * Greg Burns
 *  * Anthony Rabine
 *  * Carlos Costa
 *  * Aur\u00e9lien Bouilland
 *  * Preet Desai (Pris Matic)
 *  * judofyr (http://www.reddit.com/user/judofyr)
 *  * Jason Woofenden
 *  * Micha\u0142 Przyby\u015b
 *  * Anthony Howe
 *  * Conrad Pankoff
 *  * Jim Schimpf
 *  * Rajaran Gaunker (https://github.com/zimbabao)
 *  * Andreas \u00d6man
 *  * Doug Sanden
 *  * Remo Eichenberger (https://github.com/remoe)
 *  * David Demelier
 */

/*
 *  Public API specific typedefs
 *
 *  (duk_context *) maps directly to internal type (duk_hthread *).
 *  Currently only primitive typedefs have a '_t' suffix.
 *
 *  Many types are wrapped by Duktape for portability to rare platforms
 *  where e.g. 'int' is a 16-bit type.  See practical typing discussion
 *  in Duktape web documentation.
 */

struct duk_memory_functions;
struct duk_function_list_entry;
struct duk_number_list_entry;

typedef void duk_context;
typedef struct duk_memory_functions duk_memory_functions;
typedef struct duk_function_list_entry duk_function_list_entry;
typedef struct duk_number_list_entry duk_number_list_entry;

typedef duk_ret_t (*duk_c_function)(duk_context *ctx);
typedef void *(*duk_alloc_function) (void *udata, duk_size_t size);
typedef void *(*duk_realloc_function) (void *udata, void *ptr, duk_size_t size);
typedef void (*duk_free_function) (void *udata, void *ptr);
typedef void (*duk_fatal_function) (duk_context *ctx, duk_errcode_t code, const char *msg);
typedef void (*duk_decode_char_function) (void *udata, duk_codepoint_t codepoint);
typedef duk_codepoint_t (*duk_map_char_function) (void *udata, duk_codepoint_t codepoint);
typedef duk_ret_t (*duk_safe_call_function) (duk_context *ctx);

struct duk_memory_functions {
	duk_alloc_function alloc_func;
	duk_realloc_function realloc_func;
	duk_free_function free_func;
	void *udata;
};

struct duk_function_list_entry {
	const char *key;
	duk_c_function value;
	duk_idx_t nargs;
};

struct duk_number_list_entry {
	const char *key;
	duk_double_t value;
};

/*
 *  Context management
 */

DUK_EXTERNAL_DECL
duk_context *duk_create_heap(duk_alloc_function alloc_func,
                             duk_realloc_function realloc_func,
                             duk_free_function free_func,
                             void *alloc_udata,
                             duk_fatal_function fatal_handler);
DUK_EXTERNAL_DECL void duk_destroy_heap(duk_context *ctx);

#define duk_create_heap_default() \
	duk_create_heap(NULL, NULL, NULL, NULL, NULL)

/*
 *  Memory management
 *
 *  Raw functions have no side effects (cannot trigger GC).
 */

DUK_EXTERNAL_DECL void *duk_alloc_raw(duk_context *ctx, duk_size_t size);
DUK_EXTERNAL_DECL void duk_free_raw(duk_context *ctx, void *ptr);
DUK_EXTERNAL_DECL void *duk_realloc_raw(duk_context *ctx, void *ptr, duk_size_t size);
DUK_EXTERNAL_DECL void *duk_alloc(duk_context *ctx, duk_size_t size);
DUK_EXTERNAL_DECL void duk_free(duk_context *ctx, void *ptr);
DUK_EXTERNAL_DECL void *duk_realloc(duk_context *ctx, void *ptr, duk_size_t size);
DUK_EXTERNAL_DECL void duk_get_memory_functions(duk_context *ctx, duk_memory_functions *out_funcs);
DUK_EXTERNAL_DECL void duk_gc(duk_context *ctx, duk_uint_t flags);

/*
 *  Error handling
 */

DUK_API_NORETURN(DUK_EXTERNAL_DECL void duk_throw(duk_context *ctx));

DUK_API_NORETURN(DUK_EXTERNAL_DECL void duk_error_raw(duk_context *ctx, duk_errcode_t err_code, const char *filename, duk_int_t line, const char *fmt, ...));
#ifdef DUK_API_VARIADIC_MACROS
#define duk_error(ctx,err_code,...)  \
	duk_error_raw((ctx), (duk_errcode_t) (err_code), __FILE__, (duk_int_t) __LINE__, __VA_ARGS__)
#else
DUK_API_NORETURN(DUK_EXTERNAL_DECL void duk_error_stash(duk_context *ctx, duk_errcode_t err_code, const char *fmt, ...));
/* One problem with this macro is that expressions like the following fail
 * to compile: "(void) duk_error(...)".  But because duk_error() is noreturn,
 * they make little sense anyway.
 */
#define duk_error  \
	duk_api_global_filename = __FILE__, \
	duk_api_global_line = (duk_int_t) __LINE__, \
	duk_error_stash  /* arguments follow */
#endif

DUK_API_NORETURN(DUK_EXTERNAL_DECL void duk_fatal(duk_context *ctx, duk_errcode_t err_code, const char *err_msg));

/*
 *  Other state related functions
 */

DUK_EXTERNAL_DECL duk_bool_t duk_is_strict_call(duk_context *ctx);
DUK_EXTERNAL_DECL duk_bool_t duk_is_constructor_call(duk_context *ctx);

/*
 *  Stack management
 */

DUK_EXTERNAL_DECL duk_idx_t duk_normalize_index(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_idx_t duk_require_normalize_index(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_valid_index(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_require_valid_index(duk_context *ctx, duk_idx_t index);

DUK_EXTERNAL_DECL duk_idx_t duk_get_top(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_set_top(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_idx_t duk_get_top_index(duk_context *ctx);
DUK_EXTERNAL_DECL duk_idx_t duk_require_top_index(duk_context *ctx);

/* Although extra/top could be an unsigned type here, using a signed type
 * makes the API more robust to calling code calculation errors or corner
 * cases (where caller might occasionally come up with negative values).
 * Negative values are treated as zero, which is better than casting them
 * to a large unsigned number.  (This principle is used elsewhere in the
 * API too.)
 */
DUK_EXTERNAL_DECL duk_bool_t duk_check_stack(duk_context *ctx, duk_idx_t extra);
DUK_EXTERNAL_DECL void duk_require_stack(duk_context *ctx, duk_idx_t extra);
DUK_EXTERNAL_DECL duk_bool_t duk_check_stack_top(duk_context *ctx, duk_idx_t top);
DUK_EXTERNAL_DECL void duk_require_stack_top(duk_context *ctx, duk_idx_t top);

/*
 *  Stack manipulation (other than push/pop)
 */

DUK_EXTERNAL_DECL void duk_swap(duk_context *ctx, duk_idx_t index1, duk_idx_t index2);
DUK_EXTERNAL_DECL void duk_swap_top(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_dup(duk_context *ctx, duk_idx_t from_index);
DUK_EXTERNAL_DECL void duk_dup_top(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_insert(duk_context *ctx, duk_idx_t to_index);
DUK_EXTERNAL_DECL void duk_replace(duk_context *ctx, duk_idx_t to_index);
DUK_EXTERNAL_DECL void duk_copy(duk_context *ctx, duk_idx_t from_index, duk_idx_t to_index);
DUK_EXTERNAL_DECL void duk_remove(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_xcopymove_raw(duk_context *to_ctx, duk_context *from_ctx, duk_idx_t count, duk_bool_t is_copy);

#define duk_xmove_top(to_ctx,from_ctx,count) \
	duk_xcopymove_raw((to_ctx), (from_ctx), (count), 0 /*is_copy*/)
#define duk_xcopy_top(to_ctx,from_ctx,count) \
	duk_xcopymove_raw((to_ctx), (from_ctx), (count), 1 /*is_copy*/)

/*
 *  Push operations
 *
 *  Push functions return the absolute (relative to bottom of frame)
 *  position of the pushed value for convenience.
 *
 *  Note: duk_dup() is technically a push.
 */

DUK_EXTERNAL_DECL void duk_push_undefined(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_push_null(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_push_boolean(duk_context *ctx, duk_bool_t val);
DUK_EXTERNAL_DECL void duk_push_true(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_push_false(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_push_number(duk_context *ctx, duk_double_t val);
DUK_EXTERNAL_DECL void duk_push_nan(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_push_int(duk_context *ctx, duk_int_t val);
DUK_EXTERNAL_DECL void duk_push_uint(duk_context *ctx, duk_uint_t val);
DUK_EXTERNAL_DECL const char *duk_push_string(duk_context *ctx, const char *str);
DUK_EXTERNAL_DECL const char *duk_push_lstring(duk_context *ctx, const char *str, duk_size_t len);
DUK_EXTERNAL_DECL void duk_push_pointer(duk_context *ctx, void *p);
DUK_EXTERNAL_DECL const char *duk_push_sprintf(duk_context *ctx, const char *fmt, ...);
DUK_EXTERNAL_DECL const char *duk_push_vsprintf(duk_context *ctx, const char *fmt, va_list ap);

DUK_EXTERNAL_DECL const char *duk_push_string_file_raw(duk_context *ctx, const char *path, duk_uint_t flags);
#define duk_push_string_file(ctx,path) \
	duk_push_string_file_raw((ctx), (path), 0)

DUK_EXTERNAL_DECL void duk_push_this(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_push_current_function(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_push_current_thread(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_push_global_object(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_push_heap_stash(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_push_global_stash(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_push_thread_stash(duk_context *ctx, duk_context *target_ctx);

DUK_EXTERNAL_DECL duk_idx_t duk_push_object(duk_context *ctx);
DUK_EXTERNAL_DECL duk_idx_t duk_push_array(duk_context *ctx);
DUK_EXTERNAL_DECL duk_idx_t duk_push_c_function(duk_context *ctx, duk_c_function func, duk_idx_t nargs);
DUK_EXTERNAL_DECL duk_idx_t duk_push_thread_raw(duk_context *ctx, duk_uint_t flags);

#define duk_push_thread(ctx) \
	duk_push_thread_raw((ctx), 0 /*flags*/)

#define duk_push_thread_new_globalenv(ctx) \
	duk_push_thread_raw((ctx), DUK_THREAD_NEW_GLOBAL_ENV /*flags*/)

DUK_EXTERNAL_DECL duk_idx_t duk_push_error_object_raw(duk_context *ctx, duk_errcode_t err_code, const char *filename, duk_int_t line, const char *fmt, ...);
#ifdef DUK_API_VARIADIC_MACROS
#define duk_push_error_object(ctx,err_code,...)  \
	duk_push_error_object_raw((ctx),(err_code),__FILE__,__LINE__,__VA_ARGS__)
#else
DUK_EXTERNAL_DECL duk_idx_t duk_push_error_object_stash(duk_context *ctx, duk_errcode_t err_code, const char *fmt, ...);
#define duk_push_error_object  \
	duk_api_global_filename = __FILE__, \
	duk_api_global_line = __LINE__, \
	duk_push_error_object_stash  /* arguments follow */
#endif

DUK_EXTERNAL_DECL void *duk_push_buffer(duk_context *ctx, duk_size_t size, duk_bool_t dynamic);
DUK_EXTERNAL_DECL void *duk_push_fixed_buffer(duk_context *ctx, duk_size_t size);
DUK_EXTERNAL_DECL void *duk_push_dynamic_buffer(duk_context *ctx, duk_size_t size);

/*
 *  Pop operations
 */

DUK_EXTERNAL_DECL void duk_pop(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_pop_n(duk_context *ctx, duk_idx_t count);
DUK_EXTERNAL_DECL void duk_pop_2(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_pop_3(duk_context *ctx);

/*
 *  Type checks
 *
 *  duk_is_none(), which would indicate whether index it outside of stack,
 *  is not needed; duk_is_valid_index() gives the same information.
 */

DUK_EXTERNAL_DECL duk_int_t duk_get_type(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_check_type(duk_context *ctx, duk_idx_t index, duk_int_t type);
DUK_EXTERNAL_DECL duk_uint_t duk_get_type_mask(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_check_type_mask(duk_context *ctx, duk_idx_t index, duk_uint_t mask);

DUK_EXTERNAL_DECL duk_bool_t duk_is_undefined(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_null(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_null_or_undefined(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_boolean(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_number(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_nan(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_string(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_object(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_buffer(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_pointer(duk_context *ctx, duk_idx_t index);

DUK_EXTERNAL_DECL duk_bool_t duk_is_array(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_function(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_c_function(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_ecmascript_function(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_bound_function(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_thread(duk_context *ctx, duk_idx_t index);

DUK_EXTERNAL_DECL duk_bool_t duk_is_callable(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_dynamic_buffer(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_is_fixed_buffer(duk_context *ctx, duk_idx_t index);

DUK_EXTERNAL_DECL duk_bool_t duk_is_primitive(duk_context *ctx, duk_idx_t index);
#define duk_is_object_coercible(ctx,index) \
	duk_check_type_mask((ctx), (index), DUK_TYPE_MASK_BOOLEAN | \
	                                    DUK_TYPE_MASK_NUMBER | \
	                                    DUK_TYPE_MASK_STRING | \
	                                    DUK_TYPE_MASK_OBJECT | \
	                                    DUK_TYPE_MASK_BUFFER | \
	                                    DUK_TYPE_MASK_POINTER)

/*
 *  Get operations: no coercion, returns default value for invalid
 *  indices and invalid value types.
 *
 *  duk_get_undefined() and duk_get_null() would be pointless and
 *  are not included.
 */

DUK_EXTERNAL_DECL duk_bool_t duk_get_boolean(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_double_t duk_get_number(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_int_t duk_get_int(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_uint_t duk_get_uint(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL const char *duk_get_string(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL const char *duk_get_lstring(duk_context *ctx, duk_idx_t index, duk_size_t *out_len);
DUK_EXTERNAL_DECL void *duk_get_buffer(duk_context *ctx, duk_idx_t index, duk_size_t *out_size);
DUK_EXTERNAL_DECL void *duk_get_pointer(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_c_function duk_get_c_function(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_context *duk_get_context(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_size_t duk_get_length(duk_context *ctx, duk_idx_t index);

/*
 *  Require operations: no coercion, throw error if index or type
 *  is incorrect.  No defaulting.
 */

#define duk_require_type_mask(ctx,index,mask) \
	((void) duk_check_type_mask((ctx), (index), (mask) | DUK_TYPE_MASK_THROW))

DUK_EXTERNAL_DECL void duk_require_undefined(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_require_null(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_require_boolean(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_double_t duk_require_number(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_int_t duk_require_int(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_uint_t duk_require_uint(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL const char *duk_require_string(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL const char *duk_require_lstring(duk_context *ctx, duk_idx_t index, duk_size_t *out_len);
DUK_EXTERNAL_DECL void *duk_require_buffer(duk_context *ctx, duk_idx_t index, duk_size_t *out_size);
DUK_EXTERNAL_DECL void *duk_require_pointer(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_c_function duk_require_c_function(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_context *duk_require_context(duk_context *ctx, duk_idx_t index);

#define duk_require_object_coercible(ctx,index) \
	((void) duk_check_type_mask((ctx), (index), DUK_TYPE_MASK_BOOLEAN | \
	                                            DUK_TYPE_MASK_NUMBER | \
	                                            DUK_TYPE_MASK_STRING | \
	                                            DUK_TYPE_MASK_OBJECT | \
	                                            DUK_TYPE_MASK_BUFFER | \
	                                            DUK_TYPE_MASK_POINTER | \
	                                            DUK_TYPE_MASK_THROW))

/*
 *  Coercion operations: in-place coercion, return coerced value where
 *  applicable.  If index is invalid, throw error.  Some coercions may
 *  throw an expected error (e.g. from a toString() or valueOf() call)
 *  or an internal error (e.g. from out of memory).
 */

DUK_EXTERNAL_DECL void duk_to_undefined(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_to_null(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_bool_t duk_to_boolean(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_double_t duk_to_number(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_int_t duk_to_int(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_uint_t duk_to_uint(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_int32_t duk_to_int32(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_uint32_t duk_to_uint32(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_uint16_t duk_to_uint16(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL const char *duk_to_string(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL const char *duk_to_lstring(duk_context *ctx, duk_idx_t index, duk_size_t *out_len);
DUK_EXTERNAL_DECL void *duk_to_buffer(duk_context *ctx, duk_idx_t index, duk_size_t *out_size);
DUK_EXTERNAL_DECL void *duk_to_fixed_buffer(duk_context *ctx, duk_idx_t index, duk_size_t *out_size);
DUK_EXTERNAL_DECL void *duk_to_dynamic_buffer(duk_context *ctx, duk_idx_t index, duk_size_t *out_size);
DUK_EXTERNAL_DECL void *duk_to_pointer(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_to_object(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_to_defaultvalue(duk_context *ctx, duk_idx_t index, duk_int_t hint);
DUK_EXTERNAL_DECL void duk_to_primitive(duk_context *ctx, duk_idx_t index, duk_int_t hint);

/* safe variants of a few coercion operations */
DUK_EXTERNAL_DECL const char *duk_safe_to_lstring(duk_context *ctx, duk_idx_t index, duk_size_t *out_len);
#define duk_safe_to_string(ctx,index) \
	duk_safe_to_lstring((ctx), (index), NULL)

/*
 *  Misc conversion
 */

DUK_EXTERNAL_DECL const char *duk_base64_encode(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_base64_decode(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL const char *duk_hex_encode(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_hex_decode(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL const char *duk_json_encode(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_json_decode(duk_context *ctx, duk_idx_t index);

/*
 *  Buffer
 */

DUK_EXTERNAL_DECL void *duk_resize_buffer(duk_context *ctx, duk_idx_t index, duk_size_t new_size);

/*
 *  Property access
 *
 *  The basic function assumes key is on stack.  The _string variant takes
 *  a C string as a property name, while the _index variant takes an array
 *  index as a property name (e.g. 123 is equivalent to the key "123").
 */

DUK_EXTERNAL_DECL duk_bool_t duk_get_prop(duk_context *ctx, duk_idx_t obj_index);
DUK_EXTERNAL_DECL duk_bool_t duk_get_prop_string(duk_context *ctx, duk_idx_t obj_index, const char *key);
DUK_EXTERNAL_DECL duk_bool_t duk_get_prop_index(duk_context *ctx, duk_idx_t obj_index, duk_uarridx_t arr_index);
DUK_EXTERNAL_DECL duk_bool_t duk_put_prop(duk_context *ctx, duk_idx_t obj_index);
DUK_EXTERNAL_DECL duk_bool_t duk_put_prop_string(duk_context *ctx, duk_idx_t obj_index, const char *key);
DUK_EXTERNAL_DECL duk_bool_t duk_put_prop_index(duk_context *ctx, duk_idx_t obj_index, duk_uarridx_t arr_index);
DUK_EXTERNAL_DECL duk_bool_t duk_del_prop(duk_context *ctx, duk_idx_t obj_index);
DUK_EXTERNAL_DECL duk_bool_t duk_del_prop_string(duk_context *ctx, duk_idx_t obj_index, const char *key);
DUK_EXTERNAL_DECL duk_bool_t duk_del_prop_index(duk_context *ctx, duk_idx_t obj_index, duk_uarridx_t arr_index);
DUK_EXTERNAL_DECL duk_bool_t duk_has_prop(duk_context *ctx, duk_idx_t obj_index);
DUK_EXTERNAL_DECL duk_bool_t duk_has_prop_string(duk_context *ctx, duk_idx_t obj_index, const char *key);
DUK_EXTERNAL_DECL duk_bool_t duk_has_prop_index(duk_context *ctx, duk_idx_t obj_index, duk_uarridx_t arr_index);

DUK_EXTERNAL_DECL duk_bool_t duk_get_global_string(duk_context *ctx, const char *key);
DUK_EXTERNAL_DECL duk_bool_t duk_put_global_string(duk_context *ctx, const char *key);

/*
 *  Object prototype
 */

DUK_EXTERNAL_DECL void duk_get_prototype(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_set_prototype(duk_context *ctx, duk_idx_t index);

/*
 *  Object finalizer
 */

DUK_EXTERNAL_DECL void duk_get_finalizer(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_set_finalizer(duk_context *ctx, duk_idx_t index);

/*
 *  Global object
 */

DUK_EXTERNAL_DECL void duk_set_global_object(duk_context *ctx);

/*
 *  Duktape/C function magic value
 */

DUK_EXTERNAL_DECL duk_int_t duk_get_magic(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL void duk_set_magic(duk_context *ctx, duk_idx_t index, duk_int_t magic);
DUK_EXTERNAL_DECL duk_int_t duk_get_current_magic(duk_context *ctx);

/*
 *  Module helpers: put multiple function or constant properties
 */

DUK_EXTERNAL_DECL void duk_put_function_list(duk_context *ctx, duk_idx_t obj_index, const duk_function_list_entry *funcs);
DUK_EXTERNAL_DECL void duk_put_number_list(duk_context *ctx, duk_idx_t obj_index, const duk_number_list_entry *numbers);

/*
 *  Variable access
 */

/* XXX: These calls are incomplete and not usable now.  They are not (yet)
 * part of the public API.
 */
DUK_EXTERNAL_DECL void duk_get_var(duk_context *ctx);
DUK_EXTERNAL_DECL void duk_put_var(duk_context *ctx);
DUK_EXTERNAL_DECL duk_bool_t duk_del_var(duk_context *ctx);
DUK_EXTERNAL_DECL duk_bool_t duk_has_var(duk_context *ctx);

/*
 *  Object operations
 */

DUK_EXTERNAL_DECL void duk_compact(duk_context *ctx, duk_idx_t obj_index);
DUK_EXTERNAL_DECL void duk_enum(duk_context *ctx, duk_idx_t obj_index, duk_uint_t enum_flags);
DUK_EXTERNAL_DECL duk_bool_t duk_next(duk_context *ctx, duk_idx_t enum_index, duk_bool_t get_value);

/*
 *  String manipulation
 */

DUK_EXTERNAL_DECL void duk_concat(duk_context *ctx, duk_idx_t count);
DUK_EXTERNAL_DECL void duk_join(duk_context *ctx, duk_idx_t count);
DUK_EXTERNAL_DECL void duk_decode_string(duk_context *ctx, duk_idx_t index, duk_decode_char_function callback, void *udata);
DUK_EXTERNAL_DECL void duk_map_string(duk_context *ctx, duk_idx_t index, duk_map_char_function callback, void *udata);
DUK_EXTERNAL_DECL void duk_substring(duk_context *ctx, duk_idx_t index, duk_size_t start_char_offset, duk_size_t end_char_offset);
DUK_EXTERNAL_DECL void duk_trim(duk_context *ctx, duk_idx_t index);
DUK_EXTERNAL_DECL duk_codepoint_t duk_char_code_at(duk_context *ctx, duk_idx_t index, duk_size_t char_offset);

/*
 *  Ecmascript operators
 */

DUK_EXTERNAL_DECL duk_bool_t duk_equals(duk_context *ctx, duk_idx_t index1, duk_idx_t index2);
DUK_EXTERNAL_DECL duk_bool_t duk_strict_equals(duk_context *ctx, duk_idx_t index1, duk_idx_t index2);

/*
 *  Function (method) calls
 */

DUK_EXTERNAL_DECL void duk_call(duk_context *ctx, duk_idx_t nargs);
DUK_EXTERNAL_DECL void duk_call_method(duk_context *ctx, duk_idx_t nargs);
DUK_EXTERNAL_DECL void duk_call_prop(duk_context *ctx, duk_idx_t obj_index, duk_idx_t nargs);
DUK_EXTERNAL_DECL duk_int_t duk_pcall(duk_context *ctx, duk_idx_t nargs);
DUK_EXTERNAL_DECL duk_int_t duk_pcall_method(duk_context *ctx, duk_idx_t nargs);
DUK_EXTERNAL_DECL duk_int_t duk_pcall_prop(duk_context *ctx, duk_idx_t obj_index, duk_idx_t nargs);
DUK_EXTERNAL_DECL void duk_new(duk_context *ctx, duk_idx_t nargs);
DUK_EXTERNAL_DECL duk_int_t duk_safe_call(duk_context *ctx, duk_safe_call_function func, duk_idx_t nargs, duk_idx_t nrets);

/*
 *  Thread management
 */

/* There are currently no native functions to yield/resume, due to the internal
 * limitations on coroutine handling.  These will be added later.
 */

/*
 *  Compilation and evaluation
 */

DUK_EXTERNAL_DECL duk_int_t duk_eval_raw(duk_context *ctx, const char *src_buffer, duk_size_t src_length, duk_uint_t flags);
DUK_EXTERNAL_DECL duk_int_t duk_compile_raw(duk_context *ctx, const char *src_buffer, duk_size_t src_length, duk_uint_t flags);

/* plain */
#define duk_eval(ctx)  \
	((void) duk_push_string((ctx), __FILE__), \
	 (void) duk_eval_raw((ctx), NULL, 0, DUK_COMPILE_EVAL))

#define duk_eval_noresult(ctx)  \
	((void) duk_push_string((ctx), __FILE__), \
	 (void) duk_eval_raw((ctx), NULL, 0, DUK_COMPILE_EVAL | DUK_COMPILE_NORESULT))

#define duk_peval(ctx)  \
	((void) duk_push_string((ctx), __FILE__), \
	 duk_eval_raw((ctx), NULL, 0, DUK_COMPILE_EVAL | DUK_COMPILE_SAFE))

#define duk_peval_noresult(ctx)  \
	((void) duk_push_string((ctx), __FILE__), \
	 duk_eval_raw((ctx), NULL, 0, DUK_COMPILE_EVAL | DUK_COMPILE_SAFE | DUK_COMPILE_NORESULT))

#define duk_compile(ctx,flags)  \
	((void) duk_compile_raw((ctx), NULL, 0, (flags)))

#define duk_pcompile(ctx,flags)  \
	(duk_compile_raw((ctx), NULL, 0, (flags) | DUK_COMPILE_SAFE))

/* string */
#define duk_eval_string(ctx,src)  \
	((void) duk_push_string((ctx), __FILE__), \
	 (void) duk_eval_raw((ctx), (src), 0, DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN))

#define duk_eval_string_noresult(ctx,src)  \
	((void) duk_push_string((ctx), __FILE__), \
	 (void) duk_eval_raw((ctx), (src), 0, DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN | DUK_COMPILE_NORESULT))

#define duk_peval_string(ctx,src)  \
	((void) duk_push_string((ctx), __FILE__), \
	 duk_eval_raw((ctx), (src), 0, DUK_COMPILE_EVAL | DUK_COMPILE_SAFE | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN))

#define duk_peval_string_noresult(ctx,src)  \
	((void) duk_push_string((ctx), __FILE__), \
	 duk_eval_raw((ctx), (src), 0, DUK_COMPILE_EVAL | DUK_COMPILE_SAFE | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN | DUK_COMPILE_NORESULT))

#define duk_compile_string(ctx,flags,src)  \
	((void) duk_push_string((ctx), __FILE__), \
	 (void) duk_compile_raw((ctx), (src), 0, (flags) | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN))

#define duk_compile_string_filename(ctx,flags,src)  \
	((void) duk_compile_raw((ctx), (src), 0, (flags) | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN))

#define duk_pcompile_string(ctx,flags,src)  \
	((void) duk_push_string((ctx), __FILE__), \
	 duk_compile_raw((ctx), (src), 0, (flags) | DUK_COMPILE_SAFE | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN))

#define duk_pcompile_string_filename(ctx,flags,src)  \
	(duk_compile_raw((ctx), (src), 0, (flags) | DUK_COMPILE_SAFE | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN))

/* lstring */
#define duk_eval_lstring(ctx,buf,len)  \
	((void) duk_push_string((ctx), __FILE__), \
	 (void) duk_eval_raw((ctx), buf, len, DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE))

#define duk_eval_lstring_noresult(ctx,buf,len)  \
	((void) duk_push_string((ctx), __FILE__), \
	 (void) duk_eval_raw((ctx), buf, len, DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE | DUK_COMPILE_NORESULT))

#define duk_peval_lstring(ctx,buf,len)  \
	((void) duk_push_string((ctx), __FILE__), \
	 duk_eval_raw((ctx), buf, len, DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE | DUK_COMPILE_SAFE))

#define duk_peval_lstring_noresult(ctx,buf,len)  \
	((void) duk_push_string((ctx), __FILE__), \
	 duk_eval_raw((ctx), buf, len, DUK_COMPILE_EVAL | DUK_COMPILE_SAFE | DUK_COMPILE_NOSOURCE | DUK_COMPILE_NORESULT))

#define duk_compile_lstring(ctx,flags,buf,len)  \
	((void) duk_push_string((ctx), __FILE__), \
	 (void) duk_compile_raw((ctx), buf, len, (flags) | DUK_COMPILE_NOSOURCE))

#define duk_compile_lstring_filename(ctx,flags,buf,len)  \
	((void) duk_compile_raw((ctx), buf, len, (flags) | DUK_COMPILE_NOSOURCE))

#define duk_pcompile_lstring(ctx,flags,buf,len)  \
	((void) duk_push_string((ctx), __FILE__), \
	 duk_compile_raw((ctx), buf, len, (flags) | DUK_COMPILE_SAFE | DUK_COMPILE_NOSOURCE))

#define duk_pcompile_lstring_filename(ctx,flags,buf,len)  \
	(duk_compile_raw((ctx), buf, len, (flags) | DUK_COMPILE_SAFE | DUK_COMPILE_NOSOURCE))

/* file */
#define duk_eval_file(ctx,path)  \
	((void) duk_push_string_file_raw((ctx), (path), 0), \
	 (void) duk_push_string((ctx), (path)), \
	 (void) duk_eval_raw((ctx), NULL, 0, DUK_COMPILE_EVAL))

#define duk_eval_file_noresult(ctx,path)  \
	((void) duk_push_string_file_raw((ctx), (path), 0), \
	 (void) duk_push_string((ctx), (path)), \
	 (void) duk_eval_raw((ctx), NULL, 0, DUK_COMPILE_EVAL | DUK_COMPILE_NORESULT))

#define duk_peval_file(ctx,path)  \
	((void) duk_push_string_file_raw((ctx), (path), DUK_STRING_PUSH_SAFE), \
	 (void) duk_push_string((ctx), (path)), \
	 duk_eval_raw((ctx), NULL, 0, DUK_COMPILE_EVAL | DUK_COMPILE_SAFE))

#define duk_peval_file_noresult(ctx,path)  \
	((void) duk_push_string_file_raw((ctx), (path), DUK_STRING_PUSH_SAFE), \
	 (void) duk_push_string((ctx), (path)), \
	 duk_eval_raw((ctx), NULL, 0, DUK_COMPILE_EVAL | DUK_COMPILE_SAFE | DUK_COMPILE_NORESULT))

#define duk_compile_file(ctx,flags,path)  \
	((void) duk_push_string_file_raw((ctx), (path), 0), \
	 (void) duk_push_string((ctx), (path)), \
	 (void) duk_compile_raw((ctx), NULL, 0, (flags)))

#define duk_pcompile_file(ctx,flags,path)  \
	((void) duk_push_string_file_raw((ctx), (path), DUK_STRING_PUSH_SAFE), \
	 (void) duk_push_string((ctx), (path)), \
	 duk_compile_raw((ctx), NULL, 0, (flags) | DUK_COMPILE_SAFE))

/*
 *  Logging
 */

DUK_EXTERNAL_DECL void duk_log(duk_context *ctx, duk_int_t level, const char *fmt, ...);

/*
 *  Debugging
 */

DUK_EXTERNAL_DECL void duk_push_context_dump(duk_context *ctx);
