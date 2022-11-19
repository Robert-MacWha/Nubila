using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Nubila
{
    struct Voxel
    {
        public int x;
        public int y;
        public int z;
        public int materialID;

        public Voxel(int x, int y, int z, int materialID)
        {
            this.x = x;
            this.y = y;
            this.z = z;
            this.materialID = materialID;
        }
    }
}
