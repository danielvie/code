
// main.cpp
// A basic "Hello World" application for modern OpenGL using GLAD.
// This program initializes a window using GLFW, sets up an OpenGL context with GLAD,
// and renders a single colored triangle.

// Third-party libraries
// GLAD: Manages OpenGL function pointers (modern replacement for GLEW).
#include <glad/glad.h>
// GLFW: Handles window creation, input, and context management.
#include <GLFW/glfw3.h>

// Standard libraries
#include <iostream>
#include <string>

// --- Shader Source Code ---
// Shaders are small programs that run on the GPU.

// Vertex Shader:
// Processes each vertex of our shape. Its main job is to transform
// the 3D coordinates of the vertices into 2D screen coordinates.
const char* vertexShaderSource = R"(
    #version 330 core
    layout (location = 0) in vec3 aPos;    // The position variable has attribute position 0
    layout (location = 1) in vec3 aColor;  // The color variable has attribute position 1

    out vec3 ourColor; // Output a color to the fragment shader

    void main()
    {
        gl_Position = vec4(aPos, 1.0); // Set the final position of the vertex
        ourColor = aColor;             // Pass the color to the fragment shader
    }
)";

// Fragment Shader:
// Processes each pixel (or "fragment") on the screen that is covered by our shape.
// Its main job is to determine the final color of each pixel.
const char* fragmentShaderSource = R"(
    #version 330 core
    out vec4 FragColor; // The output color of the fragment

    in vec3 ourColor;   // The input color from the vertex shader (interpolated)

    void main()
    {
        FragColor = vec4(ourColor, 1.0f); // Set the final color of the pixel
    }
)";

// --- Main Application ---

int main() {
    // 1. Initialize GLFW
    // -------------------
    if (!glfwInit()) {
        std::cerr << "Failed to initialize GLFW" << std::endl;
        return -1;
    }

    // Set GLFW window hints for OpenGL version (3.3) and profile (Core)
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
#ifdef __APPLE__
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE); // Required on MacOS
#endif

    // 2. Create a Window
    // --------------------
    GLFWwindow* window = glfwCreateWindow(800, 600, "Hello OpenGL Triangle", NULL, NULL);
    if (window == NULL) {
        std::cerr << "Failed to create GLFW window" << std::endl;
        glfwTerminate();
        return -1;
    }
    glfwMakeContextCurrent(window);

    // 3. Initialize GLAD
    // ------------------
    // GLAD needs to be initialized after an OpenGL context has been created.
    // We pass GLAD the function to load address of OpenGL function pointers.
    if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) {
        std::cerr << "Failed to initialize GLAD" << std::endl;
        glfwDestroyWindow(window);
        glfwTerminate();
        return -1;
    }

    // Set the viewport
    int width, height;
    glfwGetFramebufferSize(window, &width, &height);
    glViewport(0, 0, width, height);

    // 4. Build and Compile Shader Program
    // ------------------------------------
    // Vertex Shader
    GLuint vertexShader = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertexShader, 1, &vertexShaderSource, NULL);
    glCompileShader(vertexShader);
    // Check for shader compile errors
    int success;
    char infoLog[512];
    glGetShaderiv(vertexShader, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(vertexShader, 512, NULL, infoLog);
        std::cerr << "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n" << infoLog << std::endl;
    }

    // Fragment Shader
    GLuint fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragmentShader, 1, &fragmentShaderSource, NULL);
    glCompileShader(fragmentShader);
    // Check for shader compile errors
    glGetShaderiv(fragmentShader, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(fragmentShader, 512, NULL, infoLog);
        std::cerr << "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n" << infoLog << std::endl;
    }

    // Link shaders into a Shader Program
    GLuint shaderProgram = glCreateProgram();
    glAttachShader(shaderProgram, vertexShader);
    glAttachShader(shaderProgram, fragmentShader);
    glLinkProgram(shaderProgram);
    // Check for linking errors
    glGetProgramiv(shaderProgram, GL_LINK_STATUS, &success);
    if (!success) {
        glGetProgramInfoLog(shaderProgram, 512, NULL, infoLog);
        std::cerr << "ERROR::SHADER::PROGRAM::LINKING_FAILED\n" << infoLog << std::endl;
    }
    // Shaders are linked, we can delete them now
    glDeleteShader(vertexShader);
    glDeleteShader(fragmentShader);

    // 5. Set up Vertex Data and Buffers
    // ---------------------------------
    // A triangle with positions and colors for each vertex.
    float vertices[] = {
        // positions         // colors
        -0.5f, -0.5f, 0.0f,  1.0f, 0.0f, 0.0f,   // bottom right (red)
         0.5f, -0.5f, 0.0f,  0.0f, 1.0f, 0.0f,   // bottom left (green)
         0.0f,  0.5f, 0.0f,  0.0f, 0.0f, 1.0f    // top (blue)
    };

    // VAO (Vertex Array Object): Stores vertex attribute configurations and bound VBOs.
    // VBO (Vertex Buffer Object): Stores the actual vertex data on the GPU.
    GLuint VBO, VAO;
    glGenVertexArrays(1, &VAO);
    glGenBuffers(1, &VBO);

    // Bind the VAO first, then bind and set VBO(s), and then configure vertex attributes.
    glBindVertexArray(VAO);

    glBindBuffer(GL_ARRAY_BUFFER, VBO);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

    // Position attribute
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 6 * sizeof(float), (void*)0);
    glEnableVertexAttribArray(0);
    // Color attribute
    glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 6 * sizeof(float), (void*)(3 * sizeof(float)));
    glEnableVertexAttribArray(1);

    // Unbind the VBO (optional, but good practice)
    glBindBuffer(GL_ARRAY_BUFFER, 0);
    // Unbind the VAO (optional, but good practice)
    glBindVertexArray(0);

    // 6. Render Loop
    // --------------
    while (!glfwWindowShouldClose(window)) {
        // Input
        if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS)
            glfwSetWindowShouldClose(window, true);

        // Rendering commands
        glClearColor(0.1f, 0.1f, 0.1f, 1.0f); // Set background color (dark grey)
        glClear(GL_COLOR_BUFFER_BIT);

        // Draw the triangle
        glUseProgram(shaderProgram);      // Use our shader program
        glBindVertexArray(VAO);           // Bind the VAO with our triangle data
        glDrawArrays(GL_TRIANGLES, 0, 3); // Draw the triangle

        // Swap buffers and poll for events
        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    // 7. De-allocate all resources
    // ----------------------------
    glDeleteVertexArrays(1, &VAO);
    glDeleteBuffers(1, &VBO);
    glDeleteProgram(shaderProgram);

    glfwDestroyWindow(window);
    glfwTerminate();
    return 0;
}
