#version 330 core

layout (location = 0) out vec3 out_position;
layout (location = 1) out vec3 out_normal;
layout (location = 2) out vec4 out_diffuse;
layout (location = 3) out vec4 out_emission;
layout (location = 4) out vec4 out_specular;

in vec3 v_position;
in vec3 v_normal;

uniform vec3 ambient_color;
uniform vec3 diffuse_color;
uniform vec3 emission_color;
uniform vec3 specular_color;
uniform float specular_exp;

uniform mat4 u_light;

void main()
{    
    // store the fragment position vector in the first gbuffer texture
    out_position = v_position;
    // also store the per-fragment normals into the gbuffer
    out_normal = normalize(v_normal);
    // and the diffuse color
    out_diffuse = vec4(diffuse_color, 1.0);
    out_emission = vec4(emission_color, 1.0);
    out_specular = vec4(specular_color, specular_exp / 1000.0);
} 