# Sepia

## Scene Graph

* A scenegraph will be made using petgraph to store scene object (entity) heirarchy
* Eventually can be constructed from serialized file
  * [RON Format](https://github.com/ron-rs/ron) 

## Structs

* BaseObjectData (needs better name)
  * tag
  * transform
  * active_self (is active or not)
  * active_in_tree (has a parent that isn't active)

## Traits

* GameObject trait
  * [Reference](https://docs.unity3d.com/ScriptReference/GameObject.html)
  * get basic object data read only
  * get basic object data mutable
  * Add Component
  * Remove Component
  * SetActive
* All game objects must implement the GameObject trait, making base data accessible

## Game Objects

* Have a behaviour that can be [scripted](https://github.com/gluon-lang/gluon)
  * Initialize - called on the frame when a script is enabled before any of the update methods are called the first time
  * Update - called every frame if enabled
  * [Reference](https://docs.unity3d.com/ScriptReference/MonoBehaviour.html)
* Executed by systems in ECS

## Components

* Transform
* Input
  * Need various kinds of input components
  * Update by input systems
  * Maybe keyboard input component and mouse input component should be separated
  * [Command pattern](http://gameprogrammingpatterns.com/command.html)
* Camera
  * Can clear to a solid color or a skybox
  * Requires a skybox component on the same game object if clearing to a skybox
* Skybox
* Mesh Renderer
  * Stores mesh primitives (gltf assets have an internal graph of primitives)
  * Mesh will be rendered if this is on it
  * Stores list of material info
  * Has info on how this mesh interacts with lights
* Skinned Mesh Renderer
* Text Mesh
  * Renders text
  * Supports [ttf font rendering](https://github.com/redox-os/rusttype)
  * Implement various [text layout options](https://docs.unity3d.com/Manual/class-TextMesh.html)
* Light component
  * type of light
    * Directional
    * Point
    * Spotlight
    * [Reference](https://docs.unity3d.com/Manual/Lighting.html)
  * range, color, intensity, etc
* Animator Component
	* Can animate mesh
	* State machine (eventually) to transition between animation states

## Rendering

* Deferred shading, then forward render the rest
  * Lighting should be done per-pixel

## Materials

* Use the same info as what's loaded from gltf
  * Look into loading gltf pbr specular glossiness info to use simple lighting shaders
* Has string with type of shader to use
  * Load from shader cache

## Misc

* Asset manager
  * Materials
  * Scripts
* Shader cache
  * Map of strings to loaded shaders
* Particle Effects, GUI, etc
