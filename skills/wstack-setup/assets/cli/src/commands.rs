#[derive(Clone, Copy)]
pub enum Builtin {
    Info,
    Doctor,
    FeaturesList,
    FeaturesShow,
    FeaturesCheck,
    Brand,
}

pub const COMMANDS: &[(Builtin, &str, &str, &str)] = &[
    (
        Builtin::Brand,
        "brand",
        "refresh|list|style [args...]",
        "Local brand guide and asset discovery",
    ),
    (
        Builtin::Info,
        "info",
        "--json [--base REF]",
        "Project identity and capabilities",
    ),
    (
        Builtin::Doctor,
        "doctor",
        "[--json]",
        "Check tools; app readiness is separate",
    ),
    (
        Builtin::FeaturesList,
        "features:list",
        "[--json]",
        "List the canonical feature map",
    ),
    (
        Builtin::FeaturesShow,
        "features:show",
        "ID [--json]",
        "Read a canonical feature recipe",
    ),
    (
        Builtin::FeaturesCheck,
        "features:check",
        "[--json]",
        "Check canonical map structure",
    ),
];

pub fn public_name(name: &str) -> String {
    let mut name = name.to_owned();
    while crate::routes::ROUTES.iter().any(|route| route.name == name) {
        name = format!("wstack:{name}");
    }
    name
}

pub fn find(name: &str) -> Option<Builtin> {
    COMMANDS
        .iter()
        .find(|(_, id, _, _)| public_name(id) == name)
        .map(|c| c.0)
}
