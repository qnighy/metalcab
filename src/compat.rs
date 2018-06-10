use raw::*;
use std::ffi::{CStr, NulError};
use std::marker::PhantomData;
use std::ops::Drop;
use std::os::raw::*;
use std::ptr::{self, NonNull};
use std::slice;
use std::str::{self, Utf8Error};

/// Model class
#[derive(Debug)]
pub struct Model(NonNull<mecab_model_t>);

impl Model {
    /// Factory method to create a new Model with a specified main's argc/argv-style parameters.
    /// Return NULL if new model cannot be initialized. Use MeCab::getLastError() to obtain the
    /// cause of the errors.
    pub fn create(args: &[&CStr]) -> Result<Self, MecabError> {
        // FIXME: once CStr becomes extern type, we can just transmut it
        let args = args.iter()
            .map(|&arg| arg.as_ptr() as *mut c_char)
            .collect::<Vec<_>>();
        let argc = args.len() as c_int;
        assert!(argc >= 0);
        assert_eq!(argc as usize, args.len());
        let ptr = unsafe { mecab_model_new(argc, args.as_ptr() as *mut *mut c_char) };
        let ptr = NonNull::new(ptr).ok_or_else(|| MecabError::last())?;
        Ok(Model(ptr))
    }

    /// Factory method to create a new Model with a string parameter representation, i.e.,
    /// "-d /user/local/mecab/dic/ipadic -Ochasen".
    /// Return NULL if new model cannot be initialized. Use MeCab::getLastError() to obtain the
    /// cause of the errors.
    pub fn create2(arg: &CStr) -> Result<Self, MecabError> {
        let ptr = unsafe { mecab_model_new2(arg.as_ptr() as *const c_char) };
        let ptr = NonNull::new(ptr).ok_or_else(|| MecabError::last())?;
        Ok(Model(ptr))
    }

    /// Create a new Tagger object.
    /// All returned tagger object shares this model object as a parsing model.
    /// Never delete this model object before deleting tagger object.
    #[allow(non_snake_case)]
    pub fn createTagger(&self) -> Result<Tagger, MecabError> {
        let tagger = unsafe { Tagger::from_ptr(mecab_model_new_tagger(self.as_ptr())) };
        let tagger = tagger.ok_or_else(|| MecabError::last())?;
        Ok(tagger)
    }

    /// Create a new Lattice object.
    #[allow(non_snake_case)]
    pub fn createLattice(&self) -> Result<Lattice, MecabError> {
        let lattice = unsafe { Lattice::from_ptr(mecab_model_new_lattice(self.as_ptr())) };
        let lattice = lattice.ok_or_else(|| MecabError::last())?;
        Ok(lattice)
    }

    /// Swap the instance with `model`.
    /// The ownership of `model` always moves to this instance,
    /// meaning that passed `model` will no longer be accessible after calling this method.
    /// return true if new model is swapped successfully.
    /// This method is thread safe. All taggers created by
    /// Model::createTagger() method will also be updated asynchronously.
    /// No need to stop the parsing thread excplicitly before swapping model object.
    ///
    /// ## Unsafety
    ///
    /// Only parsing functions takes a lock; therefore the following methods race with
    /// `Model::swap`.
    ///
    /// - `Model::dictionary_info`
    /// - `Model::transition_cost`
    /// - `Model::lookup`
    /// - `Tagger::dictionary_info`
    ///
    /// Moreover, `Model::swap` itself has a race condition, meaning that you cannot
    /// call `Model::swap` on the same model in a racy way.
    pub unsafe fn swap(&self, new_model: &Model) -> Result<(), MecabError> {
        let result = mecab_model_swap(self.as_ptr(), new_model.as_ptr());
        if result != 0 {
            Ok(())
        } else {
            Err(MecabError::last())
        }
    }

    /// Return DictionaryInfo linked list.
    pub fn dictionary_info(&self) -> Option<&DictionaryInfo> {
        let ptr = unsafe { mecab_model_dictionary_info(self.as_ptr()) };
        unsafe { DictionaryInfo::from_ptr(ptr as *mut mecab_dictionary_info_t) }
    }

    /// Return transtion cost from rcAttr to lcAttr.
    #[allow(non_snake_case)]
    pub fn transition_cost(&self, rcAttr: u16, lcAttr: u16) -> i32 {
        let rcAttr = rcAttr as c_ushort;
        let lcAttr = lcAttr as c_ushort;
        (unsafe { mecab_model_transition_cost(self.as_ptr(), rcAttr, lcAttr) }) as i32
    }

    /// perform common prefix search from the range [begin, end).
    /// `lattice` takes the ownership of return value.
    pub fn lookup(&self, begin: &CStr, end: &CStr, lattice: &Lattice) -> Option<&Node> {
        let begin = begin.as_ptr();
        let end = end.as_ptr();
        let lattice = lattice.as_ptr();
        let ptr = unsafe { mecab_model_lookup(self.as_ptr(), begin, end, lattice) };
        unsafe { Node::from_ptr(ptr) }
    }

    /// Return a version string
    pub fn version() -> &'static CStr {
        unsafe { CStr::from_ptr(mecab_version()) }
    }

    pub fn as_ptr(&self) -> *mut mecab_model_t {
        self.0.as_ptr()
    }

    pub unsafe fn from_ptr(ptr: *mut mecab_model_t) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Model(ptr))
    }

    pub unsafe fn from_ptr_unchecked(ptr: *mut mecab_model_t) -> Self {
        Model(NonNull::new_unchecked(ptr))
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

/// Tagger class
#[derive(Debug)]
pub struct Tagger<'model>(NonNull<mecab_t>, PhantomData<&'model ()>);

impl Tagger<'static> {
    pub fn create(args: &[&CStr]) -> Result<Self, MecabError> {
        // FIXME: once CStr becomes extern type, we can just transmut it
        let args = args.iter()
            .map(|&arg| arg.as_ptr() as *mut c_char)
            .collect::<Vec<_>>();
        let argc = args.len() as c_int;
        assert!(argc >= 0);
        assert_eq!(argc as usize, args.len());
        let ptr = unsafe { mecab_new(argc, args.as_ptr() as *mut *mut c_char) };
        let ptr = NonNull::new(ptr).ok_or_else(|| MecabError::last())?;
        Ok(Tagger(ptr, PhantomData))
    }

    pub fn create2(arg: &CStr) -> Result<Self, MecabError> {
        let ptr = unsafe { mecab_new2(arg.as_ptr() as *const c_char) };
        let ptr = NonNull::new(ptr).ok_or_else(|| MecabError::last())?;
        Ok(Tagger(ptr, PhantomData))
    }
}

impl<'model> Tagger<'model> {
    pub fn partial(&self) -> bool {
        (unsafe { mecab_get_partial(self.as_ptr()) }) != 0
    }

    pub fn set_partial(&mut self, partial: bool) {
        unsafe { mecab_set_partial(self.as_ptr(), partial as c_int) };
    }

    pub fn theta(&self) -> f64 {
        (unsafe { mecab_get_theta(self.as_ptr()) }) as f64
    }

    pub fn set_theta(&mut self, theta: f64) {
        unsafe { mecab_set_theta(self.as_ptr(), theta as c_float) };
    }

    pub fn lattice_level(&self) -> i32 {
        (unsafe { mecab_get_lattice_level(self.as_ptr()) }) as i32
    }

    pub fn set_lattice_level(&mut self, lattice_level: i32) {
        unsafe { mecab_set_lattice_level(self.as_ptr(), lattice_level as c_int) };
    }

    pub fn all_morphs(&self) -> bool {
        (unsafe { mecab_get_all_morphs(self.as_ptr()) }) != 0
    }

    pub fn set_all_morphs(&mut self, all_morphs: bool) {
        unsafe { mecab_set_all_morphs(self.as_ptr(), all_morphs as c_int) };
    }

    unsafe fn convert_result_str(&self, ptr: *const c_char) -> Result<&str, MecabError> {
        if ptr.is_null() {
            return Err(self.last_error());
        }
        let s = CStr::from_ptr(ptr);
        Ok(str::from_utf8(s.to_bytes())?)
    }

    pub fn parse_cstr(&mut self, s: &CStr) -> Result<&str, MecabError> {
        unsafe { self.convert_result_str(mecab_sparse_tostr(self.as_ptr(), s.as_ptr())) }
    }

    pub fn parse(&mut self, s: &str) -> Result<&str, MecabError> {
        unsafe {
            self.convert_result_str(mecab_sparse_tostr2(
                self.as_ptr(),
                s.as_ptr() as *const c_char,
                s.len(),
            ))
        }
    }

    #[allow(non_snake_case)]
    pub fn parseToNode_cstr(&mut self, s: &CStr) -> Result<&Node, MecabError> {
        let ptr = unsafe { mecab_sparse_tonode(self.as_ptr(), s.as_ptr()) };
        let ptr = unsafe { Node::from_ptr(ptr as *mut mecab_node_t) };
        ptr.ok_or_else(|| self.last_error())
    }

    #[allow(non_snake_case)]
    pub fn parseToNode(&mut self, s: &str) -> Result<&Node, MecabError> {
        let ptr =
            unsafe { mecab_sparse_tonode2(self.as_ptr(), s.as_ptr() as *const c_char, s.len()) };
        let ptr = unsafe { Node::from_ptr(ptr as *mut mecab_node_t) };
        ptr.ok_or_else(|| self.last_error())
    }

    #[allow(non_snake_case)]
    pub fn parseNBest_cstr(&mut self, n: usize, s: &CStr) -> Result<&str, MecabError> {
        unsafe { self.convert_result_str(mecab_nbest_sparse_tostr(self.as_ptr(), n, s.as_ptr())) }
    }

    #[allow(non_snake_case)]
    pub fn parseNBest(&mut self, n: usize, s: &str) -> Result<&str, MecabError> {
        unsafe {
            self.convert_result_str(mecab_nbest_sparse_tostr2(
                self.as_ptr(),
                n,
                s.as_ptr() as *const c_char,
                s.len(),
            ))
        }
    }

    #[allow(non_snake_case)]
    pub fn parseNBestInit_cstr(&mut self, s: &CStr) -> Result<(), MecabError> {
        let result = unsafe { mecab_nbest_init(self.as_ptr(), s.as_ptr()) };
        if result != 0 {
            Ok(())
        } else {
            Err(self.last_error())
        }
    }

    #[allow(non_snake_case)]
    pub fn parseNBestInit(&mut self, s: &str) -> Result<(), MecabError> {
        let result =
            unsafe { mecab_nbest_init2(self.as_ptr(), s.as_ptr() as *const c_char, s.len()) };
        if result != 0 {
            Ok(())
        } else {
            Err(self.last_error())
        }
    }

    pub fn next(&mut self) -> Result<&str, MecabError> {
        unsafe { self.convert_result_str(mecab_nbest_next_tostr(self.as_ptr())) }
    }

    #[allow(non_snake_case)]
    pub fn nextNode(&mut self) -> Result<&Node, MecabError> {
        let ptr = unsafe { mecab_nbest_next_tonode(self.as_ptr()) };
        let ptr = unsafe { Node::from_ptr(ptr as *mut mecab_node_t) };
        ptr.ok_or_else(|| self.last_error())
    }

    #[allow(non_snake_case)]
    pub fn formatNode(&mut self, node: &Node) -> Result<&str, MecabError> {
        unsafe { self.convert_result_str(mecab_format_node(self.as_ptr(), node.as_ptr())) }
    }

    /// Return DictionaryInfo linked list.
    pub fn dictionary_info(&self) -> Option<&DictionaryInfo> {
        let ptr = unsafe { mecab_dictionary_info(self.as_ptr()) };
        unsafe { DictionaryInfo::from_ptr(ptr as *mut mecab_dictionary_info_t) }
    }

    /// Return a version string
    pub fn version() -> &'static CStr {
        unsafe { CStr::from_ptr(mecab_version()) }
    }

    pub fn last_error(&self) -> MecabError {
        unsafe { MecabError::from_tagger(self.as_ptr()) }
    }

    pub fn as_ptr(&self) -> *mut mecab_t {
        self.0.as_ptr()
    }

    pub unsafe fn from_ptr(ptr: *mut mecab_t) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Tagger(ptr, PhantomData))
    }

    pub unsafe fn from_ptr_unchecked(ptr: *mut mecab_t) -> Self {
        Tagger(NonNull::new_unchecked(ptr), PhantomData)
    }
}

impl<'model> Drop for Tagger<'model> {
    fn drop(&mut self) {
        unsafe {
            mecab_destroy(self.as_ptr());
        }
    }
}

unsafe impl<'model> Send for Tagger<'model> {}
unsafe impl<'model> Sync for Tagger<'model> {}

#[derive(Debug)]
pub struct Lattice(NonNull<mecab_lattice_t>);

impl Lattice {
    pub fn new() -> Self {
        let ptr = unsafe { mecab_lattice_new() };
        let ptr = NonNull::new(ptr).expect("mecab_lattice_new() failed");
        Lattice(ptr)
    }

    pub fn clear(&mut self) {
        unsafe { mecab_lattice_clear(self.as_ptr()) };
    }

    pub fn is_available(&self) -> bool {
        (unsafe { mecab_lattice_is_available(self.as_ptr()) }) != 0
    }

    pub fn bos_node(&self) -> Option<&Node> {
        unsafe { Node::from_ptr(mecab_lattice_get_bos_node(self.as_ptr())) }
    }

    pub fn eos_node(&self) -> Option<&Node> {
        unsafe { Node::from_ptr(mecab_lattice_get_eos_node(self.as_ptr())) }
    }

    pub fn all_begin_nodes(&self) -> &[Option<&Node>] {
        let size = self.size();
        let size = if size > 0 { size + 4 } else { 0 };
        let ptr = unsafe { mecab_lattice_get_all_begin_nodes(self.as_ptr()) };
        unsafe { slice::from_raw_parts(ptr as *const Option<&Node>, size) }
    }

    pub fn all_end_nodes(&self) -> &[Option<&Node>] {
        let size = self.size();
        let size = if size > 0 { size + 4 } else { 0 };
        let ptr = unsafe { mecab_lattice_get_all_end_nodes(self.as_ptr()) };
        unsafe { slice::from_raw_parts(ptr as *const Option<&Node>, size) }
    }

    pub fn begin_nodes(&self, pos: usize) -> Option<&Node> {
        let size = self.size();
        assert!(size > 0, "size is 0");
        assert!(pos < size + 4, "index out of range");
        unsafe { Node::from_ptr(mecab_lattice_get_begin_nodes(self.as_ptr(), pos)) }
    }

    pub fn end_nodes(&self, pos: usize) -> Option<&Node> {
        let size = self.size();
        assert!(size > 0, "size is 0");
        assert!(pos < size + 4, "index out of range");
        unsafe { Node::from_ptr(mecab_lattice_get_end_nodes(self.as_ptr(), pos)) }
    }

    pub fn size(&self) -> usize {
        unsafe { mecab_lattice_get_size(self.as_ptr()) }
    }

    pub fn as_ptr(&self) -> *mut mecab_lattice_t {
        self.0.as_ptr()
    }

    pub unsafe fn from_ptr(ptr: *mut mecab_lattice_t) -> Option<Self> {
        NonNull::new(ptr).map(Lattice)
    }

    pub unsafe fn from_ptr_unchecked(ptr: *mut mecab_lattice_t) -> Self {
        Lattice(NonNull::new_unchecked(ptr))
    }
}

impl Drop for Lattice {
    fn drop(&mut self) {
        unsafe { mecab_lattice_destroy(self.as_ptr()) }
    }
}

unsafe impl Send for Lattice {}
unsafe impl Sync for Lattice {}

pub struct DictionaryInfo<'a>(mecab_dictionary_info_t, PhantomData<&'a ()>);

impl<'a> DictionaryInfo<'a> {
    pub fn filename(&self) -> Option<&'a [u8]> {
        if self.0.filename.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(self.0.filename) }.to_bytes())
        }
    }

    pub fn charset(&self) -> Option<&'a [u8]> {
        if self.0.charset.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(self.0.charset) }.to_bytes())
        }
    }

    pub fn size(&self) -> u32 {
        self.0.size as u32
    }

    pub fn type_(&self) -> DictionaryType {
        let type_ = self.0.type_;
        if type_ == 0 {
            DictionaryType::MECAB_SYS_DIC
        } else if type_ == 1 {
            DictionaryType::MECAB_USR_DIC
        } else if type_ == 2 {
            DictionaryType::MECAB_UNK_DIC
        } else {
            panic!("Unknown DictionaryType: {}", type_)
        }
    }

    pub fn lsize(&self) -> u32 {
        self.0.lsize as u32
    }

    pub fn rsize(&self) -> u32 {
        self.0.rsize as u32
    }

    pub fn version(&self) -> u16 {
        self.0.version as u16
    }

    pub fn next(&self) -> Option<&'a Self> {
        unsafe { Self::from_ptr(self.0.next) }
    }

    pub fn as_ptr(&self) -> *mut mecab_dictionary_info_t {
        &self.0 as *const mecab_dictionary_info_t as *mut mecab_dictionary_info_t
    }

    pub unsafe fn from_ptr(ptr: *mut mecab_dictionary_info_t) -> Option<&'a Self> {
        NonNull::new(ptr as *mut Self).map(|ptr| &*ptr.as_ptr())
    }

    pub unsafe fn from_ptr_unchecked(ptr: *mut mecab_dictionary_info_t) -> &'a Self {
        &*(ptr as *mut Self)
    }
}

unsafe impl<'a> Send for DictionaryInfo<'a> {}
unsafe impl<'a> Sync for DictionaryInfo<'a> {}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DictionaryType {
    /// This is a system dictionary.
    MECAB_SYS_DIC = 0,

    /// This is a user dictionary.
    MECAB_USR_DIC = 1,

    /// This is a unknown word dictionary.
    MECAB_UNK_DIC = 2,
}

pub struct Path<'a>(mecab_path_t, PhantomData<&'a ()>);

impl<'a> Path<'a> {
    pub fn rnode(&self) -> Option<&'a Node<'a>> {
        unsafe { Node::from_ptr(self.0.rnode) }
    }

    pub fn rnext(&self) -> Option<&'a Self> {
        unsafe { Self::from_ptr(self.0.rnext) }
    }

    pub fn lnode(&self) -> Option<&'a Node<'a>> {
        unsafe { Node::from_ptr(self.0.lnode) }
    }

    pub fn lnext(&self) -> Option<&'a Self> {
        unsafe { Self::from_ptr(self.0.lnext) }
    }

    pub fn cost(&self) -> i32 {
        self.0.cost as i32
    }

    pub fn prob(&self) -> f32 {
        self.0.prob as f32
    }

    pub fn as_ptr(&self) -> *mut mecab_path_t {
        &self.0 as *const mecab_path_t as *mut mecab_path_t
    }

    pub unsafe fn from_ptr(ptr: *mut mecab_path_t) -> Option<&'a Self> {
        NonNull::new(ptr as *mut Self).map(|ptr| &*ptr.as_ptr())
    }

    pub unsafe fn from_ptr_unchecked(ptr: *mut mecab_path_t) -> &'a Self {
        &*(ptr as *mut Self)
    }
}

unsafe impl<'a> Send for Path<'a> {}
unsafe impl<'a> Sync for Path<'a> {}

pub struct Node<'a>(mecab_node_t, PhantomData<&'a ()>);

impl<'a> Node<'a> {
    pub fn prev(&self) -> Option<&'a Self> {
        unsafe { Self::from_ptr(self.0.prev) }
    }

    pub fn next(&self) -> Option<&'a Self> {
        unsafe { Self::from_ptr(self.0.next) }
    }

    pub fn enext(&self) -> Option<&'a Self> {
        unsafe { Self::from_ptr(self.0.enext) }
    }

    pub fn bnext(&self) -> Option<&'a Self> {
        unsafe { Self::from_ptr(self.0.bnext) }
    }

    pub fn rpath(&self) -> Option<&'a Path<'a>> {
        unsafe { Path::from_ptr(self.0.rpath) }
    }

    pub fn lpath(&self) -> Option<&'a Path<'a>> {
        unsafe { Path::from_ptr(self.0.lpath) }
    }

    pub fn surface(&self) -> Option<&'a str> {
        self.surface_bytes()
            .and_then(|surface| str::from_utf8(surface).ok())
    }

    pub fn surface_bytes(&self) -> Option<&'a [u8]> {
        if self.0.surface.is_null() {
            None
        } else {
            Some(unsafe {
                slice::from_raw_parts(self.0.surface as *const u8, self.0.length as usize)
            })
        }
    }

    pub fn feature(&self) -> Option<&'a str> {
        self.feature_bytes()
            .and_then(|feature| str::from_utf8(feature).ok())
    }

    pub fn feature_bytes(&self) -> Option<&'a [u8]> {
        if self.0.feature.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(self.0.feature) }.to_bytes())
        }
    }

    pub fn id(&self) -> u32 {
        self.0.id as u32
    }

    pub fn length(&self) -> u16 {
        self.0.length as u16
    }

    pub fn rlength(&self) -> u16 {
        self.0.rlength as u16
    }

    #[allow(non_snake_case)]
    pub fn rcAttr(&self) -> u16 {
        self.0.rcAttr as u16
    }

    #[allow(non_snake_case)]
    pub fn lcAttr(&self) -> u16 {
        self.0.lcAttr as u16
    }

    pub fn posid(&self) -> u16 {
        self.0.posid as u16
    }

    pub fn stat(&self) -> u8 {
        self.0.stat as u8
    }

    pub fn isbest(&self) -> bool {
        self.0.isbest != 0
    }

    pub fn alpha(&self) -> f32 {
        self.0.alpha as f32
    }

    pub fn beta(&self) -> f32 {
        self.0.beta as f32
    }

    pub fn prob(&self) -> f32 {
        self.0.prob as f32
    }

    pub fn wcost(&self) -> f32 {
        self.0.wcost as f32
    }

    pub fn cost(&self) -> i64 {
        self.0.cost as i64
    }

    pub fn as_ptr(&self) -> *mut mecab_node_t {
        &self.0 as *const mecab_node_t as *mut mecab_node_t
    }

    pub unsafe fn from_ptr(ptr: *mut mecab_node_t) -> Option<&'a Self> {
        NonNull::new(ptr as *mut Self).map(|ptr| &*ptr.as_ptr())
    }

    pub unsafe fn from_ptr_unchecked(ptr: *mut mecab_node_t) -> &'a Self {
        &*(ptr as *mut Self)
    }
}

unsafe impl<'a> Send for Node<'a> {}
unsafe impl<'a> Sync for Node<'a> {}

#[derive(Debug)]
pub enum MecabError {
    NulString(NulError),
    NonUtf8String(Utf8Error),
    ModelNotAvailable,
    CurrentModelNotAvailable,
    PassedModelNotAvailable,
    InvalidModel,
    NoMoreResults,
    NBestRequired,
    NBestSizeOutOfRange,
    NodeIsNull,
    OutputBufferOverflow,
    NoAtomicSwap,
    UnknownError,
    Other(String),
    OtherBytes(Vec<u8>),
}

impl MecabError {
    unsafe fn from_tagger(ptr: *mut mecab_t) -> Self {
        let eptr = mecab_strerror(ptr);
        if eptr.is_null() {
            return MecabError::UnknownError;
        }
        let bytes = CStr::from_ptr(eptr).to_bytes();
        Self::from(bytes)
    }

    fn last() -> Self {
        unsafe { Self::from_tagger(ptr::null_mut()) }
    }
}

impl From<NulError> for MecabError {
    fn from(e: NulError) -> Self {
        MecabError::NulString(e)
    }
}

impl From<Utf8Error> for MecabError {
    fn from(e: Utf8Error) -> Self {
        MecabError::NonUtf8String(e)
    }
}

impl<'a> From<&'a str> for MecabError {
    fn from(s: &'a str) -> Self {
        use self::MecabError::*;
        match s {
            "Model is not available" => return ModelNotAvailable,
            "current model is not available" => return CurrentModelNotAvailable,
            "Passed model is not available" => return PassedModelNotAvailable,
            "Invalid model is passed" => return ModelNotAvailable,
            "no more results" => return NoMoreResults,
            "MECAB_NBEST request type is not set" => return NBestRequired,
            "nbest size must be 1 <= nbest <= 512" => return NBestSizeOutOfRange,
            "output buffer overflow" => return OutputBufferOverflow,
            "node is NULL" => return NodeIsNull,
            "atomic model replacement is not supported" => return NoAtomicSwap,
            "Unknown Error" => return UnknownError,
            _ => {}
        }
        MecabError::Other(s.to_string())
    }
}

impl<'a> From<&'a [u8]> for MecabError {
    fn from(bytes: &'a [u8]) -> Self {
        if let Ok(s) = str::from_utf8(bytes) {
            Self::from(s)
        } else {
            MecabError::OtherBytes(bytes.to_vec())
        }
    }
}
