struct Uniforms {
  inverseModelViewProjectionMatrix: mat4x4f,
  time: vec4f,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOutput {
  @builtin(position) Position: vec4f,
  @location(0) near: vec3f,
  @location(1) step: vec3f,
}

override Xray = false;
override Opaque = true;
override NumSteps: u32 = select(160u, 256u, Xray);

@vertex
fn vertex_main(
    @builtin(vertex_index) VertexIndex: u32
) -> VertexOutput {
    var a = -0.99f;
    var b = 0.99f;
    var pos = array<vec2f, 4>(
        vec2(a, b),
        vec2(a, a),
        vec2(b, a),
        vec2(b, b),
    );
    var indices = array<i32, 6>(
        0, 1, 2,
        0, 2, 3,
    );
    var idx = indices[VertexIndex];
    var xy = pos[idx];

    var near = uniforms.inverseModelViewProjectionMatrix * vec4f(xy, 0f, 1f);
    var far = uniforms.inverseModelViewProjectionMatrix * vec4f(xy, 1f, 1f);
    near /= near.w;
    far /= far.w;
    return VertexOutput(
        vec4f(xy, 0f, 1f),
        near.xyz,
        (far.xyz - near.xyz) / f32(NumSteps)
    );
}

fn sdSphere(p: vec3f, d: f32) -> f32 {
    return length(p) - d;
}

fn sdBox(p: vec3f, b: vec3f) -> f32 {
    let d = abs(p) - b;
    return min(max(d.x, max(d.y, d.z)), 0.0) + length(max(d, vec3f(0.0)));
}

fn getVoxel(c: vec3f) -> f32 {
    //let p = c + 0.5;
	//let d = min(max(-sdSphere(p, 7.5), sdBox(p, vec3(6.0))), -sdSphere(p, 25.0));

    let p = c + vec3f(0.5);
    let cutWave = sin(uniforms.time.x * 0.66);
    let boxCut = sign(cutWave) * sdSphere(p, 7.5 * abs(cutWave));
    let box = max(boxCut, sdBox(p, vec3(6.0)));
    let d = min(box, -sdSphere(p, 24.0));

    return d;
}

@fragment
fn fragment_main(
    @location(0) near: vec3f,
    @location(1) step: vec3f
) -> @location(0) vec4f {
    return frag_voxel(near, step);
}

fn ray_to_tex(pos: vec3f) -> vec3f {
    return (pos + 1f) * 0.5f;
}

fn frag_voxel(near: vec3f, step: vec3f) -> vec4f {
    let rayPos = near;
    let rayDir = step;

    var mapPos = floor(rayPos);
    let deltaDist = abs(vec3f(length(rayDir)) / rayDir);
    let rayStep = sign(rayDir);
    var sideDist = (sign(rayDir) * (mapPos - rayPos) + (sign(rayDir) * 0.5) + 0.5) * deltaDist;

    var mask: vec3<bool>;
    var count = 0u;
    var max = 0u;
    var i = 0u;
    for (; i < NumSteps; i++) {
        //let off = (vec3f(sin(uniforms.time.x * 0.2), 0f, 0f) + 1f) * 0.25;

        let s = getVoxel(mapPos);
        //let threshold = (sin(uniforms.time.x * 0.5) + 1.0) * 0.5;
        if s < 0.0 {
            count += 1u;
            if (Opaque) {
                break;
            }
        }
        max += 1u;

        mask = sideDist < min(sideDist.yzx, sideDist.zxy);
        // slow: x += vec3f(mask) * y 
        // slow: x  = select(x, x + y, mask)
        // fast: x += select(0, y, mask)
        sideDist += select(vec3f(0.), deltaDist, mask);
        mapPos += select(vec3f(0.), rayStep, mask);
    }

    if i >= NumSteps - 1 {
        return vec4f(vec3f(
            f32(count) / f32(max), 
            f32(count) / f32(NumSteps) * 1f, 
            f32(max) / f32(NumSteps)) * 1.5f, 
            1f);
    }

    let color = vec3f(mask) * vec3f(0.5, 1.0, 0.75);
    return vec4f(color, 1f);
}