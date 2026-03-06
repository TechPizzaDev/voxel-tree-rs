struct Uniforms {
  modelViewProjectionMatrix: mat4x4f,
  inverseModelViewProjectionMatrix: mat4x4f,
  time: vec4f,
  resolution: vec4f,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct Vertex {
  @location(0) position: vec3f,
  @location(1) size: f32,
  @location(2) color: u32,
};

struct VertexOutput {
  @builtin(position) Position: vec4f,
  @location(0) uv: vec2f,
  @location(1) color: u32,
}

fn sdCircle(p: vec2f, r: f32) -> f32 {
    return length(p) - r;   
}

@vertex
fn vertex_main(
    in: Vertex,
    @builtin(vertex_index) VertexIndex: u32,
) -> VertexOutput {
    let points = array(
        vec2f(-1, -1),
        vec2f( 1, -1),
        vec2f(-1,  1),
        vec2f(-1,  1),
        vec2f( 1, -1),
        vec2f( 1,  1),
    );
    let offset = points[VertexIndex];

    let clipPos = uniforms.modelViewProjectionMatrix * vec4f(in.position, 1.0);

    var out: VertexOutput;
    out.Position = clipPos + vec4f(offset * in.size / uniforms.resolution.xy, 0, 0);
    out.uv = offset;
    out.color = in.color;
    return out;
}

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4f {
    let p = in.uv;
    let color = unpack4x8unorm(in.color);

    let radius = 0.5;
    let d = sdCircle(p, radius);
    
    let dNorm = clamp(d / radius, -1.0, 1.0);
    if (dNorm >= 0.0) {
        discard;
    }

    let t = -dNorm;
    return vec4f(mix(vec3(0.), color.xyz, t), 1.0f);
}