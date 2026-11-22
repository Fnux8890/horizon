use crate::Identifier;

/// A resource-defined post chain requested by its owner for this frame only.
// exhaustive: mods construct requests directly from their chain ID and the four shader parameters.
#[derive(Clone, Debug, PartialEq)]
#[derive(horizon_mod_api_macros::Wire)]
pub struct ModPostEffect {
    /// Namespaced resource ID of the post-effect JSON owned by this mod.
    pub id: Identifier,
    /// Bound to the chain's std140 HorizonModEffect block as one vec4.
    pub params: [f32; 4],
}
