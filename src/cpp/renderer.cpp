#include <glad/glad.h>
#include <glm/glm.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <glm/gtx/vector_angle.hpp>
#define STB_IMAGE_IMPLEMENTATION
#include <stb_image.h>
#include <GLFW/glfw3.h>
#include <cstdio>
#include <iostream>
#include <fstream>
#include <ostream>
#include <sstream>

extern "C" {
    void load_gl() {
        gladLoadGL();
        glViewport(0, 0, 1920, 1080);
        glEnable(GL_DEPTH_TEST);
        glEnable(GL_CULL_FACE);
    }

    void clear_screen() {
        glClearColor(0.0, 0.4, 0.6, 1.0);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    }

    void check_shader(unsigned int shader, const char* type) {
        int success;
        char infoLog[512];
        
        if (type != "PROGRAM") {
            glGetShaderiv(shader, GL_COMPILE_STATUS, &success);

            if (!success) {
                glGetShaderInfoLog(shader, 512, NULL, infoLog);
                std::cout << "SHADER_ERROR:" << type << "\n" << infoLog << std::endl; 
            }
        }

        else {
            glGetProgramiv(shader, GL_LINK_STATUS, &success);
            if (!success) {
                glGetProgramInfoLog(shader, 512, NULL, infoLog);
                std::cout << "SHADER_ERROR:" << type << "\n" << infoLog << std::endl;
            }
        }
    }

    unsigned int create_shader() {
        std::string vertex_source_str, fragment_source_str;
        std::stringstream vertex_stream, fragment_stream;

        std::ifstream vertex_file("shaders/default.vert");
        vertex_stream << vertex_file.rdbuf();
        vertex_file.close();
        
        std::ifstream fragment_file("shaders/default.frag");
        fragment_stream << fragment_file.rdbuf();
        fragment_file.close();

        vertex_source_str = vertex_stream.str();
        fragment_source_str = fragment_stream.str();

        const char* vertex_source = vertex_source_str.c_str();
        const char* fragment_source = fragment_source_str.c_str();

        unsigned int vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vertex_shader, 1, &vertex_source, nullptr);
        glCompileShader(vertex_shader);
        check_shader(vertex_shader, "VERTEX");

        unsigned int fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fragment_shader, 1, &fragment_source, nullptr);
        glCompileShader(fragment_shader);
        check_shader(fragment_shader, "FRAGMENT");

        unsigned int program = glCreateProgram();
        glAttachShader(program, vertex_shader);
        glAttachShader(program, fragment_shader);
        glLinkProgram(program);
        check_shader(program, "PROGRAM");

        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);

        return program;
    }

    unsigned int create_2d_shader() {
        std::string vertex_source_str, fragment_source_str;
        std::stringstream vertex_stream, fragment_stream;

        std::ifstream vertex_file("shaders/2d.vert");
        vertex_stream << vertex_file.rdbuf();
        vertex_file.close();
        
        std::ifstream fragment_file("shaders/2d.frag");
        fragment_stream << fragment_file.rdbuf();
        fragment_file.close();

        vertex_source_str = vertex_stream.str();
        fragment_source_str = fragment_stream.str();

        const char* vertex_source = vertex_source_str.c_str();
        const char* fragment_source = fragment_source_str.c_str();

        unsigned int vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vertex_shader, 1, &vertex_source, nullptr);
        glCompileShader(vertex_shader);
        check_shader(vertex_shader, "VERTEX");

        unsigned int fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fragment_shader, 1, &fragment_source, nullptr);
        glCompileShader(fragment_shader);
        check_shader(fragment_shader, "FRAGMENT");

        unsigned int program = glCreateProgram();
        glAttachShader(program, vertex_shader);
        glAttachShader(program, fragment_shader);
        glLinkProgram(program);
        check_shader(program, "PROGRAM");

        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);

        return program;
    }

    void use_shader(unsigned int shader) {
        glUseProgram(shader);
    }

    unsigned int create_vao() {
        unsigned int vao;
        glGenVertexArrays(1, &vao);
        return vao;
    }

    void bind_vao(unsigned int vao) {
        glBindVertexArray(vao);
    }

    void link_attrib(int layout, int components, int stride, unsigned int byte_offset) {
        glVertexAttribPointer(layout, components, GL_FLOAT, GL_FALSE, stride, (void*)byte_offset);
        glEnableVertexAttribArray(layout);
    }

    unsigned int create_vbo() {
        unsigned int vbo;
        glGenBuffers(1, &vbo);
        return vbo;
    }

    void bind_vbo(unsigned int vbo) {
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
    }

    void vbo_data(int size, void* data) {
        glBufferData(GL_ARRAY_BUFFER, size, data, GL_STATIC_DRAW);
    }

    unsigned int create_ebo() {
        unsigned int ebo;
        glGenBuffers(1, &ebo);
        return ebo;
    }

    void bind_ebo(unsigned int ebo) {
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
    }

    void ebo_data(int size, void* data) {
        glBufferData(GL_ELEMENT_ARRAY_BUFFER, size, data, GL_STATIC_DRAW);
    }

    unsigned int create_texture(const char* image_path) {
        int imgWidth, imgHeight, colorChAmount;
        stbi_set_flip_vertically_on_load(true);
        GLubyte* bytes = stbi_load(image_path, &imgWidth, &imgHeight, &colorChAmount, STBI_rgb_alpha);    

        unsigned int texture;
        
        glGenTextures(1, &texture);
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, texture);

        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
	    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
	    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);

        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, imgWidth, imgHeight, 0, GL_RGBA, GL_UNSIGNED_BYTE, bytes);
        glGenerateMipmap(GL_TEXTURE_2D);
        stbi_image_free(bytes);
        glBindTexture(GL_TEXTURE_2D, 0);

        return texture;
    }

    void bind_texture(unsigned int texture) {
        glBindTexture(GL_TEXTURE_2D, texture);
    }

    void set_texture_uniform(unsigned int shader) {
        glUseProgram(shader);
        glUniform1i(glGetUniformLocation(shader, "tex"), GL_TEXTURE0);
    }

    glm::mat4 proj = glm::mat4(1.0f);
    glm::mat4 view = glm::mat4(1.0f);

    void create_projection() {
        proj = glm::perspective(glm::radians(45.0f), 1920.0f/1080.0f, 0.1f, 100.0f);
    }

    glm::vec3 position_vec;
    glm::vec3 direction_vec;

    void create_view_matrix(float position[3], float direction[3]) {
        position_vec = glm::vec3(position[0], position[1], position[2]);
        direction_vec = glm::vec3(direction[0], direction[1], direction[2]);

        view = glm::lookAt(position_vec, position_vec + direction_vec, glm::vec3(0.0, 1.0, 0.0));
    }

    void draw(unsigned int shader, unsigned int texture, unsigned int vao, unsigned int vertices_count, bool is3d) {        
        glUseProgram(shader);

        if (is3d) {
            glUniformMatrix4fv(glGetUniformLocation(shader, "view"), 1, GL_FALSE, glm::value_ptr(view));
            glUniformMatrix4fv(glGetUniformLocation(shader, "proj"), 1, GL_FALSE, glm::value_ptr(proj));
        }

        glBindTexture(GL_TEXTURE_2D, texture);
        glBindVertexArray(vao);
        glDrawArrays(GL_TRIANGLES, 0, vertices_count);
    }
}