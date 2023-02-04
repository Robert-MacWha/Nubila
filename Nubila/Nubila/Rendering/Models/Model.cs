using OpenGL.Core;
using System;
using System.Numerics;
using System.Runtime.InteropServices;

namespace Nubila
{
    /// <summary>
    /// Loads the model from a point ply file (exported from magica voxel)
    /// </summary>
    /// <param name="file">File source for model</param>
    internal class Model
    {
        private string m_file;

        private List<Material> m_materials;
        private List<Voxel> m_voxels;

        private Octree m_octree;

        public int m_width;
        public int m_height;
        public int m_depth;

        // https://www.khronos.org/opengl/wiki/Shader_Storage_Buffer_Object
        private uint m_modelSsboID;
        private uint m_materialSsboID;

        private OctreeStruct[] m_octreeData;

        /// <summary>
        /// Load a model from a .ply point file (exported via Magica Voxel)
        /// </summary>
        /// <param name="file"></param>
        public Model(string file)
        {
            m_modelSsboID = gl.GenBuffer();
            m_materialSsboID = gl.GenBuffer();
            m_file = file;

            // load the file from disk
            string[] lines = File.ReadAllLines(file);
            m_materials = new List<Material>();
            m_voxels = new List<Voxel>();

            // load the model into a list of voxels
            bool headerDone = false;
            foreach (string line in lines)
            {
                // skip header lines
                if (!headerDone)
                {
                    if (line == "end_header")
                        headerDone = true;
                    continue;
                }

                // load in the model
                string[] splitStr = line.Split(' ');

                int x = int.Parse(splitStr[0]);
                int z = int.Parse(splitStr[1]);
                int y = int.Parse(splitStr[2]);
                float r = int.Parse(splitStr[3]) / 255.0f;
                float g = int.Parse(splitStr[4]) / 255.0f;
                float b = int.Parse(splitStr[5]) / 255.0f;

                // see if a material with this palette already exists
                int matID = -1;
                for (int i = 0; i < m_materials.Count(); i++)
                {
                    Material m = m_materials[i];
                    if (m.r == r && m.g == g && m.b == b)
                    {
                        matID = i;
                        break;
                    }
                }

                // if no material was found, create one
                if (matID == -1)
                {
                    matID = m_materials.Count();
                    Material m = new Material(r, g, b);
                    m_materials.Add(m);
                }

                m_voxels.Add(new Voxel(x, y, z, matID));
            }

            RefreshModelOctree();
        }

        public void GenerateModelData()
        {
            // prepare the data
            List<OctreeStruct> octreeList = m_octree.Flatten();

            m_octreeData = octreeList.ToArray();
            int octreeDataSize = m_octreeData.Length * (4 * 15);

            // prepare the buffer
            // TODO: Find a way to avoid using unsafe here (byte[] instead of struct[]?)
            unsafe
            {
                // src: https://www.reddit.com/r/csharp/comments/2ttp34/how_to_create_a_intptr_for_a_struct_array/
                var handle = GCHandle.Alloc(m_octreeData, GCHandleType.Pinned);
                try
                {
                    gl.BindBuffer(glBufferTarget.SHADER_STORAGE_BUFFER, m_modelSsboID);
                    gl.BufferData(glBufferTarget.SHADER_STORAGE_BUFFER, octreeDataSize, handle.AddrOfPinnedObject(), glBufferUsage.DYNAMIC_DRAW);
                    gl.BindBufferBase(glBufferTarget.SHADER_STORAGE_BUFFER, 3, m_modelSsboID);
                    gl.BindBuffer(glBufferTarget.SHADER_STORAGE_BUFFER, 0); // unbind
                }
                finally
                {
                    handle.Free();
                }
            }
        }

        public void GenerateMaterialData()
        {
            // prepare the data
            Material[] materialData = m_materials.ToArray();
            int materialDataSize = materialData.Length * (4 * 3);

            // prepare the buffer
            // TODO: Find a way to avoid using unsafe here (byte[] instead of struct[]?)
            unsafe
            {
                // src: https://www.reddit.com/r/csharp/comments/2ttp34/how_to_create_a_intptr_for_a_struct_array/
                var handle = GCHandle.Alloc(materialData, GCHandleType.Pinned);
                try
                {
                    gl.BindBuffer(glBufferTarget.SHADER_STORAGE_BUFFER, m_materialSsboID);
                    gl.BufferData(glBufferTarget.SHADER_STORAGE_BUFFER, materialDataSize, handle.AddrOfPinnedObject(), glBufferUsage.DYNAMIC_DRAW);
                    gl.BindBufferBase(glBufferTarget.SHADER_STORAGE_BUFFER, 3, m_materialSsboID);
                    gl.BindBuffer(glBufferTarget.SHADER_STORAGE_BUFFER, 0); // unbind
                }
                finally
                {
                    handle.Free();
                }
            }
        }

        public void BindModelBuffer(uint binding)
        {
            // gl.BindBuffer(glBufferTarget.SHADER_STORAGE_BUFFER, m_modelSsboID);
            gl.BindBufferBase(glBufferTarget.SHADER_STORAGE_BUFFER, binding, m_modelSsboID);
        }

        public void BindMaterialBuffer(uint binding)
        {
            // gl.BindBuffer(glBufferTarget.SHADER_STORAGE_BUFFER, m_materialSsboID);
            gl.BindBufferBase(glBufferTarget.SHADER_STORAGE_BUFFER, binding, m_materialSsboID);
        }

        public void Print()
        {
            m_octree.Print();
        }

        public int OctreeDataLength()
        {
            return m_octreeData.Length;
        }

        private void RefreshModelOctree()
        {
            // calculate the model bounds
            int minX = int.MaxValue;
            int minY = int.MaxValue;
            int minZ = int.MaxValue;
            int maxX = int.MinValue;
            int maxY = int.MinValue;
            int maxZ = int.MinValue;

            foreach (Voxel v in m_voxels)
            {
                minX = Math.Min(minX, v.x);
                minY = Math.Min(minY, v.y);
                minZ = Math.Min(minZ, v.z);
                maxX = Math.Max(maxX, v.x);
                maxY = Math.Max(maxY, v.y);
                maxZ = Math.Max(maxZ, v.z);
            }

            m_width  = (maxX - minX) + 1;
            m_height = (maxY - minY) + 1;
            m_depth  = (maxZ - minZ) + 1;

            int size = Math.Max(m_width, Math.Max(m_height, m_depth));

            // normalize voxel positions and insert into the new octree
            m_octree = new Octree(new Vector3(0, 0, 0), size);
            foreach (Voxel v in m_voxels)
            {
                m_octree.Insert(new Vector3(v.x - minX, v.y - minY, v.z - minZ), v.materialID);
            }
        }
    }
}
