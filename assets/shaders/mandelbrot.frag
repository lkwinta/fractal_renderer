#version 460
#extension GL_NV_gpu_shader_fp64 : enable
#extension GL_NV_gpu_shader5 : enable

in vec4 gl_FragCoord;

out vec4 frag_color;

uniform vec2 screen_size = vec2(800.0f, 600.0f);
uniform vec2 x_axis_range = vec2(-1.0f, 1.0f);
uniform vec2 y_axis_range = vec2(-1.0f, 1.0f);
uniform int max_iterations = 500;
uniform bool julia = true;
uniform vec2 julia_const = vec2(-0.8f, 0.156f);

int check_convergence()
{
    float32_t real = (gl_FragCoord.x / screen_size.x) * (x_axis_range.y - x_axis_range.x) + x_axis_range.x;
    float32_t imag = (gl_FragCoord.y / screen_size.y) * (y_axis_range.y - y_axis_range.x) + y_axis_range.x;

    int iterations = 0;

    float32_t const_real = real;
    float32_t const_imag = imag;

    if(julia) {
        const_real = julia_const.x;
        const_imag = julia_const.y;
    }

    while (iterations < max_iterations)
    {
        float32_t tmp_real = real;
        real = (real * real - imag * imag) + const_real;
        imag = (2.0 * tmp_real * imag) + const_imag;

        float32_t dist = real * real + imag * imag;

        if (dist > 4.0)
            break;

        iterations++;
    }
    return iterations;
}

vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

vec4 return_color()
{
    int iter = check_convergence();
    if (iter == max_iterations)
    {
        gl_FragDepth = 0.0f;
        return vec4(0.0f, 0.0f, 0.0f, 1.0f);
    }

    float hue = float(iter) / float(max_iterations);
    float saturation = 1.0f;
    float value = 1.0f;

    vec3 color_rgb = hsv2rgb(vec3(hue, saturation, value));

    return vec4(color_rgb, 1.0f);
}

void main()
{
    frag_color = return_color();
}