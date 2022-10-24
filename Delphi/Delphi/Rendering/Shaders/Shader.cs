using OpenGL.Core;

namespace Delphi
{
    class Shader
    {
        private string m_file;
        private glShaderType m_type;

        private uint m_id;

        public static Dictionary<string, glShaderType> shaderLookup = new Dictionary<string, glShaderType>()
        {
            { "vert", glShaderType.VERTEX_SHADER },
            { "frag", glShaderType.FRAGMENT_SHADER },
            { "comp", glShaderType.VERTEX_SHADER }
        };


        public Shader(string file)
        {
            string type = file.Split(".")[1];
            if (shaderLookup.ContainsKey(type))
            {
                m_file = file;
                m_type = shaderLookup[type];
            }
            else
            {
                m_file = "";
            }
        }

        public uint LoadShader()
        {
            gl.DeleteShader(m_id);

            if (m_file == "")
                return 0;

            string shaderText = File.ReadAllText(m_file);

            m_id = gl.CreateShader(m_type);
            gl.ShaderSource(m_id, shaderText);
            gl.CompileShader(m_id);

            return m_id;
        }
    }
}
