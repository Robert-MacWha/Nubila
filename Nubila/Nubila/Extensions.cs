using System.Numerics;

namespace Nubila
{
    public static class MatrixExtensions
    {
        public static Matrix4x4 Invert(this Matrix4x4 matrix)
        {
            Matrix4x4 inverted;
            Matrix4x4.Invert(matrix, out inverted);
            return inverted;
        }

        public static float[] Flatten(this Matrix4x4 matrix)
        {
            return new float[]
            {
                matrix.M11,
                matrix.M12,
                matrix.M13,
                matrix.M14,

                matrix.M21,
                matrix.M22,
                matrix.M23,
                matrix.M24,

                matrix.M31,
                matrix.M32,
                matrix.M33,
                matrix.M34,

                matrix.M41,
                matrix.M42,
                matrix.M43,
                matrix.M44,
            };
        }
    }

    public static class VectorExtensions
    {
        public static Vector3 Normalized(this Vector3 vector)
        {
            return Vector3.Normalize(vector);
        }
    }

    public static class FloatExtensions
    { 
        public static float ToRadians(this float degrees)
        {
            return (float)(degrees * Math.PI / 180.0f);
        }

        public static float ToDegrees(this float radians)
        {
            return (float)(radians * 180.0f / Math.PI);
        }
    }
}