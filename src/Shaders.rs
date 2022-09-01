
/*
Main vertex shader
*/
pub const VERTEX_SHADER: &str = r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    in vec2 texture;
    in int material_id;

    out vec3 v_normal;
    out vec3 v_position;
    out vec2 v_texture;
    flat out int i_material;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        v_texture = texture;
        i_material = material_id;

        mat4 modelview = view * model;
        
        gl_Position = perspective * modelview * vec4(position, 1.0);

        v_position = vec3(model * vec4(position, 1.0));
        v_normal = transpose(inverse(mat3(model))) * normal;
    }
"#;

/*
Main fragment shader
*/
pub const FRAGMENT_SHADER: &str = r#"
    #version 150

    struct Material {
        int diffuse;
        int specular;
        float shininess;
    };

    struct DirectionalLight {
        vec3 direction;
        vec3 ambient_color;
        vec3 diffuse_color;
        vec3 specular_color;
    };

    struct PointLight {
        vec3 position;
        float constant;
        float linear;
        float quadratic;
        vec3 ambient_color;
        vec3 diffuse_color;
        vec3 specular_color;
    };

    struct SpotLight {
        vec3 position;
        vec3 direction;
        float cutoff;
        vec3 ambient_color;
        vec3 diffuse_color;
        vec3 specular_color;
    };

    in vec3 v_normal;
    in vec3 v_position;
    in vec2 v_texture;
    flat in int i_material;

    out vec4 color;

    uniform vec3 u_light;
    uniform vec3 v_view;

    uniform sampler2DArray textures;
    uniform Material materials[32];

    uniform int num_directional_lights;
    uniform DirectionalLight directional_lights[4];

    uniform int num_point_lights;
    uniform PointLight point_lights[128];

    uniform int num_spot_lights;
    uniform SpotLight spot_lights[128];

    vec3 calc_dir_light(DirectionalLight light, vec3 normal, vec3 view_dir) {
        vec3 light_dir = normalize(-light.direction);

        float diff = max(dot(normal, -light_dir), 0.0);
        
        vec3 reflect_dir = reflect(-light_dir, normal);
        float spec = pow(max(dot(view_dir, reflect_dir), 0.0), materials[i_material].shininess);

        vec3 ambient = light.ambient_color * vec3(texture(textures, vec3(v_texture, materials[i_material].diffuse)));
        vec3 diffuse = light.diffuse_color * diff * vec3(texture(textures, vec3(v_texture, materials[i_material].diffuse)));
        vec3 specular = light.specular_color * spec * vec3(texture(textures, vec3(v_texture, materials[i_material].specular)));

        return (ambient + diffuse + specular);
    }

    vec3 calc_point_light(PointLight light, vec3 normal, vec3 position, vec3 view_dir) {
        float distance = length(light.position - position);
        float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

        vec3 light_dir = -normalize(light.position - position);

        float diff = max(dot(normal, light_dir), 0.0);
        
        vec3 reflect_dir = reflect(light_dir, normal);
        float spec = pow(max(dot(view_dir, reflect_dir), 0.0), materials[i_material].shininess);

        vec3 ambient = light.ambient_color * vec3(texture(textures, vec3(v_texture, materials[i_material].diffuse)));
        vec3 diffuse = light.diffuse_color * diff * vec3(texture(textures, vec3(v_texture, materials[i_material].diffuse)));
        vec3 specular = light.specular_color * spec * vec3(texture(textures, vec3(v_texture, materials[i_material].specular)));

        return (ambient + diffuse + specular)*attenuation;
    }

    void main() {
        // Define accumulator vector to "accumulate" resulting color
        vec3 res_color = vec3(0.0, 0.0, 0.0);

        // Normalize normal and compute normalized view-direction vector
        vec3 norm = normalize(v_normal);
        vec3 view_dir = normalize(v_view - v_position);

        // Compute directional lights impact
        for (int i = 0; i < num_directional_lights; i++) {
            res_color += calc_dir_light(directional_lights[i], norm, view_dir);
        }

        // Compute point lights impact
        for (int i = 0; i < num_point_lights; i++) {
            res_color += calc_point_light(point_lights[i], norm, v_position, view_dir);
        }

        // Return resulting color
        color = vec4(res_color, 1.0);
    }

    
"#;

/*
Fragment shader for rendering light cubes
*/
pub const FRAGMENT_SHADER_LIGHT: &str = r#"
    #version 150

    in vec3 v_normal;
    in vec3 v_position;
    in vec2 v_texture;

    out vec4 color;

    const vec3 v_color = vec3(1.0, 1.0, 1.0);

    void main() {
        color = vec4(v_color, 1.0);
    }
"#;