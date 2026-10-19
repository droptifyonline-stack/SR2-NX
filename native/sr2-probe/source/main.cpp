// Original procedural SR2-NX hardware probe. No game assets or recovered Unity code.
// EGL setup follows switchbrew/switch-examples (public domain), commit
// 669786898205b7beb25ff1731e72982e6d0397d3.
#include <EGL/egl.h>
#include <EGL/eglext.h>
#include <glad/glad.h>
#include <switch.h>
#define GLM_FORCE_PURE
#include <algorithm>
#include <cmath>
#include <cstdarg>
#include <cstdio>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <sys/stat.h>
#include <vector>

static FILE *logfile;
static char lastError[512];
static void log(const char *format, ...) {
    va_list args;
    va_start(args, format);
    vsnprintf(lastError, sizeof(lastError), format, args);
    va_end(args);
    if (logfile) {
        fprintf(logfile, "%s\n", lastError);
        fflush(logfile);
    }
}
static EGLDisplay display = EGL_NO_DISPLAY;
static EGLSurface surface = EGL_NO_SURFACE;
static EGLContext context = EGL_NO_CONTEXT;
static void closeGraphics() {
    if (display != EGL_NO_DISPLAY) {
        eglMakeCurrent(display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
        if (context != EGL_NO_CONTEXT)
            eglDestroyContext(display, context);
        if (surface != EGL_NO_SURFACE)
            eglDestroySurface(display, surface);
        eglTerminate(display);
    }
}
static bool openGraphics() {
    display = eglGetDisplay(EGL_DEFAULT_DISPLAY);
    if (display == EGL_NO_DISPLAY || !eglInitialize(display, nullptr, nullptr)) {
        log("EGL initialize failed: 0x%x", eglGetError());
        return false;
    }
    if (!eglBindAPI(EGL_OPENGL_API)) {
        log("EGL OpenGL binding failed: 0x%x", eglGetError());
        return false;
    }
    const EGLint attributes[] = {EGL_RENDERABLE_TYPE,
                                 EGL_OPENGL_BIT,
                                 EGL_RED_SIZE,
                                 8,
                                 EGL_GREEN_SIZE,
                                 8,
                                 EGL_BLUE_SIZE,
                                 8,
                                 EGL_ALPHA_SIZE,
                                 8,
                                 EGL_DEPTH_SIZE,
                                 24,
                                 EGL_NONE};
    EGLConfig config;
    EGLint count = 0;
    if (!eglChooseConfig(display, attributes, &config, 1, &count) || count == 0) {
        log("No EGL framebuffer config");
        return false;
    }
    nwindowSetDimensions(nwindowGetDefault(), 1280, 720);
    surface = eglCreateWindowSurface(display, config, nwindowGetDefault(), nullptr);
    if (surface == EGL_NO_SURFACE) {
        log("EGL surface failed: 0x%x", eglGetError());
        return false;
    }
    const EGLint options[] = {EGL_CONTEXT_OPENGL_PROFILE_MASK_KHR,
                              EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT_KHR,
                              EGL_CONTEXT_MAJOR_VERSION_KHR,
                              4,
                              EGL_CONTEXT_MINOR_VERSION_KHR,
                              3,
                              EGL_NONE};
    context = eglCreateContext(display, config, EGL_NO_CONTEXT, options);
    if (context == EGL_NO_CONTEXT || !eglMakeCurrent(display, surface, surface, context)) {
        log("EGL context failed: 0x%x", eglGetError());
        return false;
    }
    if (!gladLoadGL()) {
        log("OpenGL loader failed");
        return false;
    }
    if (!eglSwapInterval(display, 1))
        log("Vsync request failed: 0x%x", eglGetError());
    log("GL vendor=%s renderer=%s version=%s", glGetString(GL_VENDOR), glGetString(GL_RENDERER),
        glGetString(GL_VERSION));
    return true;
}
static GLuint shader(GLenum type, const char *source) {
    GLuint id = glCreateShader(type);
    glShaderSource(id, 1, &source, nullptr);
    glCompileShader(id);
    GLint ok = 0;
    glGetShaderiv(id, GL_COMPILE_STATUS, &ok);
    if (!ok) {
        char text[1024];
        glGetShaderInfoLog(id, sizeof(text), nullptr, text);
        log("Shader failed: %s", text);
        glDeleteShader(id);
        return 0;
    }
    return id;
}
static GLuint program() {
    const char *vertex = R"(#version 330 core
layout(location=0) in vec3 position;
layout(location=1) in vec3 normal;
uniform mat4 model,viewProjection;
out vec3 worldNormal;
void main(){gl_Position=viewProjection*model*vec4(position,1);worldNormal=mat3(model)*normal;}
)";
    const char *fragment = R"(#version 330 core
in vec3 worldNormal;
uniform vec3 color;
out vec4 outputColor;
void main(){float light=0.25+0.75*max(dot(normalize(worldNormal),normalize(vec3(0.3,1,0.5))),0);outputColor=vec4(color*light,1);}
)";
    GLuint v = shader(GL_VERTEX_SHADER, vertex), f = shader(GL_FRAGMENT_SHADER, fragment);
    if (!v || !f) {
        if (v)
            glDeleteShader(v);
        if (f)
            glDeleteShader(f);
        return 0;
    }
    GLuint result = glCreateProgram();
    glAttachShader(result, v);
    glAttachShader(result, f);
    glLinkProgram(result);
    glDeleteShader(v);
    glDeleteShader(f);
    GLint ok;
    glGetProgramiv(result, GL_LINK_STATUS, &ok);
    if (!ok) {
        char text[1024];
        glGetProgramInfoLog(result, sizeof(text), nullptr, text);
        log("Program failed: %s", text);
        glDeleteProgram(result);
        return 0;
    }
    return result;
}
struct Vertex {
    glm::vec3 position, normal;
};
struct Mesh {
    GLuint vao = 0, vbo = 0;
    GLsizei count = 0;
};
static Mesh mesh(const std::vector<Vertex> &data) {
    Mesh m;
    m.count = static_cast<GLsizei>(data.size());
    glGenVertexArrays(1, &m.vao);
    glGenBuffers(1, &m.vbo);
    glBindVertexArray(m.vao);
    glBindBuffer(GL_ARRAY_BUFFER, m.vbo);
    glBufferData(GL_ARRAY_BUFFER, data.size() * sizeof(Vertex), data.data(), GL_STATIC_DRAW);
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, sizeof(Vertex),
                          reinterpret_cast<void *>(offsetof(Vertex, position)));
    glEnableVertexAttribArray(0);
    glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, sizeof(Vertex),
                          reinterpret_cast<void *>(offsetof(Vertex, normal)));
    glEnableVertexAttribArray(1);
    return m;
}
static Mesh cube() {
    std::vector<Vertex> data;
    const glm::vec3 normals[] = {{1, 0, 0},  {-1, 0, 0}, {0, 1, 0},
                                 {0, -1, 0}, {0, 0, 1},  {0, 0, -1}};
    const glm::vec2 corners[] = {{-1, -1}, {1, -1}, {1, 1}, {-1, -1}, {1, 1}, {-1, 1}};
    for (auto n : normals) {
        auto helper = std::abs(n.y) > .9f ? glm::vec3(0, 0, 1) : glm::vec3(0, 1, 0);
        auto right = glm::normalize(glm::cross(helper, n));
        auto up = glm::cross(n, right);
        for (auto c : corners)
            data.push_back({.5f * (n + right * c.x + up * c.y), n});
    }
    return mesh(data);
}
static Mesh sphere() {
    std::vector<Vertex> data;
    auto point = [](int row, int col) {
        float phi = 3.14159265f * row / 12, theta = 6.2831853f * col / 20;
        return glm::vec3(std::sin(phi) * std::cos(theta), std::cos(phi),
                         std::sin(phi) * std::sin(theta));
    };
    for (int r = 0; r < 12; r++)
        for (int c = 0; c < 20; c++) {
            glm::vec3 p[] = {point(r, c), point(r + 1, c),     point(r + 1, c + 1),
                             point(r, c), point(r + 1, c + 1), point(r, c + 1)};
            for (auto v : p)
                data.push_back({v, v});
        }
    return mesh(data);
}
static void destroy(Mesh m) {
    glDeleteBuffers(1, &m.vbo);
    glDeleteVertexArrays(1, &m.vao);
}
static float axis(s32 value) {
    float x = std::clamp(value / 32767.f, -1.f, 1.f);
    return std::abs(x) < .15f ? 0.f : x;
}
static bool run(PadState &pad) {
    GLuint p = program();
    if (!p)
        return false;
    Mesh box = cube(), ball = sphere();
    glUseProgram(p);
    glEnable(GL_DEPTH_TEST);
    GLint modelLocation = glGetUniformLocation(p, "model"),
          viewLocation = glGetUniformLocation(p, "viewProjection"),
          colorLocation = glGetUniformLocation(p, "color");
    auto draw = [&](Mesh m, glm::vec3 at, glm::vec3 scale, glm::vec3 color) {
        auto model = glm::scale(glm::translate(glm::mat4(1), at), scale);
        glUniformMatrix4fv(modelLocation, 1, GL_FALSE, glm::value_ptr(model));
        glUniform3fv(colorLocation, 1, glm::value_ptr(color));
        glBindVertexArray(m.vao);
        glDrawArrays(GL_TRIANGLES, 0, m.count);
    };
    glm::vec3 player(0, 1.7f, 5), slime(0, .55f, 0);
    float yaw = 0, pitch = 0, vertical = 0, elapsed = 0;
    bool active = true, ok = true;
    unsigned inventory = 0;
    u64 previous = armGetSystemTick();
    double report = 0;
    unsigned frames = 0;
    unsigned long long total = 0;
    log("Ready. Left stick move, right stick look, A jump, ZL suction, ZR release, X reset, + "
        "exit. 720p requested; frame rate unverified.");
    while (appletMainLoop()) {
        padUpdate(&pad);
        u64 down = padGetButtonsDown(&pad), held = padGetButtons(&pad);
        if (down & HidNpadButton_Plus)
            break;
        u64 now = armGetSystemTick();
        double actual = double(now - previous) / armGetSystemTickFreq();
        previous = now;
        float dt = std::clamp(float(actual), 0.f, .05f);
        elapsed += dt;
        auto left = padGetStickPos(&pad, 0), rightStick = padGetStickPos(&pad, 1);
        yaw += axis(rightStick.x) * dt * 2.2f;
        pitch = std::clamp(pitch + axis(rightStick.y) * dt * 2.2f, -1.45f, 1.45f);
        glm::vec3 forward(std::sin(yaw) * std::cos(pitch), std::sin(pitch),
                          -std::cos(yaw) * std::cos(pitch)),
            flat(std::sin(yaw), 0, -std::cos(yaw)), right(std::cos(yaw), 0, std::sin(yaw));
        glm::vec3 move = right * axis(left.x) + flat * axis(left.y);
        if (glm::length(move) > 1)
            move = glm::normalize(move);
        auto candidate = player + move * 5.f * dt;
        if (!(candidate.x > 2.2f && candidate.x < 5.8f && candidate.z > -5.8f &&
              candidate.z < -2.2f && player.y < 3.7f)) {
            player.x = std::clamp(candidate.x, -14.f, 14.f);
            player.z = std::clamp(candidate.z, -14.f, 14.f);
        }
        if ((down & HidNpadButton_A) && player.y <= 1.701f)
            vertical = 6;
        vertical -= 18 * dt;
        player.y += vertical * dt;
        if (player.y < 1.7f) {
            player.y = 1.7f;
            vertical = 0;
        }
        if (down & HidNpadButton_X) {
            slime = {0, .55f, 0};
            active = true;
            inventory = 0;
            log("Reset");
        }
        if (active) {
            slime.y = .55f + std::max(0.f, std::sin(elapsed * 2.5f)) * .9f;
            auto offset = slime - player;
            float distance = glm::length(offset);
            if ((held & HidNpadButton_ZL) && distance > .001f && distance < 9 &&
                glm::dot(offset / distance, forward) > .8f) {
                slime -= offset / distance * 7.f * dt;
                if (distance < 1.4f && inventory < 20) {
                    active = false;
                    inventory++;
                    log("Captured slime. inventory=%u", inventory);
                }
            }
        }
        if ((down & HidNpadButton_ZR) && !active && inventory) {
            slime = player + forward * 3.f;
            slime.y = .55f;
            active = true;
            inventory--;
            log("Released slime. inventory=%u", inventory);
        }
        EGLint width = 1280, height = 720;
        eglQuerySurface(display, surface, EGL_WIDTH, &width);
        eglQuerySurface(display, surface, EGL_HEIGHT, &height);
        glViewport(0, 0, width, height);
        glClearColor(.1f, .16f, .23f, 1);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        auto vp =
            glm::perspective(glm::radians(65.f), float(width) / std::max(1, height), .05f, 100.f) *
            glm::lookAt(player, player + forward, glm::vec3(0, 1, 0));
        glUniformMatrix4fv(viewLocation, 1, GL_FALSE, glm::value_ptr(vp));
        draw(box, {0, -.05f, 0}, {30, .1f, 30}, {.18f, .45f, .23f});
        draw(box, {4, 1, -4}, {3, 2, 3}, {.55f, .48f, .32f});
        if (active)
            draw(ball, slime, {.55f, .55f, .55f}, {1, .25f, .5f});
        // A row of up to twenty small markers makes the inventory visible without a font asset.
        glClear(GL_DEPTH_BUFFER_BIT);
        for (unsigned i = 0; i < inventory; i++)
            draw(ball, player + forward * 1.f + right * (-.35f + .035f * i) + glm::vec3(0, -.3f, 0),
                 {.012f, .012f, .012f}, {1, .25f, .5f});
        GLenum error = glGetError();
        if (error != GL_NO_ERROR) {
            log("OpenGL error 0x%x", error);
            ok = false;
            break;
        }
        if (!eglSwapBuffers(display, surface)) {
            log("Swap failed: 0x%x", eglGetError());
            ok = false;
            break;
        }
        frames++;
        total++;
        report += actual;
        if (report >= 5) {
            u64 memory = 0;
            Result memoryResult =
                svcGetInfo(&memory, InfoType_UsedMemorySize, CUR_PROCESS_HANDLE, 0);
            log("frames=%llu fps=%.2f mean_frame_ms=%.2f size=%dx%d inventory=%u "
                "memory_result=0x%x used_bytes=%llu",
                total, frames / report, 1000 * report / frames, width, height, inventory,
                memoryResult, static_cast<unsigned long long>(memory));
            frames = 0;
            report = 0;
        }
    }
    destroy(ball);
    destroy(box);
    glDeleteProgram(p);
    if (ok)
        log("Clean exit. Presented frames=%llu (visual correctness requires hardware inspection)",
            total);
    return ok;
}
int main() {
    padConfigureInput(1, HidNpadStyleSet_NpadStandard);
    PadState pad;
    padInitializeDefault(&pad);
    mkdir("sdmc:/switch/sr2-native-probe", 0777);
    rename("sdmc:/switch/sr2-native-probe/log.txt", "sdmc:/switch/sr2-native-probe/log.prev.txt");
    logfile = fopen("sdmc:/switch/sr2-native-probe/log.txt", "w");
    bool success = false;
    if (!logfile)
        snprintf(lastError, sizeof(lastError),
                 "Cannot write sdmc:/switch/sr2-native-probe/log.txt");
    else {
        log("SR2 native C++/libnx OpenGL probe 0.1; placeholder content only");
        if (openGraphics())
            success = run(pad);
    }
    closeGraphics();
    if (!success) {
        consoleInit(nullptr);
        printf("SR2 native probe failed:\n%s\n\nPress + to exit.\n", lastError);
        while (appletMainLoop()) {
            padUpdate(&pad);
            if (padGetButtonsDown(&pad) & HidNpadButton_Plus)
                break;
            consoleUpdate(nullptr);
        }
        consoleExit(nullptr);
    }
    if (logfile) {
        fclose(logfile);
    }
    return success ? 0 : 1;
}
