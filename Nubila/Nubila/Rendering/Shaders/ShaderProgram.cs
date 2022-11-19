using OpenGL.Core;
using System.Numerics;

namespace Nubila
{
    class ShaderProgram
    {
        public uint m_program;
        List<Shader> m_shaders;

        struct Parameter
        {
            public string name;
            public string type;
            public object value;

            public Parameter(string n,  string t, object v)
            {
                name = n;
                type = t;
                value = v;
            }
        }

        Dictionary<string, Parameter> parameters;

        public ShaderProgram(string file) : this(new string[] { file })
        {
            // generate the program
            m_program = gl.CreateProgram();
            m_shaders = new List<Shader>();

            // create the shader
            Shader s = new Shader(file);

            gl.AttachShader(m_program, s.LoadShader());
            m_shaders.Add(s);

            // link program & setup parameters
            gl.LinkProgram(m_program);

            parameters = new Dictionary<string, Parameter>();
            ApplyParameters();
        }

        public ShaderProgram(string[] files)
        {
            // generate the program
            m_program = gl.CreateProgram();
            m_shaders = new List<Shader>();

            // create the shader
            foreach (string file in files)
            {
                Shader s = new Shader(file);

                gl.AttachShader(m_program, s.LoadShader());
                m_shaders.Add(s);
            }

            // link program & setup parameters
            gl.LinkProgram(m_program);

            parameters = new Dictionary<string, Parameter>();
            ApplyParameters();
        }

        public void Use()
        {
            gl.UseProgram(m_program);
        }

        public void Reload()
        {
            gl.DeleteProgram(m_program);
            m_program = gl.CreateProgram();

            foreach (Shader s in m_shaders)
            {
                gl.AttachShader(m_program, s.LoadShader());
                m_shaders.Append(s);
            }

            gl.LinkProgram(m_program);
            ApplyParameters();
        }

        public void SetInt(string name, int x)
        {
            parameters[name] = new Parameter(name, "int", x);
            ApplyParameters();
        }

        public void SetFloat(string name, float x)
        {
            parameters[name] = new Parameter(name, "float", x);
            ApplyParameters();
        }

        public void SetVec2(string name, Vector2 v)
        {
            parameters[name] = new Parameter(name, "vec2", v);
            ApplyParameters();
        }

        public void SetVec3(string name, Vector3 v)
        {
            parameters[name] = new Parameter(name, "vec3", v);
            ApplyParameters();
        }

        public void SetMatrix4x4(string name, Matrix4x4 matrix)
        {
            parameters[name] = new Parameter(name, "matrix4x4", matrix);
            ApplyParameters();
        }

        protected void ApplyParameters()
        {
            Use();

            foreach (Parameter p in parameters.Values)
            {
                switch(p.type)
                {
                    case "int":
                        gl.Uniform1i(gl.GetUniformLocation(m_program, p.name), (int)p.value);
                        break;

                    case "float":
                        gl.Uniform1f(gl.GetUniformLocation(m_program, p.name), (float)p.value);
                        break;

                    case "vec2":
                        gl.Uniform2f(gl.GetUniformLocation(m_program, p.name), ((Vector2)p.value).X, ((Vector2)p.value).Y);
                        break;

                    case "vec3":
                        gl.Uniform3f(gl.GetUniformLocation(m_program, p.name), ((Vector3)p.value).X, ((Vector3)p.value).Y, ((Vector3)p.value).Z);
                        break;

                    case "matrix4x4":
                        gl.UniformMatrix4fv(gl.GetUniformLocation(m_program, p.name), 1, false, ((Matrix4x4)p.value).Flatten());
                        break;

                    default:
                        break;
                }
            }
        }
    }
}
