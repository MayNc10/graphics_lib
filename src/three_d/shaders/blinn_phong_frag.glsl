#version 140

in vec3 v_normal;
in vec3 v_position;

out vec4 color;

uniform mat4 u_light;
// The light matrix contains
// direction
// ambient
// diffuse
// specular

uniform vec3 ambient_color;
uniform vec3 diffuse_color;
uniform vec3 emission_color;
uniform vec3 specular_color;
uniform float specular_exp;

vec3 calc_dir_light(mat4 light, vec3 normal, vec3 viewdir);

void main() {

    vec3 camera_dir = normalize(-v_position);
    vec3 view_dir = normalize(normalize(u_light[0].xyz) + camera_dir);

    color = vec4(calc_dir_light(u_light, normalize(v_normal), view_dir), 1.0);
}

vec3 calc_dir_light(mat4 light, vec3 normal, vec3 view_dir) {
    vec3 light_dir = normalize(-light[0].xyz);
    // diffuse shading
    float diff = max(dot(normal, light_dir), 0.0);
    // specular shading
    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), specular_exp);
    // combine results
    vec3 ambient  = light[1].xyz  * ambient_color;
    vec3 diffuse  = light[2].xyz  * diff * diffuse_color;
    vec3 specular = light[3].xyz * spec * specular_color;
    return (ambient + diffuse + specular + emission_color);
} 