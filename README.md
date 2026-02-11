## **Bevy 3D Vector Visualizer - Final Implementation Plan**

### **Overview**

This document outlines the implementation plan for an interactive visualization tool for **N-dimensional vectors**. The tool, built with the Bevy engine, will allow users to define a custom number of dimensions for all vectors. The N-dimensional data will be visualized via a **3D projection** within a transparent reference sphere.

### **Core Concept: Visualizing N-Dimensions**

Since we cannot directly render more than three spatial dimensions, this tool will adopt a projection-based approach:

  * **N-Dimensional Data:** Vectors will be stored in memory with `n` components, as defined by the user.
  * **3D Projection:** The visualization in the 3D scene (the lines, boxes, and planes) will be generated using only the **first three components (x, y, z)** of the N-dimensional vector.
  * **UI Controls:** The side panel will provide sliders to control **all `n` components**, allowing users to manipulate the underlying data, even for dimensions that are not directly visualized.

-----

### **Phase 1: Basic Scene Setup**

1.  **Initialize Bevy App:** Set up a standard Bevy application with `DefaultPlugins`.
2.  **Environment:** Configure a dark `ClearColor` for high contrast.
3.  **Camera & Controls (Updated):** Implement a 3D camera with **free-orbit (trackball-style) controls**. This will allow the user to rotate the view around the central origin `(0,0,0)` from **any angle without restriction**, including tumbling the camera upside down to view from below. Zooming will be handled by the mouse scroll wheel.
4.  **Lighting:** Add basic ambient and directional light to ensure all objects are clearly visible.

-----

### **Phase 2: Core 3D Objects**

5.  **Central Sphere:** A sphere mesh at the origin `(0,0,0)` with a fixed radius of **1.0**, acting as a visual reference frame.
6.  **Cartesian Axes:** Red (X), Green (Y), and Blue (Z) lines from `(0,0,0)` to `(1,0,0)`, etc., to mark the 3D projection space.
7.  **Spherical Coordinate Grid:** Optional latitude/longitude lines on the sphere's surface, with UI controls for visibility and style (solid/dotted).

-----

### **Phase 3: Dimensionality Control**

8.  **Global Dimension Setting:** A "Global Settings" section in the UI will contain a number input labeled "**Dimensions**" for setting the coordinate count (`n`).
9.  **Reset Confirmation Workflow:** Changing the dimension count will trigger a confirmation pop-up. If the user proceeds, all existing vectors will be cleared from the scene and data state to ensure consistency.
10. **State Management:** The global `AppState` will store the current number of dimensions, e.g., `dimensions: usize`.

-----

### **Phase 4: 3D Vector Structure System**

11. **Structure Generation from Projection:** The generation function takes an N-dimensional vector (`Vec<f32>`).
      * It extracts the first three components as `x = coords[0]`, `y = coords[1]`, and `z = coords[2]`. If `n` is less than 3, missing components default to `0`.
      * It uses these `x, y, z` values to generate the 3D visual structure: a Resultant Vector Line, Axis Component Lines, a Projection Box, and an optional Projection Plane.

-----

### **Phase 5: UI System & Side Panel**

12. **Menu Button & Panel:** A standard slide-out panel accessible from a menu icon (☰) in the top-right corner.
13. **Panel Layout:**
      * **Global Settings:** The **Dimensions** control.
      * **Sphere Settings:** Controls for the sphere and grid (transparency, color, style).
      * **Vector Management:** An "Add Vector" button and the scrollable list of vectors.

-----

### **Phase 6: Add Vector Functionality**

14. **"Add Vector" Workflow:**
      * Reads the current dimension count `n`.
      * Generates a new vector with **`n` random coordinates**, each between `-1.0` and `1.0`.
      * Creates the corresponding 3D projection in the scene and adds a control tile to the UI list.

-----

### **Phase 7: Enhanced Vector Tile Design**

15. **Collapsed/Expanded State:** Standard accordion behavior for each vector tile.
16. **Expanded State Controls:**
      * **Name Field:** A text input for live editing of the vector's display name.
      * **Dynamic Coordinate Sliders:** The tile dynamically generates **`n` sliders**. The naming and color-coding create a clear visual hierarchy.
      * **Live Updates:** Sliders for X, Y, and Z update the 3D view in real-time. Other sliders update the vector's data without changing the 3D projection.
      * **Visibility & Appearance Controls:** Toggles and pickers to manage the appearance of the vector's 3D projection.

**Coordinate Naming and Color Summary**

| Dimension | UI Label | UI Color | 3D Visualization Role |
| :--- | :--- | :--- | :--- |
| **1** | **X** | **Red** | Controls the **X-axis** projection. |
| **2** | **Y** | **Green** | Controls the **Y-axis** projection. |
| **3** | **Z** | **Blue** | Controls the **Z-axis** projection. |
| **4+** | **Dim 4**, etc. | **Gray** | **Data-only**. Modifies vector data but has **no** direct visual effect. |

-----

### **Phase 8: State & Data Management**

17. **VectorData Structure:**
    ```rust
    struct VectorData {
        internal_id: String,
        display_name: String,
        // The core data is now a dynamically sized vector
        coordinates: Vec<f32>,
        // ... visibility, appearance, and entity handle fields ...
    }
    ```
18. **Global AppState Resource:**
    ```rust
    #[derive(Resource)]
    struct AppState {
        // The global dimension setting
        dimensions: u16,
        // ... other global settings ...
        vectors: Vec<VectorData>,
    }
    ```

-----

### **Phase 9: Technical Implementation & Polish**

19. **Hot Reloading:** Enable Bevy's hot reloading feature for rapid development iteration.
20. **Real-time Updates:** Use Bevy's ECS and event system for instant, responsive feedback between the UI and the 3D scene.
21. **Visual Polish & Error Handling:** Ensure smooth controls, a clean UI, and robust input validation. Implement proper entity cleanup, especially for the dimension reset workflow, to prevent memory leaks.
