#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTex;

out vec4 ourColor;
out vec3 FragPos;
out vec3 Normal;
out vec2 TexCords;

uniform vec4 uColor;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    ourColor = uColor;
    TexCords = aTex;
    Normal = aNormal;
    FragPos = vec3(model * vec4(aPos, 1.0));
	gl_Position = projection * view * model * vec4(aPos, 1.0);
}
