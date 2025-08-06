# Bevy 3D Vector Visualizer - Complete Implementation Plan

## Overview
Interactive 3D visualization tool for word vectors using triangular representations within a transparent sphere. Built with Bevy engine for real-time 3D rendering and intuitive controls.

## Phase 1: Basic Scene Setup
1. Initialize Bevy app with 3D renderer and default plugins
2. Create dark background/environment
3. Setup 3D camera with orbit controls (rotate around center, zoom in/out)
4. Add basic lighting (ambient + directional) for 3D visibility
5. Position camera initially at (2, 2, 2) looking at origin (0,0,0)

## Phase 2: Core 3D Objects
6. Create transparent sphere at origin with radius 1.0
   - Material: Default gray (#808080), semi-transparent (initial alpha ~0.3)
   - Wireframe or solid with transparency
7. Create 3D coordinate axes (red=X, green=Y, blue=Z) from origin to sphere surface
   - Lines from (0,0,0) to (1,0,0), (0,1,0), (0,0,1)
   - Keep these always visible as reference

## Phase 3: Vector Triangle System
8. Create triangle generation function that takes [x,y,z] coordinates
9. Triangle vertices: (x,0,0), (0,y,0), (0,0,z)
10. Triangle material: Default gray transparent surface (#808080, alpha ~0.3)
11. Triangle edges:
    - Origin to (x,0,0): RED line (#FF0000)
    - Origin to (0,y,0): GREEN line (#00FF00)
    - Origin to (0,0,z): BLUE line (#0000FF)
    - Triangle sides: Same color as surface (not highlighted)
12. **Intersection handling**: When triangles overlap, implement additive transparency (denser opacity where they intersect)

## Phase 4: UI System Structure
13. Create menu button (☰) in top-right corner
14. Implement slide-out side panel system:
    - Panel slides in from right when menu clicked
    - Panel slides out when minimized
    - Panel width: ~300px
15. **All controls inside side panel** - no floating buttons

## Phase 5: Side Panel Layout
16. **Panel Header:**
    - Title: "Vector Controls"
    - Minimize button (←)
    - **Global toggle**: "Show All Tags" (affects all vector name labels)
17. **Sphere Controls Section:**
    - Label: "Sphere Settings"
    - Transparency slider (0.0 to 1.0) with live preview
    - Color picker: Hex input field (#808080 default)
18. **Vector Management Section:**
    - "Add Vector" button (prominent, at top)
    - Scrollable list of vector tiles below

## Phase 6: Vector Naming System
19. **Internal naming system:**
    - Internal IDs: T1, T2, T3, etc. (auto-increment)
    - Display names: "Untitled(1)", "Untitled(2)", etc. (user-editable)
20. **Vector tile display:**
    - Show both: "T1: Cat Vector" or "T2: Untitled(3)"
    - Internal ID always visible for identification

## Phase 7: Add Vector Functionality
21. **Add Vector button workflow:**
    - Generate random vector values between -1 and 1: [x,y,z]
    - Assign internal ID (T1, T2, etc.)
    - Assign default display name: "Untitled(1)", "Untitled(2)"
    - Create triangle with default materials
    - Add to scene and vector list
    - Scroll to newly created vector tile

## Phase 8: Enhanced Vector Tile Design
22. **Collapsed tile state:**
    - Header: "T1: Vector Name"
    - Click anywhere on tile to expand/collapse
    - Compact view showing just essential info

23. **Expanded tile state:**
    - **Name field:** Direct text input with live editing (no save button needed)
    - **Real-time coordinate sliders:**
      - X slider: -1 to 1 (red label/indicator)
      - Y slider: -1 to 1 (green label/indicator)
      - Z slider: -1 to 1 (blue label/indicator)
      - **Live updates** in 3D scene while user drags sliders
    - **Visibility toggles:**
      - Triangle surface visible/hidden (👁️)
      - Origin edges visible/hidden (📏)
      - Completely visible/hidden (🚫)
      - Show individual tag label (🏷️)
    - **Appearance controls:**
      - Transparency slider (0.0-1.0) with real-time preview
      - Color hex input field for triangle surface
    - **Delete button:** Remove vector entirely (🗑️)

## Phase 9: 3D Text Label System
24. **Tag/Label implementation:**
    - **Position:** At midpoint of each origin edge line
    - **Content:** Vector display name (e.g., "T1: Cat Vector")
    - **Visibility:** Controlled by global toggle + individual vector toggle
    - **Style:** Billboard text (always faces camera for readability)
    - **Appearance:** Readable font, contrasting color against dark background

## Phase 10: Advanced Material System
25. **Individual vector transparency:**
    - Each triangle has independent alpha value
    - Slider updates material in real-time
    - Transparency affects surface only, origin edges maintain visibility when enabled
26. **Color system:**
    - Hex color picker updates triangle surface color
    - Origin edges remain red/green/blue regardless of surface color
    - Invalid hex codes show error state/revert to previous valid color
27. **Additive intersection rendering:**
    - Use additive blending for overlapping triangles
    - Overlapping areas accumulate opacity (more visible where triangles intersect)
    - Single triangle: alpha = 0.3, Two overlapping: alpha = 0.6, etc.
    - Maintain proper depth sorting for transparent rendering

## Phase 11: Toggle System Implementation
28. **Triangle surface toggle:**
    - Hides/shows triangle surface mesh only
    - Origin edges visibility controlled separately
    - Toggle state saved per vector
29. **Origin edges toggle:**
    - Hides/shows red/green/blue lines from origin to axis points
    - Independent of surface visibility
    - Allows users to reduce visual clutter while keeping triangles
30. **Complete visibility toggle:**
    - Hides all visual elements (surface + edges + tags)
    - Vector remains in list but appears grayed out/disabled
    - Easy way to temporarily remove vectors from view

## Phase 12: State Management Structure
31. **Vector data structure:**
```rust
VectorData {
    internal_id: String,        // "T1", "T2", etc.
    display_name: String,       // "Cat Vector", "Untitled(1)"
    coordinates: [f32; 3],      // [x, y, z] values
    surface_visible: bool,      // Triangle surface toggle
    edges_visible: bool,        // Origin edges toggle
    completely_visible: bool,   // Complete visibility toggle
    tag_visible: bool,          // Individual tag toggle
    transparency: f32,          // 0.0 to 1.0
    color: String,             // Hex color "#808080"
    triangle_entity: Entity,    // Bevy entity reference
    edges_entities: Vec<Entity>, // Origin edge line entities
    tag_entity: Entity         // 3D text label entity
}
```

32. **Global state:**
```rust
AppState {
    sphere_transparency: f32,
    sphere_color: String,
    show_all_tags: bool,        // Global tag visibility
    next_vector_id: u32,        // For T1, T2, T3 generation
    vectors: Vec<VectorData>
}
```

## Phase 13: Interaction & Camera Controls
33. **Camera controls:**
    - Mouse drag: Rotate around sphere center (orbit behavior)
    - Mouse scroll: Zoom in/out with reasonable min/max constraints
    - Smooth interpolation for camera movements
34. **UI event handling:**
    - All button clicks and interactions within side panel
    - Slider updates with immediate visual feedback
    - Color picker validation and real-time updates
    - Toggle state management with visual state reflection
    - Tile expand/collapse animations

## Phase 14: Real-time Updates System
35. **Live coordinate editing:**
    - X/Y/Z sliders update triangle vertices immediately
    - Smooth interpolation during slider drag for fluid motion
    - Triangle shape morphs in real-time as user adjusts coordinates
36. **Material updates:**
    - Transparency sliders affect materials instantly
    - Color changes apply immediately to triangle surfaces
    - Visibility toggles show/hide elements without delay

## Phase 15: Visual Polish & Styling
37. **Panel styling:**
    - Clean, dark theme matching 3D scene background
    - Proper spacing and padding between vector tiles
    - Scrollable vector list with consistent tile heights
    - Visual feedback for expanded/collapsed states
38. **3D scene polish:**
    - Smooth camera transitions and controls
    - Proper lighting setup for transparent materials
    - Clear visual hierarchy (coordinate axes always visible)
    - Appropriate ambient lighting to see transparent objects clearly

## Phase 16: Error Handling & Validation
39. **Input validation:**
    - Coordinate slider values automatically clamped to [-1, 1] range
    - Hex color format validation with error indicators
    - Duplicate name handling (auto-append numbers: "Cat", "Cat(2)", etc.)
40. **Performance considerations:**
    - Reasonable limit on maximum number of vectors (e.g., 50-100)
    - Efficient material updates (batch when possible)
    - Proper cleanup of Bevy entities on vector deletion
    - Optimize transparent rendering pipeline

## Technical Implementation Notes
- **Bevy Systems:** Use Bevy's ECS for vector management and UI state
- **Materials:** Custom transparent materials with proper alpha blending
- **UI Framework:** Bevy's built-in UI system for side panel and controls
- **3D Text:** Bevy's text rendering for 3D labels/tags
- **Camera:** Implement or use existing orbit camera controller
- **Events:** Bevy's event system for UI interactions and updates
- **Resources:** Store global app state in Bevy resources

## Key Features Summary
- ✅ Interactive 3D sphere (radius 1.0) with adjustable transparency
- ✅ Vector triangles with real-time coordinate adjustment
- ✅ Multiple visibility controls (surface, edges, complete, tags)
- ✅ Additive transparency for intersection visualization
- ✅ Expandable vector tiles with live editing
- ✅ Individual and global tag/label system
- ✅ Real-time material and coordinate updates
- ✅ Orbit camera controls with zoom
- ✅ Clean side panel UI with all controls
- ✅ Proper state management and error handling

## Expected User Flow
1. **Start:** Dark scene with empty transparent sphere and coordinate axes
2. **Add Vector:** Click "Add Vector" → generates "T1: Untitled(1)" with random coordinates
3. **Customize:** Expand tile → adjust name, coordinates, colors, visibility in real-time
4. **Multiple Vectors:** Add more vectors, see intersections with additive transparency
5. **Manage:** Toggle visibility, adjust sphere transparency, show/hide tags as needed
6. **Explore:** Rotate camera, zoom in/out to examine vector relationships in 3D space
