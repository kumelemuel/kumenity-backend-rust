#![allow(dead_code)]

use std::{
    fs,
    path::{Path, PathBuf},
};

/// Marcadores de modelo del kernel (HEX-8): abiertos, sin vocabulario de un contexto.
pub const MODEL_MARKERS: &[&str] = &["ValueObject", "Entity", "AggregateRoot", "Identifier"];

/// Capacidades técnicas transversales del kernel: lista cerrada de HEX-8.
/// Agregar una exige cambiar HEX-8 (`core/principles/hexagonal.md`) y este test
/// en el mismo commit.
pub const CAPABILITIES: &[&str] = &["Clock", "IdGenerator", "FaultReporter"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Kernel,
    Domain,
    Application,
    Infrastructure,
    /// Adaptadores driving: `contexts/{ctx}/api` (por contexto) o `apps/api` (central).
    Api,
    /// Borde compartido por los adaptadores driving: `edge/*`.
    Edge,
    Bootstrap,
}

#[derive(Debug)]
pub struct Crate {
    pub name: String,
    pub layer: Layer,
    pub context: Option<String>,
    /// Directorio del paquete (donde está su `Cargo.toml`).
    pub dir: PathBuf,
    /// Raíz del código de esta capa: `{dir}/src`, o `{dir}/src/{capa}` si es legacy.
    pub src: PathBuf,
    /// Capa que es un módulo de un crate por contexto (`crates/{ctx}/src/{capa}`),
    /// no un crate propio. Comparte manifiesto con las otras capas del contexto.
    pub legacy: bool,
    pub manifest: toml::Table,
}

impl Crate {
    /// Ruta bajo la capa, con la convención de la plantilla: `"src"`, `"src/ports/inbound"`,
    /// `"tests"`. En una capa legacy, `src` es el módulo de la capa y no hay `tests/` propio.
    pub fn path(&self, sub: &str) -> PathBuf {
        if sub == "src" {
            return self.src.clone();
        }
        if let Some(rest) = sub.strip_prefix("src/") {
            return self.src.join(rest);
        }
        if self.legacy {
            // Los `tests/` de un crate legacy no pertenecen a ninguna capa.
            return self.src.join("__sin_tests_por_capa__");
        }
        self.dir.join(sub)
    }

    /// Ruta del manifiesto, relativa a la raíz.
    pub fn manifest_path(&self) -> String {
        relative(&self.dir.join("Cargo.toml"))
    }

    pub fn dependencies(&self) -> Vec<String> {
        ["dependencies", "dev-dependencies", "build-dependencies"]
            .iter()
            .filter_map(|section| self.manifest.get(*section).and_then(|v| v.as_table()))
            .flat_map(|table| table.keys().cloned())
            .collect()
    }
}

pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("invariante: la raíz del workspace existe")
}

fn subdirs(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut dirs: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.join("Cargo.toml").exists() || p.is_dir())
        .collect();
    dirs.sort();
    dirs
}

fn load(dir: PathBuf, layer: Layer, context: Option<String>) -> Crate {
    let text = fs::read_to_string(dir.join("Cargo.toml")).expect("invariante: Cargo.toml legible");
    let manifest: toml::Table = text.parse().expect("invariante: Cargo.toml válido");
    let name = manifest["package"]["name"]
        .as_str()
        .expect("invariante: package.name")
        .to_string();
    Crate {
        name,
        layer,
        context,
        src: dir.join("src"),
        dir,
        legacy: false,
        manifest,
    }
}

/// Kernel de este repo mientras no exista `kernel/` (RS-1): `crates/shared`.
pub const LEGACY_KERNEL: &str = "crates/shared";
/// Contextos con un solo crate y las capas como módulos: `crates/{ctx}/src/{capa}`.
pub const LEGACY_CONTEXTS: &str = "crates";
const LEGACY_LAYERS: &[(&str, Layer)] = &[
    ("domain", Layer::Domain),
    ("application", Layer::Application),
    ("infrastructure", Layer::Infrastructure),
];

/// Clasifica cada crate del workspace por su ubicación.
///
/// - `kernel/` → Kernel
/// - `edge/*` → Edge
/// - `infrastructure/*` → Infrastructure compartida, sin contexto: adaptadores de
///   las capacidades del kernel (HEX-8). Por HEX-1 solo ve el kernel.
/// - `contexts/{ctx}/{domain|application|infrastructure|api}/` → capa del contexto
/// - `apps/api/` → Api central · otros `apps/*` y `app/` → Bootstrap
///
/// Un contexto con `Cargo.toml` propio (`contexts/{ctx}/Cargo.toml`) es *legacy*:
/// aún no está separado por capas y no se clasifica. `tools/` tampoco.
pub fn crates() -> Vec<Crate> {
    let root = root();
    let kernel_dir = if root.join("kernel/Cargo.toml").exists() {
        root.join("kernel")
    } else {
        root.join(LEGACY_KERNEL)
    };
    let mut crates = vec![load(kernel_dir.clone(), Layer::Kernel, None)];

    // Layout legacy de este repo: una unidad por módulo de capa.
    for package_dir in subdirs(&root.join(LEGACY_CONTEXTS)) {
        if package_dir == kernel_dir || !package_dir.join("Cargo.toml").exists() {
            continue;
        }
        let package = load(package_dir, Layer::Bootstrap, None);
        for (module, layer) in LEGACY_LAYERS {
            let src = package.dir.join("src").join(module);
            if src.is_dir() {
                crates.push(Crate {
                    name: package.name.clone(),
                    layer: *layer,
                    context: Some(package.name.clone()),
                    dir: package.dir.clone(),
                    src,
                    legacy: true,
                    manifest: package.manifest.clone(),
                });
            }
        }
    }

    for edge_dir in subdirs(&root.join("edge")) {
        if edge_dir.join("Cargo.toml").exists() {
            crates.push(load(edge_dir, Layer::Edge, None));
        }
    }

    for shared_dir in subdirs(&root.join("infrastructure")) {
        if shared_dir.join("Cargo.toml").exists() {
            crates.push(load(shared_dir, Layer::Infrastructure, None));
        }
    }

    for context_dir in subdirs(&root.join("contexts")) {
        if context_dir.join("Cargo.toml").exists() {
            continue; // legacy: un solo crate por contexto
        }
        let context = context_dir
            .file_name()
            .expect("invariante: directorio con nombre")
            .to_string_lossy()
            .to_string();
        for layer_dir in subdirs(&context_dir) {
            if !layer_dir.join("Cargo.toml").exists() {
                continue;
            }
            let layer = match layer_dir.file_name().and_then(|n| n.to_str()) {
                Some("domain") => Layer::Domain,
                Some("application") => Layer::Application,
                Some("infrastructure") => Layer::Infrastructure,
                Some("api") => Layer::Api,
                other => panic!("capa desconocida en contexts/{context}: {other:?}"),
            };
            crates.push(load(layer_dir, layer, Some(context.clone())));
        }
    }

    for app_dir in subdirs(&root.join("apps")) {
        if !app_dir.join("Cargo.toml").exists() {
            continue;
        }
        let layer = if app_dir.ends_with("api") {
            Layer::Api
        } else {
            Layer::Bootstrap
        };
        crates.push(load(app_dir, layer, None));
    }

    if root.join("app/Cargo.toml").exists() {
        crates.push(load(root.join("app"), Layer::Bootstrap, None));
    }

    crates
}

pub fn rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return files;
    };
    for path in entries.filter_map(Result::ok).map(|e| e.path()) {
        if path.is_dir() {
            files.extend(rust_files(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            files.push(path);
        }
    }
    files.sort();
    files
}

pub fn parse(path: &Path) -> syn::File {
    let text = fs::read_to_string(path).expect("invariante: archivo legible");
    syn::parse_file(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

pub fn relative(path: &Path) -> String {
    path.strip_prefix(root())
        .unwrap_or(path)
        .display()
        .to_string()
}

/// Variable de entorno que convierte un conjunto vacío en un fallo.
///
/// Un test que no encuentra nada sobre qué actuar es indistinguible de uno que
/// verificó todo. En un proyecto recién creado puede ser legítimo que aún no
/// haya, por ejemplo, `queries/`, así que sin la variable un conjunto vacío
/// pasa. El CI del harness la activa sobre la plantilla, que debe ejercitar
/// cada regla.
pub const REQUIRE_MATERIAL: &str = "ARCH_REQUIRE_MATERIAL";

fn material_required() -> bool {
    std::env::var(REQUIRE_MATERIAL).is_ok_and(|v| v == "1")
}

/// Línea base del trinquete: `tools/architecture/baseline.txt`.
pub const BASELINE: &str = "tools/architecture/baseline.txt";

/// ID de la regla: lo que precede a `:` y a ` / ` (`"HEX-2 / TDD-2: …"` → `HEX-2`).
fn rule_id(rule: &str) -> &str {
    let head = rule.split(':').next().unwrap_or(rule);
    head.split(" / ").next().unwrap_or(head).trim()
}

/// Ruta de una violación: todo mensaje empieza por `ruta:` o `ruta: …`.
fn violation_path(violation: &str) -> &str {
    violation.split(':').next().unwrap_or(violation).trim()
}

/// Entradas `ID ruta` de la sección `[test]` de la línea base, con multiplicidad.
fn baseline_section(test: &str) -> Vec<String> {
    let text = fs::read_to_string(root().join(BASELINE))
        .expect("invariante: tools/architecture/baseline.txt existe");
    let mut current = None;
    let mut entries = Vec::new();
    for line in text.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            current = Some(name.to_string());
        } else if current.as_deref() == Some(test) {
            entries.push(line.split_whitespace().collect::<Vec<_>>().join(" "));
        }
    }
    entries
}

/// Cierra un test de arquitectura con trinquete.
///
/// - `test`: nombre del test; su sección `[test]` en la línea base lista las
///   violaciones conocidas como `ID ruta`, una línea por violación.
/// - `material`: cada conjunto que recorrió la regla, `(qué, cuántos)`. Se
///   informa siempre (visible con `--nocapture`). Con `ARCH_REQUIRE_MATERIAL=1`,
///   un conjunto vacío es un fallo que nombra la regla y lo que faltó.
/// - `violations`: lo que la regla encontró mal. Cada mensaje empieza por la ruta.
///
/// Falla si aparece una violación que no está en la línea base (nueva) o si una
/// entrada de la línea base ya no viola (hay que borrarla): la lista solo se achica.
pub fn assert_no_violations(
    test: &str,
    rule: &str,
    material: &[(&str, usize)],
    violations: Vec<String>,
) {
    let id = rule_id(rule);
    for (what, count) in material {
        eprintln!("{id}: revisó {count} {what}");
    }
    let missing: Vec<&str> = material
        .iter()
        .filter(|(_, count)| *count == 0)
        .map(|(what, _)| *what)
        .collect();
    assert!(
        missing.is_empty() || !material_required(),
        "\n{rule}\nSin material ({REQUIRE_MATERIAL}=1): la regla no encontró sobre qué actuar.\n{}\n",
        missing
            .iter()
            .map(|what| format!("  - 0 {what}"))
            .collect::<Vec<_>>()
            .join("\n")
    );

    let mut known = baseline_section(test);
    let mut new = Vec::new();
    for violation in &violations {
        let key = format!("{id} {}", violation_path(violation));
        match known.iter().position(|k| *k == key) {
            Some(at) => {
                known.remove(at);
            }
            None => new.push(violation.clone()),
        }
    }
    eprintln!(
        "{id}: {} violaciones conocidas en la línea base [{test}]",
        violations.len() - new.len()
    );

    let mut report = Vec::new();
    if !new.is_empty() {
        report.push(format!(
            "Violaciones nuevas (no están en {BASELINE}). Corrígelas; no las agregues a la línea base:"
        ));
        report.extend(new.iter().map(|v| format!("  - {v}")));
    }
    if !known.is_empty() {
        report.push(format!(
            "Ya no violan: borra estas líneas de la sección [{test}] de {BASELINE}:"
        ));
        report.extend(known.iter().map(|k| format!("  - {k}")));
    }
    assert!(report.is_empty(), "\n{rule}\n{}\n", report.join("\n"));
}
