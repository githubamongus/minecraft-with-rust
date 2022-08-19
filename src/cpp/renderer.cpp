#include <glad/glad.h>
#include <iostream>
#include <sstream>
#include <fstream>
#define STB_IMAGE_IMPLEMENTATION
#include <stb_image.h>

extern "C" {
    int WINDOW_WIDTH = 1920;
    int WINDOW_HEIGHT = 1080;

    void load_gl() {
        gladLoadGL();
        glViewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
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

    
    unsigned int create_shader(const char* vertex_path, const char* fragment_path) {
        std::string vertex_source_str, fragment_source_str;
        std::stringstream vertex_stream, fragment_stream;

        std::ifstream vertex_file(vertex_path);
        vertex_stream << vertex_file.rdbuf();
        vertex_file.close();
        
        std::ifstream fragment_file(fragment_path);
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

    unsigned int create_buffer(unsigned int buffer_type) {
        unsigned int buffer;
        glGenBuffers(1, &buffer);
        return buffer;
    }

    void bind_buffer(unsigned int buffer, unsigned int buffer_type) {
        glBindBuffer(buffer_type, buffer);
    }

    void buffer_data(unsigned int buffer_type, int size, void* data) {
        glBufferData(buffer_type, size, data, GL_STATIC_DRAW);
    }

    void draw_array(unsigned int count) {
        glDrawArrays(GL_TRIANGLES, 0, count);
    }

    void draw_element(unsigned int count) {
        glDrawElements(GL_TRIANGLES, count, GL_UNSIGNED_INT, 0);
    }

    unsigned int create_texture(const char* image_path) {
        int imgWidth, imgHeight, colorChAmount;
        stbi_set_flip_vertically_on_load(true);
        GLubyte* bytes = stbi_load(image_path, &imgWidth, &imgHeight, &colorChAmount, STBI_rgb_alpha);    

        unsigned int texture;
        
        glGenTextures(1, &texture);
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, texture);

        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
	    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
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

    void set_matrix_uniform(unsigned int shader, const char* name, float matrix[16]) {
        glUseProgram(shader);
        glUniformMatrix4fv(
            glGetUniformLocation(shader, name),
            1,
            GL_FALSE,
            matrix
        );
    }
}
