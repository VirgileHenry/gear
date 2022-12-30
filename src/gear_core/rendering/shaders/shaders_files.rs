
/* CORE SHADERS */

// Vertex shaders
pub static DEFAULT_VERT_SHADER: &str = include_str!("core/default.vert.glsl");

// Fragment shaders
pub static MONOCHROME_LIT_FRAG_SHADER: &str = include_str!("core/monochrome_lit.frag.glsl");
pub static MISSING_FRAG_SHADER: &str = include_str!("core/missing.frag.glsl");
pub static UNLIT_TEX_FRAG_SHADER: &str = include_str!("core/unlit_tex.frag.glsl");
pub static UNLIT_UV_FRAG_SHADER: &str = include_str!("core/unlit_uv.frag.glsl");

/* CORE SHADERS - UI SHADERS */

// Fragment
pub static UI_UNLIT_UV_FRAG_SHADER: &str = include_str!("core/ui/unlit_uv.frag.glsl");
pub static UI_DEFAULT_FRAG_SHADER: &str = include_str!("core/ui/ui_default.frag.glsl");

// Vertex
pub static UI_DEFAULT_VERT_SHADER: &str = include_str!("core/ui/default.vert.glsl");
