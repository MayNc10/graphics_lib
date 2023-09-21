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

// The point light matrix contains
// position
// ambient
// diffuse
// specular

#define NR_POINT_LIGHTS 10  
uniform mat4 point_lights[NR_POINT_LIGHTS];

uniform vec3 ambient_color;
uniform vec3 diffuse_color;
uniform vec3 emission_color;
uniform vec3 specular_color;
uniform float specular_exp;

vec3 calc_dir_light(mat4 light, vec3 normal, vec3 viewdir);
vec3 calc_point_light(mat4 light, vec3 normal, vec3 frag_pos, vec3 viewdir);

void main() {


    //vec3 camera_dir = normalize(-v_position);
    //vec3 view_dir = normalize(normalize(u_light[0].xyz) + camera_dir);
    //vec3 normal = normalize(v_normal);

    vec3 normal = normalize(v_normal);
    // Does this work? Camera is always at 0.0, 0.0, 0.0 (right?)
    vec3 view_dir = normalize((0.0, 0.0, 0.0) - v_position);

    vec3 result = calc_dir_light(u_light, normal, view_dir);
    for (int i = 0; i < NR_POINT_LIGHTS; i++) {
        result += calc_point_light(point_lights[i], normal, v_position, view_dir);
    }
    result += emission_color;

    color = vec4(result, 1.0);
}

vec3 calc_dir_light(mat4 light, vec3 normal, vec3 view_dir) {
    vec3 light_dir = normalize(light[0].xyz);
    // Diffuse lighting
    float diff = max(dot(normal, light_dir), 0.0);
    // Specular lighting
    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), specular_exp);
    // combine results
    vec3 ambient = light[1].xyz * ambient_color;
    vec3 diffuse = light[2].xyz * diff * diffuse_color;
    vec3 specular = light[3].xyz * spec * specular_color;
    return (ambient + diffuse + specular);
} 

vec3 calc_point_light(mat4 light, vec3 normal, vec3 frag_pos, vec3 view_dir)
{
    vec3 light_dir = normalize(light[0].xyz - frag_pos);
    // diffuse shading
    float diff = max(dot(normal, light_dir), 0.0);
    // specular shading
    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), specular_exp);
    // attenuation
    //float distance    = length(light.position - fragPos);
    //float attenuation = 1.0 / (light.constant + light.linear * distance + 
  	//		     light.quadratic * (distance * distance));    
    // combine results
    vec3 ambient = light[1].xyz * ambient_color;
    vec3 diffuse = diffuse_color * diff;
    //light[2].xyz * diff * diffuse_color;
    vec3 specular = light[3].xyz * spec * specular_color;
    //ambient  *= attenuation;
    //diffuse  *= attenuation;
    //specular *= attenuation;
    return (ambient + diffuse + specular);
} 




