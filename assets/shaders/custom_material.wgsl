// #import bevy_sprite::mesh2d_view_bindings
// #import bevy_pbr::utils

@group(1) @binding(0)
var source_color_texture: texture_2d<f32>;
@group(1) @binding(1)
var source_texture_sampler: sampler;
@group(1) @binding(2)
var lookup_color_texture: texture_2d<f32>;
@group(1) @binding(3)
var lookup_texture_sampler: sampler;


@fragment
fn fragment(
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // let dims = vec2<f32>(32.,32.);
    // let dims = vec2<f32>(textureDimensions(source_color_texture));

    // let map_uv = uv - (uv % (1f/dims)) + (1f / (dims * 2f));

    // let source_uv = textureSample(source_color_texture, vec2<i32>(i32(map_uv.x),i32(map_uv.y)), 0);
    // //example: source_uv.r == 15/255 so when *255 == 15, then 15/32 = the new uv for the lookup
    // let mapped_x = (source_uv.r)/32.; 
    // let mapped_y = (source_uv.g)/32.;
    // let color = textureSample(lookup_color_texture, lookup_texture_sampler, vec2<f32>(mapped_x, mapped_y));

    // return vec4<f32>(1.,1.,0.,1.);
    let uv_map_dims = vec2<f32>(textureDimensions(source_color_texture));
    let palette_dims = vec2<f32>(textureDimensions(lookup_color_texture));

    let uv_map_uv = uv - (uv % (1f/32f)) + (1f / 64f);

    let uv_map = textureSample(
        source_color_texture,
        source_texture_sampler,
        uv_map_uv
    );

    let palette_uv = (uv_map.rg * 255f + vec2<f32>(0.5)) / palette_dims;

    let color = textureSample(
        lookup_color_texture,
        lookup_texture_sampler,
        palette_uv.xy
    );

    return vec4<f32>(color.rgb ,1f);
        // return vec4<f32>(1.,1.,0.,1.);

}
