using OpenGL.Core;

namespace Nubila
{
    internal class RenderTexture2D
    {
        private Texture2D m_texture;
        private uint vao;
        private int indexCount;

        // renderTexture = new RenderTexture2D(new Texture2D("res/Textures/test-small.jpg"), -1, -1, 2, 2);
        public RenderTexture2D(Texture2D texture, float x, float y, float w, float h)
        {
            m_texture = texture;

            float[] positions =
            {
                x    , y,
                x + w, y,
                x    , y + h,
                x + w, y + h
            };

            float[] texCoords =
            {
                0, 0,
                1, 0,
                0, 1,
                1, 1
            };

            int[] indices =
            {
                0, 1, 2,
                3, 2, 1
            };

            // setup the vbo
            vao = gl.GenVertexArray();
            gl.BindVertexArray(vao);

            // apply the indices
            uint ebo = gl.GenBuffer();
            gl.BindBuffer(glBufferTarget.ELEMENT_ARRAY_BUFFER, ebo);
            gl.BufferData(glBufferTarget.ELEMENT_ARRAY_BUFFER, indices.Length * sizeof(int), indices, glBufferUsage.STATIC_DRAW);

            indexCount = indices.Length;

            // insert vertex attributes
            storeInAttributeList(0, 2, positions);
            storeInAttributeList(1, 2, texCoords);
        }

        public Texture2D GetTexture()
        {
            return m_texture;
        }

        public void Render()
        {
            m_texture.BindTexture(glTextureUnit.TEXTURE0);

            gl.BindVertexArray(vao);
            gl.DrawElements(glDrawMode.TRIANGLES, indexCount, glDrawElementsType.UNSIGNED_INT, 0);
        }

        private void storeInAttributeList(uint attribute, int coordinateSize, float[] data)
        {
            uint vbo = gl.GenBuffer();
            gl.BindBuffer(glBufferTarget.ARRAY_BUFFER, vbo);

            gl.BufferData(glBufferTarget.ARRAY_BUFFER, data.Length * sizeof(float), data, glBufferUsage.STATIC_DRAW);
            gl.VertexAttribPointer(attribute, coordinateSize, glVertexAttribType.FLOAT, false, 0, 0);
            gl.EnableVertexAttribArray(attribute);
        }
    }
}
