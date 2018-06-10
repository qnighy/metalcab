//! Raw FFI declarations for MeCab
//!
//! This module is a mere collection of FFI declarations copied from `mecab.h`.

use std::os::raw::*;

/// Raw C Interface for [`Tagger`][compat::Tagger].
///
/// [compat::Tagger]: ../compat/struct.Tagger.html
///
/// ## Note on extern type
///
/// It is intended to be an external opaque type, of which size and alignment are unknown.
///
/// Once [RFC 1861][RFC1861] is stabilized, the declaration will be changed to:
///
/// ```rust,ignore
/// extern {
///     type mecab_t;
/// }
/// ```
///
/// [RFC1861]: http://rust-lang.github.io/rfcs/1861-extern-types.html
///
// extern { type mecab_t; }
#[repr(C)]
pub struct mecab_t(pub c_void);

/// Raw C Interface for [`Model`][compat::Model].
///
/// [compat::Model]: ../compat/struct.Model.html
///
/// ## Note on extern type
///
/// It is intended to be an external opaque type, of which size and alignment are unknown.
///
/// Once [RFC 1861][RFC1861] is stabilized, the declaration will be changed to:
///
/// ```rust,ignore
/// extern {
///     type mecab_model_t;
/// }
/// ```
///
/// [RFC1861]: http://rust-lang.github.io/rfcs/1861-extern-types.html
///
// extern { type mecab_model_t; }
#[repr(C)]
pub struct mecab_model_t(pub c_void);

/// Raw C Interface for [`Lattice`][compat::Lattice].
///
/// [compat::Lattice]: ../compat/struct.Lattice.html
///
/// ## Note on extern type
///
/// It is intended to be an external opaque type, of which size and alignment are unknown.
///
/// Once [RFC 1861][RFC1861] is stabilized, the declaration will be changed to:
///
/// ```rust,ignore
/// extern {
///     type mecab_lattice_t;
/// }
/// ```
///
/// [RFC1861]: http://rust-lang.github.io/rfcs/1861-extern-types.html
///
// extern { type mecab_lattice_t; }
#[repr(C)]
pub struct mecab_lattice_t(pub c_void);

/// Raw C Interface for [`DictionaryInfo`][compat::DictionaryInfo].
///
/// [compat::DictionaryInfo]: ../compat/struct.DictionaryInfo.html
#[repr(C)]
pub struct mecab_dictionary_info_t {
    /// The filename of the dictionary.
    ///
    /// On Windows, filenames are stored in the UTF-8 encoding.
    pub filename: *const c_char,

    /// The character set of the dictionary, e.g., `SHIFT-JIS` and `UTF-8`.
    pub charset: *const c_char,

    /// The number of words registered in the dictionary.
    pub size: c_uint,

    /// Dictionary type.
    ///
    /// The value is one of [`MECAB_USR_DIC`][MECAB_USR_DIC],
    /// [`MECAB_SYS_DIC`][MECAB_SYS_DIC], or [`MECAB_UNK_DIC`][MECAB_UNK_DIC].
    ///
    /// [MECAB_USR_DIC]: ../constant.MECAB_USR_DIC.html
    /// [MECAB_SYS_DIC]: ../constant.MECAB_SYS_DIC.html
    /// [MECAB_UNK_DIC]: ../constant.MECAB_UNK_DIC.html
    pub type_: c_int,

    /// The number of left attribute IDs.
    pub lsize: c_uint,

    /// The number of right attribute IDs.
    pub rsize: c_uint,

    /// The version of the dictonary format.
    pub version: c_ushort,

    /// The pointer to the next dictionary info.
    ///
    /// NULL indicates the end of the linked list.
    pub next: *mut mecab_dictionary_info_t,
}

/// Raw C Interface for [`Path`][compat::Path].
///
/// [compat::Path]: ../compat/struct.Path.html
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

/// Raw C Interface for [`Node`][compat::Node].
///
/// [compat::Node]: ../compat/struct.Node.html
#[allow(non_snake_case)]
#[repr(C)]
pub struct mecab_node_t {
    /// The pointer to the previous node.
    ///
    /// NULL indicates the beginning of the linked list.
    pub prev: *mut mecab_node_t,

    /// The pointer to the next node.
    ///
    /// NULL indicates the end of the linked list.
    pub next: *mut mecab_node_t,

    /// The pointer to the node which ends at the same position.
    pub enext: *mut mecab_node_t,

    /// The pointer to the node which starts at the same position.
    pub bnext: *mut mecab_node_t,

    /// The pointer to the right path.
    ///
    /// In `MECAB_ONE_BEST` mode, it has NULL value.
    pub rpath: *mut mecab_path_t,

    /// The pointer to the left path.
    ///
    /// In `MECAB_ONE_BEST` mode, it has NULL value.
    pub lpath: *mut mecab_path_t,

    /// The surface string.
    ///
    /// **This is not a NUL-terminated string**. Use `length` or `rlength` to obtain
    /// its byte-length.
    pub surface: *const c_char,

    /// The feature string, e.g., `名詞,固有名詞,人名,名,*,*,太郎,タロウ,タロー`.
    pub feature: *const c_char,

    /// The unique node id given to this node.
    pub id: c_uint,

    /// The length of the surface form.
    pub length: c_ushort,

    /// The length of the surface form, including white space before the morph.
    pub rlength: c_ushort,

    /// The right attribute id.
    pub rcAttr: c_ushort,

    /// The left attribute id.
    pub lcAttr: c_ushort,

    /// The unique part-of-speech id, as defined in the `pos-id.def` file. For the ipadic, they are:
    /// - その他 (others)
    ///   - 間投 (interjections) = 0
    /// - フィラー (fillers) = 1
    /// - 感動詞 (interjections) = 2
    /// - 記号 (symbols)
    ///   - アルファベット (european characters) = 3
    ///   - 一般 (general symbols) = 4
    ///   - 括弧開 (open parentheses) = 5
    ///   - 括弧閉 (close parentheses) = 6
    ///   - 句点 (periods) = 7
    ///   - 空白 (spaces) = 8
    ///   - 読点 (commata) = 9
    /// - 形容詞 (adjectives)
    ///   - 自立 (autonomous adjectives) 10
    ///   - 接尾 (adjective-making suffixes) 11
    ///   - 非自立 (non-autonomous adjectives) 12
    /// - 助詞 (particles)
    ///   - 格助詞 (case markers)
    ///     - 一般 (general case markers) = 13
    ///     - 引用 (quoting case markers) = 14
    ///     - 連語 (combined case markers and words) = 15
    ///   - 係助詞 (binding particles) = 16
    ///   - 終助詞 (sentence-ending particles) = 17
    ///   - 接続助詞 (conjunctive particles) = 18
    ///   - 特殊 (special particles) = 19
    ///   - 副詞化 (adverb-making particles) = 20
    ///   - 副助詞 (adverbial particles) = 21
    ///   - 副助詞／並立助詞／終助詞 (adverbial, parallel, or sentence-ending particles) = 22
    ///   - 並立助詞 (parallel particles) = 23
    ///   - 連体化 (adjective-making particles) = 24
    /// - 助動詞 (auxiliary verbs) = 25
    /// - 接続詞 (conjunctions) = 26
    /// - 接頭詞 (prefixes)
    ///   - 形容詞接続 (prefixes joining with adjectives) = 27
    ///   - 数接続 (prefixes joining with numerals) = 28
    ///   - 動詞接続 (prefixes joining with verbs) = 29
    ///   - 名詞接続 (prefixes joining with nouns) = 30
    /// - 動詞 (verbs)
    ///   - 自立 (autonomous verbs) = 31
    ///   - 接尾 (verb-making suffixes) = 32
    ///   - 非自立 (non-autonomous verbs) = 33
    /// - 副詞 (adverbs)
    ///   - 一般 (general adverbs)  =34
    ///   - 助詞類接続 (adverbs joining with particles) = 35
    /// - 名詞 (nouns)
    ///   - サ変接続 (nouns connecting to sa-irregular verbs) = 36
    ///   - ナイ形容詞語幹 (stems of nai-adjectives) = 37
    ///   - 一般 (general nouns) = 38
    ///   - 引用文字列 (quoted strings) = 39
    ///   - 形容動詞語幹 (stems of na-adjectives) = 40
    ///   - 固有名詞 (proper nouns)
    ///     - 一般 (general proper nouns) = 41
    ///     - 人名 (names of persons)
    ///       - 一般 (general names of persons) = 42
    ///       - 姓 (last names) = 43
    ///       - 名 (first names) = 44
    ///     - 組織 (names of organizations) = 45
    ///     - 地域 (names of regions)
    ///       - 一般 (names of general regions) = 46
    ///       - 国 (names of countries) = 47
    ///   - 数 (numerals) = 48
    ///   - 接続詞的 (conjunction-like nouns) = 49
    ///   - 接尾 (suffixes)
    ///     - サ変接続 (noun suffixes connecting to sa-irregular verbs) = 50
    ///     - 一般 (general noun suffixes) = 51
    ///     - 形容動詞語幹 (stem suffixes of na-adjectives) = 52
    ///     - 助数詞 (counter words) = 53
    ///     - 助動詞語幹 (stems suffixes of auxiliary verbs) = 54
    ///     - 人名 (suffixes of names of persons) = 55
    ///     - 地域 (suffixes of names of regions) = 56
    ///     - 特殊 (special noun suffixes) = 57
    ///     - 副詞可能 (suffixes of adverbial nouns) = 58
    ///   - 代名詞 (pronouns)
    ///     - 一般 (general pronouns) = 59
    ///     - 縮約 (abbreviated pronouns) = 60
    ///   - 動詞非自立的 (nouns from non-autonomous verbs) = 61
    ///   - 特殊 (special nouns)
    ///     - 助動詞語幹 (stems of auxiliary verbs) = 62
    ///   - 非自立 (non-autonomous nouns)
    ///     - 一般 (general non-autonomous nouns) = 63
    ///     - 形容動詞語幹 (non-autonomous stems of na-adjectives) = 64
    ///     - 助動詞語幹 (non-autonomous stems of auxiliary verbs) = 65
    ///     - 副詞可能 (non-autonomous adverbial nouns) = 66
    ///   - 副詞可能 (adverbial nouns) = 67
    /// - 連体詞 (nonconjugating adjectives; attributives) = 68
    pub posid: c_ushort,

    /// The character type, as defined in the `char.def` file. For the ipadic, they are:
    ///
    /// - DEFAULT = 0
    /// - SPACE = 1
    /// - KANJI = 2
    /// - SYMBOL = 3
    /// - NUMERIC = 4
    /// - ALPHA = 5
    /// - HIRAGANA = 6
    /// - KATAKANA = 7
    /// - KANJINUMERIC = 8
    /// - GREEK = 9
    /// - CYRILLIC = 10
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
