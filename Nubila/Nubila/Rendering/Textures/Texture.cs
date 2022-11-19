using OpenGL.Core;
using GLFW;

namespace Nubila
{
    abstract class Texture
    {
        protected uint m_texture;
        protected glTextureTarget m_type;
        protected glInternalFormat m_format;

        public Texture(glTextureTarget type, glInternalFormat format)
        {
            m_type = type;
            m_format = format;
        }

        /// <summary>
        /// Bind the image to a specific sampler in glsl
        /// </summary>
        /// <param name="binding"></param>
        public void BindImage(uint binding)
        {
            gl.BindImageTexture(binding, m_texture, 0, false, 0, glAccess.READ_WRITE, m_format);
        }
        
        /// <summary>
        /// Bind the texture to a specific texture in glsl
        /// </summary>
        /// <param name="unit"></param>
        public void BindTexture(glTextureUnit unit)
        {
            gl.ActiveTexture(unit);
            gl.BindTexture(m_type, m_texture);
        }

        /// <summary>
        /// Set the filter mode to linear
        /// </summary>
        public void SetFilterLinear()
        {
            gl.TexParameteri(m_type, glTextureParameter.TEXTURE_MIN_FILTER, glFilter.LINEAR);
            gl.TexParameteri(m_type, glTextureParameter.TEXTURE_MAG_FILTER, glFilter.LINEAR);
        }
        
        /// <summary>
        /// Set the filter mode to nearest
        /// </summary>
        public void SetFilterNearest()
        {
            gl.TexParameteri(m_type, glTextureParameter.TEXTURE_MIN_FILTER, glFilter.NEAREST);
            gl.TexParameteri(m_type, glTextureParameter.TEXTURE_MAG_FILTER, glFilter.NEAREST);
        }
    }
}
