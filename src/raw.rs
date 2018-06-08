use std::os::raw::*;

// extern type mecab_t;
#[repr(C)]
pub struct mecab_t(pub c_void);

// extern type mecab_model_t;
#[repr(C)]
pub struct mecab_model_t(pub c_void);

// extern type mecab_model_lattice_t;
#[repr(C)]
pub struct mecab_lattice_t(pub c_void);

#[repr(C)]
pub struct mecab_dictionary_info_t {
    /// filename of dictionary.
    /// On Windows, filename is stored in UTF-8 encoding
    pub filename: *const c_char,

    /// character set of the dictionary. e.g., "SHIFT-JIS", "UTF-8"
    pub charset: *const c_char,

    /// How many words are registered in this dictionary.
    pub size: c_uint,

    /// dictionary type
    /// this value should be MECAB_USR_DIC, MECAB_SYS_DIC, or MECAB_UNK_DIC.
    pub type_: c_int,

    /// left attributes size
    pub lsize: c_uint,

    /// right attributes size
    pub rsize: c_uint,

    /// version of this dictionary
    pub version: c_ushort,

    /// pointer to the next dictionary info.
    pub next: *mut mecab_dictionary_info_t,
}

/// Path structure
#[repr(C)]
pub struct mecab_path_t {
    /// pointer to the right node
    pub rnode: *mut mecab_node_t,

    /// pointer to the next right path
    pub rnext: *mut mecab_path_t,

    /// pointer to the left node
    pub lnode: *mut mecab_node_t,

    /// pointer to the next left path
    pub lnext: *mut mecab_path_t,

    /// local cost
    pub cost: c_int,

    /// marginal probability
    pub prob: c_float,
}

/// Node structure
#[allow(non_snake_case)]
#[repr(C)]
pub struct mecab_node_t {
    /// pointer to the previous node.
    pub prev: *mut mecab_node_t,

    /// pointer to the next node.
    pub next: *mut mecab_node_t,

    /// pointer to the node which ends at the same position.
    pub enext: *mut mecab_node_t,

    /// pointer to the node which starts at the same position.
    pub bnext: *mut mecab_node_t,

    /// pointer to the right path.
    /// this value is NULL if MECAB_ONE_BEST mode.
    pub rpath: *mut mecab_path_t,

    /// pointer to the right path.
    /// this value is NULL if MECAB_ONE_BEST mode.
    pub lpath: *mut mecab_path_t,

    /// surface string.
    /// this value is not 0 terminated.
    /// You can get the length with length/rlength members.
    pub surface: *const c_char,

    /// feature string
    pub feature: *const c_char,

    /// unique node id
    pub id: c_uint,

    /// length of the surface form.
    pub length: c_ushort,

    /// length of the surface form including white space before the morph.
    pub rlength: c_ushort,

    /// right attribute id
    pub rcAttr: c_ushort,

    /// left attribute id
    pub lcAttr: c_ushort,

    /// unique part of speech id. This value is defined in "pos.def" file.
    pub posid: c_ushort,

    /// character type
    pub char_type: c_uchar,

    /// status of this model.
    /// This value is MECAB_NOR_NODE, MECAB_UNK_NODE, MECAB_BOS_NODE, MECAB_EOS_NODE, or MECAB_EON_NODE.
    pub stat: c_uchar,

    /// set 1 if this node is best node.
    pub isbest: c_uchar,

    /// forward accumulative log summation.
    /// This value is only available when MECAB_MARGINAL_PROB is passed.
    pub alpha: c_float,

    /// backward accumulative log summation.
    /// This value is only available when MECAB_MARGINAL_PROB is passed.
    pub beta: c_float,

    /// marginal probability.
    /// This value is only available when MECAB_MARGINAL_PROB is passed.
    pub prob: c_float,

    /// word cost.
    pub wcost: c_short,

    /// best accumulative cost from bos node to this node.
    pub cost: c_long,
}

// Parameters for MeCab::Node::stat

/// Normal node defined in the dictionary.
pub const MECAB_NOR_NODE: c_uchar = 0;
/// Unknown node not defined in the dictionary.
pub const MECAB_UNK_NODE: c_uchar = 1;
/// Virtual node representing a beginning of the sentence.
pub const MECAB_BOS_NODE: c_uchar = 2;
/// Virtual node representing a end of the sentence.
pub const MECAB_EOS_NODE: c_uchar = 3;

/// Virtual node representing a end of the N-best enumeration.
pub const MECAB_EON_NODE: c_uchar = 4;

// Parameters for MeCab::DictionaryInfo::type

/// This is a system dictionary.
pub const MECAB_SYS_DIC: c_int = 0;

/// This is a user dictionary.
pub const MECAB_USR_DIC: c_int = 1;

/// This is a unknown word dictionary.
pub const MECAB_UNK_DIC: c_int = 2;

// Parameters for MeCab::Lattice::request_type

/// One best result is obtained (default mode)
pub const MECAB_ONE_BEST: c_int = 1;
/// Set this flag if you want to obtain N best results.
pub const MECAB_NBEST: c_int = 2;
/// Set this flag if you want to enable a partial parsing mode.
/// When this flag is set, the input |sentence| needs to be written
/// in partial parsing format.
pub const MECAB_PARTIAL: c_int = 4;
/// Set this flag if you want to obtain marginal probabilities.
/// Marginal probability is set in MeCab::Node::prob.
/// The parsing speed will get 3-5 times slower than the default mode.
pub const MECAB_MARGINAL_PROB: c_int = 8;
/// Set this flag if you want to obtain alternative results.
/// Not implemented.
pub const MECAB_ALTERNATIVE: c_int = 16;
/// When this flag is set, the result linked-list (Node::next/prev)
/// traverses all nodes in the lattice.
pub const MECAB_ALL_MORPHS: c_int = 32;

/// When this flag is set, tagger internally copies the body of passed
/// sentence into internal buffer.
pub const MECAB_ALLOCATE_SENTENCE: c_int = 64;

// Parameters for MeCab::Lattice::boundary_constraint_type

/// The token boundary is not specified.
pub const MECAB_ANY_BOUNDARY: c_int = 0;

/// The position is a strong token boundary.
pub const MECAB_TOKEN_BOUNDARY: c_int = 1;

/// The position is not a token boundary.
pub const MECAB_INSIDE_TOKEN: c_int = 2;

#[link(name = "mecab")]
extern "C" {
    // old mecab interface

    /// C wrapper of MeCab::Tagger::create(argc, argv)
    pub fn mecab_new(argc: c_int, argv: *mut *mut c_char) -> *mut mecab_t;

    /// C wrapper of MeCab::Tagger::create(arg)
    pub fn mecab_new2(arg: *const c_char) -> *mut mecab_t;

    /// C wrapper of MeCab::Tagger::version()
    pub fn mecab_version() -> *const c_char;

    /// C wrapper of MeCab::getLastError()
    pub fn mecab_strerror(mecab: *mut mecab_t) -> *const c_char;

    /// C wrapper of MeCab::deleteTagger(tagger)
    pub fn mecab_destroy(mecab: *mut mecab_t);

    /// C wrapper of MeCab::Tagger:set_partial()
    pub fn mecab_get_partial(mecab: *mut mecab_t) -> c_int;

    /// C wrapper of MeCab::Tagger::partial()
    pub fn mecab_set_partial(mecab: *mut mecab_t, partial: c_int);

    /// C wrapper of MeCab::Tagger::theta()
    pub fn mecab_get_theta(mecab: *mut mecab_t) -> c_float;

    /// C wrapper of  MeCab::Tagger::set_theta()
    pub fn mecab_set_theta(mecab: *mut mecab_t, theta: c_float);

    /// C wrapper of MeCab::Tagger::lattice_level()
    pub fn mecab_get_lattice_level(mecab: *mut mecab_t) -> c_int;

    /// C wrapper of MeCab::Tagger::set_lattice_level()
    pub fn mecab_set_lattice_level(mecab: *mut mecab_t, level: c_int);

    /// C wrapper of MeCab::Tagger::all_morphs()
    pub fn mecab_get_all_morphs(mecab: *mut mecab_t) -> c_int;

    /// C wrapper of MeCab::Tagger::set_all_moprhs()
    pub fn mecab_set_all_morphs(mecab: *mut mecab_t, all_morphs: c_int);

    /// C wrapper of MeCab::Tagger::parse(MeCab::Lattice *lattice)
    pub fn mecab_parse_lattice(mecab: *mut mecab_t, lattice: *mut mecab_lattice_t) -> c_int;

    /// C wrapper of MeCab::Tagger::parse(const char *str)
    pub fn mecab_sparse_tostr(mecab: *mut mecab_t, str: *const c_char) -> *const c_char;

    /// C wrapper of MeCab::Tagger::parse(const char *str, size_t len)
    pub fn mecab_sparse_tostr2(
        mecab: *mut mecab_t,
        str: *const c_char,
        len: usize,
    ) -> *const c_char;

    /// C wrapper of MeCab::Tagger::parse(const char *str, char *ostr, size_t olen)
    pub fn mecab_sparse_tostr3(
        mecab: *mut mecab_t,
        str: *const c_char,
        len: usize,
        ostr: *mut c_char,
        olen: usize,
    ) -> *mut c_char;

    /// C wrapper of MeCab::Tagger::parseToNode(const char *str)
    pub fn mecab_sparse_tonode(mecab: *mut mecab_t, _: *const c_char) -> *const mecab_node_t;

    /// C wrapper of MeCab::Tagger::parseToNode(const char *str, size_t len)
    pub fn mecab_sparse_tonode2(
        mecab: *mut mecab_t,
        _: *const c_char,
        _: usize,
    ) -> *const mecab_node_t;

    /// C wrapper of MeCab::Tagger::parseNBest(size_t N, const char *str)
    pub fn mecab_nbest_sparse_tostr(
        mecab: *mut mecab_t,
        N: usize,
        str: *const c_char,
    ) -> *const c_char;

    /// C wrapper of MeCab::Tagger::parseNBest(size_t N, const char *str, size_t len)
    pub fn mecab_nbest_sparse_tostr2(
        mecab: *mut mecab_t,
        N: usize,
        str: *const c_char,
        len: usize,
    ) -> *const c_char;

    /// C wrapper of MeCab::Tagger::parseNBest(size_t N, const char *str, char *ostr, size_t olen)
    pub fn mecab_nbest_sparse_tostr3(
        mecab: *mut mecab_t,
        N: usize,
        str: *const c_char,
        len: usize,
        ostr: *mut c_char,
        olen: usize,
    ) -> *mut c_char;

    /// C wrapper of MeCab::Tagger::parseNBestInit(const char *str)
    pub fn mecab_nbest_init(mecab: *mut mecab_t, str: *const c_char) -> c_int;

    /// C wrapper of MeCab::Tagger::parseNBestInit(const char *str, size_t len)
    pub fn mecab_nbest_init2(mecab: *mut mecab_t, str: *const c_char, len: usize) -> c_int;

    /// C wrapper of MeCab::Tagger::next()
    pub fn mecab_nbest_next_tostr(mecab: *mut mecab_t) -> *const c_char;

    /// C wrapper of MeCab::Tagger::next(char *ostr, size_t olen)
    pub fn mecab_nbest_next_tostr2(
        mecab: *mut mecab_t,
        ostr: *mut c_char,
        olen: usize,
    ) -> *mut c_char;

    /// C wrapper of MeCab::Tagger::nextNode()
    pub fn mecab_nbest_next_tonode(mecab: *mut mecab_t) -> *const mecab_node_t;

    /// C wrapper of MeCab::Tagger::formatNode(const Node *node)
    pub fn mecab_format_node(mecab: *mut mecab_t, node: *const mecab_node_t) -> *const c_char;

    /// C wrapper of MeCab::Tagger::dictionary_info()
    pub fn mecab_dictionary_info(mecab: *mut mecab_t) -> *const mecab_dictionary_info_t;

    // lattice interface
    /// C wrapper of MeCab::createLattice()
    pub fn mecab_lattice_new() -> *mut mecab_lattice_t;

    /// C wrapper of MeCab::deleteLattice(lattice)
    pub fn mecab_lattice_destroy(lattice: *mut mecab_lattice_t);

    /// C wrapper of MeCab::Lattice::clear()
    pub fn mecab_lattice_clear(lattice: *mut mecab_lattice_t);

    /// C wrapper of MeCab::Lattice::is_available()
    pub fn mecab_lattice_is_available(lattice: *mut mecab_lattice_t) -> c_int;

    /// C wrapper of MeCab::Lattice::bos_node()
    pub fn mecab_lattice_get_bos_node(lattice: *mut mecab_lattice_t) -> *mut mecab_node_t;

    /// C wrapper of MeCab::Lattice::eos_node()
    pub fn mecab_lattice_get_eos_node(lattice: *mut mecab_lattice_t) -> *mut mecab_node_t;

    /// C wrapper of MeCab::Lattice::begin_nodes()
    pub fn mecab_lattice_get_all_begin_nodes(
        lattice: *mut mecab_lattice_t,
    ) -> *mut *mut mecab_node_t;
    /// C wrapper of MeCab::Lattice::end_nodes()
    pub fn mecab_lattice_get_all_end_nodes(lattice: *mut mecab_lattice_t)
        -> *mut *mut mecab_node_t;

    /// C wrapper of MeCab::Lattice::begin_nodes(pos)
    pub fn mecab_lattice_get_begin_nodes(
        lattice: *mut mecab_lattice_t,
        pos: usize,
    ) -> *mut mecab_node_t;

    /// C wrapper of MeCab::Lattice::end_nodes(pos)
    pub fn mecab_lattice_get_end_nodes(
        lattice: *mut mecab_lattice_t,
        pos: usize,
    ) -> *mut mecab_node_t;

    /// C wrapper of MeCab::Lattice::sentence()
    pub fn mecab_lattice_get_sentence(lattice: *mut mecab_lattice_t) -> *const c_char;

    /// C wrapper of MeCab::Lattice::set_sentence(sentence)
    pub fn mecab_lattice_set_sentence(lattice: *mut mecab_lattice_t, sentence: *const c_char);

    /// C wrapper of MeCab::Lattice::set_sentence(sentence, len)
    pub fn mecab_lattice_set_sentence2(
        lattice: *mut mecab_lattice_t,
        sentence: *const c_char,
        len: usize,
    );

    /// C wrapper of MeCab::Lattice::size()
    pub fn mecab_lattice_get_size(lattice: *mut mecab_lattice_t) -> usize;

    /// C wrapper of MeCab::Lattice::Z()
    pub fn mecab_lattice_get_z(lattice: *mut mecab_lattice_t) -> c_double;

    /// C wrapper of MeCab::Lattice::set_Z()
    pub fn mecab_lattice_set_z(lattice: *mut mecab_lattice_t, Z: c_double);

    /// C wrapper of MeCab::Lattice::theta()
    pub fn mecab_lattice_get_theta(lattice: *mut mecab_lattice_t) -> c_double;

    /// C wrapper of MeCab::Lattice::set_theta()
    pub fn mecab_lattice_set_theta(lattice: *mut mecab_lattice_t, theta: c_double);

    /// C wrapper of MeCab::Lattice::next()
    pub fn mecab_lattice_next(lattice: *mut mecab_lattice_t) -> c_int;

    /// C wrapper of MeCab::Lattice::request_type()
    pub fn mecab_lattice_get_request_type(lattice: *mut mecab_lattice_t) -> c_int;

    /// C wrapper of MeCab::Lattice::has_request_type()
    pub fn mecab_lattice_has_request_type(
        lattice: *mut mecab_lattice_t,
        request_type: c_int,
    ) -> c_int;

    /// C wrapper of MeCab::Lattice::set_request_type()
    pub fn mecab_lattice_set_request_type(lattice: *mut mecab_lattice_t, request_type: c_int);

    /// C wrapper of MeCab::Lattice::add_request_type()
    pub fn mecab_lattice_add_request_type(lattice: *mut mecab_lattice_t, request_type: c_int);

    /// C wrapper of MeCab::Lattice::remove_request_type()
    pub fn mecab_lattice_remove_request_type(lattice: *mut mecab_lattice_t, request_type: c_int);

    /// C wrapper of MeCab::Lattice::newNode();
    pub fn mecab_lattice_new_node(lattice: *mut mecab_lattice_t) -> *mut mecab_node_t;

    /// C wrapper of MeCab::Lattice::toString()
    pub fn mecab_lattice_tostr(lattice: *mut mecab_lattice_t) -> *const c_char;

    /// C wrapper of MeCab::Lattice::toString(buf, size)
    pub fn mecab_lattice_tostr2(
        lattice: *mut mecab_lattice_t,
        buf: *mut c_char,
        size: usize,
    ) -> *const c_char;

    /// C wrapper of MeCab::Lattice::enumNBestAsString(N)
    pub fn mecab_lattice_nbest_tostr(lattice: *mut mecab_lattice_t, N: usize) -> *const c_char;

    /// C wrapper of MeCab::Lattice::enumNBestAsString(N, buf, size)
    pub fn mecab_lattice_nbest_tostr2(
        lattice: *mut mecab_lattice_t,
        N: usize,
        buf: *mut c_char,
        size: usize,
    ) -> *const c_char;

    /// C wrapper of MeCab::Lattice::has_constraint()
    pub fn mecab_lattice_has_constraint(lattice: *mut mecab_lattice_t) -> c_int;

    /// C wrapper of MeCab::Lattice::boundary_constraint(pos)
    pub fn mecab_lattice_get_boundary_constraint(
        lattice: *mut mecab_lattice_t,
        pos: usize,
    ) -> c_int;

    /// C wrapper of MeCab::Lattice::feature_constraint(pos)
    pub fn mecab_lattice_get_feature_constraint(
        lattice: *mut mecab_lattice_t,
        pos: usize,
    ) -> *const c_char;

    /// C wrapper of MeCab::Lattice::boundary_constraint(pos, type)
    pub fn mecab_lattice_set_boundary_constraint(
        lattice: *mut mecab_lattice_t,
        pos: usize,
        boundary_type: c_int,
    );

    /// C wrapper of MeCab::Lattice::set_feature_constraint(begin_pos, end_pos, feature)
    pub fn mecab_lattice_set_feature_constraint(
        lattice: *mut mecab_lattice_t,
        begin_pos: usize,
        end_pos: usize,
        feature: *const c_char,
    );

    /// C wrapper of MeCab::Lattice::set_result(result);
    pub fn mecab_lattice_set_result(lattice: *mut mecab_lattice_t, result: *const c_char);

    /// C wrapper of MeCab::Lattice::what()
    pub fn mecab_lattice_strerror(lattice: *mut mecab_lattice_t) -> *const c_char;

    // model interface
    /// C wapper of MeCab::Model::create(argc, argv)
    pub fn mecab_model_new(argc: c_int, argv: *mut *mut c_char) -> *mut mecab_model_t;

    /// C wapper of MeCab::Model::create(arg)
    pub fn mecab_model_new2(arg: *const c_char) -> *mut mecab_model_t;

    /// C wapper of MeCab::deleteModel(model)
    pub fn mecab_model_destroy(model: *mut mecab_model_t);

    /// C wapper of MeCab::Model::createTagger()
    pub fn mecab_model_new_tagger(model: *mut mecab_model_t) -> *mut mecab_t;

    /// C wapper of MeCab::Model::createLattice()
    pub fn mecab_model_new_lattice(model: *mut mecab_model_t) -> *mut mecab_lattice_t;

    /// C wrapper of MeCab::Model::swap()
    pub fn mecab_model_swap(model: *mut mecab_model_t, new_model: *mut mecab_model_t) -> c_int;

    /// C wapper of MeCab::Model::dictionary_info()
    pub fn mecab_model_dictionary_info(model: *mut mecab_model_t)
        -> *const mecab_dictionary_info_t;

    /// C wrapper of MeCab::Model::transition_cost()
    pub fn mecab_model_transition_cost(
        model: *mut mecab_model_t,
        rcAttr: c_ushort,
        lcAttr: c_ushort,
    ) -> c_int;

    /// C wrapper of MeCab::Model::lookup()
    pub fn mecab_model_lookup(
        model: *mut mecab_model_t,
        begin: *const c_char,
        end: *const c_char,
        lattice: *mut mecab_lattice_t,
    ) -> *mut mecab_node_t;

    // static functions
    pub fn mecab_do(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn mecab_dict_index(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn mecab_dict_gen(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn mecab_cost_train(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn mecab_system_eval(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn mecab_test_gen(argc: c_int, argv: *mut *mut c_char) -> c_int;
}
