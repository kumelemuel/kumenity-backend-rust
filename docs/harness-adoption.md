# Adopción del harness craft-backend — estado persistente

| Campo | Valor |
|---|---|
| Última auditoría | 2026-11-18 |
| Rama / commit auditado | `test/characterization-safety-net` @ `e473da7` |
| Plugin | `craft-backend` 0.10.0 |
| Generado por | agente `craft-backend:architecture-reviewer`, en solo lectura |
| Red de seguridad | tests de caracterización en `apps/api/tests/` (38) y `list_public_communities.rs` (4), commit `d910100` |

**Límites de la auditoría.**
- El reviewer no ejecutó `cargo`: todas las líneas se verificaron leyendo los archivos.
- No leyó `.env`, porque se le denegó el acceso.
- No hay `tools/architecture`, así que ninguna regla se comprobó con tests automáticos.

## Cómo usar este documento

- Cada hallazgo es un checklist: `[ ]` significa pendiente y `[x]` resuelto. Al resolver uno, añade el hash del commit al final de la línea: `— resuelto en abc1234`.
- Los números de línea corresponden al commit auditado. Tras un refactor pueden moverse: identifica cada hallazgo por su descripción, no por la línea.
- Al volver a auditar, actualiza la cabecera (fecha y commit) y la columna *Pendientes* del resumen.
- La red de caracterización debe seguir en verde durante toda la migración (`cargo test --workspace`).
- Algunos tests fijan defectos a propósito y cambiarán cuando estos se corrijan. Cada cambio de esos tests se justifica en el commit; nunca se ajustan solo para que pasen (TDD-1). Son estos:
  - `apps/api/tests/communities_api.rs`: un `sub` inválido da 400 con un código de IAM, y no existe la ruta de listado.
  - `apps/api/tests/auth_api.rs`: sign-in de un usuario inexistente da 404, lo que revela qué cuentas existen.
  - Los asserts sobre `{code, message}` cambiarán con Problem Details.

## Trinquete de arquitectura (`tools/architecture`)

Los tests de arquitectura vienen de `ai-harness/stacks/rust/template/tools/architecture` (stack v0.2). Se ejecutan con `cargo test -p architecture`, y también desde `cargo test --workspace` y los hooks de craft-backend. Las violaciones que ya existían están en `tools/architecture/baseline.txt` (74 líneas `ID ruta`, una sección por test).

- **Cuándo falla.** Si aparece una violación que no está en la línea base, hay que corregirla, no agregarla. Si una línea de la línea base ya no viola, hay que borrarla, y el test indica cuál. Así la lista solo puede achicarse.
- **Cómo se compara.** La clave es `ID ruta`, sin número de línea, así que mover código dentro de un archivo no rompe nada. Una ruta aparece repetida si tiene varias violaciones de la misma regla.
- **Adaptación al layout legacy** (`tests/common/mod.rs`):
  - `crates/shared` cuenta como kernel.
  - Cada `crates/{ctx}/src/{domain,application,infrastructure}` cuenta como una capa del contexto.
  - Las violaciones de Cargo que afectan a un contexto legacy se atribuyen a su `Cargo.toml` una sola vez.
- **Test añadido.** `hex_1_legacy_modules_point_inward` comprueba HEX-1 a partir de las rutas del código (`use`, y también rutas sueltas), porque Cargo no distingue los módulos de un mismo crate. Reporta una violación por archivo y destino.
- **Migración a crates por capa (RS-1).** Al migrar un contexto a `contexts/{ctx}/{capa}`, la plantilla lo clasifica sola y las entradas de `crates/{ctx}` desaparecen. Las violaciones que persistan en otra ruta aparecerán como nuevas: corregirlas o moverlas explícitamente en el mismo commit.

**Qué no automatiza todavía.** Estos hallazgos del inventario solo se pueden revisar a mano:
- HEX-4: implementaciones `*UseCase` en `use_cases/`. La plantilla solo mira `commands/` y `queries/`.
- HEX-4: que el contrato exponga el dominio en crates legacy, porque la plantilla necesita un crate `{ctx}-domain`.
- RS-2: `unwrap`, `todo!` y `expect`. Lo cubrirá `[workspace.lints]`.
- RS-3, ERR-2 y ERR-3: firmas de error.
- ERR-4: mapeo exhaustivo, Problem Details y `.env`.
- TDD-2: baterías de contrato.
- RS-1: dependencias entre contextos declaradas en Cargo.

**Diferencias con la auditoría manual.** El test encontró dos traits más en el kernel que violan HEX-8: `DomainError` (`crates/shared/src/domain/domain_error.rs`) e `InfrastructureError` (`crates/shared/src/infrastructure/infrastructure_error.rs`). También clasifica como HEX-1 los `use` de infraestructura del composition root (`apps/api/src/lib.rs` y `state/*.rs`), porque la plantilla espera el composition root en un `apps/server` aparte.

## Resumen

| ID | Regla | Violaciones | Pendientes |
|----|-------|---:|---:|
| HEX-1 | Dependencias hacia adentro; `api` solo ve `application` | 3 | 3 |
| HEX-2 | Núcleo libre de framework y de librerías | 3 | 3 |
| HEX-3 | Los puertos viven en `application/ports` | 1 | 1 |
| HEX-4 | Un `{Verb}{Noun}UseCase` + `Handler` por caso de uso | 4 | 4 |
| HEX-5 | Puertos salientes por rol; adaptadores `{Tec}{Concepto}Adapter` | 3 | 3 |
| HEX-6 | Vocabulario sin colisiones (`{Caso}HttpRequest`…) | 2 | 2 |
| HEX-7 | El dominio decide, la aplicación orquesta | 4 | 4 |
| HEX-8 | Kernel delgado | 3 | 3 |
| VER-1 | Lo verificable se expresa como test | 1 | 1 |
| ERR-1 | Dos árboles sin jerarquía común | 7 | 7 |
| ERR-2 | Contrato de error acotado por caso de uso | 6 | 6 |
| ERR-3 | Semántica de adaptadores | 2 | 2 |
| ERR-4 | Transporte y logging solo en el borde; sin datos sensibles | 7 | 7 |
| TDD-1 | Test antes que producción | 0 | 0 |
| TDD-2 | Estrategia por capa, dobles y baterías de contrato | 6 | 6 |
| RS-1 | Un crate por capa, sin dependencias entre contextos | 4 | 4 |
| RS-2 | `Fault`/`Failure`; `panic!` solo para invariantes; sin `unwrap` | 7 | 7 |
| RS-3 | Puertos async con `BoxFuture` | 1 | 1 |
| RS-4 | `cargo test -p architecture` | 1 | 0 |
| **Total** | | **65** | **64** |

## Arreglos inmediatos

Se pueden hacer ya, porque no dependen de la reestructuración:

- [ ] **ERR-4**: `.env` está versionado (lo está desde `6a29cf7`) y `.gitignore` no lo excluye. Hay que añadirlo a `.gitignore`, sacarlo del índice, **rotar `JWT_SECRET`** y versionar solo un `.env.example` sin valores. El secreto ya forma parte del historial de git.
- [ ] **ERR-4**: `crates/iam/src/application/use_cases/register_account.rs:64` contiene un `dbg!(&account)` que vuelca a stderr el hash de la contraseña, el email y el código de verificación.
- [ ] **RS-2**: `crates/communities/src/infrastructure/persistence/in_memory/community_repository.rs:27` es un `todo!()` en un adaptador de producción. Hay que implementarlo con su test.
- [ ] **RS-2**: `crates/membership/src/application/use_cases/create_community.rs:55` hace `unwrap` de `AccountId::from_str`, así que un `sub` inválido provoca un panic.
- [ ] **RS-2**: `apps/api/src/config/jwt.rs:14` hace `unwrap` al parsear `JWT_EXPIRATION_TIME`.

## Inventario

Formato de cada línea: `ID · ruta:línea · problema · severidad — corrección`.

### HEX-1 — Dependencias hacia adentro
- [ ] HEX-1 · `apps/api/src/authentication/token_validator.rs:2` · el adaptador driving importa `iam::infrastructure` (`JwtTokenGenerator`, `JwtError`) · alta — exponer la validación de tokens como un puerto o caso de uso de `iam::application`; el composition root inyecta la implementación.
- [ ] HEX-1 · `apps/api/src/http/communities/errors/error_mapper.rs:11` · `api` importa `communities::domain::errors::error_codes` (y en la línea 15, `iam::domain`) · alta — hacer `match` sobre el enum del contrato de `application`, sin códigos de dominio.
- [ ] HEX-1 · `apps/api/src/http/iam/errors/error_mapper.rs:12` · `api` importa `iam::domain::errors::error_codes` · alta — igual que el anterior.

### HEX-2 — Núcleo libre de librerías
- [ ] HEX-2 · `crates/iam/src/domain/value_objects/account_id.rs:2` · `uuid` en el dominio (`generate()` en la línea 8). Se repite en `crates/communities/src/domain/value_objects/community_id.rs:2` y `crates/membership/src/domain/value_objects/community_id.rs:2` · media — el id envuelve un tipo estándar y se genera con `kernel::IdGenerator`, cuyo adaptador `UuidIdGeneratorAdapter` vive en infraestructura.
- [ ] HEX-2 · `crates/iam/src/domain/value_objects/code_validation.rs:2` · `rand` en el dominio; `generate()` (líneas 19-21) no es determinista · media — inyectar un puerto generador de códigos desde `application`, o recibir el código ya generado.
- [ ] HEX-2 · `crates/iam/Cargo.toml:8` · `argon2`, `rand`, `jsonwebtoken` y `serde` (líneas 8-11) están en el mismo crate que `domain` y `application`, así que el núcleo puede importarlos · media — se resuelve con RS-1: esas dependencias van solo en el crate de infraestructura de iam.

### HEX-3 — Puertos en `application/ports`
- [ ] HEX-3 · `apps/api/src/authentication/token_validator.rs:7` · `trait TokenValidator` es un puerto declarado en el borde · baja — moverlo a `application/ports` de iam, o reemplazarlo por un caso de uso de iam.

### HEX-4 — Un caso de uso: contrato + handler
- [ ] HEX-4 · `crates/iam/src/application/ports/inbound/account_registration.rs:6` · los puertos de entrada se llaman `*Port`. También `account_authentication.rs:7` y `communities/.../community_creation.rs:6`; en total 7 traits en `crates/*/src/application/ports/inbound/*.rs` · media — renombrar a `RegisterAccountUseCase`, `CreateCommunityUseCase`, etc.
- [ ] HEX-4 · `crates/iam/src/application/use_cases/register_account.rs:22` · la implementación se llama `*UseCase` en vez de `*Handler`. También `communities/.../create_community.rs:22`; en total 7 tipos en `crates/*/src/application/use_cases/*.rs` · media — renombrar a `RegisterAccountHandler`, etc.
- [ ] HEX-4 · `crates/iam/src/application/commands/register_account.rs:1` · la disposición separa `commands/`, `results/`, `errors/` y `use_cases/`, y los DTOs no llevan sufijo (`RegisterAccount`; `AccountRegistered` en `results/account_registered.rs:2`) · media — usar `commands/{caso}/{command,response,error,handler}.rs` con `RegisterAccountCommand` y `RegisterAccountResponse`.
- [ ] HEX-4 · `crates/communities/src/application/commands/list_public_communities.rs:1` · las lecturas están en `commands/`; también `crates/iam/src/application/commands/identify_account.rs:1` · baja — moverlas a `queries/{caso}/` como `...Query`.

### HEX-5 — Puertos por rol y nombres de adaptadores
- [ ] HEX-5 · `crates/iam/src/application/ports/outbound/account_repository.rs:5` · los puertos salientes llevan sufijo `Port`: también `password_hasher.rs:3`, `token_generator.rs:3`, `communities/.../community_repository.rs:6`, y en `membership/...` `community_repository.rs:3` y `membership_repository.rs:3` · media — `AccountRepository` y `CommunityRepository`; para el hasher y el generador de tokens, un sufijo de rol (p. ej. `PasswordHashingService`).
- [ ] HEX-5 · `crates/iam/src/infrastructure/persistence/in_memory/account_repository.rs:13` · adaptadores sin la forma `{Tec}{Concepto}Adapter`: `InMemoryAccountRepository`, `Argon2PasswordHasher` (`argon2_password_hasher.rs:11`), `JwtTokenGenerator` (`jwt_token_generator.rs:11`) y los `InMemoryCommunityRepository` (`communities/...:13`, `membership/...:10`) · media — `InMemoryAccountAdapter`, `InMemoryCommunityAdapter`, `Argon2PasswordHashingAdapter`, `JwtTokenAdapter`.
- [ ] HEX-5 · `apps/api/src/authentication/token_validator.rs:11` · adaptador de producción (`JwtValidator`) fuera de infraestructura y sin sufijo `Adapter` · media — moverlo a la infraestructura de iam como `JwtTokenValidationAdapter`.

### HEX-6 — Vocabulario sin colisiones
- [ ] HEX-6 · `apps/api/src/http/communities/requests/create.rs:5` · los DTOs de transporte no indican el transporte en el nombre: `CreateRequest`, `CreatedResponse` (`responses/created.rs:5`), `SignUpRequest`, `SignInRequest`, `VerifyRequest`, `IdentifyRequest` y las respuestas de iam; en total 9 tipos en `apps/api/src/http/*/{requests,responses}/*.rs` · baja — `CreateCommunityHttpRequest`, `CreateCommunityHttpResponse`, `RegisterAccountHttpRequest`, etc.
- [ ] HEX-6 · `apps/api/src/http/iam/handlers/verify.rs:22` · serializa tal cual la respuesta del caso de uso (`bool`) · baja — definir `VerifyAccountHttpResponse`.

### HEX-7 — El dominio decide
- [ ] HEX-7 · `crates/iam/src/application/policies/password_policy.rs:8` · regla de negocio (longitud mínima de contraseña) en `application`, invocada desde el handler en `register_account.rs:53` · media — un value object de dominio (p. ej. `PlainPassword::new -> Result`) con su test puro.
- [ ] HEX-7 · `crates/membership/src/domain/entities/membership.rs:11` · campos `pub` (`role`, `nickname`, `status`, líneas 11-13) que permiten saltarse las invariantes de rol y estado · media — campos privados y solo métodos.
- [ ] HEX-7 · `crates/iam/src/domain/aggregates/account.rs:73` · agregado mutable: `&mut self` en `change_password`, `change_email`, `change_username`, `deactivate`, `activate`, `suspend` y `confirm_registration` (líneas 73-108). Igual en `crates/membership/src/domain/aggregates/community.rs:69` · baja — métodos que devuelven `Result<Self, E>`.
- [ ] HEX-7 · `crates/communities/src/domain/aggregates/community.rs:12` · agregado anémico: `_owner_id` nunca se lee y no hay comportamiento · baja — que las reglas de propiedad vivan en el agregado cuando existan.

### HEX-8 — Kernel delgado
- [ ] HEX-8 · `crates/shared/src/error.rs:16` · el kernel define una jerarquía cerrada clasificada por capa (`SystemError`, `LayerError`, `ErrorCategory`) en vez de las primitivas `Fault` y `Failure<E>` · alta — el kernel expone `Fault`, `Failure<E>`, `BoxFuture`, marcadores e `IdGenerator`; los errores pasan a cada contexto.
- [ ] HEX-8 · `crates/shared/src/application/common_application_error.rs:2` · error compartido con vocabulario de negocio (`Unauthorized`) · media — eliminarlo; las hojas van en el `application/errors` de cada contexto.
- [ ] HEX-8 · `crates/shared/src/application/auth_context.rs:2` · la identidad autenticada del borde vive en el kernel · baja — moverla a un edge http compartido (`AuthenticatedAccount`).

### VER-1 — Lo verificable se expresa como test
- [ ] VER-1 · `Cargo.toml:2` · no hay tests de arquitectura ni CI (no existen `tools/` ni `.github/`), así que ninguna regla HEX, ERR o RS está automatizada · alta — copiar `tools/architecture` de la plantilla rust-stack (`hex_1`, `hex_2_core`, `hex_4`, `hex_5`, `hex_6`, `hex_8`, `err_1` y `err_4`; este último habría detectado el `dbg!`). — *parcial: tests de arquitectura con trinquete añadidos (ver arriba); falta CI.*

### ERR-1 — Dos árboles sin jerarquía común
- [ ] ERR-1 · `crates/shared/src/error.rs:16` · un único tipo para los dos árboles (`Domain`/`Application` frente a `Infrastructure`); el `From` genérico de la línea 32 hace que `?` absorba cualquier error · alta — `Failure<E> { Expected(E), Unexpected(Fault) }`.
- [ ] ERR-1 · `crates/iam/src/domain/errors/account.rs:27` · los errores de dominio y de `application` implementan `std::error::Error`. También `communities/.../invalid_community_slug.rs:14` e `iam/.../authenticate_account.rs:24`; en total 17 impls en `crates/{iam,communities}/src/{domain,application}/errors/*.rs` · alta — dejar solo `Debug + Display`.
- [ ] ERR-1 · `crates/iam/src/application/errors/account_repository.rs:17` · un fallo técnico del repositorio se tipa como categoría `Application` dentro del mismo árbol (igual `token_generator.rs:17` y `communities/.../community_repository.rs:17`) · media — el puerto devuelve `Result<_, Fault>`.
- [ ] ERR-1 · `crates/iam/src/application/ports/outbound/account_repository.rs:6` · lecturas que no pueden fallar en la firma: `find_* -> Option`, `get_public_list -> Vec` (`communities/.../community_repository.rs:7`), `hash` y `verify` (`password_hasher.rs:4-5`). Obligan al adaptador a hacer panic · media — `Result<Option<T>, Fault>`.
- [ ] ERR-1 · `crates/membership/src/application/use_cases/create_community.rs:59` · un fallo técnico se convierte en la variante esperada `CommonApplicationError::Infrastructure` y se descarta la causa · media — propagarlo como `Fault`.
- [ ] ERR-1 · `crates/communities/src/application/ports/inbound/public_communities_listing.rs:12` · caso de uso sin árbol A que aun así tiene tipo de error (`SystemError`) · baja — `Result<Response, Fault>`.
- [ ] ERR-1 · `crates/iam/src/domain/aggregates/account.rs:114` · `let _ =` descarta en silencio el `Result` de `transition_status` · baja — propagarlo o documentar el invariante con `expect("invariante: …")`.

### ERR-2 — Contrato de error acotado por caso de uso
- [ ] ERR-2 · `crates/iam/src/application/ports/inbound/account_registration.rs:7` · todas las firmas usan el error global `SystemError` y no existe ningún `{Verb}{Noun}Error`. También `account_authentication.rs:8`, `account_verification.rs:5`, `account_identification.rs:7` y `communities/.../community_creation.rs:11` · alta — un enum cerrado por caso de uso junto a su handler, con hojas en `application/errors`.
- [ ] ERR-2 · `crates/shared/src/error.rs:17` · los errores se envuelven (`Domain(Box<dyn LayerError>)`) y los handlers propagan el error de dominio con `?` sin traducirlo a una hoja (`register_account.rs:56-57`; en communities, `create_community.rs:40`, `:47` y `:49`) · alta — `.map_err(|_| Failure::expected(Validation { field: … }))`.
- [ ] ERR-2 · `crates/communities/src/application/use_cases/create_community.rs:47` · un `sub` inválido en el token aparece como `IAM_INVALID_ACCOUNT_ID_FORMAT` con 400: código de otro contexto y semántica de autenticación · media — el borde rechaza con 401 al construir la identidad, y el caso de uso recibe un id ya válido.
- [ ] ERR-2 · `crates/membership/src/application/errors/application_error.rs:4` · error global del contexto que envuelve `Common(...)` (línea 5) e incluye una variante imposible, `Common(Unauthorized)` · media — `CreateCommunityError` acotado.
- [ ] ERR-2 · `crates/communities/src/domain/errors/community_error.rs:10` · ningún caso de uso puede producir las 6 variantes de `CommunityError`, y el mapper de iam traduce códigos que nunca llegan (`error_mapper.rs:50`, `:53`) · baja — eliminarlas hasta que exista el caso de uso.
- [ ] ERR-2 · `crates/iam/src/application/use_cases/register_account.rs:131` · faltan tests por variante (ver la tabla) y los existentes afirman sobre `err.code()` (un string), no sobre la variante · media — `matches!(r, Err(Failure::Expected(Variante(..))))`.

Tests por variante. Son pseudo-variantes, identificadas por código, porque aún no hay contratos tipados:

| Caso | Variante | Test del handler | Test del endpoint |
|---|---|---|---|
| RegisterAccount | `PASSWORD_TOO_SHORT` | **falta** `fails_when_password_too_short` | ok |
| RegisterAccount | `ACCOUNT_REPOSITORY_ERROR` | ok | **falta** (500) |
| AuthenticateAccount | `TOKEN_GENERATOR_ERROR` | **falta** `fails_when_token_generation_fails` | **falta** |
| VerifyAccount | `INVALID_CODE_VALIDATION` | **falta** `fails_when_code_out_of_range` | ok |
| VerifyAccount | `ACCOUNT_REPOSITORY_ERROR` | **falta** `fails_when_repository_fails` | **falta** |
| CreateCommunity | `INVALID_ACCOUNT_ID(_FORMAT)` | **falta** `fails_when_owner_id_is_invalid` | ok (solo `_FORMAT`) |
| CreateCommunity | `COMMUNITIES_REPOSITORY_ERROR` | ok | **falta** |
| Resto (email o username existentes, NotFound, LoginFailed, CannotAuthenticate, slug o nombre inválidos, slug existente) | — | ok | ok |

### ERR-3 — Semántica de adaptadores
- [ ] ERR-3 · `crates/iam/src/application/errors/token_generator.rs:25` · el error que devuelve el adaptador ya viene pensado para el transporte, con `message()` y código para el cliente (igual `account_repository.rs:25` y `communities/.../community_repository.rs:25`); `JwtTokenGenerator` lo construye en `jwt_token_generator.rs:55` y `:67` · media — `Fault` opaco; el borde decide la respuesta genérica.
- [ ] ERR-3 · `crates/membership/src/application/ports/outbound/community_repository.rs:7` · firma de error no declarada (`Result<(), String>`; también `membership_repository.rs:4`) · media — `Result<(), Fault>`, o un `{Contrato}Error` si hay fallos declarados.

### ERR-4 — Transporte y logging solo en el borde
- [ ] ERR-4 · `crates/iam/src/application/use_cases/register_account.rs:64` · `dbg!(&account)` vuelca datos sensibles a stderr (`Account` deriva `Debug` en `account.rs:6`) · alta — eliminarlo. *(Repetido en Arreglos inmediatos.)*
- [ ] ERR-4 · `apps/api/src/http/communities/errors/error_mapper.rs:38` · mapeo por string con fallback `_ => 500` en la línea 46, no exhaustivo (igual en `iam/errors/error_mapper.rs:41` y `:60`) · alta — `match` exhaustivo sobre el enum del contrato, con un helper por hoja.
- [ ] ERR-4 · `.gitignore:1` · `.env` está versionado y no se ignora; puede contener `JWT_SECRET` · alta — ver Arreglos inmediatos.
- [ ] ERR-4 · `crates/iam/src/domain/errors/error_codes.rs:1` · los códigos estables y los mensajes de usuario los define el núcleo (también `iam/application/errors/error_codes.rs:1`, `communities/{domain,application}/errors/error_codes.rs:1`, `membership/.../membership_error.rs:30` y `LayerError::code/message` en `shared/src/error.rs:11-12`) · media — mover los códigos a `apps/api` (`codes.rs`); el núcleo solo tiene variantes.
- [ ] ERR-4 · `apps/api/src/http/common/errors/api_error_response.rs:4` · formato propio `{code, message}` en vez de RFC 9457; el 401 de `create.rs:24` va con cuerpo vacío · media — Problem Details con extensión `code`.
- [ ] ERR-4 · `apps/api/src/lib.rs:34` · no hay manejador global, `FaultReporter` ni `CatchPanicLayer` (líneas 34-37): los fallos técnicos no se registran y un panic (`todo!`, `expect`) corta la conexión sin respuesta · media — rama `Unexpected → unexpected()` con 500 genérico y reporte, más `CatchPanicLayer`.
- [ ] ERR-4 · `apps/api/src/http/iam/requests/sign_in.rs:4` · DTOs con la contraseña en claro que derivan `Debug` (también `sign_up.rs:4`) · baja — quitar `Debug` o redactar el campo.

### TDD-1 — Test antes que producción
Sin violaciones en `main..HEAD`.
- `d910100` solo añade tests. La extracción de `build_app` es un refactor del composition root sin cambio de comportamiento, cubierto por los tests de endpoint del mismo commit.
- `e473da7` es solo formato.

### TDD-2 — Estrategia por capa, dobles y baterías de contrato
- [ ] TDD-2 · `crates/iam/src/application/ports/outbound/account_repository.rs:22` · el prefijo `Fake` no es cierto: `FakeAccountRepository` no guarda estado (`save` descarta en la línea 97), así que es un Stub. Igual `FakeCommunityRepository` (`communities/...:24`, `membership/...:17`), `FakePasswordHasher` (`password_hasher.rs:15`) y `FakeTokenGenerator` (`token_generator.rs:14`, que es un Fixed) · media — un Fake con `HashMap` y estado real, o renombrar a `Stub`/`Fixed`.
- [ ] TDD-2 · `crates/communities/src/application/use_cases/list_public_communities.rs:63` · los dobles no siguen `{Prefijo}{Concepto}Adapter`: `SpyCommunityRepositoryAdapter` repite el rol, y los cinco del hallazgo anterior no llevan `Adapter` · baja — `SpyCommunityAdapter`, `FakeAccountAdapter`, etc.
- [ ] TDD-2 · `crates/communities/src/application/ports/outbound/community_repository.rs:80` · doble incompleto (`get_public_list` es `todo!()`) y con constructores muertos copiados (`with_existing_id` en `:59`, `active_with_existing_username` en `:68`, campo `_activated`) · media — implementarlo y borrar lo que sobra.
- [ ] TDD-2 · `crates/iam/src/application/ports/outbound/account_repository.rs:5` · ninguno de los 6 puertos salientes tiene batería de contrato, así que el doble y el adaptador divergen sin aviso (el Spy devuelve una lista; el adaptador en memoria hace panic) · alta — `testing/contracts/{puerto}.rs` con su macro, invocada contra el doble y contra cada adaptador.
- [ ] TDD-2 · `crates/iam/src/infrastructure/security/password_hasher/argon2_password_hasher.rs:11` · adaptadores sin test propio: Argon2, `JwtTokenGenerator`, `InMemoryAccount`, `InMemoryCommunity` y `JwtValidator` · media — la batería de contrato más los tests propios de cada tecnología.
- [ ] TDD-2 · `apps/api/tests/communities_api.rs:208` · reglas de negocio probadas desde el endpoint contra la app completa (trim en `:208`, orden de validación en `:287`; `auth_api.rs:88` y `:95`) en vez de con un UseCase stub · baja — se justifican como red de caracterización; cuando haya tests del handler, el endpoint se reduce a la traducción de cada variante.

### RS-1 — Un crate por capa, sin dependencias entre contextos
- [ ] RS-1 · `crates/iam/src/lib.rs:1` · un crate por contexto con las capas como módulos (igual en communities y membership); al no existir crates `{ctx}-{capa}`, Cargo no puede hacer cumplir HEX-1, 2 ni 3 · alta — `kernel/` y `contexts/{ctx}/{domain,application,infrastructure,api}`, migrando un contexto a la vez.
- [ ] RS-1 · `crates/communities/Cargo.toml:10` · dependencia entre contextos communities → iam (usada en `domain/aggregates/community.rs:7` y `use_cases/create_community.rs:18`); membership depende de iam y communities en `Cargo.toml:10-11`, y communities ni siquiera se usa · alta — un `OwnerId` propio en cada contexto.
- [ ] RS-1 · `contexts/iam` · directorio obsoleto que solo contiene `target/`; además `crates/iam/Cargo.lock` es un lockfile suelto y versionado · baja — borrar ambos.
- [ ] RS-1 · `Cargo.toml:8` · las versiones externas no se declaran en `[workspace.dependencies]` (`uuid` aparece en 3 manifiestos) · baja — centralizarlas.

### RS-2 — `Fault`/`Failure`, panics solo en invariantes, sin `unwrap`
- [ ] RS-2 · `crates/communities/src/infrastructure/persistence/in_memory/community_repository.rs:27` · `todo!()` en un adaptador de producción · alta — implementarlo, con test.
- [ ] RS-2 · `crates/membership/src/application/use_cases/create_community.rs:55` · `unwrap` de `AccountId::from_str` (línea 49): un `sub` inválido provoca panic · alta — traducirlo a una hoja, o validar en el borde.
- [ ] RS-2 · `apps/api/src/config/jwt.rs:14` · `unwrap` al parsear `JWT_EXPIRATION_TIME` · media — devolver `ConfigError::Invalid`.
- [ ] RS-2 · `crates/iam/src/infrastructure/security/password_hasher/argon2_password_hasher.rs:39` · `expect` sobre un hash persistido corrupto y sobre un fallo técnico de hashing (`:30`), que deberían ser `Fault` · media — `Fault::new("hash persistido inválido…")` a mano.
- [ ] RS-2 · `Cargo.toml:1` · no hay `[workspace.lints]` con `unwrap_used` ni `unsafe` en deny · media — añadir los lints del workspace y `[lints] workspace = true` en cada crate.
- [ ] RS-2 · `apps/api/src/authentication/bearer.rs:17` · `unwrap` sobre un invariante (también `main.rs:13` y `:15`, y `authenticate_account.rs:51`, `identify_account.rs:43` y `verify_account.rs:30`, todos tras un `is_some`) · baja — `expect("invariante: …")`, o `match`/`ok_or`.
- [ ] RS-2 · `crates/iam/src/infrastructure/persistence/in_memory/account_repository.rs:27` · `expect` sin el prefijo `"invariante:"` (`:27`, `:35` y `:43`; en communities `:31`, `:39` y `:47`; en membership `:24`, `:32` y `:40`; en argon2 `:30` y `:34`) · baja — `expect("invariante: mutex no envenenado")`.

### RS-3 — Puertos async
- [ ] RS-3 · `crates/iam/src/application/ports/outbound/account_repository.rs:5` · todos los puertos son síncronos y se llaman desde handlers axum async (`create.rs:27-31`), lo que bloqueará en cuanto haya E/S real · media — `BoxFuture<'a, Result<_, Fault>>`.

### RS-4 — Tests de arquitectura
- [x] RS-4 · `Cargo.toml:2` · `members` no incluye `tools/architecture` · alta — ver VER-1. — resuelto en el commit del trinquete.

## Lo que ya cumple (no tocar)

- El dominio no declara puertos ni importa `application` o `infrastructure`, y `application` no importa `infrastructure` (HEX-3, y HEX-1 dentro de cada crate).
- Los `Command` y `Result` de los casos de uso usan solo tipos primitivos; el contrato no expone tipos de dominio (HEX-4).
- `api` inyecta `Arc<dyn …Port>`, nunca el handler concreto, y el composition root (`build_app` y `state/*.rs`) expone contratos (HEX-2, HEX-4).
- Los tests de caso de uso no levantan el framework, usan dobles en lugar de mocks de librería y afirman sobre `Result` (TDD-2, ERR-1).
- `find_*` devuelve `Option` y es el handler quien decide entre NotFound y Conflict (ERR-1).
- `SpyCommunityRepositoryAdapter` sí registra llamadas. Los dobles no viven en `infrastructure` y los adaptadores en memoria sí (TDD-2, HEX-5).
- Los value objects validan con `Result` y tienen tests puros.
- Los errores de dominio de `membership` no implementan `std::error::Error`.
- `shared` no declara dependencias, no se usa `async-trait` y los códigos HTTP solo aparecen en `apps/api`.

## Orden de corrección y dependencias

1. **Arreglos inmediatos**, independientes de todo lo demás (ver la sección del mismo nombre).
2. **Mantener verde la red de caracterización** durante toda la migración y justificar cada cambio en sus tests (TDD-1).
3. **Kernel antes que contratos.** Introducir `Fault`, `Failure<E>`, `BoxFuture` e `IdGenerator` junto a `SystemError`, y borrar `SystemError` solo cuando ningún caso de uso lo use.
4. **Cortar las dependencias entre contextos** (`OwnerId` propio) **antes** de partir en crates. Si no, los crates nuevos heredan el acoplamiento y `hex_1_*` falla desde el primer día.
5. **Partir en crates por capa y añadir `tools/architecture` con los lints del workspace** (RS-1, RS-4, VER-1), un contexto a la vez. Se recomienda empezar por `communities`, siguiendo la plantilla.
6. **Por caso de uso:** primero el contrato `{Verb}{Noun}Error`, sus hojas y el renombrado a `UseCase`/`Handler`; después el `match` exhaustivo en `api`, Problem Details y los códigos en `api`. El mapper no puede ser exhaustivo hasta que exista el enum.
7. **Puertos salientes en un solo cambio de firma:** renombrar (HEX-5), pasar a async (RS-3) y devolver `Result<_, Fault>` (ERR-1). Después, la batería de contrato de cada puerto y los dobles renombrados según su semántica real (TDD-2).
8. **Validación de tokens:** primero el puerto o caso de uso en `iam::application`; después quitar el `use iam::infrastructure` de `api` y mover `AuthContext` al borde.
9. **Manejador global y `FaultReporter`** cuando ya exista `Fault`, añadiendo `CatchPanicLayer` al mismo tiempo.
10. **Decidir qué pasa con `membership`** (eliminarlo o convertirlo en un contexto real) antes de invertir en migrarlo.

## Observaciones sin ID

- **`membership` es código muerto.** Ningún crate depende de él.
- **Duplica la propiedad de "crear comunidad".** Su `CreateCommunity` repite el caso de uso de `communities`, así que dos contextos son dueños del mismo caso.
- **Correcciones a premisas de la auditoría:**
  - `FakeAccountRepository` no tiene ningún `todo!()`; el único `todo!()` en un doble es `FakeCommunityRepository::get_public_list`.
  - Las baterías de contrato aplican a los puertos **salientes**, no a los de entrada.
  - Que `membership` duplique los value objects de `communities` no viola RS-1: cada contexto tiene su propia vista.
