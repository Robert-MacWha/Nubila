using Delphi;
using GLFW;
using OpenGL.Core;

class Game : Engine
{
    public Game() { }

    uint vao;
    uint vbo;
    uint shader;
    ShaderProgram shaderProgram;

    protected override void Initialize()
    {
    }

    protected override void LoadContent()
    {
        shaderProgram = new ShaderProgram(new string[] { "res/Shaders/test.vert", "res/Shaders/test.frag" });

        // create vertex buffer
        vao = gl.GenVertexArray();
        vbo = gl.GenBuffer();

        gl.BindVertexArray(vao);
        gl.BindBuffer(glBufferTarget.ARRAY_BUFFER, vbo);

        float[] vertices =
        {
            0.5f, -0.5f,
            -0.5f, -0.5f,
            0f, 0.5f
        };

        gl.BufferData(glBufferTarget.ARRAY_BUFFER, vertices.Length * sizeof(float), vertices, glBufferUsage.STATIC_DRAW);

        gl.VertexAttribPointer(0, 2, glVertexAttribType.FLOAT, false, 2 * sizeof(float), 0);
        gl.EnableVertexAttribArray(0);

        gl.BindBuffer(glBufferTarget.ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    protected override void Render()
    {
        shaderProgram.Use();

        gl.BindVertexArray(vbo);
        gl.DrawArrays(glDrawMode.TRIANGLES, 0, 3);
        gl.BindVertexArray(0);
    }

    protected override void Update()
    {
        shaderProgram.Reload();
    }
}
