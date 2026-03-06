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

    let clip_pos = uniforms.modelViewProjectionMatrix * vec4(in.position, 1.0);

    let ratio = uniforms.resolution.x / uniforms.resolution.y;
    let adjusted_offset = vec2(offset.x / ratio, offset.y);

    var out: VertexOutput;
    out.Position = clip_pos + vec4(adjusted_offset * in.size, 0, 0);
    out.uv = offset;
    out.color = in.color;
    return out;
}

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4f {
    let p = in.uv;

    let radius = 1.;
    let sd = sdCircle(p, radius);
    
    let dNorm = clamp(sd / radius, -1.0, 1.0);
    if (dNorm >= 0.0) {
        discard;
    }
    let color = unpack4x8unorm(in.color);
    
    let w = fwidth(sd); // estimates how much sd changes across a pixel
    let mask = smoothstep(w, -w, sd); // soft edge proportional to pixel size

    let t = -dNorm;
    return vec4f(mix(vec3(0.), color.xyz, t * 0.75 + 0.25), mask);
}