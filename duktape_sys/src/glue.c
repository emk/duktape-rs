#include "duktape.h"

/// A custom add-on to the duktape API, replacing the macro
/// `duk_push_error_object`,
extern duk_idx_t
duk_push_error_object_string(duk_context *ctx, duk_errcode_t err_code,
                             const char *filename, duk_int_t line,
                             const char *message)
{
    return duk_push_error_object_raw(ctx, err_code, filename, line, "%s",
                                     message);
}
