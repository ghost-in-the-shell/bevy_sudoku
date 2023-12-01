#import bevy_ui::ui_vertex_output::UiVertexOutput

struct CustomMaterial {
   @location(0) color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> input: CustomMaterial;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color.rgb, 1.0);
}



