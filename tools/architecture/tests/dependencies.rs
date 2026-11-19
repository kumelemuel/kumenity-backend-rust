//! Dirección de dependencias (HEX-1, HEX-2).
//!
//! Entre crates, Cargo ya impide usar lo que no está declarado; estos tests
//! impiden *declarar* lo que la arquitectura no permite. Entre los módulos de
//! capa de un crate legacy (`crates/{ctx}/src/{capa}`) Cargo no ve nada, así que
//! `hex_1_legacy_modules_point_inward` lo verifica sobre las rutas del código.

#![allow(
    clippy::expect_used,
    reason = "helpers de tests de integración: allow-expect-in-tests de clippy.toml solo cubre #[test] y #[cfg(test)]"
)]

mod common;

use common::{Crate, Layer, assert_no_violations, crates, parse, relative, rust_files};
use std::collections::{BTreeSet, HashMap};
use syn::visit::Visit;

/// Frameworks de servidor: solo en el borde driving y en el composition root.
const SERVER_FRAMEWORKS: &[&str] = &["axum", "actix-web", "rocket", "warp", "poem", "tower-http"];

fn allowed(from: (Layer, &Option<String>), to: (Layer, &Option<String>)) -> bool {
    use Layer::*;
    let same_context = from.1 == to.1;
    match (from.0, to.0) {
        (Kernel, _) => false,
        (_, Kernel) => true,
        (Domain, _) => false,
        (Application, Domain) => same_context,
        (Infrastructure, Domain | Application) => same_context,
        // Un api de contexto solo ve su application; uno central (`apps/api`) ve todos.
        (Api, Application) => from.1.is_none() || same_context,
        (Api, Edge) => true,
        (Edge, _) => false,
        (Bootstrap, _) => true,
        _ => false,
    }
}

fn may_depend_on(from: &Crate, to: &Crate) -> bool {
    allowed((from.layer, &from.context), (to.layer, &to.context))
}

/// Una entrada por manifiesto: las capas legacy de un contexto comparten
/// `Cargo.toml`, y una dependencia declarada ahí se cuenta una sola vez, con la
/// capa más interna del contexto (la que más restringe).
fn by_manifest<'a>(crates: impl Iterator<Item = &'a Crate>) -> Vec<&'a Crate> {
    let mut seen = BTreeSet::new();
    crates.filter(|c| seen.insert(c.dir.clone())).collect()
}

#[test]
fn hex_1_dependencies_point_inward() {
    // Las capas legacy no son crates: no se las puede nombrar en un manifiesto.
    let crates: Vec<Crate> = crates().into_iter().filter(|c| !c.legacy).collect();
    let by_name: HashMap<&str, &Crate> = crates.iter().map(|c| (c.name.as_str(), c)).collect();
    let edges = crates
        .iter()
        .flat_map(|c| c.dependencies())
        .filter(|dep| by_name.contains_key(dep.as_str()))
        .count();

    let violations = crates
        .iter()
        .flat_map(|from| {
            from.dependencies().into_iter().filter_map(|dep| {
                let to = by_name.get(dep.as_str())?;
                (!may_depend_on(from, to)).then(|| {
                    format!(
                        "{}: {} ({:?}) → {} ({:?})",
                        from.manifest_path(),
                        from.name,
                        from.layer,
                        to.name,
                        to.layer
                    )
                })
            })
        })
        .collect();

    assert_no_violations(
        "hex_1_dependencies_point_inward",
        "HEX-1: las dependencias apuntan hacia adentro; api solo ve el application de su contexto y edge; \
         api e infrastructure no se ven entre sí; sin dependencias entre contextos",
        &[
            ("crates", crates.len()),
            ("dependencias entre crates del workspace", edges),
        ],
        violations,
    );
}

/// Rutas de un archivo con al menos dos segmentos: `(línea, segmentos)`.
#[derive(Default)]
struct Paths(Vec<(usize, Vec<String>)>);

impl Paths {
    fn walk(&mut self, tree: &syn::UseTree, prefix: &mut Vec<(usize, String)>) {
        match tree {
            syn::UseTree::Path(p) => {
                prefix.push((p.ident.span().start().line, p.ident.to_string()));
                self.walk(&p.tree, prefix);
                prefix.pop();
            }
            syn::UseTree::Group(g) => {
                for item in &g.items {
                    self.walk(item, prefix);
                }
            }
            syn::UseTree::Name(syn::UseName { ident })
            | syn::UseTree::Rename(syn::UseRename { ident, .. }) => {
                let mut segments: Vec<String> = prefix.iter().map(|(_, s)| s.clone()).collect();
                segments.push(ident.to_string());
                self.push(prefix.first().map_or(0, |(l, _)| *l), segments);
            }
            syn::UseTree::Glob(_) => {
                let segments: Vec<String> = prefix.iter().map(|(_, s)| s.clone()).collect();
                self.push(prefix.first().map_or(0, |(l, _)| *l), segments);
            }
        }
    }

    fn push(&mut self, line: usize, segments: Vec<String>) {
        if segments.len() >= 2 {
            self.0.push((line, segments));
        }
    }
}

impl<'ast> Visit<'ast> for Paths {
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        self.walk(&node.tree, &mut Vec::new());
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        let segments = path.segments.iter().map(|s| s.ident.to_string()).collect();
        let line = path
            .segments
            .first()
            .map_or(0, |s| s.ident.span().start().line);
        self.push(line, segments);
        syn::visit::visit_path(self, path);
    }
}

/// Capa y contexto a los que apunta una ruta, si es de un paquete del workspace.
fn target(
    segments: &[String],
    from: &Crate,
    kernel: &str,
    legacy: &[String],
) -> Option<(Layer, Option<String>)> {
    let package = match segments[0].as_str() {
        "crate" if from.legacy => from.name.clone(),
        "crate" | "self" | "super" => return None,
        other => other.to_string(),
    };
    if package == kernel {
        return Some((Layer::Kernel, None));
    }
    if !legacy.contains(&package) {
        return None;
    }
    let layer = match segments[1].as_str() {
        "domain" => Layer::Domain,
        "application" => Layer::Application,
        "infrastructure" => Layer::Infrastructure,
        _ => return None,
    };
    Some((layer, Some(package)))
}

#[test]
fn hex_1_legacy_modules_point_inward() {
    let all = crates();
    let kernel = all
        .iter()
        .find(|c| c.layer == Layer::Kernel)
        .map(|c| c.name.clone())
        .expect("invariante: el workspace tiene kernel");
    let legacy: Vec<String> = all
        .iter()
        .filter(|c| c.legacy)
        .map(|c| c.name.clone())
        .collect();

    let mut violations = Vec::new();
    let mut files = 0;
    // Los módulos legacy, y todo crate que pueda nombrar un paquete legacy.
    for from in all.iter().filter(|c| c.layer != Layer::Kernel) {
        for file in rust_files(&from.path("src")) {
            files += 1;
            let mut paths = Paths::default();
            paths.visit_file(&parse(&file));
            // Una violación por archivo y destino (capa + contexto).
            let mut reported = BTreeSet::new();
            for (line, segments) in paths.0 {
                let Some(to) = target(&segments, from, &kernel, &legacy) else {
                    continue;
                };
                let same_module =
                    from.legacy && to.1.as_ref() == Some(&from.name) && to.0 == from.layer;
                if same_module || allowed((from.layer, &from.context), (to.0, &to.1)) {
                    continue;
                }
                if reported.insert((to.0 as u8, to.1.clone())) {
                    violations.push(format!(
                        "{}:{line}: `{}` — {:?} de {} → {:?} de {}",
                        relative(&file),
                        segments.join("::"),
                        from.layer,
                        from.context.as_deref().unwrap_or(&from.name),
                        to.0,
                        to.1.as_deref().unwrap_or("kernel"),
                    ));
                }
            }
        }
    }

    assert_no_violations(
        "hex_1_legacy_modules_point_inward",
        "HEX-1: en los crates legacy (capas como módulos) y en quien los usa, las rutas \
         apuntan hacia adentro; sin rutas entre contextos; api solo ve application",
        &[("archivos fuera del kernel", files)],
        violations,
    );
}

/// Cubre también TDD-2: `Crate::dependencies` incluye `[dev-dependencies]`, y
/// Cargo impide usar en un test lo que no está declarado. Así, los tests de
/// dominio y de caso de uso no pueden usar `tokio` ni un framework.
#[test]
fn hex_2_core_depends_only_on_std_and_the_kernel() {
    let crates = crates();
    let internal: Vec<&str> = crates.iter().map(|c| c.name.as_str()).collect();
    let core = by_manifest(
        crates
            .iter()
            .filter(|c| matches!(c.layer, Layer::Kernel | Layer::Domain | Layer::Application)),
    );

    let violations = core
        .iter()
        .flat_map(|c| {
            c.dependencies()
                .into_iter()
                .filter(|dep| !internal.contains(&dep.as_str()))
                .map(move |dep| {
                    format!(
                        "{}: {} ({:?}) usa el crate externo `{dep}`",
                        c.manifest_path(),
                        c.name,
                        c.layer
                    )
                })
        })
        .collect();

    assert_no_violations(
        "hex_2_core_depends_only_on_std_and_the_kernel",
        "HEX-2 / TDD-2: domain, application y kernel solo usan std y el kernel, \
         también en [dev-dependencies] (sus tests no levantan runtime ni framework)",
        &[("crates de kernel, domain y application", core.len())],
        violations,
    );
}

#[test]
fn hex_2_server_frameworks_only_at_the_edge() {
    let crates = crates();
    let inner = by_manifest(
        crates
            .iter()
            .filter(|c| !matches!(c.layer, Layer::Api | Layer::Edge | Layer::Bootstrap)),
    );
    let violations = inner
        .iter()
        .flat_map(|c| {
            c.dependencies()
                .into_iter()
                .filter(|dep| SERVER_FRAMEWORKS.contains(&dep.as_str()))
                .map(move |dep| {
                    format!(
                        "{}: {} ({:?}) usa `{dep}`",
                        c.manifest_path(),
                        c.name,
                        c.layer
                    )
                })
        })
        .collect();

    assert_no_violations(
        "hex_2_server_frameworks_only_at_the_edge",
        "HEX-2: los frameworks de servidor solo viven en api, edge y bootstrap",
        &[(
            "crates fuera de api, edge y bootstrap (kernel, domain, application, infrastructure)",
            inner.len(),
        )],
        violations,
    );
}
