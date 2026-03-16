@group(0) @binding(0) var<uniform> size: vec2<f32>;
@group(0) @binding(1) var<uniform> scale: f32;

fn to_clip(pixel: vec2<f32>) -> vec2<f32> {
    // Pixel (0,0) is top-left
    // Clip (-1,1) is top-left
    return vec2(
         2.0 * pixel.x / size.x - 1.0,
        -2.0 * pixel.y / size.y + 1.0,
    );
}

// Thickness grows border outwards
struct InstanceInput {
    @location(0) left: f32,
    @location(1) right: f32,
    @location(2) top: f32,
    @location(3) bottom: f32,
    @location(4) thickness: f32,
    @location(5) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) center: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) thickness: f32,
    @location(3) color: vec4<f32>,
};

fn sdBox(p: vec2<f32>, b: vec2<f32>) -> f32 {
    var d = abs(p) - b;
    var outside = length(max(d, vec2(0.0)));
    var inside = min(max(d.x, d.y), 0.0);
    return outside + inside;
}

@vertex
fn vs_main(
    @builtin(vertex_index) vid: u32,
    in: InstanceInput,
) -> VertexOutput {
    // Add thickness padding so the border isn't clipped at the quad edge
    let padding = in.thickness * scale;
    let left   = in.left * scale   - padding;
    let right  = in.right * scale  + padding;
    let top    = in.top * scale    - padding;
    let bottom = in.bottom * scale + padding;

    let x = select(left, right, (vid & 1u) != 0u);
    let y = select(top, bottom, (vid & 2u) != 0u);

    var out: VertexOutput;
    out.clip_position = vec4<f32>(to_clip(vec2(x, y)), 0.0, 1.0);
    
    // Pass through everything the fragment shader needs
    out.center = vec2(
        (in.left + in.right)  / 2.0,
        (in.top  + in.bottom) / 2.0,
    ) * scale;
    out.size      = vec2(in.right - in.left, in.bottom - in.top) * scale;
    out.thickness = in.thickness * scale;
    out.color     = in.color;
    return out;
}

// Fragment shader

@fragment
fn fs_border(in: VertexOutput) -> @location(0) vec4<f32> {
    var sdf = sdBox(in.clip_position.xy - in.center, in.size / 2.0);
    var mask = abs(sdf - in.thickness / 2.0) - in.thickness / 2.0;
    let alpha = clamp(-mask, 0.0, 1.0);
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}

@fragment
fn fs_fill(in: VertexOutput) -> @location(0) vec4<f32> {
    let d = sdBox(in.clip_position.xy - in.center, in.size / 2.0);
    let alpha = clamp(-d, 0.0, 1.0);
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
