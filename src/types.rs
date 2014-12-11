use libc::types::os::arch::c95::c_double;
use std::str::CowString;
use cesu8::to_cesu8;
use ffi::*;
use errors::*;
use context::Context;

/// A value that can be passed to and from JavaScript.  This does not
/// include all the types that can be stored internally!
#[deriving(Show, PartialEq)]
pub enum Value<'a> {
    /// An undefined JavaScript value.
    Undefined,
    /// A JavaScript `null` value.
    Null,
    /// A JavaScript boolean value.
    Bool(bool),
    /// A JavaScript numeric value.
    Number(c_double),
    /// A JavaScript string value.
    String(CowString<'a>)
}

/// Translates Rust values into JavaScript values.
pub struct Encoder<'a> {
    ctx: &'a mut Context
}

impl<'a> Encoder<'a> {
    /// Create a new encoder which pushes values to the stack of a `Context`.
    pub unsafe fn new(ctx: &mut Context) -> Encoder {
        Encoder{ctx: ctx}
    }
}

type EncodeResult = DuktapeResult<()>;

impl<'a> ::serialize::Encoder<DuktapeError> for Encoder<'a> {
    fn emit_nil(&mut self) -> EncodeResult {
        unsafe { duk_push_null(self.ctx.as_mut_ptr()) }; Ok(())
    }

    // Integral types map to floats.
    fn emit_uint(&mut self, v: uint) -> EncodeResult { self.emit_f64(v as f64) }
    fn emit_u64(&mut self, v: u64) -> EncodeResult { self.emit_f64(v as f64) }
    fn emit_u32(&mut self, v: u32) -> EncodeResult { self.emit_f64(v as f64) }
    fn emit_u16(&mut self, v: u16) -> EncodeResult { self.emit_f64(v as f64) }
    fn emit_u8(&mut self, v: u8) -> EncodeResult  { self.emit_f64(v as f64) }
    fn emit_int(&mut self, v: int) -> EncodeResult { self.emit_f64(v as f64) }
    fn emit_i64(&mut self, v: i64) -> EncodeResult { self.emit_f64(v as f64) }
    fn emit_i32(&mut self, v: i32) -> EncodeResult { self.emit_f64(v as f64) }
    fn emit_i16(&mut self, v: i16) -> EncodeResult { self.emit_f64(v as f64) }
    fn emit_i8(&mut self, v: i8) -> EncodeResult  { self.emit_f64(v as f64) }

    fn emit_bool(&mut self, v: bool) -> EncodeResult {
        unsafe { duk_push_boolean(self.ctx.as_mut_ptr(), if v { 1 } else { 0 }) }
        Ok(())
    }

    fn emit_f64(&mut self, v: f64) -> EncodeResult {
        unsafe {duk_push_number(self.ctx.as_mut_ptr(), v) }; Ok(())
    }
    fn emit_f32(&mut self, v: f32) -> EncodeResult { self.emit_f64(v as f64) }

    fn emit_char(&mut self, v: char) -> EncodeResult {
        let s = v.to_string();
        self.emit_str(s.as_slice())
    }
    fn emit_str(&mut self, v: &str) -> EncodeResult {
        let encoded = to_cesu8(v.deref());
        let buf = encoded.deref();
        unsafe {
            duk_push_lstring(self.ctx.as_mut_ptr(), buf.as_ptr() as *const i8,
                             buf.len() as duk_size_t);
        }
        Ok(())
    }

    fn emit_enum(&mut self, _name: &str,
                 f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult
    {
        f(self)
    }

    #[allow(unused_variables)]
    fn emit_enum_variant(&mut self, v_name: &str, v_id: uint,
                         len: uint, f: |&mut Encoder<'a>| -> EncodeResult) ->
        EncodeResult
    {
        if len == 0 {
            self.emit_str(v_name.as_slice())
        } else {
            unsafe {
                duk_push_object(self.ctx.as_mut_ptr());
                self.emit_str("variant").unwrap();
                self.emit_str(v_name.as_slice()).unwrap();
                duk_put_prop(self.ctx.as_mut_ptr(), -3);

                self.emit_str("fields").unwrap();
                duk_push_array(self.ctx.as_mut_ptr());
                f(self).unwrap();
                duk_put_prop(self.ctx.as_mut_ptr(), -3);
            }
            Ok(())
        }
    }

    #[allow(unused_variables)]
    fn emit_enum_variant_arg(&mut self, a_idx: uint,
                             f: |&mut Encoder<'a>| -> EncodeResult) ->
        EncodeResult
    {
        unsafe {
            f(self).unwrap();
            duk_put_prop_index(self.ctx.as_mut_ptr(), -2, a_idx as u32);
        }
        Ok(())
    }

    #[allow(unused_variables)]
    fn emit_enum_struct_variant(&mut self, v_name: &str, v_id: uint, len: uint,
                                f: |&mut Encoder<'a>| -> EncodeResult) ->
        EncodeResult
    {
        // TODO: Not called during normal serialization.
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_enum_struct_variant_field(&mut self, f_name: &str, f_idx: uint,
                                      f: |&mut Encoder<'a>| -> EncodeResult) ->
        EncodeResult
    {
        // TODO: Not called during normal serialization.
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_struct(&mut self, name: &str, len: uint,
                   f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult
    {
        unsafe {
            duk_push_object(self.ctx.as_mut_ptr());
        }
        f(self)
    }

    #[allow(unused_variables)]
    fn emit_struct_field(&mut self, f_name: &str, f_idx: uint,
                         f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult
    {
        self.emit_str(f_name).unwrap();
        f(self).unwrap();
        unsafe { duk_put_prop(self.ctx.as_mut_ptr(), -3); }
        Ok(())
    }

    #[allow(unused_variables)]
    fn emit_tuple(&mut self, len: uint, f: |&mut Encoder<'a>| -> EncodeResult) ->
        EncodeResult
    {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_tuple_arg(&mut self, idx: uint,
                      f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult
    {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_tuple_struct(&mut self, name: &str, len: uint,
                         f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult
    {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_tuple_struct_arg(&mut self, f_idx: uint,
                             f: |&mut Encoder<'a>| -> EncodeResult) ->
        EncodeResult
    {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_option(&mut self, f: |&mut Encoder<'a>| -> EncodeResult) ->
        EncodeResult
    {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_option_none(&mut self) -> EncodeResult
    {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_option_some(&mut self, f: |&mut Encoder<'a>| -> EncodeResult) ->
        EncodeResult
    {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_seq(&mut self, len: uint, f: |this: &mut Encoder<'a>| ->
                EncodeResult) -> EncodeResult
    {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_seq_elt(&mut self, idx: uint,
                    f: |this: &mut Encoder<'a>| -> EncodeResult) -> EncodeResult
    {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_map(&mut self, len: uint, f: |&mut Encoder<'a>| -> EncodeResult) ->
        EncodeResult
    {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_map_elt_key(&mut self, idx: uint,
                        f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult
    {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_map_elt_val(&mut self, idx: uint,
                        f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult
    {
        unimplemented!()
    }
}

#[test]
fn test_encoder() {
    use std::borrow::Cow;

    let mut ctx = Context::new().unwrap();
    ctx.eval(r"
function assert_json(expected, value) {
    var value_json = JSON.stringify(value);
    print('checking', expected, value_json);
    return expected == value_json || value_json;
}").unwrap();

    unsafe fn assert_json_setup(ctx: &mut Context, expected: &str) {
        duk_push_global_object(ctx.as_mut_ptr());
        "assert_json".with_c_str(|c_str| {
            duk_get_prop_string(ctx.as_mut_ptr(), -1, c_str);
        });
        ctx.push(&Value::String(Cow::Borrowed(expected)));
    }

    unsafe fn assert_json_call(ctx: &mut Context) ->
        DuktapeResult<Value<'static>>
    {
        let status = duk_pcall(ctx.as_mut_ptr(), 2i32);
        let result = ctx.pop_result(status);
        duk_pop(ctx.as_mut_ptr()); // Remove global object.
        result
    }

    macro_rules! assert_encode {
        ($val:expr) => {
            unsafe {
                let v = $val;
                let expected = ::serialize::json::encode(&v);
                assert_json_setup(&mut ctx, expected.as_slice());
                ctx.push2(&v);
                match assert_json_call(&mut ctx) {
                    Ok(Value::Bool(true)) => {},
                    Ok(Value::String(ref got)) =>
                        panic!("expected {}, got {}", expected, got),
                    ref result => panic!("unexpected value: {}", result)
                }
            }
        }
    }

    // Simple types.
    assert_encode!(1u);
    assert_encode!(1u64);
    assert_encode!(1u32);
    assert_encode!(1u16);
    assert_encode!(1u8);
    assert_encode!(-1i);
    assert_encode!(-1i64);
    assert_encode!(-1i32);
    assert_encode!(-1i16);
    assert_encode!(-1i8);
    assert_encode!(true);
    assert_encode!(false);
    assert_encode!(1.0f64);
    assert_encode!(1.0f32);
    assert_encode!("string");
    // serialize::json::encode handles characters below U+10000 incorrectly.
    //assert_encode!('c'); // https://github.com/rust-lang/rust/issues/19719
    assert_encode!('𓀀');

    // Enums.
    #[deriving(Encodable)]
    enum ExEnum { Foo, Bar(f64), Baz{x: f64, y: f64} }
    assert_encode!(&ExEnum::Foo);
    assert_encode!(&ExEnum::Bar(1.0));
    assert_encode!(&ExEnum::Baz{x: 1.0, y: 2.0});

    // Structs.
    #[deriving(Encodable)]
    struct ExStruct { x: f64, y: f64 }
    assert_encode!(&ExStruct{x: 1.0, y: 2.0});
}
