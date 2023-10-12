#version 330 core

out vec4 color;

in vec2 TexCoords;

uniform sampler2D gPosition;
uniform sampler2D gNormal;
uniform sampler2D gColorDiffuse;
uniform sampler2D gColorEmission;
uniform sampler2D gColorSpecular;

uniform mat4 light;

vec3 calc_dir_light(mat4 light, vec3 normal, vec3 viewdir);

void main() {
    vec3 v_position = texture(gPosition, TexCoords).xyz;
    vec3 v_normal = texture(gNormal, TexCoords).xyz;

    vec3 camera_dir = normalize(-v_position);
    vec3 view_dir = normalize(normalize(light[0].xyz) + camera_dir);

    color = vec4(calc_dir_light(light, normalize(v_normal), view_dir), 1.0);
}

vec3 calc_dir_light(mat4 light, vec3 normal, vec3 view_dir) {
    vec3 light_dir = normalize(light[0].xyz);
    // Diffuse lighting
    float diff = max(dot(normal, light_dir), 0.0);
    // Specular lighting
    float spec = pow(max(dot(view_dir, normal), 0.0), texture(gColorSpecular, TexCoords).w * 1000.0);
    // combine results
    vec3 ambient  = vec3(0.0, 0.0, 0.0); //light[1].xyz * ambient_color;
    vec3 diffuse  = light[2].xyz * diff * texture(gColorDiffuse, TexCoords).xyz;
    vec3 specular = light[3].xyz * spec * texture(gColorSpecular, TexCoords).xyz;
    return (ambient + diffuse + specular + texture(gColorEmission, TexCoords).xyz);
} 