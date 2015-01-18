use std::ops::Deref;
use std::ptr::null_mut;
use rustc_serialize::Encodable;
use cesu8::to_cesu8;
use ffi::*;
use errors::*;
use context::Context;

/// Translates Rust values into JavaScript values.
pub struct Encoder {
    /// An internal `Context` object, for convenience.  We own this,
    /// because if we use a reference to somebody else's, the lifetimes
    /// make it very hard to work with &Encodable references.
    ctx: Context
}

impl Encoder {
    /// Create a new encoder which pushes values to `ctx`.  If you create
    /// one of these, you're responsible for making sure it gets used
    /// safely.
    pub unsafe fn new(ctx: *mut duk_context) -> Encoder {
        Encoder{ctx: Context::from_borrowed_mut_ptr(ctx)}
    }
}

type EncodeResult = DuktapeResult<()>;

/// A value which can be encoded and passed to JavaScript code.
pub trait DuktapeEncodable {
    /// An object-safe wrapper around Encodable::encode.
    fn duktape_encode(&self, s: &mut Encoder) -> EncodeResult;
}

impl<T: Encodable> DuktapeEncodable for T {
    fn duktape_encode(&self, s: &mut Encoder) -> EncodeResult {
        self.encode(s)
    }
}

impl ::rustc_serialize::Encoder for Encoder {
    type Error = DuktapeError;

    fn emit_nil(&mut self) -> EncodeResult {
        unsafe { duk_push_null(self.ctx.as_mut_ptr()); }
        Ok(())
    }

    // Integral types map to floats.
    fn emit_usize(&mut self, v: usize) -> EncodeResult { self.emit_f64(v as f64)}
    fn emit_u64(&mut self, v: u64) -> EncodeResult { self.emit_f64(v as f64) }
    fn emit_u32(&mut self, v: u32) -> EncodeResult { self.emit_f64(v as f64) }
    fn emit_u16(&mut self, v: u16) -> EncodeResult { self.emit_f64(v as f64) }
    fn emit_u8(&mut self, v: u8) -> EncodeResult  { self.emit_f64(v as f64) }
    fn emit_isize(&mut self, v: isize) -> EncodeResult { self.emit_f64(v as f64)}
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
        let encoded = to_cesu8(v);
        let buf = encoded.deref();
        unsafe {
            duk_push_lstring(self.ctx.as_mut_ptr(), buf.as_ptr() as *const i8,
                             buf.len() as duk_size_t);
        }
        Ok(())
    }

    fn emit_enum<F>(&mut self, _name: &str, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        f(self)
    }

    fn emit_enum_variant<F>(&mut self, v_name: &str, _v_id: usize,
                            len: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
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

    fn emit_enum_variant_arg<F>(&mut self, a_idx: usize, f: F) ->
        DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        unsafe {
            f(self).unwrap();
            duk_put_prop_index(self.ctx.as_mut_ptr(), -2, a_idx as u32);
        }
        Ok(())
    }

    #[allow(unused_variables)]
    fn emit_enum_struct_variant<F>(&mut self, v_name: &str, v_id: usize,
                                   len: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        // TODO: Not called during normal serialization.
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_enum_struct_variant_field<F>(&mut self, f_name: &str, f_idx: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        // TODO: Not called during normal serialization.
        unimplemented!()
    }

    fn emit_struct<F>(&mut self, _name: &str, _len: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        unsafe { duk_push_object(self.ctx.as_mut_ptr()); }
        f(self)
    }

    fn emit_struct_field<F>(&mut self, f_name: &str, _f_idx: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        self.emit_str(f_name).unwrap();
        f(self).unwrap();
        unsafe { duk_put_prop(self.ctx.as_mut_ptr(), -3); }
        Ok(())
    }

    fn emit_tuple<F>(&mut self, len: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        self.emit_seq(len, f)
    }

    fn emit_tuple_arg<F>(&mut self, idx: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        self.emit_seq_elt(idx, f)
    }

    #[allow(unused_variables)]
    fn emit_tuple_struct<F>(&mut self, name: &str, len: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        // TODO: Not currently used.
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn emit_tuple_struct_arg<F>(&mut self, f_idx: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        // TODO: Not currently used.
        unimplemented!()
    }

    fn emit_option<F>(&mut self, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        f(self)
    }

    fn emit_option_none(&mut self) -> EncodeResult
    {
        self.emit_nil()
    }

    fn emit_option_some<F>(&mut self, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        f(self)
    }

    fn emit_seq<F>(&mut self, _len: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        unsafe { duk_push_array(self.ctx.as_mut_ptr()); }
        f(self)
    }

    fn emit_seq_elt<F>(&mut self, idx: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        f(self).unwrap();
        unsafe { duk_put_prop_index(self.ctx.as_mut_ptr(), -2, idx as u32); }
        Ok(())
    }

    fn emit_map<F>(&mut self, _len: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        unsafe { duk_push_object(self.ctx.as_mut_ptr()); }
        f(self)
    }

    fn emit_map_elt_key<F>(&mut self, _idx: usize, mut f: F) -> DuktapeResult<()>
        where F: FnMut(&mut Encoder) -> DuktapeResult<()>
    {
        f(self).unwrap();
        unsafe { duk_safe_to_lstring(self.ctx.as_mut_ptr(), -1, null_mut()); }
        Ok(())
    }

    fn emit_map_elt_val<F>(&mut self, _idx: usize, f: F) -> DuktapeResult<()>
        where F: FnOnce(&mut Encoder) -> DuktapeResult<()>
    {
        f(self).unwrap();
        unsafe { duk_put_prop(self.ctx.as_mut_ptr(), -3); }
        Ok(())
    }
}

#[test]
fn test_encoder() {
    use std::collections::HashMap;
    use types::Value;

    let mut ctx = Context::new().unwrap();
    ctx.eval(r"
function assert_json(expected, value) {
    var value_json = JSON.stringify(value);
    //print('checking', expected, value_json);
    return JSON.stringify(JSON.parse(expected)) == value_json || value_json;
}").unwrap();

    fn assert_json<T: DuktapeEncodable>(
        ctx: &mut Context, expected: &str, value: &T)
    {
        match ctx.call("assert_json", &[&expected, value]) {
            Ok(Value::Bool(true)) => {},
            Ok(Value::String(ref got)) =>
                panic!("expected {}, got {}", expected, got),
            ref result => panic!("unexpected value: {:?}", result)
        }
    }

    macro_rules! assert_encode {
        ($val:expr) => {
            {
                let v = $val;
                let expected = ::rustc_serialize::json::encode(&v);
                assert_json(&mut ctx, expected.as_slice(), &v);
            }
        }
    }

    // Simple types.
    assert_encode!(1us);
    assert_encode!(1u64);
    assert_encode!(1u32);
    assert_encode!(1u16);
    assert_encode!(1u8);
    assert_encode!(-1is);
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
    #[derive(RustcEncodable)]
    enum ExEnum { Foo, Bar(f64), Baz{x: f64, y: f64} }
    assert_encode!(&ExEnum::Foo);
    assert_encode!(&ExEnum::Bar(1.0));
    assert_encode!(&ExEnum::Baz{x: 1.0, y: 2.0});

    // Structs.
    #[derive(RustcEncodable)]
    struct ExStruct { x: f64, y: f64 }
    assert_encode!(&ExStruct{x: 1.0, y: 2.0});

    // Tuples.
    assert_encode!(&(1us, 2us));

    // Tuple structs.
    #[derive(RustcEncodable)]
    struct ExTupleStruct(f64);
    assert_encode!(&ExTupleStruct(1.0));

    // Options.
    let none_f64: Option<f64> = None;
    assert_encode!(&none_f64);
    assert_encode!(&Some(1.0f64));

    // Sequences.
    let seq = [1.0f64];
    assert_encode!(seq.as_slice());

    // Maps.
    let mut hash: HashMap<String,i32> = HashMap::new();
    hash.insert("test".to_string(), 3);
    assert_encode!(&hash);    
    let mut hash2: HashMap<i32,i32> = HashMap::new();
    hash2.insert(7, 3);
    assert_encode!(&hash2);    
}
