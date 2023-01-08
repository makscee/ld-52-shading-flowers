uniform vec4 u_color_1;
uniform vec4 u_color_2;
uniform float u_hue_shift;
uniform float u_time;
uniform float u_spawn = 0;

vec3 hueShift(vec3 color, float hueAdjust) // hue in radians
{
    const vec3 kRGBToYPrime = vec3(0.299, 0.587, 0.114);
    const vec3 kRGBToI = vec3(0.596, -0.275, -0.321);
    const vec3 kRGBToQ = vec3(0.212, -0.523, 0.311);

    const vec3 kYIQToR = vec3(1.0, 0.956, 0.621);
    const vec3 kYIQToG = vec3(1.0, -0.272, -0.647);
    const vec3 kYIQToB = vec3(1.0, -1.107, 1.704);

    float YPrime = dot(color, kRGBToYPrime);
    float I = dot(color, kRGBToI);
    float Q = dot(color, kRGBToQ);
    float hue = atan(Q, I);
    float chroma = sqrt(I * I + Q * Q);

    hue += hueAdjust;

    Q = chroma * sin(hue);
    I = chroma * cos(hue);

    vec3 yIQ = vec3(YPrime, I, Q);

    return vec3(dot(yIQ, kYIQToR), dot(yIQ, kYIQToG), dot(yIQ, kYIQToB));
}

vec3 hue(float value) {
    return hueShift(vec3(1, 0, 0), value * PI * 2.);
}

float vecAngle(vec2 v) {
    if(v == vec2(0.))
        return 0.;
    float r = acos(dot(normalize(v), vec2(0., 1.)));
    return (r + float(v.x > 0.) * (PI - r) * 2.);
}

vec2 rotateCW(vec2 p, float a) {
    mat2 m = mat2(cos(a), -sin(a), sin(a), cos(a));
    return p * m;
}

vec2 N22(vec2 p) {
    vec3 a = fract(p.xyx * vec3(123.34, 234.34, 345.65));
    a += dot(a, a + 34.45);
    return fract(vec2(a.x * a.y, a.y * a.z));
}

float rand(int i) {
    return N22(vec2(i * .001)).x;
}

vec2 randVec(int i) {
    return N22(vec2(i * .001));
}

vec2 randCircle(int i) {
    float r2p = rand(i) * PI * 2.;
    return vec2(cos(r2p), sin(r2p));
}