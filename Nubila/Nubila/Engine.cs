using GLFW;

namespace Nubila
{
    abstract class Engine
    {
        // engine limits
        public static int MATERIAL_SIZE = 128;

        // local vars
        public int windowWidth = 800;
        public int windowHeight = 600;
        public string windowTitle = "Voxel Engine";

        // game engine loop
        public void Run()
        {
            Awake();
            DisplayManager.CreateWindow(windowWidth, windowHeight, windowTitle, 1);
            Input.Initialize();
            Start();

            while (!Glfw.WindowShouldClose(DisplayManager.window))
            {
                Time.DeltaTime = (float)Glfw.Time - Time.ElapsedTime;
                Time.ElapsedTime = (float)Glfw.Time;
                Time.Ticks ++;

                Update();
                Input.UpdateInputs();
                Glfw.PollEvents();

                DisplayManager.PreRender();
                Render();
                DisplayManager.PostRender();
            }

            DisplayManager.DestroyWindow();
        }

        protected abstract void Awake();
        protected abstract void Start();

        protected abstract void Update();
        protected abstract void Render();
    }
}
