
/* CORE SHADERS */

use crate::ShaderSource;

lazy_static! {

// Vertex shaders
pub static ref DEFAULT_VERT_SHADER: ShaderSource = ShaderSource::new("core/default.vert.glsl");

// Fragment shaders
pub static ref MONOCHROME_LIT_FRAG_SHADER: ShaderSource = ShaderSource::new("core/monochrome_lit.frag.glsl");
pub static ref MISSING_FRAG_SHADER: ShaderSource = ShaderSource::new("core/missing.frag.glsl");
pub static ref UNLIT_TEX_FRAG_SHADER: ShaderSource = ShaderSource::new("core/unlit_tex.frag.glsl");
pub static ref UNLIT_UV_FRAG_SHADER: ShaderSource = ShaderSource::new("core/unlit_uv.frag.glsl");

/* CORE SHADERS - UI SHADERS */

// Copy shader
pub static ref COPY_FRAG_SHADER: ShaderSource = ShaderSource::new("core/ui/copy.frag.glsl");
pub static ref COPY_VERT_SHADER: ShaderSource = ShaderSource::new("core/ui/copy.vert.glsl");

pub static ref RENDER_FRAG_SHADER: ShaderSource = ShaderSource::new("core/render.frag.glsl");


// Fragment
pub static ref UI_MONOCHROME_FRAG_SHADER: ShaderSource = ShaderSource::new("core/ui/monochrome.frag.glsl");

// Vertex
pub static ref UI_DEFAULT_VERT_SHADER: ShaderSource = ShaderSource::new("core/ui/default.vert.glsl");

/* PIPELINE */
pub static ref PIPELINE_DEFAULT_VERT: ShaderSource = ShaderSource::new("core/pipeline/default.vert.glsl");

}