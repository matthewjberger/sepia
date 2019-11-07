# Notes

- Use Specs parallel ecs for the main ecs
    - Make renderable component
    - gltf assets can store graphs internally
- Add error handling (preferably with snafu)
- Check out async/await and see if it would be useful
- Use rayon where applicable
- Add physics system with nphysics
- Load names for scenes and animations as Option<string>
- Add animator component
- replace material_index property with Option<usize> (default material if None)
- framebuffer wrapper
- deferred shading
- 3D picking (megabyte softworks maybe?)
- [https://github.com/ron-rs/ron](https://github.com/ron-rs/ron)
- [https://github.com/gluon-lang/gluon](https://github.com/gluon-lang/gluon)

Next:

- Make Material and Shader cache
- Have render system use the material and shader cache
- Have components specify what shader and material they want
- Migrate code in ecs example to use app/statemachine wrapper
- Eventually do something about examples that are not quite working
