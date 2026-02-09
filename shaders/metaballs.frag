#version 100
precision highp float;

varying lowp vec2 uv;

uniform vec3 metaballs[100]; 
uniform int count;           
uniform float threshold;     
uniform vec2 resolution;
uniform vec3 base_color;     

void main() {

    vec2 pixel = uv * resolution;
    float avg_screen = 0.5 * (resolution.x + resolution.y);

    float total = 0.0;

    for (int i = 0; i < 100; i++) {
        if (i >= count) break;
        vec2 center = metaballs[i].xy * resolution;
        float radius_px = metaballs[i].z * avg_screen;
        vec2 delta = pixel - center;
        float dist2 = dot(delta, delta) + 0.000001; 
        total += (radius_px * radius_px) / dist2;
    }

    float mask = step(threshold, total);
    gl_FragColor = vec4(base_color, 1.0) * mask;

}
