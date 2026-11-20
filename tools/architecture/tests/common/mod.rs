#![allow(dead_code)]

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

/// Marcadores de modelo del kernel (HEX-8): abiertos, sin vocabulario de un contexto.
/// La misma lista está en HEX-8 (`core/principles/hexagonal.md`). Sin marcadores de
/// error: HEX-8 los admite solo si el lenguaje los necesita, y en Rust el `enum`
/// del contrato ya cierra la jerarquía.
pub const MODEL_MARKERS: &[&str] = &[
    "ValueObject",
    "Entity",
    "AggregateRoot",
    "Identifier",
    "DomainEvent",
];

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
    /// Directorio del `Cargo.toml`.
    pub dir: PathBuf,
    /// Raíz del código de la capa. En un crate RS-1 es `dir/src`; en un crate
    /// legacy es el módulo de la capa (`contexts/iam/src/domain`).
    pub src: PathBuf,
    /// Tests de integración de la capa. En un crate legacy no hay (los
    /// `tests/` del crate no pertenecen a una sola capa).
    pub tests: PathBuf,
    pub manifest: toml::Table,
}

impl Crate {
    /// Ruta de un subdirectorio escrito como en un crate RS-1 (`src/ports/inbound`,
    /// `tests`), resuelta contra la raíz real de la capa.
    pub fn path(&self, sub: &str) -> PathBuf {
        let mut parts = sub.splitn(2, '/');
        let (head, rest) = (parts.next().unwrap_or_default(), parts.next());
        let base = match head {
            "src" => &self.src,
            "tests" => &self.tests,
            _ => return self.dir.join(sub),
        };
        rest.map_or_else(|| base.clone(), |rest| base.join(rest))
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
        tests: dir.join("tests"),
        dir,
        manifest,
    }
}

/// Capas de un crate legacy (un solo crate con las capas como módulos).
///
/// - `src/{domain,application,infrastructure}` → la capa del mismo nombre.
/// - `src/infrastructure/http` → Api: es el adaptador driving. Sus archivos
///   también cuentan como Infrastructure, porque viven dentro de ese módulo.
///
/// Todas comparten el manifiesto del crate: lo que declara está disponible en
/// cada capa, que es justo lo que HEX-1/HEX-2 tienen que ver.
fn legacy_layers(dir: &Path, context: Option<String>) -> Vec<Crate> {
    let whole = load(dir.to_path_buf(), Layer::Bootstrap, context);
    let modules = [
        ("domain", Layer::Domain),
        ("application", Layer::Application),
        ("infrastructure", Layer::Infrastructure),
        ("infrastructure/http", Layer::Api),
    ];
    modules
        .into_iter()
        .filter(|(module, _)| whole.src.join(module).is_dir())
        .map(|(module, layer)| Crate {
            name: whole.name.clone(),
            layer,
            context: whole.context.clone(),
            dir: whole.dir.clone(),
            src: whole.src.join(module),
            tests: whole.dir.join("tests/__sin_capa__"),
            manifest: whole.manifest.clone(),
        })
        .collect()
}

/// Clasifica cada crate del workspace por su ubicación.
///
/// - `kernel/` → Kernel
/// - `edge/*` → Edge
/// - `infrastructure/*` → Infrastructure compartida, sin contexto: adaptadores de
///   las capacidades del kernel (HEX-8). Por HEX-1 solo ve el kernel.
/// - `contexts/{ctx}/{domain|application|infrastructure|api}/` → capa del contexto
/// - `apps/api/` → Api central · otros `apps/*` y `app/` → Bootstrap
///
/// Adaptación de este repo (aún no migrado a RS-1):
///
/// - Sin `kernel/`, `shared/` es el kernel (así lo audita `docs/harness-adoption.md`).
/// - Un contexto con `Cargo.toml` propio (`contexts/{ctx}/Cargo.toml`) es *legacy*:
///   cada módulo de capa se clasifica como un crate virtual (ver `legacy_layers`).
///
/// `tools/` no se clasifica.
pub fn crates() -> Vec<Crate> {
    let root = root();
    let kernel = if root.join("kernel/Cargo.toml").exists() {
        root.join("kernel")
    } else {
        root.join("shared")
    };
    let mut crates = vec![load(kernel, Layer::Kernel, None)];

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
        let context = context_dir
            .file_name()
            .expect("invariante: directorio con nombre")
            .to_string_lossy()
            .to_string();
        if context_dir.join("Cargo.toml").exists() {
            crates.extend(legacy_layers(&context_dir, Some(context)));
            continue;
        }
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

/// Línea base de violaciones conocidas: `tools/architecture/baseline.txt`.
pub const BASELINE: &str = "baseline.txt";

/// Ruta de una violación: lo que va antes del primer `:` del mensaje.
fn location(violation: &str) -> &str {
    violation.split(':').next().unwrap_or(violation).trim()
}

/// Rutas de la línea base registradas para `id`.
fn baseline_for(id: &str) -> BTreeSet<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(BASELINE);
    let text = fs::read_to_string(&path).unwrap_or_default();
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter_map(|l| l.split_once(char::is_whitespace))
        .filter(|(entry, _)| *entry == id)
        .map(|(_, ruta)| ruta.trim().to_string())
        .collect()
}

/// Cierra un test de arquitectura con trinquete sobre la línea base.
///
/// - `id`: identificador del test (el nombre de la función), primera columna
///   de `baseline.txt`.
/// - `material`: cada conjunto que recorrió la regla, `(qué, cuántos)`. Se
///   informa siempre (visible con `--nocapture`). Con `ARCH_REQUIRE_MATERIAL=1`,
///   un conjunto vacío es un fallo que nombra la regla y lo que faltó.
/// - `violations`: lo que la regla encontró mal. Cada una empieza por su ruta.
///
/// Falla si aparece una violación en una ruta que no está en la línea base, o
/// si una entrada de la línea base ya no viola (hay que borrarla). Así la
/// línea base solo puede achicarse.
pub fn assert_no_violations(
    id: &str,
    rule: &str,
    material: &[(&str, usize)],
    violations: Vec<String>,
) {
    let rule_id = rule.split(':').next().unwrap_or(rule);
    for (what, count) in material {
        eprintln!("{rule_id}: revisó {count} {what}");
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

    let baseline = baseline_for(id);
    let current: BTreeSet<String> = violations.iter().map(|v| location(v).to_string()).collect();
    let new: Vec<&String> = violations
        .iter()
        .filter(|v| !baseline.contains(location(v)))
        .collect();
    let fixed: Vec<&String> = baseline.difference(&current).collect();
    let tolerated = violations.len() - new.len();
    if tolerated > 0 {
        eprintln!("{rule_id}: {tolerated} violaciones toleradas por {BASELINE}");
    }

    let mut report = String::new();
    if !new.is_empty() {
        report.push_str(&format!(
            "\nViolaciones nuevas (corregirlas; {BASELINE} no se agranda):\n{}\n",
            new.iter()
                .map(|v| format!("  - {v}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
    if !fixed.is_empty() {
        report.push_str(&format!(
            "\nYa no violan: borrar de {BASELINE}:\n{}\n",
            fixed
                .iter()
                .map(|ruta| format!("  {id} {ruta}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
    assert!(report.is_empty(), "\n{rule}\n{report}");
}
