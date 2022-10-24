using OpenGL.Core;

namespace Delphi
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
        }

        public ShaderProgram(string[] files)
        {
            m_program = gl.CreateProgram();
            m_shaders = new List<Shader>();

            foreach (string file in files)
            {
                Shader s = new Shader(file);

                gl.AttachShader(m_program, s.LoadShader());
                m_shaders.Add(s);
            }

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

        private void ApplyParameters()
        {
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

                    default:
                        break;
                }
            }
        }
    }
}
