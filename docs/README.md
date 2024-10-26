# Rust OpenGL Engine (OGL)
**Physics Engine for Space Bodies**
This project is a **Rust**-based physics engine designed to simulate the gravitational interactions between celestial bodies. It allows users to explore basic physics principles, especially orbital mechanics, through an interactive sandbox environment.

## Future Features
- Procedural noise generated planets, and texturing the planets procedurally,
- more mecanics that include inertia of the planet
- inluding all laws of motion of Newton for a specific body
- Realistic Time Acceleration

## Feature Ideas

### 1. Realistic Time Acceleration
   - **Description**: Implement adjustable time scaling to simulate long-term planetary motion, ranging from hours to millennia in a compressed time frame. This allows users to observe and analyze extended orbital paths, gravitational encounters, and planetary migrations.
   - **Use Case**: Observe the progression of orbital precession, planetary alignment patterns, or asteroid belt formation over extended periods.

### 2. N-Body Gravitation
   - **Description**: Expand the engine to include an N-body simulation mode, where all bodies influence each other gravitationally. This creates complex interactions, enabling realistic modeling of star clusters, asteroid fields, and solar systems with multiple bodies affecting each other's trajectories.
   - **Use Case**: Simulate the Milky Way's galactic core or the gravitational clustering and dispersion in an asteroid belt.

### 3. Orbital Resonances and Tidal Locking
   - **Description**: Implement orbital resonance mechanics (e.g., 1:2 or 1:3 resonances) to model how gravitational interactions between bodies influence their orbital periods and stability. Include tidal locking for close-proximity objects, where one body always faces another.
   - **Use Case**: Model Jupiter and its moons, showcasing how resonances impact orbits and drive geological activity, or simulate the tidal locking of Earth and its moon.

### 4. Planetary Perturbation Analysis
   - **Description**: Add the ability to visualize and calculate gravitational perturbations from nearby planets or stars, which slightly alter the paths of orbiting bodies over time.
   - **Use Case**: Demonstrate how Jupiter's gravity influences the asteroid belt or how small perturbations from a passing body can affect long-term orbital stability.

### 5. Lagrange Points and Stable Orbits
   - **Description**: Include the computation and visualization of Lagrange points, where gravitational forces between two large bodies (e.g., a planet and a star) create stable points in space for smaller objects.
   - **Use Case**: Place spacecraft in Earth's Lagrange points or simulate the formation of Trojan asteroids in Jupiter's orbit.

### 6. Collision and Gravitational Slingshots
   - **Description**: Model close encounters between bodies that can result in gravitational slingshots, where a smaller object gains or loses momentum by passing near a larger body. Include collision mechanics for bodies that come within a certain distance, altering mass and trajectory.
   - **Use Case**: Simulate asteroid flybys of planets or slingshot maneuvers used by spacecraft for energy-efficient travel.

### 7. Planetary Migration and System Evolution
   - **Description**: Add simulations for the gradual inward or outward migration of planets within a solar system due to gravitational interactions or loss of angular momentum. This would allow for studying how solar systems evolve over time.
   - **Use Case**: Model how young gas giants may migrate inward, as theorized in the early Solar System, or how interactions can push smaller bodies toward outer regions.

### 8. Binary Star Systems and Multi-Star Orbits
   - **Description**: Support the simulation of binary or trinary star systems, where planets orbit around two or more stars. This adds complexity to orbital mechanics, as planets in such systems can have non-circular, highly dynamic orbits.
   - **Use Case**: Show how binary star systems can influence the orbit stability of surrounding planets and test the habitability zone in multi-star systems.

### 9. Visualized Orbital Prediction and Path Projection
   - **Description**: Implement a tool to project and display future orbital paths based on current velocities and gravitational forces. This would be helpful for identifying potential collisions, resonances, or orbital drift.
   - **Use Case**: Visualize a comet’s path as it approaches the Sun or the altered trajectory of a small moon due to a planetary close encounter.

### 10. Escape Velocity and Hill Spheres
   - **Description**: Add calculations and visual indicators for escape velocity (speed required for an object to escape another's gravitational pull) and Hill spheres (regions within which a planet's gravity dominates over its star’s).
   - **Use Case**: Demonstrate how far a moon can orbit a planet without being pulled away by a nearby star or simulate spacecraft trying to escape a planet’s gravity well.

### 11. Perturbations from Passing Stars or Rogue Planets
   - **Description**: Introduce the possibility of external objects, such as rogue planets or passing stars, temporarily entering the system, causing gravitational perturbations. These could alter orbits or even eject smaller bodies from the system.
   - **Use Case**: Model hypothetical encounters with rogue planets and study their effects on planetary systems' stability and asteroid trajectories.

### 12. Orbital Decay Due to Tidal Forces
   - **Description**: Simulate orbital decay for bodies in close orbits, accounting for tidal forces that gradually reduce their distance. This is especially relevant for moons orbiting large planets or planets close to their stars.
   - **Use Case**: Show how moons like Phobos are slowly spiraling into Mars or simulate a hypothetical planet losing its orbit around a star over eons.

### 13. Resonant Orbit Transfers and Station-Keeping
   - **Description**: Implement support for simulating resonant orbit transfers, a technique used by spacecraft to change orbits with minimal fuel by using natural gravitational assists. Include station-keeping mechanics for orbit stabilization.
   - **Use Case**: Show how a spacecraft uses Earth’s gravity for a resonant transfer to reach Mars or how satellites maintain their geostationary orbits through small corrections.

### 14. Eccentricity and Inclination Variations
   - **Description**: Model the variations in orbital eccentricity (shape) and inclination (tilt) over time due to gravitational interactions, such as Kozai oscillations that exchange inclination for eccentricity.
   - **Use Case**: Demonstrate how a comet’s orbit can evolve from circular to highly elliptical or how a planet’s axial tilt can change over millennia, affecting climate.

### 15. Three-Body Problems and Chaotic Orbits
   - **Description**: Add a feature to analyze chaotic orbits and three-body problem scenarios, where three gravitational bodies interact, leading to unpredictable orbital changes. This is particularly useful for studying highly dynamic systems.
   - **Use Case**: Simulate the Earth, Moon, and Sun interactions or the complex orbit of a moon in a binary planet system.

# How it Works

### 1. Planet generation
##### Sphere Generation
In order to create a sphere a structure known as the sphere constructor where you input the radius longitude, and position is given.
- for each point in the sphere, at a specific longitude and latitude, we get two angles phi and theta, which will help us construct the sphere.
- to change the location of the sphere, a transform matrix is used
###### Indices
In 3D rendering with indexed primitives, the **primitive index type** used for a sphere depends on the range of vertices you’re indexing:
1. **`u16` (16-bit unsigned integer)**: Use this type if you have fewer than 65,536 vertices. It’s common for lower-resolution spheres or less detailed meshes, as it saves memory and can be faster for the GPU to process.
2. **`u32` (32-bit unsigned integer)**: Use this type if you have 65,536 vertices or more. Higher-resolution spheres or very detailed meshes generally require `u32` indices since `u16` wouldn’t cover the total vertex count.

For spheres, the top and bottom vertices act as "singularities":
- **Top Pole**: The northernmost vertex is shared by all segments of the first ring.
- **Bottom Pole**: The southernmost vertex is shared by all segments of the last ring.
The indexing system works because each `quad` has four vertices that logically represent a grid cell on the sphere's surface. Dividing this grid cell into two triangles allows OpenGL (or the rendering API) to rasterize them correctly, creating a smooth spherical appearance when enough latitude and longitude divisions are used.
![quad](./quad.png)


### 2. Introduction to Gravitational Force
To model an astral body, we inherit mesh data and add essential physical properties like acceleration, velocity, and mass. We calculate its motion by first applying Newton’s Second Law, ${F}=ma$, to determine acceleration as the result of the sum of forces acting on the body. Here, we incorporate Newton's law of universal gravitation for interactions between bodies, calculating gravitational force based on the formula $F = G \frac{m_1 m_2}{r^2}$​​.

We write $µ = G{m_1 m_2}$ as its a constant, we are just missing r, which is the distance between the two bodies, one we have that, we can apply our force on our body.

But for the universal gravitational force, we need to distance vector between the two bodies, but all we have is the transformation matrices of the two matrices.

To simulate the motion accurately:
1. **Force Aggregation**: Compute all forces, including gravitational, applied, and resistive forces.
2. **Integration Step**: Use numerical integration (e.g., Euler or Verlet) to update the velocity and position based on current acceleration.
3. **Matrix Transformation**: Apply transformation matrices for position updates, ensuring that the simulation accurately reflects forces and resulting movement over discrete time steps.

### 3. Computation
In the graphics layer, implemented using OpenGL and the Rust-based Glium framework, transformation matrices handle 3D positioning and rendering. These matrices allow translation, rotation, and scaling operations, enabling your code to reflect each body's real-time position in 3D space. OpenGL’s transformation pipeline combines world space with view and projection matrices, rendering each body in an interactive environment that displays real-time physics effects.

In OpenGL, we use transformation matrices to represent affine transformations within $\mathbb{R}^3$. These matrices act as linear maps adapted to the canonical basis of $\mathbb{R}^3$.

By combining force computation with continuous integration and rendering transformations, your engine showcases an accurate, physics-based simulation of gravitational interactions. This project design in Rust with Glium demonstrates a solid foundation for creating and visualizing gravitational forces within a 3D sandbox environment. For more technical specifics, you can refer to the GitHub project repository for code details and documentation.

## Modifiable Parameters

The following parameters are customizable:
- **Star Mass**: Controls the gravitational pull exerted on planets.
- **Planet Position and Momentum**: Adjust initial conditions to generate different orbital behaviors.
- **Time Step**: Fine-tune the precision of the simulation for more accurate results.

# How to Use

1. Clone the repository:
   ```bash
   git clone https://github.com/MadebyDaris/ogl.git
   cd ogl
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the simulation:
   ```bash
   cargo run
   ```

4. Modify the parameters in the configuration file or directly in the code to experiment with different planetary systems.

## Dependencies

- **glium**: For rendering the graphical output.
- **image**: For handling textures and visualization.

Install all dependencies using:

```bash
cargo build
```

---

Feel free to modify and extend the project to suit your needs