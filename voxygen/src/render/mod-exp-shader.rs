/// Experimental shader modes.
///
/// You can enable these using Voxygen's `settings.ron`. See
/// [here](https://book.veloren.net/players/voxygen.html#experimental-shaders) for more information.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    strum::EnumIter,
    strum::Display,
    strum::EnumString,
)]
pub enum ExperimentalShader {
    /// Add brick-like normal mapping to the world.
    Brickloren,
    /// Remove the default procedural noise from terrain.
    NoNoise,
    /// Add a sobel filter that draws lines in post-process by detecting edges
    /// inbetween colors. This does perform 8 times more texture samples in
    /// post-processing so there is potentially a significant performance
    /// impact especially with anti aliasing enabled.
    Sobel,
    /// Simulate a curved world.
    CurvedWorld,
    /// Adds extra detail to distant LoD (Level of Detail) terrain procedurally.
    ProceduralLodDetail,
    /// Add a warping effect when underwater.
    Underwarper,
    /// Remove caustics from underwater terrain when shiny water is enabled.
    NoCaustics,
    /// Don't dither color in post-processing.
    NoDither,
    /// Don't use the nonlinear srgb space for dithering color.
    NonSrgbDither,
    /// Use triangle PDF noise for dithering instead of uniform noise.
    TriangleNoiseDither,
    /// Removes as many effects (including lighting) as possible in the name of
    /// performance.
    BareMinimum,
    /// Lowers strength of the glow effect for lights near the camera.
    LowGlowNearCamera,
    /// Disable the fake voxel effect on LoD features.
    NoLodVoxels,
    /// Disable the 'pop-in' effect when loading terrain.
    NoTerrainPop,
    /// Display grid lines to visualize the distribution of shadow map texels
    /// for the directional light from the sun.
    DirectionalShadowMapTexelGrid,
    /// Disable rainbows
    NoRainbows,
    /// Add extra detailing to puddles.
    PuddleDetails,
}
