#version 330 core

out vec4 color;

in vec2 TexCoords;

uniform sampler2D gPosition;
uniform sampler2D gNormal;
uniform sampler2D gColorDiffuse;
uniform sampler2D gColorEmission;
uniform sampler2D gColorSpecular;

uniform mat4 light;

vec3 CalcPointLight(mat4 light, vec3 normal, vec3 fragPos, vec3 viewDir);  

void main() {
    vec3 v_position = texture(gPosition, TexCoords).xyz;
    vec3 v_normal = texture(gNormal, TexCoords).xyz;

    vec3 camera_dir = normalize(-v_position);
    vec3 view_dir = normalize(normalize(light[0].xyz) + camera_dir);

    color = vec4(CalcPointLight(light, normalize(v_normal), v_position, view_dir), 1.0);
}

vec3 CalcPointLight(mat4 light, vec3 normal, vec3 fragPos, vec3 viewDir)
{
    vec3 lightDir = normalize(light[0].xyz - fragPos);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0);
    // specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    float specular_exp = texture(gColorSpecular, TexCoords).w * 1000.0;
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), specular_exp);
    // attenuation
    float distance = length(light[0].xyz - fragPos);
    float attenuation = 1.0 / (light[1].w + light[2].w * distance + light[3].w * (distance * distance));    
    // combine results
    vec3 diffuse = light[2].xyz * diff * texture(gColorDiffuse, TexCoords).rgb;
    vec3 specular = light[3].xyz * spec * vec3(texture(gColorSpecular, TexCoords));
    //ambient  *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;
    return (diffuse + specular);
} 