use std::iter::Iterator;
use rustc_serialize::Decodable;
use ffi::*;
use errors::*;
use context::{Context, from_lstring};

/// Translates JavaScript values into Rust values.
#[allow(dead_code)] // WIP
pub struct Decoder {
    /// An internal `Context` object, for convenience.  We own this,
    /// because if we use a reference to somebody else's, the lifetimes
    /// make it very hard to work with &Encodable references.
    ctx: Context
}

impl Decoder {
    /// Create a new decoder which pops values from `ctx`.  If you create
    /// one of these, you're responsible for making sure it gets used
    /// safely.
    #[allow(dead_code)] // WIP
    pub unsafe fn new(ctx: *mut duk_context) -> Decoder {
        Decoder{ctx: Context::from_borrowed_mut_ptr(ctx)}
    }
}

/// A value which can be encoded and passed to JavaScript code.
pub trait DuktapeDecodable: Decodable {}
impl<T: Decodable> DuktapeDecodable for T {}

macro_rules! read_and_convert {
    ($name:ident -> $ty:ident, $reader:ident -> $in_ty:ident) => {
        fn $name(&mut self) -> DuktapeResult<$ty> {
            self.$reader().map(|: v: $in_ty| v as $ty)
        }
    }
}

macro_rules! read_with {
    ($name:ident -> $ty:ident, $tester:ident,
     |$slf:ident, $idx:ident| $reader:block) => {
        fn $name(&mut $slf) -> DuktapeResult<$ty> {
            unsafe {
                let $idx = -1;
                if $tester($slf.ctx.as_mut_ptr(), $idx) != 0 {
                    let result = $reader;
                    duk_pop($slf.ctx.as_mut_ptr());
                    result
                } else {
                    duk_pop($slf.ctx.as_mut_ptr());
                    Err(DuktapeError::from_str("Expected number"))
                }
            }
        }
    }
}

#[allow(unused_variables)]
impl ::rustc_serialize::Decoder for Decoder {
    type Error = DuktapeError;

    fn read_nil(&mut self) -> DuktapeResult<()> 
    {
        unimplemented!()
    }

    read_and_convert!(read_usize-> usize,read_f64 -> f64);
    read_and_convert!(read_u64  -> u64,  read_f64 -> f64);
    read_and_convert!(read_u32  -> u32,  read_f64 -> f64);
    read_and_convert!(read_u16  -> u16,  read_f64 -> f64);
    read_and_convert!(read_u8   -> u8,   read_f64 -> f64);
    read_and_convert!(read_isize-> isize,read_f64 -> f64);
    read_and_convert!(read_i64  -> i64,  read_f64 -> f64);
    read_and_convert!(read_i32  -> i32,  read_f64 -> f64);
    read_and_convert!(read_i16  -> i16,  read_f64 -> f64);
    read_and_convert!(read_i8   -> i8,   read_f64 -> f64);

    read_with!(read_bool -> bool, duk_is_boolean, |self, idx| {
        Ok(duk_get_boolean(self.ctx.as_mut_ptr(), idx) != 0)
    });

    read_with!(read_f64 -> f64, duk_is_number, |self, idx| {
        Ok(duk_get_number(self.ctx.as_mut_ptr(), idx))
    });
    read_and_convert!(read_f32 -> f32, read_f64 -> f64);

    fn read_char(&mut self) -> DuktapeResult<char> {
        fn err(msg: &str) -> DuktapeResult<char> {
            Err(DuktapeError::from_str(msg))
        }
        match self.read_str() {
            Ok(ref s) => {
                // Try to read exactly one character.
                let mut iter = s.chars();
                let result = match iter.next() {
                    None => return err("Expected char, got \"\""),
                    Some(c) => c
                };
                match iter.next() {
                    None => Ok(result),
                    Some(_) => {
                        err(format!("Expected char, got \"{}\"", s).as_slice())
                    }
                }
            }
            Err(err) => Err(err)
        }
    }

    read_with!(read_str -> String, duk_is_string, |self, idx| {
        let mut len = 0;
        let ptr = duk_get_lstring(self.ctx.as_mut_ptr(), idx, &mut len);
        from_lstring(ptr, len)
    });

    // Compound types:
    fn read_enum<T,F>(&mut self, name: &str,
                    f: F) -> DuktapeResult<T>
        where F: FnOnce(&mut Decoder) -> DuktapeResult<T>
    {
        unimplemented!()
    }

    fn read_enum_variant<T,F>(&mut self,
                            names: &[&str],
                            f: F)
                            -> DuktapeResult<T>
        where F: FnMut(&mut Decoder, usize) -> DuktapeResult<T>
    {
        unimplemented!()
    }
    fn read_enum_variant_arg<T,F>(&mut self,
                                a_idx: usize,
                                f: F)
                                -> DuktapeResult<T>
        where F: FnOnce(&mut Decoder) -> DuktapeResult<T>        
    {
        unimplemented!()
    }

    fn read_enum_struct_variant<T,F>(&mut self,
                                   names: &[&str],
                                   f: F)
                                   -> DuktapeResult<T>
        where F: FnMut(&mut Decoder, usize) -> DuktapeResult<T>
    {
        unimplemented!()
    }
    fn read_enum_struct_variant_field<T,F>(&mut self,
                                         f_name: &str,
                                         f_idx: usize,
                                         f: F)
                                         -> DuktapeResult<T>
        where F: FnOnce(&mut Decoder) -> DuktapeResult<T>
    {
        unimplemented!()
    }

    fn read_struct<T,F>(&mut self, s_name: &str, len: usize, f: F)
                      -> DuktapeResult<T>
        where F: FnOnce(&mut Decoder) -> DuktapeResult<T>
    {
        unimplemented!()
    }
    fn read_struct_field<T,F>(&mut self,
                            f_name: &str,
                            f_idx: usize,
                            f: F)
                            -> DuktapeResult<T>
        where F: FnOnce(&mut Decoder) -> DuktapeResult<T>
    {
        unimplemented!()
    }

    fn read_tuple<T,F>(&mut self, len: usize, f: F) -> DuktapeResult<T>
        where F: FnOnce(&mut Decoder) -> DuktapeResult<T>
    {
        unimplemented!()
    }
    fn read_tuple_arg<T,F>(&mut self, a_idx: usize, f: F) -> DuktapeResult<T>
        where F: FnOnce(&mut Decoder) -> DuktapeResult<T>
    {
        unimplemented!()
    }

    fn read_tuple_struct<T,F>(&mut self,
                            s_name: &str,
                            len: usize,
                            f: F)
                            -> DuktapeResult<T>
        where F: FnOnce(&mut Decoder) -> DuktapeResult<T>
    {
        unimplemented!()
    }
    fn read_tuple_struct_arg<T,F>(&mut self,
                                a_idx: usize,
                                f: F)
                                -> DuktapeResult<T> 
        where F: FnOnce(&mut Decoder) -> DuktapeResult<T>
    {
        unimplemented!()
    }

    // Specialized types:
    fn read_option<T,F>(&mut self, f: F) -> DuktapeResult<T> 
        where F: FnMut(&mut Decoder, bool) -> DuktapeResult<T>
    {
        unimplemented!()
    }

    fn read_seq<T,F>(&mut self, f: F) -> DuktapeResult<T> 
        where F: FnOnce(&mut Decoder, usize) -> DuktapeResult<T>
    {
        unimplemented!()
    }
    fn read_seq_elt<T,F>(&mut self, idx: usize, f: F) -> DuktapeResult<T> 
        where F: FnOnce(&mut Decoder) -> DuktapeResult<T>
    {
        unimplemented!()
    }

    fn read_map<T,F>(&mut self, f: F) -> DuktapeResult<T> 
        where F: FnOnce(&mut Decoder, usize) -> DuktapeResult<T>
    {
        unimplemented!()
    }
    fn read_map_elt_key<T,F>(&mut self, idx: usize, f: F) -> DuktapeResult<T> 
        where F: FnOnce(&mut Decoder) -> DuktapeResult<T>
    {
        unimplemented!()
    }
    fn read_map_elt_val<T,F>(&mut self, idx: usize, f: F) -> DuktapeResult<T> 
        where F: FnOnce(&mut Decoder) -> DuktapeResult<T>
    {
        unimplemented!()
    }

    // Failure
    fn error(&mut self, err: &str) -> DuktapeError 
    {
        unimplemented!()
    }
}

#[test]
fn test_decoder() {
    //use std::collections::HashMap;
    use std::fmt::Debug;
    use encoder::{Encoder, DuktapeEncodable};

    let mut ctx = Context::new().unwrap();

    fn assert_decode<T>(ctx: &mut Context, value: &T)
        where T: DuktapeEncodable + DuktapeDecodable + PartialEq + Debug
    {
        let mut encoder = unsafe { Encoder::new(ctx.as_mut_ptr()) };
        value.duktape_encode(&mut encoder).unwrap();
        let mut decoder = unsafe { Decoder::new(ctx.as_mut_ptr()) };
        let decoded: DuktapeResult<T> = Decodable::decode(&mut decoder);
        println!("decoding {:?} {:?}", value, decoded);
        assert_eq!(value, &decoded.unwrap());
    }

    macro_rules! assert_decode {
        ($val:expr) => { assert_decode(&mut ctx, &$val) }
    }

    // TODO: Refactor everything below into a combined Encode/Decode test
    // suite.

    // Simple types.
    assert_decode!(1us);
    assert_decode!(1u64);
    assert_decode!(1u32);
    assert_decode!(1u16);
    assert_decode!(1u8);
    assert_decode!(-1is);
    assert_decode!(-1i64);
    assert_decode!(-1i32);
    assert_decode!(-1i16);
    assert_decode!(-1i8);
    assert_decode!(true);
    assert_decode!(false);
    assert_decode!(1.0f64);
    assert_decode!(1.0f32);
    assert_decode!("string".to_string());
    // serialize::json::encode handles characters below U+10000 incorrectly.
    //assert_decode!('c'); // https://github.com/rust-lang/rust/issues/19719
    assert_decode!('𓀀');

    //// Enums.
    //#[derive(RustcEncodable, Decodable, PartialEq, Show)]
    //enum ExEnum { Foo, Bar(f64), Baz{x: f64, y: f64} }
    //assert_decode!(ExEnum::Foo);
    //assert_decode!(ExEnum::Bar(1.0));
    //assert_decode!(ExEnum::Baz{x: 1.0, y: 2.0});

    //// Structs.
    //#[derive(RustcEncodable, Decodable, PartialEq, Show)]
    //struct ExStruct { x: f64, y: f64 }
    //assert_decode!(ExStruct{x: 1.0, y: 2.0});

    //// Tuples.
    //assert_decode!((1u, 2us));

    //// Tuple structs.
    //#[derive(RustcEncodable, Decodable, PartialEq, Show)]
    //struct ExTupleStruct(f64);
    //assert_decode!(ExTupleStruct(1.0));

    //// Options.
    //let none_f64: Option<f64> = None;
    //assert_decode!(none_f64);
    //assert_decode!(Some(1.0f64));

    //// Sequences.
    //let seq = vec!(1.0f64);
    //assert_decode!(seq);

    // Maps.
    //let mut hash: HashMap<String,int> = HashMap::new();
    //hash.insert("test".to_string(), 3);
    //assert_decode!(&hash);    
    //let mut hash2: HashMap<int,int> = HashMap::new();
    //hash2.insert(7, 3);
    //assert_decode!(hash2);    
}
