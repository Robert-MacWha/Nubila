using Nubila;
using GLFW;
using OpenGL.Core;

class Game : Engine
{
    static float cameraSpeed = 4;
    static float cameraSensitivity = 0.5f;

    Camera camera;
    Texture2D skybox;
    RenderTexture2D renderTexture;
    ComputeShader computeShader;
    ShaderProgram renderShader;

    Model model;

    float yaw = 0;
    float pitch = 0;

    protected override void Awake()
    {
        windowWidth = 800;
        windowHeight = 600;
    }

    protected override void Start()
    {
        renderShader = new ShaderProgram(new string[] { "res/Shaders/test.vert", "res/Shaders/test.frag" });

        renderTexture = new RenderTexture2D(new Texture2D(windowWidth, windowHeight, glInternalFormat.RGBA32F), -1, -1, 2, 2);

        camera = new Camera(45, DisplayManager.aspectRatio);
        Vector3 direction = new Vector3(
            (float)(Math.Cos(yaw.ToRadians()) * Math.Cos(pitch.ToRadians())),
            (float)(Math.Sin(-pitch.ToRadians())),
            (float)(Math.Sin(yaw.ToRadians()) * Math.Cos(pitch.ToRadians()))
        );

        camera.SetPosition(new Vector3(-25, 100, 100));
        camera.SetDirection(direction);

        skybox = new Texture2D("res/Textures/skybox-1920.png");
        model = new Model("res/Models/monu2.ply");
        model.GenerateModelData();
        model.GenerateMaterialData();

        computeShader = new ComputeShader("res/Shaders/voxelRenderer.comp");
        computeShader.SetInt("_modelSampler", 1);
        computeShader.SetInt("_octreeCount", model.OctreeDataLength());
        computeShader.SetVec2("_textureResolution", new Vector2(windowWidth, windowHeight));
        computeShader.SetVec3("_modelSize", new Vector3(model.m_width, model.m_height, model.m_depth));
        model.Print();
    }

    protected override void Render()
    {
        // render which voxel coresponds to each pixel
        computeShader.Use();
        renderTexture.GetTexture().BindImage(0);
        model.BindModelBuffer(1);
        model.BindMaterialBuffer(2);

        computeShader.SetMatrix4x4("_viewInverse", camera.m_viewMatrix.Invert());
        computeShader.SetMatrix4x4("_projectionInverse", camera.m_projectionMatrix.Invert());
        computeShader.Dispatch((uint)Math.Ceiling(windowWidth / 32.0), (uint)Math.Ceiling(windowHeight / 16.0), 1);

        // render the output texture
        renderShader.Use();
        renderTexture.Render();
    }

    protected override void Update()
    {
        if (Time.Ticks % 60 == 0)
        {
            Console.WriteLine("DeltaTime: " + Time.DeltaTime + ", FrameRate: " + (1 / Time.DeltaTime));
        }

        // movement
        float effectiveCameraSpeed = cameraSpeed;
        if (Input.GetKey(Keys.LeftShift))
            effectiveCameraSpeed *= 4;

        if (Input.GetKey(Keys.W))
            camera.Translate(camera.Direction() * effectiveCameraSpeed * Time.DeltaTime);

        if (Input.GetKey(Keys.S))
            camera.Translate(-camera.Direction() * effectiveCameraSpeed * Time.DeltaTime);

        if (Input.GetKey(Keys.A))
            camera.Translate(-Vector3.Cross(camera.Direction(), Camera.Up).Normalized() * effectiveCameraSpeed * Time.DeltaTime);

        if (Input.GetKey(Keys.D))
            camera.Translate(Vector3.Cross(camera.Direction(), Camera.Up).Normalized() * effectiveCameraSpeed * Time.DeltaTime);

        // rotation
        Vector2 mouseDelta = Input.GetMouseDelta();
        if (mouseDelta != new Vector2(0, 0))
        {
            // src: https://learnopengl.com/Getting-started/Camera
            float dx = mouseDelta.X * cameraSensitivity;
            float dy = mouseDelta.Y * cameraSensitivity;

            yaw += dx;
            pitch += dy;

            // stop the camera from vertically looping
            if (pitch > 89.0f)
                pitch = 89.0f;
            if (pitch < -89.0f)
                pitch = -89.0f;

            Vector3 direction = new Vector3(
                (float)(Math.Cos(yaw.ToRadians()) * Math.Cos(pitch.ToRadians())),
                (float)(Math.Sin(-pitch.ToRadians())),
                (float)(Math.Sin(yaw.ToRadians()) * Math.Cos(pitch.ToRadians()))
            );

            camera.SetDirection(direction);
        }

        if (Input.GetKeyDown(Keys.R))
        {
            // Reload Shaders
            computeShader.Reload();
            renderShader.Reload();
        }

        if (Input.GetKeyDown(Keys.P))
        {
            Console.WriteLine(camera.Position());
        }
    }
}
