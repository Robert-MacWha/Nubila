using OpenGL.Core;

namespace Nubila
{
    // wrapper for ShaderProgram specifically focusing on compute shaders
    internal class ComputeShader : ShaderProgram
    {
        public ComputeShader(string file): base(file)
        {
        }

        public void Dispatch(uint x, uint y, uint z)
        {
            ApplyParameters();

            gl.DispatchCompute(x, y, z);
            gl.MemoryBarrier(glMemoryBarrierMask.ALL_BARRIER_BITS);
        }
    }
}
