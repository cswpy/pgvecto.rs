use pgrx::{GucContext, GucFlags, GucRegistry, GucSetting};
use std::ffi::CStr;

pub static OPENAI_API_KEY_GUC: GucSetting<Option<&'static CStr>> =
    GucSetting::<Option<&'static CStr>>::new(None);

pub static K: GucSetting<i32> = GucSetting::<i32>::new(64);

pub static ENABLE_VECTOR_INDEX: GucSetting<bool> = GucSetting::<bool>::new(true);

pub static ENABLE_PREFILTER: GucSetting<bool> = GucSetting::<bool>::new(true);

pub unsafe fn init() {
    GucRegistry::define_string_guc(
        "vectors.openai_api_key",
        "The API key of OpenAI.",
        "The OpenAI API key is required to use OpenAI embedding.",
        &OPENAI_API_KEY_GUC,
        GucContext::Userset,
        GucFlags::default(),
    );
    GucRegistry::define_int_guc(
        "vectors.k",
        "The number of nearest neighbors to return for searching.",
        "The number of nearest neighbors to return for searching.",
        &K,
        1,
        u16::MAX as _,
        GucContext::Userset,
        GucFlags::default(),
    );
    GucRegistry::define_bool_guc(
        "vectors.enable_vector_index",
        "Whether to enable vector index.",
        "When enabled, it will use existing vector index to speed up the search.",
        &ENABLE_PREFILTER,
        GucContext::Userset,
        GucFlags::default(),
    );
    GucRegistry::define_bool_guc(
        "vectors.enable_prefilter",
        "Whether to enable prefilter.",
        "When enabled, it will use prefilter to reduce the number of vectors to search.",
        &ENABLE_PREFILTER,
        GucContext::Userset,
        GucFlags::default(),
    );
}
