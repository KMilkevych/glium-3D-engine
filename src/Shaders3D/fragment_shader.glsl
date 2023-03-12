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
    float outer_cutoff;
    float constant;
    float linear;
    float quadratic;
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
uniform DirectionalLight directional_lights[2];

uniform int num_point_lights;
uniform PointLight point_lights[124];

uniform int num_spot_lights;
uniform SpotLight spot_lights[2];

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

vec3 calc_spot_light(SpotLight light, vec3 normal, vec3 position, vec3 view_dir) {
    float distance = length(light.position - position);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));
    
    // Compute light direction
    vec3 light_dir = -normalize(light.position - position);

    float diff = max(dot(normal, light_dir), 0.0);

    //vec3 reflect_dir = reflect(light_dir, normal);
    vec3 reflect_dir = reflect(light.direction, normal);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), materials[i_material].shininess);

    float theta = dot(-light_dir, normalize(-light.direction));
    float epsilon = light.cutoff - light.outer_cutoff;
    //float intensity = clamp((theta - light.outer_cutoff) / epsilon, 0.0, 1.0);
    float intensity = smoothstep(0.0, 1.0, (theta - light.outer_cutoff) / epsilon);

    // Combine
    vec3 ambient = light.ambient_color * vec3(texture(textures, vec3(v_texture, materials[i_material].diffuse)));
    vec3 diffuse = light.diffuse_color * diff * vec3(texture(textures, vec3(v_texture, materials[i_material].diffuse)));
    vec3 specular = light.specular_color * spec * vec3(texture(textures, vec3(v_texture, materials[i_material].specular)));
    
    return (ambient + diffuse + specular)*attenuation*intensity;
    //return specular*attenuation*intensity;
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

    // Compute spot lights impact
    for (int i = 0; i < num_spot_lights; i++) {
        res_color += calc_spot_light(spot_lights[i], norm, v_position, view_dir);
    }

    // Return resulting color
    color = vec4(res_color, 1.0);
}
