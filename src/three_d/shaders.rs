use glium::Program;

// Shader types
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    None,
    Gouraud,
    BlinnPhong,
}

// Create an OpenGL vextex shader for a vertex
pub const DEFAULT_3D_SHADER: &str = r#"
    #version 140

    in vec3 position;
    in vec3 normal;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        mat4 modelview = view * model;
        gl_Position = perspective * modelview * vec4(position, 1.0);
    }
"#;

// Create an OpenGL fragment shader for a color vertex
// This determines the color (and other aspects) of a fragment (which is bacially just a pixel)
pub const DEFAULT_3D_FRAG_SHADER: &str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

// Create an OpenGL vextex shader for a vertex, using gouraud shading
pub const GOURAUD_3D_SHADER: &str = r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    out vec3 v_normal;

    
    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        mat4 modelview = view * model;
        v_normal = transpose(inverse(mat3(modelview))) * normal;
        gl_Position = perspective * modelview * vec4(position, 1.0);
    }
"#;

// Create an OpenGL fragment shader for a color vertex
// This determines the color (and other aspects) of a fragment (which is bacially just a pixel
// This uses gouraud shading
pub const GOURAUD_3D_FRAG_SHADER: &str = r#"
    #version 140

    in vec3 v_normal;
    out vec4 color;
    uniform vec3 u_light;

    void main() {
        float brightness = dot(normalize(v_normal), normalize(u_light));
        vec3 dark_color = vec3(0.6, 0.0, 0.0);
        vec3 regular_color = vec3(1.0, 0.0, 0.0);
        color = vec4(mix(dark_color, regular_color, brightness), 1.0);
    }
"#;

pub const BLINN_PHONG_3D_SHADER: &str = r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    out vec3 v_normal;
    out vec3 v_position;

    
    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        mat4 modelview = view * model;
        v_normal = transpose(inverse(mat3(modelview))) * normal;
        gl_Position = perspective * modelview * vec4(position, 1.0);
        v_position = gl_Position.xyz / gl_Position.w;
    }
"#;

pub const BLINN_PHONG_3D_FRAG_SHADER: &str = r#"
    #version 140

    in vec3 v_normal;
    in vec3 v_position;

    out vec4 color;

    uniform vec3 u_light;

    uniform vec3 ambient_color = vec3(0.2, 0.0, 0.0);
    uniform vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
    uniform vec3 specular_color = vec3(1.0, 1.0, 1.0);
    uniform float specular_exp = 16.0;

    void main() {
        float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

        vec3 camera_dir = normalize(-v_position);
        vec3 half_direction = normalize(normalize(u_light) + camera_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), specular_exp);

        color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
    }
"#;