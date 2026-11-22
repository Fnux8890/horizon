//! Frame-local world effects that compose with the selected shader pack.
pub trait Hooks {
    /// Request a resource-defined post chain in a namespace your mod owns. Chains
    /// run after existing world effects, before the HUD, in qualified ID order.
    /// The four parameters belong to this chain. For distortion, scale the
    /// strength by distortion_scale and omit the request when it is zero.
    #[hook(
        side = "client",
        contract = "collect",
        out = "effects",
        answered_by = "mod",
        neoforge = "none (Horizon composable post chains)"
    )]
    fn mod_post_effects();
}
