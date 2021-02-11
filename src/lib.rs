#![no_std]

//! Crate with the purporse of using more easely and powerfull the functionality of the C library
//! [`dyncall`](https://www.dyncall.org). Refer to the page for platforms supported.
//! 
//! Python have to be installed and in `PATH` to use this library at the moment.

extern crate alloc;

use ::{
    alloc::{
        alloc::Layout, 
        format
    },
    core::{
        mem::{self, align_of, size_of}, 
        ptr::{self, NonNull}
    },
    libc::{ 
        c_char, 
        c_double, 
        c_float, 
        c_int, 
        c_long, 
        c_longlong, 
        c_short,
        c_void
    },
};

// most items imported here are included things from the dyncall repository comments are made in
// the interface when they're not,the linked file lies in `./dyncall_ext.c`.
#[link(name = "dyncall_ext")]
extern "C" {
    fn dlLoadLibrary(library_name: *const u8) -> Option<Dll>;
    fn dlFindSymbol(library: *mut c_void, symbol_name: *const u8) -> Option<NonNull<c_void>>;
    fn dlFreeLibrary(library: *mut c_void);

    fn dcNewCallVM(size: usize) -> Option<DynCaller>;
    fn dcFree(vm: *mut c_void);
    fn dcReset(vm: *mut c_void);
    fn dcMode(vm: *mut c_void, mode: c_int);

    fn dcArgBool(vm: *mut c_void, value: bool);
    fn dcArgChar(vm: *mut c_void, value: c_char);
    fn dcArgShort(vm: *mut c_void, value: c_short);
    fn dcArgInt(vm: *mut c_void, value: c_int);
    fn dcArgLong(vm: *mut c_void, value: c_long);
    fn dcArgLongLong(vm: *mut c_void, value: c_longlong);
    fn dcArgFloat(vm: *mut c_void, value: c_float);
    fn dcArgDouble(vm: *mut c_void, value: c_double);
    fn dcArgPointer(vm: *mut c_void, value: *mut c_void);
    fn dcArgStruct(vm: *mut c_void, s: *mut c_void, value: *mut c_void);

    fn dcCallVoid(vm: *mut c_void, funcptr: *mut c_void);
    fn dcCallBool(vm: *mut c_void, funcptr: *mut c_void) -> bool;
    fn dcCallChar(vm: *mut c_void, funcptr: *mut c_void) -> c_char;
    fn dcCallShort(vm: *mut c_void, funcptr: *mut c_void) -> c_short;
    fn dcCallInt(vm: *mut c_void, funcptr: *mut c_void) -> c_int;
    fn dcCallLong(vm: *mut c_void, funcptr: *mut c_void) -> c_long;
    fn dcCallLongLong(vm: *mut c_void, funcptr: *mut c_void) -> c_longlong;
    fn dcCallFloat(vm: *mut c_void, funcptr: *mut c_void) -> c_float;
    fn dcCallDouble(vm: *mut c_void, funcptr: *mut c_void) -> c_double;
    fn dcCallPointer(vm: *mut c_void, funcptr: *mut c_void) -> *mut c_void;
    fn dcCallStruct(
        vm: *mut c_void,
        funcptr: *mut c_void,
        s: *mut c_void,
        return_value: *mut c_void,
    );

    fn dcGetError(vm: *mut c_void) -> c_int;

    fn dcNewStruct(field_count: usize, alignment: c_int) -> Option<Struct>;
    fn dcStructField(s: *mut c_void, type_: c_int, alignment: c_int, array_length: usize);

    // appends a new struct as a field,like dcSubStruct but this requires a pointer to an existing
    // struct and the struct it's closed with `dcCloseStruct`  so it's unneeded to do so after and
    // will close the original one.
    fn dcAppendStruct(s: *mut c_void, s2: *const c_void);

    fn dcCloseStruct(s: *mut c_void);

    fn dcStructSize(s: *mut c_void) -> usize;
    fn dcStructAlignment(s: *mut c_void) -> usize;
    fn dcFreeStruct(s: *mut c_void);

    // some constants defined by me and linked thought a function because ffi constants do not
    // exist

    // fn returning the size of the internal DCCallVM,required for
    // `DynCaller::{reallocate, vm_head}`.
    fn vm_size() -> usize;

    // fn returning the align of the internal DCCallVM,required for guessing correctly the size
    // gotta be needed for each arg and reallocate if bigger than the actual due to dcVecAppend
    // not implementing that.
    fn vm_align() -> usize;

    // fn returning macro constant `dyncall::DC_ERROR_UNSUPPORTED_MODE`.
    fn unsupported_mode() -> c_int;

    fn freeMem(x: *mut c_void);
}

/// A handle to a dynamically linked library that can load symbols.
#[repr(transparent)]
pub struct Dll(NonNull<c_void>);

impl Dll {
    /// Loads a dll and get a handle to it.
    ///
    /// # Errors
    ///
    /// This function returns `None` if the dll cannot be found.
    ///
    /// # Panics
    ///
    /// This function panics in debug if `library_name` contains a '\0' character or "nul" often
    /// referred,this it's perfectly safe otherwise because the underlying foreign function only
    /// read from `library_name` and cannot deallocate it,the string will be truncated althought.
    #[inline]
    pub fn new(library_name: &str) -> Option<Self> {
        debug_assert!(!library_name.contains(0 as char), "nul character detected");

        unsafe { dlLoadLibrary(format!("{}\0", library_name).as_ptr()) }
    }

    /// Loads a symbol and gets a [`NonNull`] to it.
    ///
    /// # Errors
    ///
    /// This function returns `None` if the symbol cannot be found.
    ///
    /// # Panics
    ///
    /// This function panics in debug if `symbol_name` contains a '\0' character or "nul" often
    /// referred,this it's perfectly safe otherwise because the underlying foreign function only
    /// read from `symbol_name` and cannot deallocate it,the string will be truncated althought.
    #[inline]
    pub fn find_symbol(&self, symbol_name: &str) -> Option<NonNull<c_void>> {
        debug_assert!(!symbol_name.contains(0 as char), "nul character detected");

        unsafe { dlFindSymbol(self.0.as_ptr(), format!("{}\0", symbol_name).as_ptr()) }
    }
}

impl Drop for Dll {
    fn drop(&mut self) {
        unsafe { dlFreeLibrary(self.0.as_ptr()) }
    }
}

/// Structure describing a type that gotta be dynamically passed to [`call_func`].
pub enum Type {
    /// The [`c_void`] type.
    Void,
    // The [`bool`] type.
    Bool,
    /// The [`c_char`] type.
    Char,
    /// The [`c_uchar`][`libc::c_uchar`] type.
    UChar,
    /// The [`c_short`] type.
    Short,
    /// The [`c_ushort`][`libc::c_ushort`] type.
    UShort,
    /// The [`c_int`] type.
    Int,
    /// The [`c_uint`][`libc::c_uint`] type.
    UInt,
    /// The [`c_long`] type.
    Long,
    /// The [`c_ulong`][`libc::c_ulong`] type.
    ULong,
    /// The [`c_longlong`][`libc::c_longlong`] type.
    LongLong,
    /// The [`c_ulonglong`][`libc::c_ulonglong`] type.
    ULongLong,
    /// The [`c_float`] type.
    Float,
    /// The [`c_double`] type.
    Double,
    /// The [`*mut c_void`] type.
    Pointer,
    /// An struct,see [`Struct`].
    Struct(Struct),
}

impl Type {
    /// The alignment of the type,all memory positions for this type have to be a multiple of this value.
    pub fn align(&self) -> usize {
        use Type::*;

        match self {
            Void => align_of::<c_void>(),
            Bool => 1,
            Char | UChar => align_of::<c_char>(),
            Short | UShort => align_of::<c_short>(),
            Int | UInt => align_of::<c_int>(),
            Long | ULong => align_of::<c_long>(),
            LongLong | ULongLong => align_of::<c_longlong>(),
            Float => align_of::<c_float>(),
            Double => align_of::<c_double>(),
            Pointer => align_of::<usize>(),
            Struct(x) => x.align(),
        }
    }
    
    /// The size of the type.
    pub fn size(&self) -> usize {
        use Type::*;
        match self {
            Void => size_of::<c_void>(),
            Bool => 1,
            Char | UChar => size_of::<c_char>(),
            Short | UShort => size_of::<c_short>(),
            Int | UInt => size_of::<c_int>(),
            Long | ULong => size_of::<c_long>(),
            LongLong | ULongLong => size_of::<c_longlong>(),
            Float => size_of::<c_float>(),
            Double => size_of::<c_double>(),
            Pointer => size_of::<usize>(),
            Struct(x) => x.size(),
        }
    }

    fn sig_code(&self) -> u8 {
        use Type::*;
        (match self {
            Void => 'v',
            Bool => 'B',
            Char => 'c',
            UChar => 'C',
            Short => 's',
            UShort => 'S',
            Int => 'i',
            UInt => 'I',
            Long => 'j',
            ULong => 'J',
            LongLong => 'l',
            ULongLong => 'L',
            Float => 'f',
            Double => 'd',
            Pointer => 'p',
            _ => return 0,
        }) as _
    }
}

/// A dynamically declared structure,like the ones declared with the [`struct`] keyword.
#[repr(transparent)]
pub struct Struct(NonNull<c_void>);

impl Struct {
    /// Creates a `#[repr(C)]` struct out of enumerating all it's types thought `types`.
    #[inline]
    pub fn new<'a>(types: impl ExactSizeIterator<Item = &'a Type> + Clone) -> Self {
        use Type::*;

        // don't ask for the dumb names
        unsafe {
            let s1 = dcNewStruct(types.len(), types.clone().map(|x| x.align()).max().unwrap_or(1) as _).unwrap();

            let s = s1.0.as_ptr();

            for e in types {
                match e {
                    Struct(s2) => dcAppendStruct(s, s2.0.as_ptr()),
                    x => dcStructField(s, x.sig_code() as _, x.align() as _, x.size()),
                }
            }

            dcCloseStruct(s);

            s1
        }
    }

    /// The size of the struct.
    #[inline]
    pub fn size(&self) -> usize {
        unsafe { dcStructSize(self.0.as_ptr()) }
    }

    /// The align of the struct,see [`Type::align`].
    #[inline]
    pub fn align(&self) -> usize {
        unsafe { dcStructAlignment(self.0.as_ptr()) }
    }
}

impl Drop for Struct {
    fn drop(&mut self) {
        unsafe { dcFreeStruct(self.0.as_ptr()) }
    }
}

/// The struct that's used with [`call_func`] and [`call_function`].
#[repr(transparent)]
pub struct DynCaller(NonNull<c_void>);

#[repr(C)]
#[derive(Clone, Copy)]
struct VecHead {
    cap: usize,
    len: usize,
}

impl DynCaller {
    /// Creates a new `DynCaller` with an initial `arg_size` that will grow if needed at
    /// [`call_function`].
    ///
    /// # Errors
    ///
    /// Returns `None` at an internal allocator error.
    #[inline]
    pub fn new(arg_size: usize) -> Option<Self> {
        unsafe { dcNewCallVM(arg_size) }
    }

    #[inline]
    fn vec_head(&self) -> &VecHead {
        let vm_ptr = self.0.as_ptr();

        // SAFETY: the layout of DCCallVM it's architecture dependent,but the last field it's
        // guaranteed to be of a type `VecHead`.
        //
        // Further than that the implementation requires the arg contents to go after this type
        // and,as they are dynamically sized,changing the position will require to refactorize
        // all code to do offsetting of `cap` for the fields that are after that would de-optimise
        // slightly the code,so this will probably not change.
        unsafe { &*vm_ptr.add(vm_size() - size_of::<VecHead>()).cast::<VecHead>() }
    }

    /// Reallocates the args size to `new_arg_size`.
    ///
    /// # Safety
    ///
    /// `new_arg_size` has to be greater than the previous or this will write data out the new
    /// allocated item.
    unsafe fn reallocate(&mut self, new_arg_size: usize) {
        let size = vm_size() + self.vec_head().len;
        let new_vm = Self::new(new_arg_size).unwrap();

        self.0
            .cast::<u8>()
            .as_ptr()
            .copy_to_nonoverlapping(new_vm.0.as_ptr().cast(), size);
        let old_vm = mem::replace(self, new_vm);

        freeMem(old_vm.0.as_ptr());
        mem::forget(old_vm);
    }

    /// Reserve space for `size` extra size if future calls to [`call_func`] with arguments of
    /// arguments or return type of that size are expected.
    #[inline]
    pub fn reserve(&mut self, size: usize) {
        unsafe {
            let VecHead { len, cap } = *self.vec_head();

            let new_len = Layout::from_size_align_unchecked(len + size, vm_align())
                .pad_to_align()
                .size();

            if new_len > cap {
                self.reallocate(new_len.max(cap * 2));
            }
        }
    }

    /// Sets the calling convention or abi for this caller to use in the next `call_function`
    /// invocation,valid `name`s are:
    ///
    /// ```
    /// "c_default"
    /// "c_ellipsis"
    /// "c_ellipsis_varargs"
    /// "c_x86_cdecl"
    /// "c_x86_win32_std"
    /// "c_x86_win32_fast_ms"
    /// "c_x86_win32_fast_gnu"
    /// "c_x86_win32_this_ms"
    /// "c_x86_win32_this_gnu"
    /// "c_x64_win64"
    /// "c_x64_sysv"
    /// "c_ppc32_darwin"
    /// "c_ppc32_osx"
    /// "c_arm_arm_eabi"
    /// "c_arm_thumb_eabi"
    /// "c_arm_armhf"
    /// "c_mips32_eabi"
    /// "c_mips32_pspsdk"
    /// "c_ppc32_sysv"
    /// "c_ppc32_linux"
    /// "c_arm_arm"
    /// "c_arm_thumb"
    /// "c_mips32_o32"
    /// "c_mips64_n32"
    /// "c_mips64_n64"
    /// "c_x86_plan9"
    /// "c_sparc32"
    /// "c_sparc64"
    /// "c_arm64"
    /// "c_ppc64"
    /// "c_ppc64_linux"
    /// "sys_default"
    /// "sys_x86_int80h_linux"
    /// "sys_x86_int80h_bsd"
    /// "sys_x64_syscall_sysv"
    /// "sys_ppc32"
    /// "sys_ppc64"
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics on debug if the underlying foreign function yields an error,describing
    /// it thought the message,only one possible at the time of this writing is call it with an
    /// unsupported mode.
    pub fn abi(&mut self, name: &str) -> bool {
        unsafe {
            dcMode(
                self.0.as_ptr(),
                match name {
                    "c_default" => 0,
                    "c_ellipsis" => 100,
                    "c_ellipsis_varargs" => 101,
                    "c_x86_cdecl" => 1,
                    "c_x86_win32_std" => 2,
                    "c_x86_win32_fast_ms" => 3,
                    "c_x86_win32_fast_gnu" => 4,
                    "c_x86_win32_this_ms" => 3,
                    "c_x86_win32_this_gnu" => return self.abi("c_x86_cdecl"),
                    "c_x64_win64" => 7,
                    "c_x64_sysv" => 8,
                    "c_ppc32_darwin" => 9,
                    "c_ppc32_osx" => return self.abi("c_ppc32_darwin"),
                    "c_arm_arm_eabi" => 10,
                    "c_arm_thumb_eabi" => 11,
                    "c_arm_armhf" => 30,
                    "c_mips32_eabi" => 12,
                    "c_mips32_pspsdk" => return self.abi("c_mips32_eabi"),
                    "c_ppc32_sysv" => 13,
                    "c_ppc32_linux" => return self.abi("c_ppc32_sysv"),
                    "c_arm_arm" => 14,
                    "c_arm_thumb" => 15,
                    "c_mips32_o32" => 16,
                    "c_mips64_n32" => 17,
                    "c_mips64_n64" => 18,
                    "c_x86_plan9" => 19,
                    "c_sparc32" => 20,
                    "c_sparc64" => 21,
                    "c_arm64" => 22,
                    "c_ppc64" => 23,
                    "c_ppc64_linux" => return self.abi("c_ppc64"),
                    "sys_default" => 200,
                    "sys_x86_int80h_linux" => 201,
                    "sys_x86_int80h_bsd" => 202,
                    "sys_x64_syscall_sysv" => 204,
                    "sys_ppc32" => 210,
                    "sys_ppc64" => 211,
                    _ => return false,
                },
            );

            if cfg!(debug_assertions) {
                let x = dcGetError(self.0.as_ptr());

                if x == unsupported_mode() {
                    panic!("a mode supplied it's no longer supported or the internal code has changed,error!")
                }
            }

            true
        }
    }
}

impl Drop for DynCaller {
    fn drop(&mut self) {
        unsafe { dcFree(self.0.as_ptr()) }
    }
}

/// Use `dyn_caller` to call the fn `f` with arguments type defined by `args_type` and the values
/// pointed by `args`,return type specified by `return_type` and stored in `return_place`.
///
/// # Safety
///
/// [`ptr::read`] it's invoked on `args`,see it's safety section in the docs,if you want to easely
/// uphold this invariants,see [`call_function`]. Also the dyn caller can have any abi,not
/// neccesary the one that `f` has,by last,`f` does not neccesary point to a function with the
/// arguments and return type provided.
pub unsafe fn call_func<T: ?Sized>(
    dyn_caller: &mut DynCaller,
    return_type: Type,
    return_place: *mut c_void,
    f: NonNull<c_void>,
    args_type: impl Iterator<Item = Type>,
    mut args: *const c_void,
) {
    use Type::*;

    let mut vm_ptr = dyn_caller.0.as_ptr();
    let fn_ptr = f.as_ptr();

    dcReset(vm_ptr);

    for e in args_type {
        if let Void = e {
            continue
        }

        let size = e.size();

        dyn_caller.reserve(size);
        vm_ptr = dyn_caller.0.as_ptr();

        match e {
            Bool => dcArgBool(vm_ptr, ptr::read(args as _)),
            Char | UChar => dcArgChar(vm_ptr, ptr::read(args as _)),
            Short | UShort => dcArgShort(vm_ptr, ptr::read(args as _)),
            Int | UInt => dcArgInt(vm_ptr, ptr::read(args as _)),
            Long | ULong => dcArgLong(vm_ptr, ptr::read(args as _)),
            LongLong | ULongLong => dcArgLongLong(vm_ptr, ptr::read(args as _)),
            Float => dcArgFloat(vm_ptr, ptr::read(args as _)),
            Double => dcArgDouble(vm_ptr, ptr::read(args as _)),
            Pointer => dcArgPointer(vm_ptr, ptr::read(args as _)),
            Struct(x) => dcArgStruct(vm_ptr, x.0.as_ptr(), args as _),
            Void => {}
        }

        args = args.add(size);
    }

    dyn_caller.reserve(return_type.size());
    vm_ptr = dyn_caller.0.as_ptr();

    match return_type {
        Void => dcCallVoid(vm_ptr, fn_ptr),
        Bool => ptr::write(return_place as _, dcCallBool(vm_ptr, fn_ptr)),
        Char | UChar => ptr::write(return_place as _, dcCallChar(vm_ptr, fn_ptr)),
        Short | UShort => ptr::write(return_place as _, dcCallShort(vm_ptr, fn_ptr)),
        Int | UInt => ptr::write(return_place as _, dcCallInt(vm_ptr, fn_ptr)),
        Long | ULong => ptr::write(return_place as _, dcCallLong(vm_ptr, fn_ptr)),
        LongLong | ULongLong => ptr::write(return_place as _, dcCallLongLong(vm_ptr, fn_ptr)),
        Float => ptr::write(return_place as _, dcCallFloat(vm_ptr, fn_ptr)),
        Double => ptr::write(return_place as _, dcCallDouble(vm_ptr, fn_ptr)),
        Pointer => ptr::write(return_place as _, dcCallPointer(vm_ptr, fn_ptr)),
        Struct(x) => dcCallStruct(vm_ptr, fn_ptr, x.0.as_ptr(), return_place),
    }
}

/// Invokes [`call_func`] with a pointer to the variadic arguments in a tuple setted to `args`,then
/// [`mem::forget`]s that tuple to simulate a semantical move.
///
/// # Safety
///
/// This is still unsafe due to the reasons [`call_func`] pinpoints except for the first.
#[macro_export]
macro_rules! call_function {
    ($dyn_caller:expr, $return_place:expr, $f:expr, $sig:expr, $($a:expr),*) => {
        {
            let args = ($($a),*);

            $crate::call_func($dyn_caller, $return_place, $f, $sig, &args as *const _ as *const c_void);
            core::mem::forget(args);
        }
    }
}
