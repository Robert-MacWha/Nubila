using System.Drawing;
using System.Numerics;
namespace Nubila
{
    struct OctreeStruct
    {
        public OctreeStruct(int empty, int value, Vector3 pos, int size, int parent)
        {
            m_empty = empty;
            m_value = value;
            m_x = (int)pos.X;
            m_y = (int)pos.Y;
            m_z = (int)pos.Z;
            m_size = size;

            m_parent = parent;
            _000 = -1;
            _100 = -1;
            _010 = -1;
            _110 = -1;
            _001 = -1;
            _101 = -1;
            _011 = -1;
            _111 = -1;
        }

        public int m_parent;
        public int m_empty;
        public int m_value;
        public int m_x;
        public int m_y;
        public int m_z;
        public int m_size;

        // links to the indice of all children
        public int _000;
        public int _100;
        public int _010;
        public int _110;
        public int _001;
        public int _101;
        public int _011;
        public int _111;
    }

    internal class Octree
    {
        private Vector3 m_position;
        private int m_size;

        private Octree[] m_subNodes;
        private int m_value;

        private int m_empty;

        public Octree(Vector3 position, int size)
        {
            m_position = position;
            m_empty = 1;
            m_value = -1;

            //? because of the structure of the data, octree size must be a power of 2
            // src: https://stackoverflow.com/questions/466204/rounding-up-to-next-power-of-2
            m_size = (int)Math.Pow(2, Math.Ceiling(Math.Log(size) / Math.Log(2.0)));
        }

        public void Insert(Vector3 position, int material)
        {
            if (!InRange(position))
            {
                // disregard voxel that aren't within this node
                return;
            }

            if (m_size == 1)
            {
                // reached depth limit - set this subnode to the voxel
                m_empty = 0;
                m_value = material;
            }
            else
            {
                // create new subnodes the node was previously empty
                if (m_empty == 1)
                    m_subNodes = new Octree[8];

                for (int i = 0; i < m_subNodes.Length; i++)
                {
                    // determin the position of each subnode
                    Vector3 newPos = m_position;
                    if ((i & 4) == 4)
                    {
                        newPos.Z = m_position.Z + m_size * 0.5f;
                    }

                    if ((i & 2) == 2)
                    {
                        newPos.Y = m_position.Y + m_size * 0.5f;
                    }

                    if ((i & 1) == 1)
                    {
                        newPos.X = m_position.X + m_size * 0.5f;
                    }

                    // initialize the subnode if the node was previously empty
                    if (m_empty == 1)
                        m_subNodes[i] = new Octree(newPos, m_size / 2);

                    //? insert will fail for all subnodes that don't contain the position
                    // Console.WriteLine(position + ", " + newPos + ", " + m_size / 2);
                    m_subNodes[i].Insert(position, material);
                }

                m_empty = 0;
            }
        }

        public void Print()
        {
            List<OctreeStruct> struc = Flatten();
            Console.WriteLine("x:" + struc[1].m_x + ", y:" + struc[1].m_y + ", z:" + struc[1].m_z);
            Console.WriteLine(struc[1].m_value);
            Console.WriteLine(struc[1].m_size);
        }

        public List<OctreeStruct> Flatten()
        {
            List<OctreeStruct> octreeData = new List<OctreeStruct>();
            Flatten(ref octreeData);
            return octreeData;
        }

        private int Flatten(ref List<OctreeStruct> octreeData, int index=0, int parentIndex=-1)
        {
            // create a new octreestruct to represent this
            OctreeStruct octree = new OctreeStruct(m_empty, m_value, m_position, m_size, parentIndex);
            int voxelIndex = index;
            index++;
            octreeData.Add(octree);

            if (m_empty == 0 && m_value == -1)
            {
                // if there are subdividions (making sure the node's not a child), return the recursive struct
                int i = 0;
                foreach (Octree v in m_subNodes)
                {
                    OctreeStruct s = octreeData[voxelIndex];
                    if (i == 0)
                        s._000 = index;
                    else if (i == 1)
                        s._100 = index;
                    else if (i == 2)
                        s._010 = index;
                    else if (i == 3)
                        s._110 = index;
                    else if (i == 4)
                        s._001 = index;
                    else if (i == 5)
                        s._101 = index;
                    else if (i == 6)
                        s._011 = index;
                    else if (i == 7)
                        s._111 = index;

                    octreeData[voxelIndex] = s;
                    index = v.Flatten(ref octreeData, index, voxelIndex);
                    i++;
                }
            }

            return index;
        }

        private bool InRange(Vector3 position)
        {
            return 
            (
                position.X >= m_position.X && position.X < m_position.X + m_size &&
                position.Y >= m_position.Y && position.Y < m_position.Y + m_size &&
                position.Z >= m_position.Z && position.Z < m_position.Z + m_size
            );
        }
    }
}
