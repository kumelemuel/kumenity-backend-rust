# Adopción del harness craft-backend

Este archivo guarda el estado entre sesiones de la adopción de los principios craft-backend (HEX, ERR, TDD, VER, RS) en este repositorio.

| Dato | Valor |
|---|---|
| Fecha de la auditoría | 2026-11-19; auditoría completa de P1: 2026-11-23 |
| Plugin | `craft-backend@ai-harness` 0.12.0 |
| Commit auditado | `d0b0eeb` (rama `ci/sync-development`); P1: `bab68c5` (`main`, después de R10b) |
| Revisor | `craft-backend:architecture-reviewer`, auditoría completa en solo lectura |
| Hallazgos | 110: 26 de severidad alta, 52 media, 32 baja. 94 de la auditoría inicial (RS-1.4 agregado en R2) y 16 de la de P1 (ERR-4.9 a ERR-4.12, HEX-7.10 a HEX-7.12, HEX-8.10, ERR-1.8, TDD-1.8, TDD-2.6 y VER-1.3 a VER-1.7) |

## Cómo mantener este archivo

- Al resolver un hallazgo, cambiar su **Estado** de `abierto` a `resuelto (<commit>)`. Si se descarta, poner `descartado` y el motivo.
- Los IDs de hallazgo (`HEX-1.3`, `ERR-4.2`…) son estables: no se renumeran. Un hallazgo nuevo recibe el siguiente número libre de su regla.
- Después de cada slice de refactor, volver a auditar con architecture-reviewer y actualizar la cabecera (fecha, commit, conteos).
- Tests de caracterización en `app/tests/` (PR #18): son la red de seguridad del refactor y tienen que seguir pasando.
- Cada rebanada se hace con `/craft-backend:adoption R<n> [notas]`. Las reglas propias de este proyecto, que la skill no trae, están en §5.1 ("Ritual de rebanada").

## Prioridades inmediatas

1. **ERR-4.1:** `.env` ya no se versiona (0df9baa), pero `JWT_SECRET` sigue en el historial: **rotar el secreto** en cada entorno donde se haya usado.
2. **ERR-1.1 / RS-2.1:** `SystemError`, con su `From` genérico, mezcla los errores de negocio con los técnicos. Hay que reemplazarlo por `{Verb}{Noun}Error` por caso de uso y `Fault`/`Failure<E>`.
3. **RS-1 / HEX-1 / HEX-2:** separar cada contexto en crates por capa, empezando por `communities` (HEX-1.1).

Ya resueltas: ERR-4.2 (`dbg!`), ERR-4.3 (`Debug` con el secreto), VER-1.1/RS-4.1 (`tools/architecture` con trinquete sobre `baseline.txt`), VER-1.2 (CI con fmt, clippy y test), RS-2.5 (`[workspace.lints]`) ERR-1.2/ERR-1.3 (los errores ya no implementan `std::error::Error`) HEX-8.4/HEX-8.7 (código muerto del kernel) y RS-1.3/RS-1.4 (herencia del workspace, sin `anyhow`). En P1, ERR-4.9 (un `JWT_SECRET` vacío o corto ya no arranca el servidor; PR #40).

## 1. Resumen

| Regla | Alta | Media | Baja | Total |
|---|---|---|---|---|
| HEX-1 Dependencias hacia adentro | 6 | 3 | 0 | 9 |
| HEX-2 Núcleo sin framework | 3 | 3 | 0 | 6 |
| HEX-4 Puertos entrantes | 0 | 2 | 2 | 4 |
| HEX-5 Puertos por rol | 0 | 2 | 0 | 2 |
| HEX-6 Vocabulario | 0 | 0 | 2 | 2 |
| HEX-7 El dominio decide | 1 | 7 | 4 | 12 |
| HEX-8 Kernel delgado | 3 | 5 | 2 | 10 |
| VER-1 Verificación automática | 2 | 0 | 5 | 7 |
| ERR-1 Dos árboles | 2 | 4 | 2 | 8 |
| ERR-2 Contrato por caso de uso | 2 | 5 | 3 | 10 |
| ERR-3 Semántica de adaptadores | 0 | 3 | 0 | 3 |
| ERR-4 Transporte en el borde | 3 | 6 | 3 | 12 |
| TDD-1 Test primero | 0 | 4 | 4 | 8 |
| TDD-2 Tests por capa y dobles | 1 | 2 | 3 | 6 |
| RS-1 Crate por capa | 1 | 1 | 2 | 4 |
| RS-2 `Fault` y `panic!` | 1 | 4 | 0 | 5 |
| RS-3 Puertos async | 0 | 1 | 0 | 1 |
| RS-4 Tests de arquitectura | 1 | 0 | 0 | 1 |
| **Total** | **26** | **52** | **32** | **110** |

Cada fila del inventario cuenta como un hallazgo. Cuando una misma violación se repite igual en varios archivos, la fila lista todas las ubicaciones.

`shared/` se audita como si fuera el kernel, porque hoy cumple ese papel aunque no tenga la forma que pide RS-1. Ver la sección 4.

## 2. Inventario

### HEX (hexagonal-architecture) y VER

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| HEX-1.1 | HEX-1 Dependencias | contexts/communities/Cargo.toml:16 | `communities` declara `iam` como dependencia: un contexto depende de otro. | alta | Quitar `iam`; `communities` modela su propio `OwnerId`. | abierto |
| HEX-1.2 | HEX-1 Dependencias | contexts/communities/src/domain/model/community/community.rs:7 | El dominio de `communities` importa `iam::…::AccountId` y lo guarda como `_owner_id` (línea 12). | alta | Value object `OwnerId` propio del contexto. | abierto |
| HEX-1.3 | HEX-1 Dependencias | contexts/communities/src/application/commands/create_community/create_community.rs:17; application/testing/contracts/community_repository.rs:14 | El handler importa `AccountId` de `iam` y valida con él el sujeto del token (línea 47). También lo importan la batería de contrato (desde R7) y el módulo de tests del handler (línea 86). | alta | Validar con `OwnerId::new` del propio contexto. | abierto |
| HEX-1.4 | HEX-1 Dependencias | contexts/communities/src/infrastructure/http/errors/error_mapper.rs:12 | El borde HTTP de `communities` importa códigos de error del dominio de `iam`. | media | Códigos propios en `communities-api/src/codes.rs`. | abierto |
| HEX-1.5 | HEX-1 Autenticación | shared/src/application/ports/outbound/token_verifier.rs:6 | `TokenVerifier` vive en un `shared` que ven todos los contextos: es la violación típica que nombra la regla. | alta | `VerifyAccessTokenUseCase` en `iam/application/queries` + puerto `AccessTokenService`. | resuelto (517e4cd, 101f652, 9be8d19, 3c1a6bb): `AccessTokenService` en R10a; la query `VerifyAccessToken`, el middleware sobre ella y el borrado de `TokenVerifier` en R10b |
| HEX-1.6 | HEX-1 Autenticación | app/src/middleware/auth.rs:17 | El middleware vive en el composition root, no en el `api` de `iam`. Inyecta `VerifiedIdentity` (de `shared::domain`, línea 36) en vez de un tipo del borde. Desde R10b consume `VerifyAccessTokenUseCase`, pero importa `iam::infrastructure::http::errors::error_mapper` (línea 12), un detalle del `api` de iam. | alta | `IamApi::protect(routes)` en `iam-api`, que inserta `edge_http::AuthenticatedAccount`. | abierto |
| HEX-1.7 | HEX-1 Dependencias | contexts/iam/src/infrastructure/http/errors/error_mapper.rs:3 | El adaptador driving (HTTP) importa `crate::domain::errors::error_codes`: `api` ve `domain`. | media | `api` solo ve el contrato `{Verb}{Noun}Error` de `application`. | abierto |
| HEX-1.8 | HEX-1 Dependencias | contexts/communities/src/infrastructure/http/errors/error_mapper.rs:3 | Igual que HEX-1.7: el `api` de `communities` importa códigos de `domain`. | media | Igual que HEX-1.7. | abierto |
| HEX-1.9 | HEX-1 Dependencias | contexts/iam/src/infrastructure/mod.rs:1; contexts/communities/src/infrastructure/mod.rs:1 | El adaptador driving (`http`) es un submódulo de `infrastructure`: `api` e `infrastructure` no están separados y pueden importarse entre sí. | alta | Crate `{ctx}-api` separado (RS-1). | abierto |
| HEX-2.1 | HEX-2 Núcleo sin framework | contexts/iam/Cargo.toml:11-17 | `domain` y `application` comparten crate con `uuid`, `argon2`, `rand`, `jsonwebtoken`, `serde`, `chrono` y `axum`, así que todos están disponibles en el núcleo. | alta | Crates `iam-domain` e `iam-application` con solo el kernel. | abierto |
| HEX-2.2 | HEX-2 Núcleo sin framework | contexts/iam/src/domain/model/account/account_id.rs:3 | El dominio usa `uuid::Uuid`. | alta | El id llega de `kernel::IdGenerator`; `uuid` solo en infraestructura. | abierto |
| HEX-2.3 | HEX-2 Núcleo sin framework | contexts/iam/src/domain/model/account/code_validation.rs:2 | El dominio usa `rand` para generar el código (líneas 29-33). | media | Generar el código con un puerto o capacidad y pasárselo a `Account::create`. | abierto |
| HEX-2.4 | HEX-2 Núcleo sin framework | contexts/iam/src/domain/model/account/events/account_event.rs:1 | El dominio usa `chrono`. | media | El instante llega de `Clock`. | abierto |
| HEX-2.5 | HEX-2 Núcleo sin framework | contexts/communities/Cargo.toml:11-14 | `uuid`, `axum`, `serde` y `http` (este último, visto en S1) están disponibles en `domain` y `application`. | alta | Separar crates por capa. | abierto |
| HEX-2.6 | HEX-2 Núcleo sin framework | contexts/communities/src/domain/model/community/community_id.rs:2 | El dominio usa `uuid`. | media | `IdGenerator` + value object sobre `String`. | abierto |
| HEX-4.1 | HEX-4 Casos de uso | iam/…/authenticate_account.rs:17; register_account.rs:18; verify_account.rs:14; identify_account.rs:16; communities/…/create_community.rs:21; list_public_communities.rs:14 | Las implementaciones no se llaman `{Verb}{Noun}Handler` (`AuthenticateAccount`, `RegisterAccount`…). | media | Renombrar a `AuthenticateAccountHandler` y demás, en `handler.rs`. | abierto: R18 (a603969) borró `list_public_communities.rs`; quedan los otros 5 |
| HEX-4.2 | HEX-4 Casos de uso | contexts/communities/src/application/ports/inbound/create_community_use_case.rs:10 | `execute` recibe un segundo argumento, `VerifiedIdentity`, del dominio del kernel: el contrato expone un tipo de `domain` fuera del `Command`. | media | Campo `owner_id: String` en `CreateCommunityCommand`. | abierto |
| HEX-4.3 | HEX-4 Casos de uso | contexts/communities/src/application/ports/inbound/list_public_communities_use_case.rs:6 | Le falta `Send + Sync`, recibe un `AuthContext` que no usa (línea 10) y no está cableado. | baja | Firma `execute(query)`, con `Send + Sync`, o eliminarlo. | resuelto (a603969): eliminado (D3) |
| HEX-4.4 | HEX-4 Casos de uso | contexts/iam/src/application/errors/authenticate_account.rs:6 | El `Error` del caso de uso vive en `errors/`, no junto al handler. | baja | `commands/authenticate_account/error.rs`. | abierto |
| HEX-5.1 | HEX-5 Rol | iam/…/ports/outbound/account_repository.rs:6; password_hasher.rs:1; token_generator.rs:3; communities/…/community_repository.rs:6 | Sufijo `Port`, prohibido. En `PasswordHasher` y `TokenGenerator`, al quitarlo, no queda rol. | media | `AccountRepository`, `CommunityRepository`, `PasswordHashingService`, `AccessTokenService`. | resuelto (692bc7c, ebd0819, 73f9c44, 517e4cd): `CommunityRepository` en R7, `AccountRepository` en R8, `PasswordHashingService` en R9 y `AccessTokenService` (fusión con `TokenVerifier`) en R10a |
| HEX-5.2 | HEX-5 Adaptadores | iam/…/in_memory/account_repository.rs:14; communities/…/in_memory/community_repository.rs:13; iam/…/argon2_password_hasher.rs:8; iam/…/jwt_service.rs:15 | Los adaptadores no siguen `{Tecnología}{Concepto}Adapter`: repiten el rol o terminan en `Service`. | media | `InMemoryAccountAdapter`, `InMemoryCommunityAdapter`, `Argon2PasswordHashingAdapter`, `JwtAccessTokenAdapter`. | resuelto (6b808a2, 9a90d8a, 17ba68e, c9db5fa): el último, `JwtAccessTokenAdapter`, en R10a |
| HEX-6.1 | HEX-6 Transporte | iam/…/requests/sign_up.rs:5, sign_in.rs:5, verify.rs:5, identify.rs:5; responses/signed_up.rs:5, signed_in.rs:5, identified.rs:5; communities/…/requests/create.rs:5, responses/created.rs:5 | Los DTOs HTTP no son `{Caso}HttpRequest`/`{Caso}HttpResponse`, y `CreateRequest` colisiona con el vocabulario de puerto saliente. | baja | `RegisterAccountHttpRequest`, `CreateCommunityHttpResponse` y demás. | abierto |
| HEX-6.2 | HEX-6 Vocabulario | contexts/communities/src/application/queries/list_public_communities/response.rs:5 | `CommunityResult` usa el sufijo `Result`, reservado para control de flujo. | baja | `PublicCommunity` o `CommunitySummary`. | resuelto (a603969): eliminado (D3) |
| HEX-7.1 | HEX-7 Lógica en handler | contexts/iam/src/application/commands/register_account/register_account.rs:40 | La unicidad se consulta con el texto crudo (`cmd.email`, y `cmd.username` en la línea 48), no con el value object normalizado. Resultado: se registran duplicados por mayúsculas o espacios, como confirma el test `register_accepts_same_email_with_different_casing`. | alta | Construir `Email`/`Username` antes y buscar por el valor normalizado (como hace `CreateCommunity` con el slug). | abierto |
| HEX-7.2 | HEX-7 Lógica en handler | contexts/iam/src/application/commands/authenticate_account/authenticate_account.rs:70 | `if !account.can_authenticate()` es un `if` de negocio en el handler. | media | `account.authenticate() -> Result<_, CannotAuthenticate>` en el agregado. | abierto |
| HEX-7.3 | HEX-7 Inmutabilidad | contexts/iam/src/domain/model/account/account.rs:89-134 | El agregado es mutable (`&mut self`): `change_password`, `change_email`, `deactivate`, `confirm_registration`… | media | `fn confirm(&self, …) -> Result<Self, …>`, que devuelve el estado nuevo. | abierto |
| HEX-7.4 | HEX-7 Eventos | contexts/iam/src/domain/model/account/account.rs:32-46 | `record_event`/`pull_events` son `todo!()`; ninguna transición registra eventos (`AccountEvent::Registered` nunca se emite). | media | `create` registra `Registered`, más `take_events`; o quitar el andamiaje. | abierto: R21 (D4: se retira) |
| HEX-7.5 | HEX-7 Crear/reconstituir | contexts/iam/src/domain/model/account/account_factory.rs:14 | `AccountFactory::create`/`reconstitute` (línea 32) reciben `String`. Además, `Account::new` es público (account.rs:49) y permite saltarse las reglas de alta. | media | `Account::create`/`Account::restore` con value objects; `new` privado. | abierto |
| HEX-7.6 | HEX-7 Identificador | contexts/iam/src/domain/model/account/account_factory.rs:21; contexts/communities/src/application/commands/create_community/create_community.rs:48 | El id se genera en el dominio (`AccountId::generate`, `CommunityId::generate`) en vez de recibirse de `IdGenerator`. | media | Inyectar `Arc<dyn IdGenerator>` en el handler y pasar el id. | abierto |
| HEX-7.7 | HEX-7 Mapeo de persistencia | contexts/iam/src/domain/model/account/account_status.rs:76 | `from_str` define en el dominio el formato persistido `"registered:123123"`. | baja | El adaptador traduce de `String` a `AccountStatus`. | abierto |
| HEX-7.8 | HEX-7 Servicio de dominio | contexts/communities/src/domain/services/membership_policy.rs:2 | `MembershipPolicy` es un enum de opciones (value object) archivado en `services/`. | baja | Moverlo a `model/community/`. | abierto: R22 (D4: se retira) |
| HEX-7.9 | HEX-7 Comportamiento | contexts/communities/src/domain/model/community/community.rs:19-56 | Agregado anémico: `membership_policy` siempre es `None` y `MembershipPolicyChanged` (membership_policy_changed.rs:4) no se usa. | baja | Agregar el comportamiento con su evento, o retirar lo muerto. | abierto: R22 (D4: se retira) |
| HEX-7.10 | HEX-7 Lógica en handler | contexts/iam/src/application/commands/verify_account/verify_account.rs:28 | `find_by_email` recibe `cmd.email` crudo, no el `Email` normalizado. El alta guarda el email en minúsculas, así que verificar con `Ana@Example.com` responde 404 a una cuenta que existe. Misma causa que HEX-7.1. Hallado en P1. | media | Construir `Email` y buscar por su valor. | abierto: R12b |
| HEX-7.11 | HEX-7 Lógica en handler | contexts/iam/src/application/commands/authenticate_account/authenticate_account.rs:47 | `find_by_username` recibe `cmd.username` sin el recorte que aplica `Username`. Misma causa que HEX-7.1. Hallado en P1. | media | Buscar por `Username`; si no se puede construir, `LoginFailed` con la verificación del señuelo (D6). | abierto: R12b |
| HEX-7.12 | HEX-7 Comportamiento | contexts/communities/src/domain/errors/community_error.rs:10 | `CommunityError` está muerto: 6 variantes, con sus códigos en `domain/errors/error_codes.rs:4-9`, que nada produce. Residuo de la membresía, como HEX-7.9. Hallado en P1. | baja | Borrarlo con sus códigos. | abierto: R19 |
| HEX-8.1 | HEX-8 Kernel sin dependencias | shared/Cargo.toml:7-8 | El kernel depende de `chrono` y `serde`. | alta | Kernel solo `std`. | abierto: desde S1, `chrono` sale en R21 y `serde` en R25 |
| HEX-8.2 | HEX-8 Marcadores | shared/src/domain/domain_event.rs:5 | El marcador `DomainEvent` exige `DateTime<Utc>` de `chrono`. | media | Sin dependencias; el instante llega de `Clock`. | abierto |
| HEX-8.3 | HEX-8 Kernel | shared/src/error.rs:10 | `ErrorLayout` y `SystemError` (línea 17), un error global por capas, viven en el kernel. | alta | `Fault` + `Failure<E>` (RS-2). | abierto |
| HEX-8.4 | HEX-8 Marcadores de error | shared/src/domain/domain_error.rs:3; shared/src/infrastructure/infrastructure_error.rs:3 | Traits `DomainError` e `InfrastructureError`: Rust no los necesita (y aquí nadie los usa). | media | Eliminarlos. | resuelto (80c00e4) |
| HEX-8.5 | HEX-8 Sin puertos | shared/src/application/ports/outbound/token_verifier.rs:6 | Un puerto de negocio (`TokenVerifier`) y su `TokenError` (error.rs:2) están en el kernel; no es una capacidad de la lista. | alta | Moverlo a `iam` (ver HEX-1.5). | resuelto (3c1a6bb): R10b borró `TokenVerifier`, `TokenError` y `shared::application` |
| HEX-8.6 | HEX-8 Sin modelos | shared/src/domain/model/verified_identity.rs:2; shared/src/application/auth_context.rs:2 | Tipos de identidad en el kernel. | media | `AuthenticatedAccount` en `edge/http`. | abierto: R18 (a603969) borró `AuthContext` (sin usos tras quitar `ListPublicCommunities`); queda `VerifiedIdentity` (R27) |
| HEX-8.7 | HEX-8 Kernel | shared/src/application/common_application_error.rs:2 | `CommonApplicationError` está muerto y es un error compartido de aplicación. | baja | Eliminarlo. | resuelto (80c00e4) |
| HEX-8.8 | HEX-8 Kernel | shared/src/infrastructure/http/api_error_response.rs:4 | Un tipo de transporte HTTP (con `serde`) en el kernel. | media | `edge/http` con `Problem`. | abierto |
| HEX-8.9 | HEX-8 Marcadores | shared/src/domain/aggregate_root.rs:6-8 | `AggregateRoot` no es un marcador: impone `record_event(&mut self)`, `pull_events` y `version`, una API mutable. Además trae el helper mutable `EventRecorder` (líneas 11-32). | baja | Marcador vacío; los eventos se manejan con `take_events(self)` en el agregado. | abierto |
| HEX-8.10 | HEX-8 Marcadores | shared/src/domain/value_object.rs:1 | `ValueObject` no es un marcador: impone `new(Value)` y `value()`. Obliga a los `todo!()` de `AccountId` (account_id.rs:13,17) y `AccountStatus` (account_status.rs:18,22), 4 de los 10 de RS-2.2. Hallado en P1. | media | Marcador vacío, o borrarlo, con `new`/`as_str` propios en cada value object. | abierto: R21 (R20 ya quita los `impl` de `AccountId` y `AccountStatus`) |
| VER-1.1 | VER-1 Tests de arquitectura | Cargo.toml:2-5 | No existe `tools/architecture`: ninguna regla de dependencias o de nombres tiene test. | alta | Agregar `tools/architecture` (la plantilla de rust-stack). | resuelto (ad558fc) |
| VER-1.2 | VER-1 CI | .github/workflows/sync-development.yml:22-58 | El único workflow sincroniza ramas; no corre `fmt`, `clippy` ni `test`. | alta | Workflow con el CI mínimo de RS-4. | resuelto (c54f6ae) |
| VER-1.3 | VER-1 Tests de arquitectura | tools/architecture/tests/naming.rs:537 | El test de contratos no trata `shared::domain` como dominio ni cuenta los argumentos de `execute`: HEX-4.2 (`VerifiedIdentity` como segundo argumento) pasa en verde. Hallado en P1. | baja | Proponer al harness: el dominio del kernel cuenta como dominio, y un solo parámetro además de `&self`. | abierto: harness (§6) |
| VER-1.4 | VER-1 Tests de arquitectura | tools/architecture/tests/naming.rs | Ningún test prohíbe que el adaptador driving legacy (`infrastructure/http`) importe `crate::domain`: HEX-1.4, HEX-1.7 y HEX-1.8 no tienen entrada en la línea base. Hallado en P1. | baja | Proponer al harness un test de imports `api → domain` sobre la capa virtual. | abierto: harness (§6) |
| VER-1.5 | VER-1 Tests de arquitectura | tools/architecture/tests/naming.rs | Nada prohíbe `shared::error::SystemError` en `ports/inbound` (ERR-2.1) ni exige que `{Verb}{Noun}Error` viva junto al handler (HEX-4.4, ERR-2.4). Hallado en P1. | baja | Proponer al harness un test de firmas de los puertos entrantes. | abierto: harness (§6) |
| VER-1.6 | VER-1 Tests de arquitectura | tools/architecture/tests/kernel.rs:69 | El test acepta como marcadores `AggregateRoot` y `ValueObject` con métodos (HEX-8.9, HEX-8.10). Hallado en P1. | baja | Proponer al harness: un marcador es un trait sin ítems. | abierto: harness (§6) |
| VER-1.7 | VER-1 Lints | Cargo.toml:41 | `[workspace.lints.clippy]` solo prohíbe `unwrap_used`. Nada impide un `todo!()` (RS-2.2) o un `dbg!` (ERR-4.2) nuevo. Hallado en P1. | baja | `todo = "deny"` y `dbg_macro = "deny"`. | abierto: R21 (cuando se borra el último `todo!()` de RS-2.2) |

### ERR (error-handling)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| ERR-1.1 | ERR-1 Dos árboles | shared/src/error.rs:17 | `SystemError` mezcla los fallos de negocio con los técnicos (`Infrastructure`) en un tipo. El `impl<T: ErrorLayout> From<T>` (línea 40) hace que `?` los absorba implícitamente. | alta | `Failure<{Verb}{Noun}Error>` y `Fault` separados. | abierto |
| ERR-1.2 | ERR-1 Errores de dominio | iam/src/domain/errors/account.rs:27, account_id.rs:20, account_status.rs:20, code_validation.rs:18, email.rs:18, hashed_password.rs:18, password_policy.rs:20, username.rs:18; communities/src/domain/errors/community_error.rs:38, invalid_community_id.rs:14, invalid_community_name.rs:14, invalid_community_slug.rs:14 | Los errores de dominio implementan `std::error::Error`: un `?` los manda al canal técnico. | alta | Solo `Debug` + `Display`. | resuelto (eb30039) |
| ERR-1.3 | ERR-1 Errores de application | iam/src/application/errors/account_repository.rs:14, authenticate_account.rs:24, token_generator.rs:14; communities/src/application/errors/community_creation.rs:20, community_repository.rs:14 | Las hojas y los errores de puerto implementan `std::error::Error`. | media | Quitar el `impl`. | resuelto (eb30039) |
| ERR-1.4 | ERR-1 Fallo silenciado | contexts/iam/src/domain/model/account/account.rs:127-129 | `let _ = self.transition_status(…).map_err(…)` descarta el `Result` y responde `Ok`, aunque la transición falle. | media | Propagar con `?`, o `expect("invariante: Registered→Active siempre válida")`. | abierto |
| ERR-1.5 | ERR-1 Conversión implícita | contexts/iam/src/domain/model/account/account_factory.rs:39-43 | En `reconstitute`, un dato persistido corrupto se convierte en error de negocio (400) con `?`. | media | En el adaptador, conversión a `Fault` a mano y con contexto. | abierto |
| ERR-1.6 | ERR-1 Sin árbol A | contexts/communities/src/application/ports/inbound/list_public_communities_use_case.rs:11 | El caso de uso no tiene fallos de negocio, pero declara un tipo de error (`SystemError`). | baja | `Result<Response, Fault>`. | resuelto (a603969): eliminado (D3) |
| ERR-1.7 | ERR-1 Firma del puerto | iam/…/ports/outbound/account_repository.rs:7-9; password_hasher.rs:2-3; communities/…/community_repository.rs:7-10 | `find_*` devuelve `Option` sin `Result`, `hash` devuelve `String` y `verify`, `bool`: un adaptador real no puede reportar una falla técnica y tiene que entrar en pánico. | media | `Result<Option<T>, Fault>`, `Result<String, Fault>`. | resuelto (692bc7c, ebd0819, 73f9c44): `CommunityRepository` en R7, `AccountRepository` en R8 y `PasswordHashingService` en R9 |
| ERR-1.8 | ERR-1 Canal declarado | app/src/config/app.rs:11 | `AppConfig::load()` declara `Result<_, ConfigError>` pero termina el proceso dentro con `process::exit(1)`: la rama `Err` de main.rs:5 nunca corre. Hallado en P1. | baja | `let jwt = JwtConfig::from_env()?;`. | abierto: R26 |
| ERR-2.1 | ERR-2 Contrato por caso de uso | iam/…/ports/inbound/authenticate_account_use_case.rs:10, identify_account_use_case.rs:7, register_account_use_case.rs:10, verify_account_use_case.rs:5; communities/…/create_community_use_case.rs:11 | Ningún caso de uso tiene su `{Verb}{Noun}Error` cerrado: todos devuelven el global `SystemError`. | alta | `RegisterAccountError { Validation, Conflict }` y demás, junto al handler. | abierto |
| ERR-2.2 | ERR-2 Sin envolver | shared/src/error.rs:18-20 | Los errores se envuelven (`Domain(Box<dyn ErrorLayout>)`), y el error de dominio llega tal cual al contrato (p. ej. `AccountError::EmailAlreadyExists.into()` en register_account.rs:43). | alta | Traducir a una hoja plana (`Conflict { resource }`). | abierto |
| ERR-2.3 | ERR-2 Contrato incompleto | contexts/iam/src/application/errors/authenticate_account.rs:6 | `AuthenticateAccountError` no cubre `NotFound` (viene de `AccountError`) ni la falla de token: hay contratos parciales mezclados. | media | Un enum cerrado con todas las variantes que el caso produce. | abierto: R2 (ad2f129) quitó `NotFound` del caso; R10a (65f455d) hizo de la falla de token un `Fault` (árbol B), que no va en el contrato. Falta el enum cerrado (R14) |
| ERR-2.4 | ERR-2 Nombre | contexts/communities/src/application/errors/community_creation.rs:6 | `CommunityCreationError` en vez de `CreateCommunityError`, y fuera del directorio del caso. | baja | `commands/create_community/error.rs`. | abierto |
| ERR-2.5 | ERR-2 Variante correcta | contexts/iam/src/domain/model/account/account_status.rs:82, 90 | Un estado ilegible se reporta como `InvalidTransition`. | baja | Dato corrupto → `Fault` en el adaptador. | abierto |
| ERR-2.6 | ERR-2 Tests por variante | contexts/iam/src/application/commands/register_account/register_account.rs:147 (patrón en todos los tests de handler) | Los tests afirman sobre `err.code()` (un string), no sobre la variante. | baja | `matches!(r, Err(Failure::Expected(…)))`. | abierto |
| ERR-2.7 | ERR-2 Test por variante | contexts/iam/src/application/commands/register_account/register_account.rs:65 | Falta el test del handler para contraseña corta (`fails_when_password_is_too_short`). Falla de repositorio sin test de endpoint (solo el unit test del mapper). | media | Agregar ambos. | abierto: R8 (ebd0819, cc3d60c) agrega los tests de handler de cada falla del repositorio, y esa falla es árbol B (`Fault`), cubierta por `unexpected_fault_maps_to_internal_server_error`. Falta `fails_when_password_is_too_short` (R11) |
| ERR-2.8 | ERR-2 Test por variante | contexts/iam/src/application/commands/authenticate_account/authenticate_account.rs:77 | La falla del generador de tokens no tiene test de handler (`fails_when_token_generation_fails`) ni de endpoint. | media | Doble que falle + test. | resuelto (65f455d, c65817c): R10a agrega `fails_with_unexpected_when_issue_fails`. La falla es árbol B, no una variante del contrato, así que el test por variante de ERR-2 no aplica; el 500 lo fijan `unexpected_fault_maps_to_internal_server_error` y `a_fault_verifying_the_token_returns_500`. El resto del manejador global sigue en ERR-4.7 |
| ERR-2.9 | ERR-2 Test por variante | contexts/iam/src/application/commands/verify_account/verify_account.rs:41 | Faltan el test de handler para código fuera de rango (`fails_when_code_is_out_of_range`) y los de handler y endpoint para la falla del repositorio (`fails_when_repository_fails`). | media | Agregar los tests. | abierto: R8 (ebd0819, cc3d60c) agrega los de handler para la falla del repositorio (verify_account.rs:72, 85), que es árbol B; el de endpoint del código existe (auth_api.rs:367). Falta `fails_when_code_is_out_of_range` (R13) |
| ERR-2.10 | ERR-2 Test por variante | contexts/communities/src/application/commands/create_community/create_community.rs:62 | El owner con formato inválido o nil no tiene test de handler (`fails_when_owner_id_is_invalid`), solo de endpoint. La falla de repositorio no tiene test de endpoint. | media | Agregar los tests. | abierto: R7 (692bc7c, ba15189) agrega los de handler para la falla del repositorio, que es árbol B. Falta `fails_when_owner_id_is_invalid` (R17) |
| ERR-3.1 | ERR-3 Respondió vs no pudo | contexts/iam/src/infrastructure/services/jwt_service/jwt_service.rs:85-92 | Un token inválido o vencido se devuelve como `Err(TokenError)`; es un veredicto, no un fallo. | media | `Ok(AccessTokenVerdict::Invalid)`. | resuelto (c9db5fa, 62d5456): `JwtAccessTokenAdapter` responde `Invalid` a todo error del token, y `Fault` a uno de clave o de validación |
| ERR-3.2 | ERR-3 Errores para transporte | iam/src/application/errors/account_repository.rs:16-27, token_generator.rs:16-27; communities/src/application/errors/community_repository.rs:16-27 | Los errores de puerto traen un `code()` y un `message()` pensados para el cliente HTTP. | media | Sin texto de usuario; el borde decide. | resuelto (692bc7c, ebd0819, 65f455d): R10a borró `TokenGeneratorError` |
| ERR-3.3 | ERR-3 Falla técnica = árbol B | iam/src/application/errors/account_repository.rs:6, token_generator.rs:6; communities/src/application/errors/community_repository.rs:6 | Fallas técnicas no declarables (persistencia, codificación JWT) modeladas como errores tipados del contrato. | media | `Fault`, propagado con `?`. | resuelto (692bc7c, ebd0819, 65f455d): R10a borró `TokenGeneratorError` |
| ERR-4.1 | ERR-4 Secretos | .env:1 | `JWT_SECRET` está versionado; `.gitignore:1-2` no excluye `.env`. | alta | Sacar `.env` del índice y rotar el secreto; `.env.example` sin valores. | en curso (0df9baa): fuera del índice; falta rotar el secreto |
| ERR-4.2 | ERR-4 Logging en el núcleo | contexts/iam/src/application/commands/register_account/register_account.rs:55 | `dbg!(&account)` escribe en stderr la cuenta completa (email y hash de la contraseña). | alta | Eliminarlo. | resuelto (57e814e) |
| ERR-4.3 | ERR-4 Secretos | app/src/config/jwt.rs:3; app/src/config/app.rs:3 | `#[derive(Debug)]` sobre una configuración que contiene el secreto. | media | Sin `Debug`. | resuelto (8195600) |
| ERR-4.4 | ERR-4 Códigos en el borde | iam/src/domain/errors/error_codes.rs:1-12; iam/src/application/errors/error_codes.rs:1-2; communities/src/domain/errors/error_codes.rs:1-9; communities/src/application/errors/error_codes.rs:1-2 | Los códigos estables y los mensajes de cliente viven en el núcleo. | media | `{ctx}-api/src/codes.rs`. | abierto |
| ERR-4.5 | ERR-4 `fold` exhaustivo | contexts/iam/src/infrastructure/http/errors/error_mapper.rs:60-77; contexts/communities/src/infrastructure/http/errors/error_mapper.rs:58-66 | El status HTTP se decide comparando strings, con un comodín `_ => 500` (líneas 76 y 65). | media | `match` exhaustivo sobre el enum del contrato. | abierto |
| ERR-4.6 | ERR-4 Problem Details | shared/src/infrastructure/http/api_error_response.rs:4; app/src/middleware/auth.rs:29, 42 | Las respuestas de error usan `{code,message}` y el 401 va con cuerpo vacío, no en RFC 9457. | media | `edge_http::Problem` con un helper por hoja. | abierto |
| ERR-4.7 | ERR-4 Manejador global | app/src/server.rs:24-27 | No hay manejador del árbol B, ni `FaultReporter`, ni `CatchPanicLayer`. Los `todo!`/`expect` cortan la conexión y las fallas técnicas no se registran. | media | `CatchPanicLayer` + `unexpected()` que reporta y devuelve un 500 genérico. | abierto: R5 (29890cb) existe el trait `FaultReporter`, y los mappers escriben el `Fault` en stderr como interino; desde R10b (c65817c) el `Fault` del middleware también responde el 500 de D7; faltan el adaptador, `unexpected()` y `CatchPanicLayer` (R26, R30) |
| ERR-4.8 | ERR-4 Identidad faltante | contexts/communities/src/infrastructure/http/handlers/community_handlers.rs:18 | `Extension<VerifiedIdentity>` obligatoria: en una ruta sin proteger responde 500, no 401. | baja | `Option<Extension<AuthenticatedAccount>>` → 401. | abierto |
| ERR-4.9 | ERR-4 Secretos | app/src/config/jwt.rs:26 | `from_vars` aceptaba un `JWT_SECRET` vacío: copiando `.env.example` (`JWT_SECRET=`), el servidor arrancaba firmando HS256 con clave vacía y cualquiera podía falsificar un token. Hallado en P1. | alta | `ConfigError::Invalid("JWT_SECRET")` por debajo de 32 bytes. | resuelto (e1b613b, PR #40) |
| ERR-4.10 | ERR-4 Secretos | contexts/iam/src/infrastructure/http/requests/sign_in.rs:4; sign_up.rs:4 | `SignInRequest` y `SignUpRequest` derivan `Debug` con `password` en claro: un `{:?}` futuro (log, `TraceLayer` con cuerpo) filtra la contraseña. Igual que ERR-4.3. Hallado en P1. | media | Sin `Debug`, o a mano con `<redacted>`. Se corrige en el lugar: las entradas `hex_6` de esos archivos prohíben moverlos, no editarlos. | abierto: R11 (`sign_up.rs`), R14 (`sign_in.rs`) |
| ERR-4.11 | ERR-4 Secretos | contexts/iam/src/application/queries/verify_access_token/query.rs:1; commands/authenticate_account/response.rs:1 | `VerifyAccessTokenQuery` y `AuthenticateAccountResponse` derivan `Debug` con el token. Hallado en P1. | baja | `Debug` a mano con el token redactado. | abierto: R14 |
| ERR-4.12 | ERR-4 Manejador global | app/src/main.rs:11 | Un `Fault` al arrancar se imprime con `Display`, que es opaco a propósito: el operador solo ve "unexpected failure", no la causa (p. ej. el hash del señuelo). Hallado en P1. | baja | Imprimir `detail()` o pasarlo por el `FaultReporter`. | abierto: R26 |

### TDD (tdd)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| TDD-1.1 | TDD-1 Test que lo exija | contexts/communities/src/application/queries/list_public_communities/list_public_communities.rs:46-143 | Los tests del handler están comentados (desde `56e43e8`) y apuntan a tipos que ya no existen: el handler no tiene ningún test. | media | Rojo primero con un doble que implemente `get_public_list`, o eliminar el caso. | resuelto (a603969): caso y tests comentados eliminados (D3) |
| TDD-1.2 | TDD-1 Test que lo exija | contexts/iam/src/infrastructure/persistence/in_memory/account_repository.rs:14; contexts/communities/src/infrastructure/persistence/in_memory/community_repository.rs:13 | Adaptadores en memoria sin tests propios; `find_by_id` de communities no se ejercita en ningún test. | media | Batería de contrato (TDD-2.1). | resuelto (6b808a2, 9a90d8a): R18 (a603969) borró `find_by_id`; R7 y R8 corren la batería contra `InMemoryCommunityAdapter` e `InMemoryAccountAdapter` |
| TDD-1.3 | TDD-1 Test que lo exija | contexts/iam/src/infrastructure/security/password_hasher/argon2_password_hasher.rs:20; contexts/iam/src/infrastructure/services/jwt_service/jwt_service.rs:52 | Adaptadores cubiertos solo indirectamente (`account_lifecycle.rs`, tests HTTP). | baja | Test del adaptador contra su tecnología. | resuelto (17ba68e, dddc146, c9db5fa, 62d5456): R10a corre la batería contra JWT |
| TDD-1.4 | TDD-1 Test que lo exija | contexts/iam/src/domain/model/account/account_status.rs:74 | Los caminos de error de `from_str` (código no numérico, estado desconocido) no tienen test. | baja | Tests de dominio. | resuelto (4355c9f) |
| TDD-1.5 | TDD-1 Orden | commits `030479b`, `9d4c54e`, `2ad183c`, `3a0e403`, `9f33df1`/`2d68755`, `022885c` | Commits `feat`/`feature` que cambian producción sin agregar ni modificar ningún test. | media | El test, antes o en el mismo commit. | descartado (histórico; TDD-1 rige desde R1). D8 |
| TDD-1.6 | TDD-1 Refactor | commit `56e43e8` "chore: redesign of architecture" | 260 archivos `.rs`, cero tests nuevos. Cambia firmas (use cases con `VerifiedIdentity`), comenta los tests de TDD-1.1 y borra unos 30 tests de membresía junto con su funcionalidad. No es un refactor sin cambio de comportamiento. | media | Separar el refactor de los cambios de comportamiento, y cada cambio con su test. | descartado (histórico; TDD-1 rige desde R1). D8 |
| TDD-1.7 | TDD-1 Orden | commit `f12e2a9` | Tests de caracterización escritos después de la producción. Es legítimo como red de seguridad, pero no es rojo primero. | baja | Ninguna; dejarlo explícito en el PR. | descartado (histórico; TDD-1 rige desde R1). D8 |
| TDD-1.8 | TDD-1 Test que lo exija | app/src/containers/iam.rs | La propagación del `Fault` del hash del señuelo al arrancar (R9) no tiene test: `IamContainer::initialize` construye Argon2 por dentro y no admite un doble que falle. Hallado en R9 y registrado en P1. | baja | Test de arranque con un hasher que falla, cuando `IamApi` reciba sus adaptadores. | abierto: R28 |
| TDD-2.1 | TDD-2 Batería de contrato | iam/…/ports/outbound/account_repository.rs:6, password_hasher.rs:1, token_generator.rs:3; communities/…/community_repository.rs:6; shared/…/token_verifier.rs:6 | Ningún puerto tiene batería de contrato; dobles y adaptadores no comparten comportamiento verificado. | alta | `testing/contracts/{puerto}.rs` + macro, contra el doble y el adaptador. | resuelto (9eacddc, de6c308, c04f69d, ce6d64b, 3c1a6bb): R7 (692bc7c, 9eacddc, 6b808a2) creó la de `CommunityRepository` y R8 (de6c308, 9a90d8a) la de `AccountRepository`; R9 (c04f69d, 17ba68e) la de `PasswordHashingService`; R10a (ce6d64b, c9db5fa) la de `AccessTokenService`, que reemplaza a `TokenGeneratorPort` y a `TokenVerifier`; R10b (3c1a6bb) borró `TokenVerifier` |
| TDD-2.2 | TDD-2 Nombre de dobles | iam/…/account_repository.rs:23, password_hasher.rs:10, token_generator.rs:14; communities/…/community_repository.rs:24 | Dobles sin el sufijo `Adapter` (`FakeAccountRepository`…). | baja | `{Prefijo}{Concepto}Adapter`. | resuelto (9eacddc, de6c308, c04f69d, ce6d64b): el último, `FakeAccessTokenAdapter`, en R10a |
| TDD-2.3 | TDD-2 Semántica del prefijo | iam/…/password_hasher.rs:10; token_generator.rs:14; account_repository.rs:23; communities/…/community_repository.rs:24 | `Fake` sin estado. `FakePasswordHasher` y `FakeTokenGenerator` devuelven respuestas fijas. Los repositorios se configuran por flags y `save` no guarda nada (account_repository.rs:98-106). | media | `FixedPasswordHashingAdapter`, `FixedAccessTokenAdapter`, `StubAccountAdapter`, o hacerlos `Fake` con estado real. | resuelto (9eacddc, de6c308, c04f69d, ce6d64b): R7 y R8 resolvieron los repositorios (`FakeCommunityAdapter` y `FakeAccountAdapter`, con estado); R9 (c04f69d) cambió `FakePasswordHasher` por `FixedPasswordHashingAdapter`; R10a (ce6d64b, 1b08cea) cambió `FakeTokenGenerator` por `FakeAccessTokenAdapter`, con estado |
| TDD-2.4 | TDD-2 Doble fiel | contexts/iam/src/application/ports/outbound/password_hasher.rs:19-21 | `verify` compara texto plano contra el hash, y el repositorio devuelve una cuenta cuyo email no es el consultado (account_repository.rs:95). El test de éxito tiene que mandar el hash como contraseña (authenticate_account.rs:163). | media | Doble coherente + batería de contrato. | resuelto (de6c308, cc3d60c, c04f69d): R8 resolvió el repositorio; R9, `verify` (`FixedPasswordHashingAdapter` solo acepta la contraseña con la que generó el hash) |
| TDD-2.5 | TDD-2 Doble completo | contexts/communities/src/application/ports/outbound/community_repository.rs:80 | El doble tiene `todo!()` en `get_public_list`, helpers muertos copiados de iam (líneas 59 y 68) y el campo `_activated` sin uso (línea 26). | baja | Implementar el método o quitarlo del puerto. | resuelto (a603969): `get_public_list`, los helpers muertos y `_activated` quitados |
| TDD-2.6 | TDD-2 Doble completo | contexts/communities/src/application/commands/create_community/create_community.rs:169 | El `StubCommunityAdapter` local (ba15189) falla siempre en `save` y no puede pasar la batería de contrato. Fricción ya anotada en §6. Hallado en P1. | baja | `FakeCommunityAdapter::failing_on(CommunityOperation::Save)`, como `FakeAccountAdapter`, y borrar el Stub. | abierto: R16 |

### RS (rust-stack)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| RS-1.1 | RS-1 Crate por capa | Cargo.toml:2-5 | Un crate por contexto (`contexts/*`), con las capas como módulos; no hay `kernel/` ni `edge/http`. | alta | `contexts/{ctx}/{domain,application,infrastructure,api}` (migración incremental). | abierto |
| RS-1.2 | RS-1 Kernel | shared/Cargo.toml:2 | El kernel se llama `shared` y tiene capas propias (`application`, `infrastructure/http`). | media | `kernel/` (solo std) + `edge/http`. | abierto |
| RS-1.3 | RS-1 Herencia | Cargo.toml:1-11 | Sin `[workspace.package]` ni `[workspace.lints]`; versiones repetidas por crate (`uuid` en contexts/iam/Cargo.toml:7 y contexts/communities/Cargo.toml:7, `serde` en cuatro crates). | baja | Herencia del workspace. | resuelto (991e672) |
| RS-1.4 | RS-1 Dependencias | app/Cargo.toml:18 | `anyhow` está declarado y no se usa: cargo avisa `unused dependency`. Hallado en R2, fuera de la auditoría. | baja | Quitarlo, junto con RS-1.3 (R6). | resuelto (991e672) |
| RS-2.1 | RS-2 Árbol B = `Fault` | shared/src/error.rs:17 | No existen `Fault` ni `Failure<E>`; el árbol B viaja como `SystemError::Infrastructure`. | alta | `kernel::Fault` y `kernel::Failure`. | abierto: R5 (e76c162, 29890cb, 446726b) agrega `Fault` y `Failure<E>` y el puente `SystemError::Unexpected`; falta retirar `SystemError` (R19) |
| RS-2.2 | RS-2 `panic!` solo invariantes | iam/…/account.rs:36,40,44; account_id.rs:13,17; account_status.rs:18,22; events/account_event.rs:10,14,18; communities/…/in_memory/community_repository.rs:27 | 11 `todo!()` en código de producción. El de `get_public_list` haría caer la petición si se cablea. | media | Implementar o eliminar los `impl` de trait vacíos. | abierto: R18 (a603969) quitó el de `in_memory/community_repository.rs`; quedan los 10 de iam (R20, R21) |
| RS-2.3 | RS-2 `unwrap` prohibido | iam/…/authenticate_account.rs:55; verify_account.rs:30; identify_account.rs:44; account_status.rs:79 (dos veces), 81; app/src/config/jwt.rs:15; app/src/main.rs:12,14 | `unwrap` en producción. En `account_status.rs` panica ante un dato persistido; en `jwt.rs`, ante un `JWT_EXPIRATION_TIME` no numérico. | media | `let Some(..) else`, `?` o `ConfigError`. | resuelto (0415e3e, 4355c9f, ebcd593, 9315004) |
| RS-2.4 | RS-2 `expect("invariante: …")` | iam/…/argon2_password_hasher.rs:26,32; iam/…/in_memory/account_repository.rs:28,36,44; communities/…/in_memory/community_repository.rs:31,39,47 | Los mensajes no empiezan con "invariante:". Argon2 usa `expect` para una falla técnica (línea 26) y para un hash persistido corrupto (línea 32), que deberían ser `Fault`. | media | `Fault` en Argon2; "invariante: mutex sin envenenar" en los mutex. | resuelto (6b808a2, 9a90d8a, 17ba68e, dddc146): los mutex en R7 y R8; Argon2 en R9, con sus errores como `Fault` |
| RS-2.5 | RS-2 Lints | Cargo.toml:1-11 | Sin `[workspace.lints]` con `unwrap_used = "deny"` ni `unsafe_code`. | media | Agregarlos. | resuelto (9315004) |
| RS-3.1 | RS-3 Puertos async | iam/…/ports/outbound/*.rs; communities/…/community_repository.rs:6; los 5 `ports/inbound/*.rs` (`list_public_communities_use_case.rs` se borró en R18 y `verify_access_token_use_case.rs` llegó en R10b) | Todos los puertos son síncronos: un adaptador con E/S real (Postgres) no cabe. Además, Argon2 bloquea el runtime dentro de handlers async. | media | `BoxFuture<'a, Result<…>>` del kernel. | abierto |
| RS-4.1 | RS-4 Tests de arquitectura | Cargo.toml:2-5 | No hay `cargo test -p architecture` (ver VER-1.1). | alta | `tools/architecture`. | resuelto (ad558fc) |

## 3. Reglas que se cumplen

Sin ninguna violación:
- **HEX-3:** ningún `domain` declara puertos ni usa primitivas de E/S. Los traits del dominio son marcadores o value objects, y `PasswordPolicy` es puro.

Partes que sí se cumplen dentro de reglas que tienen hallazgos:
- **HEX-2 (composition root):** `app/src/containers/*.rs` expone `Arc<dyn …UseCase>`, nunca el tipo concreto, y el cableado solo ocurre ahí.
- **HEX-4 (inyección):** los adaptadores HTTP inyectan el `UseCase` (`State<Arc<dyn RegisterAccountUseCase>>`), nunca el handler; cada `UseCase` tiene un solo `execute`.
- **HEX-7 (servicio de dominio):** `PasswordPolicy` no tiene estado ni E/S y no se llama `…Service`. Ningún tipo de `domain` ni de `application` termina en `Service`.
- **ERR-1 (`find_*`):** las búsquedas devuelven `Option`, y es el handler quien decide entre `NotFound` y `Conflict`.
- **ERR-3 (éxito degradado):** no aplica; no hay colaboradores que se llamen después de confirmar el estado.
- **ERR-4 (`HttpStatus` fuera del núcleo):** `StatusCode` solo aparece en `infrastructure/http` y en `app`.
- **TDD-2 (sin framework):** los tests de dominio y de handler no levantan axum ni tokio, y no usan librerías de mocks.
- **RS-3:** no se usa `async-trait`.

## 4. Dudas y hallazgos plausibles sin confirmar

No cuentan en el inventario. Si se confirman, pasan a él con el siguiente ID libre de su regla.

- ~~**Enumeración de usuarios (ERR-4).** `authenticate` responde 404 a un usuario inexistente y 401 a una contraseña incorrecta, así que se puede saber qué nombres de usuario existen.~~ Confirmada por el usuario y resuelta en R2 (ad2f129, 4149660): 401 `IAM_LOGIN_FAILED` y verificación contra un hash señuelo (D6).
- **El señuelo y los parámetros de Argon2 (R2).** `verify` usa los parámetros del hash guardado. Si `Argon2::default()` cambia al actualizar el crate, el señuelo (generado al arrancar con los parámetros nuevos) y los hashes viejos dejan de costar lo mismo. Hoy no pasa; habría que revisarlo al subir `argon2` o al cambiar sus parámetros.
- **`Account` deriva `Debug` con `HashedPassword` dentro** (account.rs:14). Cualquier log futuro con `{:?}` filtra el hash. No viola nada hoy, quitando el `dbg!` de ERR-4.2.
- **Orden en `RegisterAccount`.** Hashea con Argon2 (register_account.rs:49) antes de validar la política de contraseña dentro de la factory. Es orquestación cuestionable (costo y contraseña en claro pasada al dominio), no claramente un `if` de negocio en el handler.
- **Tests de caracterización en `app/tests/`.** Prueban reglas de negocio (longitudes, formatos) por el endpoint, y con adaptadores reales en vez de `Stub…UseCase`. Es la violación típica de TDD-2, pero están ahí a propósito como red de seguridad para el refactor. No los conté.
- **Verificar dos veces** devuelve `InvalidVerification` (400). Podría ser `Conflict`, pero depende de la semántica del producto (ERR-2).
- **Tratar `shared/` como el kernel.** Según rust-stack, los tests de arquitectura ignorarían `shared` como crate *legacy*. Se auditó contra HEX-8 porque hoy cumple ese papel. Si se decide que no es el kernel, HEX-8.1 a HEX-8.9 pasan a ser hallazgos de HEX-1/HEX-2 (un crate compartido entre contextos).
- **`contexts/iam/Cargo.lock` y `.idea/` versionados.** No violan ningún principio; no se contaron.
- **Enumeración por `identify` y `verify` (P1).** `/auth/identify` (público) y `/auth/verify` responden 404 o 200 según exista la cuenta, e `identify` además expone el estado. Queda la enumeración que D6 cerró en el login.
- **Código de verificación sin límite de intentos (P1).** Son 6 dígitos y nada limita los intentos.
- **Nadie entrega el código de verificación (P1).** Ningún puerto lo envía al usuario, así que la verificación solo se completa en los tests.
- **`identify` sin normalizar (P1).** La auditoría de P1 lo reportó como HEX-7 (identify_account.rs:30, 36). No se registra: rige D10, que lo fija con `identify_with_different_email_casing_returns_404`.
- **`HashedPasswordError` como respuesta de cliente (400).** Solo se produce si el hasher devuelve algo raro, así que sería árbol B. Queda dentro de ERR-1.1 y no aparte.

## 5. Plan de rebanadas

Las rebanadas se citan como **R1…R30**. Cada una es un PR que tiene que estar verde en CI. En ese mismo PR se actualiza el inventario (`resuelto (<commit>)`) y, al terminar, se vuelve a auditar con `craft-backend:architecture-reviewer`. El avance de cada rebanada se anota en §5.8.

Datos verificados que condicionan el plan:
- ~~`hex_4_use_case_contracts_do_not_expose_domain_types` no actúa sobre los contextos legacy.~~ Desde S1 (ai-harness `v1.0.0-rc.1`) sí actúa: en legacy, el dominio de application es `crate::domain`. Revisa 17 tipos de contrato y pasa en verde.
- Un contexto legacy no puede depender de un crate `edge/*`. Todas las capas virtuales comparten el manifiesto, y `(Domain, Edge)` está prohibido. En `iam` aparecería una entrada nueva en `contexts/iam/Cargo.toml`. En `communities` seguiría viva la entrada de HEX-1.1.
- `hex_2_core_depends_only_on_std_and_the_kernel` cuenta también las `[dev-dependencies]`. Después de la separación, los tests de application no pueden usar tokio.
- Los tests de caracterización fijan `IAM_INVALID_ACCOUNT_ID(_FORMAT)` en `/communities/create`, los bodies `{code,message}` exactos y el 401 con cuerpo vacío. Además documentan como "CURRENT BEHAVIOR" el duplicado por mayúsculas (HEX-7.1).
- `AccountFactory::reconstitute` y `AccountStatus::from_str` no se usan fuera de los tests: ningún adaptador persiste en texto.
- Decidido por el usuario: `account_lifecycle.rs` admite **adaptación mecánica**.
- `tools/architecture/baseline.txt` tiene 63 líneas: 8 de comentario y **55 entradas**. Coincide con la suma de §5.6. El "56" del encargo original era un error de conteo; ninguna entrada queda sin rebanada. *(En R1. Después de R3 y R4 quedaron 36, y S1 las reescribe como 47 con el formato nuevo. S2 (`v1.0.0-rc.2`) las conserva, idénticas: ver §5.10.)*

### 5.1 Reglas transversales

- **Trinquete.** Cuando un PR mueve o renombra un archivo que figura en `baseline.txt`, en ese mismo PR la violación tiene que desaparecer. Si no, el archivo no se toca. Antes de abrir cada PR: `cargo test -p architecture`.
- **Red de seguridad.** `app/tests/` no se modifica, salvo en los cambios de contrato listados en §5.5.
- **Ritual de rebanada (`/craft-backend:adoption R<n>`).** Lo que este proyecto agrega a la skill. Antes estaba en `.claude/commands/slice.md`:
  - commits en inglés, con Conventional Commits;
  - si la rebanada lo indica en §5.3, se quitan los `#[allow(...)]` que apuntan a sus hallazgos;
  - cada rebanada termina en un PR contra `main`, aunque la invocación no lo pida, con la fila de §5.8 en `Tokens: pendiente` hasta que el usuario pase el `/cost`.
- **`account_lifecycle.rs` (decidido: adaptación mecánica).** Se permite cambiar imports, nombres, `.await`/`block_on` y reemplazar `code()` por un `matches!` sobre la variante equivalente. Ningún escenario ni aserto cambia de significado. El PR enumera cada adaptación.
- **Helpers de `app/tests/common/` (decidido en P1: adaptación mecánica, como lifecycle).** Se permite cambiar imports, nombres, la construcción de la app (como el `.expect("app builds")` de R9) y `.await`, sin que cambie ningún escenario ni aserto. El PR enumera cada adaptación. Los cambios de contrato de §5.5 (p. ej. `assert_error` en R30) siguen siendo cambios de contrato, no adaptaciones.
- **Fallas por operación en los dobles (desde R8).** Cada `Fake…Adapter` tiene `failing()` y `failing_on({Concepto}Operation)`, y cada llamada al puerto de un handler tiene su `fails_with_unexpected_when_<operación>_fails`. `communities` lo adopta en R16.
- **TDD.** El "rojo" de los tests de arquitectura es la entrada de la línea base. Al corregir la violación, el test falla con "Ya no violan: borrar", y el PR borra esa línea. Los tests que hoy afirman sobre `code()` se reemplazan por la versión que afirma sobre la variante: primero se escribe la nueva, en rojo, y después se borra la vieja (ERR-2.6).
- ~~**Verificación manual mientras haya legacy.**~~ Desde S1, el test de contratos (HEX-4) actúa sobre legacy: una filtración de `crate::domain` en `commands/*/{command,response,error}.rs` o en `ports/inbound/` es una violación nueva en la misma rebanada que la introduce. Ya no hace falta el `grep` a mano en R11–R18.
- **Una dependencia, una entrada.** Desde S1, las claves de los tests de dependencias son `ruta/Cargo.toml#sección.paquete`, con `n` = número de capas virtuales que la ven. La rebanada que quita un paquete de un `Cargo.toml` borra sus entradas en el mismo PR, aunque el plan asignara el cierre a otra rebanada.
- **La línea base solo crece en una sincronización con el harness.** Esa sincronización va en un PR propio, sin código de producción, y con el análisis de la línea base vieja frente a la nueva: entradas que se mantienen, las que cambian de clave, las que desaparecen y las nuevas, con ID del inventario y rebanada (ver §5.10). **Nunca en una rebanada.** Si una entrada desaparece sin un cambio de formato que lo explique, el PR se detiene: puede ser un cierre en falso.

### 5.2 Errores y estructura: recomendación

**Migrar los errores antes de separar los crates, en la disposición legacy.** La hipótesis "corregir en el lugar antes de mover" se sostiene, con estos ajustes:

1. **La excepción del HTTP es más chica de lo propuesto.** ERR-4.4 y ERR-4.5 se pueden hacer en el lugar. Cada caso de uso migrado trae su `fold` exhaustivo, en `infrastructure/http/<caso>.rs`, y sus códigos en `infrastructure/http/codes.rs`. Esos archivos no violan nada y se mudan intactos a `{ctx}-api/src`. Lo que sí espera a la separación es lo que necesita `edge/http`: ERR-4.6, ERR-4.7 (la parte de `unexpected()` en los handlers), ERR-4.8, HEX-1.6, HEX-8.6 y HEX-8.8. La razón es la del contexto: un contexto legacy no puede depender de `edge-http` sin agregar una entrada en `contexts/iam/Cargo.toml` o sin bloquear la de `contexts/communities/Cargo.toml`.
2. **Lo estructural por definición.** HEX-1.7, HEX-1.9, HEX-2.1, HEX-2.5 y RS-1.1 se resuelven solo separando crates. Las entradas `hex_2_*` de los `Cargo.toml` de contexto no desaparecen mientras el HTTP comparta manifiesto con el dominio.
3. **Lo que la separación verifica por primera vez.** La separación es la primera verificación real de HEX-4: el test de contratos solo empieza a actuar ahí. Por eso conviene que R27 y R28 muevan código que ya se revisó a mano (§5.1).
4. **Antes de separar.** Hay que sacar del dominio `uuid`, `rand` y `chrono` (R20–R22). Además, los puertos ya tienen que ser async con un `block_on` de std (R23–R24). Si no, los manifiestos nuevos de domain y application nacen con violaciones de `hex_2_core` que la línea base no admite.

Si se hiciera al revés (separar primero), habría que mover las 17 entradas ERR-1 y las 6 HEX-4 a rutas nuevas, todavía violando, y el trinquete lo prohíbe. Las alternativas serían arreglarlas en el PR de la mudanza, que deja de ser chico, o escribir dos veces el mapeo de `SystemError`.

### 5.3 Secuencia

Tamaños: XS (<50 líneas), S (<200), M (<500), L (>500 o muchos archivos).

#### Fase 0: base

**R1. Fijar el nightly.** *Hecha (ver §5.8).*
- **Objetivo:** `rust-toolchain.toml` con `channel = "nightly-AAAA-MM-DD"`. La fecha es la del último CI verde: el `rustc -V` del paso "Install toolchain". Localmente hoy es `2026-03-02`.
- **Resultado:** `channel = "nightly-2026-11-19"`. El último CI verde (run 44647947435, `fix/security-slice-0`, 2026-11-20) usó `rustc 1.102.0-nightly (65ce8611f 2026-11-18)`. rustup publica ese build en el canal del 2026-11-19, un día después de la fecha del commit que muestra `rustc -V`. La toolchain local (`2026-03-02`) era ocho meses más vieja y nunca pasó por CI; se descartó.
- **IDs:** ninguno (higiene de VER-1).
- **Toca:** `rust-toolchain.toml`.
- **Línea base:** sin cambios.
- **Tests:** ninguno nuevo. Se verifica con fmt, clippy y test verdes en CI con la fecha fijada.
- **Riesgo para la caracterización:** nulo.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** XS.

**R2. Sign-in responde 401 a un usuario inexistente (decidido).** *Hecha (ver §5.8).*
- **IDs:** ninguno. Viene de la duda de la §4 confirmada por el usuario.
- **Toca:** `authenticate_account.rs` y sus tests, y el cableado de `AuthenticateAccount` en `app/src/containers/`.
- **Alcance de D6 (decidido: sí, dentro de R2).** Cuando el usuario no existe, el handler igual llama a `verify` del hasher, con la contraseña recibida contra un **hash señuelo**, y descarta el resultado. Así el 401 tarda lo mismo que con una contraseña incorrecta. El composition root genera el señuelo una sola vez al arrancar (`hasher.hash(<valor aleatorio>)`) y se lo pasa al handler, así que tiene los mismos parámetros de Argon2 que los hashes reales. El handler no conoce el formato del hash.
- **Línea base:** sin cambios.
- **Rojo:**
  - test de handler `fails_with_login_failed_when_account_does_not_exist`: hoy da `IAM_ACCOUNT_NOT_FOUND`;
  - test de handler `verifies_against_the_decoy_hash_when_account_does_not_exist` (D6): un doble del hasher que registra sus llamadas tiene que recibir un `verify` con la contraseña y el señuelo. Hoy no recibe ninguno;
  - tests HTTP `authenticate_unknown_username_returns_401_login_failed` y `authenticate_by_email_returns_401`.
- **Contrato HTTP: cambia.** `POST /auth/authenticate` con un usuario inexistente pasa de 404 `IAM_ACCOUNT_NOT_FOUND` a 401 `IAM_LOGIN_FAILED`. Se reemplazan `authenticate_unknown_username_returns_404` y `authenticate_by_email_returns_404`.
- **Fuera de alcance:** un test HTTP que mida tiempos. Sería inestable en CI; el test del handler alcanza para fijar la llamada al señuelo.
- **Tamaño:** S.
- **Resultado:**
  - Commits `ad2f129` (401 + reemplazo de los dos tests HTTP), `4149660` (señuelo) y `8a70093` (`std::hint::black_box` sobre el `verify` descartado, a pedido del revisor).
  - El doble se llama `SpyPasswordHasherPortAdapter` y no `SpyPasswordHashingAdapter`. El test de arquitectura deriva el concepto del puerto actual, `PasswordHasherPort`, y la línea base no admite entradas nuevas. R9 lo renombra junto con el puerto y lo mueve a `application/testing.rs`. Devuelve `true` en `verify` para probar que el handler descarta el resultado.
  - Se borró el test de handler `fails_when_account_not_found`: fijaba el 404 que reemplaza `fails_with_login_failed_when_account_does_not_exist`.
  - `account_lifecycle.rs`: adaptación mecánica (decidida por el usuario en R2). `iam()` genera el señuelo con `hasher.hash("lifecycle-decoy")` y lo pasa a `AuthenticateAccount::new`, más el import de `PasswordHasherPort`. Ningún escenario ni aserto cambia.
  - Los tests nuevos afirman sobre `err.code()`, como el resto del archivo. Pasar a variantes es ERR-2.6, y la migración cubre `fails_with_login_failed_when_account_does_not_exist` y `verifies_against_the_decoy_hash_when_account_does_not_exist`, que afirman el mismo código.
  - Revisión con `craft-backend:architecture-reviewer`: ningún hallazgo bloqueante. Lo que señaló de otras rebanadas ya figura en el inventario y no se corrigió aquí:
    - el sufijo `Port` de `PasswordHasherPort` (HEX-5.1, R9);
    - `FakePasswordHasher` sin estado ni batería (TDD-2.1, TDD-2.3, R9);
    - los `expect` de Argon2, de los que `hash` ahora corre también al arrancar para generar el señuelo (RS-2.4, R9);
    - `if !verify` e `if !can_authenticate` en el handler (HEX-7.2);
    - `SystemError` y los asertos sobre `code()` (ERR-2.1, ERR-2.6, R14).

#### Fase 1: limpieza barata en el lugar

**R3. Los errores dejan de implementar `std::error::Error`.** *Hecha (ver §5.8).*
- **IDs:** ERR-1.2, ERR-1.3.
- **Toca:**
  - `shared/src/error.rs`: `ErrorLayout` pasa a exigir `Debug` en lugar de `Error`;
  - los 17 archivos `*/errors/*.rs`: se quita el `impl`.
- **Línea base: −17.** Son todas las entradas `err_1_*`.
- **Tests:** el trinquete.
- **Riesgo:** nulo.
- **Tamaño:** S.
- **Resultado:**
  - Commit `eb30039`. Ningún tipo usaba `thiserror`, y todos tenían `Display` a mano, así que solo se quitaron los 17 `impl std::error::Error for … {}` vacíos y el `use std::error::Error` de `shared/src/error.rs`.
  - El rojo fue el trinquete: con los `impl` quitados, `err_1_domain_errors_do_not_implement_std_error` (12) y `err_1_application_errors_do_not_implement_std_error` (5) fallaron con "Ya no violan: borrar". Se borraron las 17 líneas de `baseline.txt`.
  - Nada dependía de que fueran `Error`: no hay `?` hacia `anyhow`, ni `Box<dyn Error>`, ni `.source()`. Compiló sin tocar otro archivo.
  - Ningún test cambió, salvo la línea base. `app/tests/` y `account_lifecycle.rs` quedaron intactos.
  - Commit aparte `f552921` (`test(iam)`): se separaron los dos tests de R2. `verifies_against_the_decoy_hash_when_account_does_not_exist` afirma solo la llamada al Spy. `fails_with_login_failed_when_account_does_not_exist` afirma solo la respuesta y ahora usa el Spy, que responde `true` en `verify`, para seguir probando que el handler descarta el veredicto del señuelo (con `FakePasswordHasher` el señuelo da `false` y esa garantía se perdía).
  - Fuera de alcance, sin tocar: `ErrorWithCode` de los tests de los dos `error_mapper.rs` sigue implementando `Error`, que ya no hace falta (se borra con los mappers en R19); `app/src/config/error.rs` (`ConfigError`) implementa `Error` y es del composition root, donde está permitido; `anyhow` en `app/Cargo.toml` no se usa (advertencia `unused_dependencies` de cargo, ya presente en `main`).
  - Revisión con `craft-backend:architecture-reviewer`: sin hallazgos en el diff ni hallazgos nuevos. Lo que señaló de otras rebanadas ya figura en el inventario y no se corrigió aquí:
    - el sufijo `Port` en `SpyPasswordHasherPortAdapter` (HEX-5.1, R9);
    - los asertos sobre `err.code()` (ERR-2.6);
    - la falta de `fails_when_token_generation_fails` (ERR-2.8);
    - `if !account.can_authenticate()` en el handler (HEX-7.2);
    - `ErrorLayout`/`SystemError`, que siguen mezclando los dos árboles (HEX-8.3, ERR-1.1, ERR-2.2; R5 a R19).

**R4. Borrar código muerto del kernel.** *Hecha (ver §5.8).*
- **IDs:** HEX-8.4, HEX-8.7.
- **Toca:** `shared/src/domain/domain_error.rs`, `shared/src/infrastructure/infrastructure_error.rs` y `shared/src/application/common_application_error.rs`.
- **Línea base: −2** (`hex_8_kernel_traits`: `domain_error.rs` e `infrastructure_error.rs`).
- **Tamaño:** XS.
- **Resultado:**
  - Commit `80c00e4`. Se borraron los tres archivos, los `pub mod` de los tres `mod.rs` y los `pub use DomainError` y `pub use InfrastructureError`.
  - Antes de borrar se buscó cada nombre (tipo, trait y módulo) en todo el workspace, tests incluidos. Nadie los usaba: el único rastro era un `use` y un `CommonApplicationError::Infrastructure` **comentados** en `list_public_communities.rs` (líneas 51 y 128), que no compilan y desaparecen con el archivo en R18. No se tocaron.
  - Ningún módulo quedó vacío: `shared/src/domain/` conserva `aggregate_root`, `domain_event`, `entity`, `model` y `value_object`; `shared/src/infrastructure/` conserva `http`; `shared/src/application/` conserva `auth_context` y `ports`. Ninguna otra entrada de la línea base cambió de ruta.
  - El rojo fue el trinquete: `hex_8_kernel_traits_are_model_markers_or_listed_capabilities` falló con "Ya no violan: borrar" para `domain_error.rs` e `infrastructure_error.rs`. Se borraron las 2 líneas de `baseline.txt`; quedan 2 (`token_verifier.rs` y `error.rs`).
  - Ningún test cambió, salvo la línea base. `app/tests/` y `account_lifecycle.rs` quedaron intactos. 188 tests, fmt y clippy en verde.
  - Revisión con `craft-backend:architecture-reviewer`: sin hallazgos en el diff ni hallazgos nuevos. Lo que señaló de otras rebanadas ya figura en el inventario y no se corrigió aquí:
    - las referencias comentadas en `list_public_communities.rs`, dentro del bloque de tests comentado (TDD-1.1, R18);
    - las 2 entradas que quedan de `hex_8_kernel_traits` (`token_verifier.rs` en R10, `error.rs` en R19);
    - `anyhow` sin usar en `app/Cargo.toml` (advertencia de cargo, anotada en R3).

**R5. Primitivas del kernel en `shared`.** *Hecha (ver §5.8).*
- **IDs:** RS-2.1 (parcial).
- **Objetivo:** `shared/src/{fault,failure,id}.rs` copiados de `template/kernel` (`Fault`, `Failure<E>`, `IdGenerator`, `FaultReporter`), con la misma disposición que `kernel/src`, para que R29 sea un `git mv`. Se agrega un puente temporal `SystemError::Unexpected(Fault)`, que los dos mappers responden como 500 con el cuerpo de D7: `{"code":"INTERNAL_ERROR","message":"We couldn't complete your request right now. Please try again."}`. Se borra en R19.
- **Línea base:** sin cambios. Los traits nuevos son capacidades de la lista.
- **Rojo:** los tests del template `display_never_exposes_the_technical_detail`, `question_mark_converts_technical_errors_into_fault` y `map_expected_translates_only_the_expected_tree` (no compilan). Además, un test del mapper: `unexpected_fault_maps_to_internal_server_error`, que afirma el status y el cuerpo exacto de D7.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** S.
- **Resultado:**
  - Commit `e76c162`: `fault.rs`, `failure.rs` e `id.rs` copiados de ai-harness `97a8f2973efd05642f40df39cd8c0671a4fc658c` (`stacks/rust/template/kernel/src`), con sus tests y la misma disposición plana. La única diferencia con el template es que el `rustfmt.toml` del repo (`imports_granularity = "Crate"`) une los dos `use std::…` de `fault.rs`. `lib.rs` declara los módulos privados y reexporta `Failure`, `Fault`, `FaultReporter` e `IdGenerator`, como el template.
  - `Clock` no se copió: el template no lo define (su README dice "aún sin uso en la plantilla"). Decidido por el usuario: entra en R21, con su primer uso.
  - `shared/Cargo.toml` no cambia. `hex_8_kernel_depends_only_on_std` y `hex_8_kernel_traits_…` siguen con sus entradas (1 y 2); el segundo revisa 8 traits en lugar de 6.
  - Commit `29890cb`: `SystemError::Unexpected(Fault)`. Los dos `error_mapper.rs` lo responden con `unexpected()`, que da 500 con el cuerpo de D7. El detalle del `Fault` va a stderr con `eprintln!` (no hay `tracing` en el proyecto), marcado como interino hasta R26 (ERR-4.7). Ese registro no tiene test.
  - Commit `446726b`: a pedido del revisor, el código de D7 vive en `SystemError::UNEXPECTED_CODE`, que usan `code()` y los dos mappers. Así el test del mapper cubre también la rama de `code()`. Se comprobó que el test falla si cambia el literal.
  - Rojo: con solo los tests del template, `shared` no compila (faltan `Fault` y `Failure`). `unexpected_fault_maps_to_internal_server_error` (iam y communities) no compila sin la variante. Con un brazo provisorio que respondía 500 sin cuerpo, falla por el cuerpo (`left: ""`). El test lee el cuerpo con un `poll` y `Waker::noop()` de std, para no agregar `tokio` a las dev-dependencies de los contextos (`hex_2_core` las cuenta).
  - 194 tests (188 + 4 del template + 2 de los mappers), fmt y clippy en verde. `app/tests/` y `account_lifecycle.rs` quedaron intactos.
  - Revisión con `craft-backend:architecture-reviewer`: aplicado el hallazgo de `code()` (arriba). Lo demás queda para otras rebanadas:
    - el `eprintln!` escribe solo el `Display` del error de primer nivel, sin la cadena de `source()`; `StderrFaultReporterAdapter` debería recorrerla (R26);
    - `unexpected()` está duplicado en los dos mappers (R19 retira los mappers; R26/R30 dan el borde común);
    - el `Debug` de `Fault` imprime el `source` (igual que el template): un `{:?}` sobre `SystemError` expondría el detalle. Hoy nada en producción lo hace.

**R6. Herencia del workspace.** *Hecha (ver §5.8).*
- **IDs:** RS-1.3 y RS-1.4 (quitar `anyhow` de `app`).
- **Toca:** todos los `Cargo.toml`, con `[workspace.package]` y las dependencias externas en `[workspace.dependencies]`. No cambia ninguna ruta.
- **Línea base:** sin cambios.
- **Riesgo:** el `Cargo.lock`; se verifica con `--locked`.
- **Tamaño:** S.
- **Resultado:**
  - Commit `991e672`. `[workspace.package]` con `version` y `edition`; los seis crates los heredan (`publish = false` sigue en `architecture`). Las 17 dependencias externas pasan a `[workspace.dependencies]`, con las features que comparten todos sus usuarios. La única feature propia de un crate es `util` de `tower`, en las `[dev-dependencies]` de `app`.
  - Cuando dos crates pedían requisitos distintos (`serde` `"1"` y `"1.0.228"`, `tower` `"0.5"` y `"0.5.3"`), el workspace toma el más específico. Ya era el que fijaba la resolución, así que la versión resuelta no cambia.
  - Verificado contra `main`: el `Cargo.lock` solo pierde `anyhow`; `cargo tree --workspace -e features,normal,dev,build` es idéntico salvo `anyhow`, y las claves de `[dependencies]`, `[dev-dependencies]` y `[build-dependencies]` de cada manifiesto, que es lo único que leen los tests `hex_1_*`, `hex_2_*` y `hex_8_*` (`Crate::dependencies`), también: solo falta `anyhow`. `dep.workspace = true` conserva la clave, así que ningún test perdió detección.
  - Sin rojo: es un refactor de manifiestos sin cambio de comportamiento, y la línea base no cambia. Desaparece el aviso `unused dependency` de `anyhow`.
  - `contexts/iam/Cargo.lock` (versionado, §4) no se tocó.
  - Revisión con `craft-backend:architecture-reviewer`: sin hallazgos en el diff. Señaló dos huecos de verificación (VER-1), que no son violaciones actuales y quedan fuera de esta rebanada porque cerrarlos exige tests nuevos en `tools/architecture`:
    - `Crate::dependencies` (tests/common/mod.rs:69) lee solo la clave y no resuelve `package =`. Con la herencia, un renombre en la raíz (`foo = { package = "axum" }`) más `foo.workspace = true` en domain o application pasaría `hex_2_*`. Hoy no hay ningún `package =` en el workspace. Corrección propuesta: resolver `workspace = true` contra `[workspace.dependencies]` de la raíz y usar su `package`, o leer `cargo metadata`.
    - Ningún test exige `workspace = true` en las dependencias externas de los miembros, así que un crate nuevo puede volver a fijar una versión propia (RS-1.3) sin que nada falle.
    - `iam` en `contexts/communities/Cargo.toml`, que ya figura como HEX-1.1 (R16).

#### Fase 2: puertos salientes, uno por rebanada

Patrón de cada rebanada:
- se quita el sufijo `Port`;
- la firma pasa a `Result<Option<T>, Fault>` o `Result<T, Fault>` y se borra el error de puerto;
- el doble sale del archivo del puerto a `application/testing.rs` (`#[cfg(any(test, feature = "testing"))]`), con un nombre de TDD-2 y estado real;
- se crea la batería `application/testing/contracts/<puerto>.rs` y corre contra el doble y contra el adaptador;
- el adaptador toma el nombre `{Tecnología}{Concepto}Adapter` en un archivo con ese nombre, que reemplaza al viejo;
- los `expect` de los mutex pasan a `"invariante: mutex sin envenenar"`.

Una falla del puerto llega al handler todavía no migrado como `SystemError::Unexpected` (R5).

**R7. `CommunityRepository`.** *Hecha (ver §5.8).*
- **IDs:**
  - HEX-5.1 y HEX-5.2 (communities);
  - ERR-1.7, ERR-3.2 y ERR-3.3 (communities);
  - TDD-2.1, TDD-2.2 y TDD-2.3 (communities);
  - TDD-1.2 y RS-2.4 (communities);
  - TDD-2.5 y RS-2.2 (`get_public_list`), ya cerrados por R18 (D3 decidida: eliminar). R18 (a603969) borró también `find_by_id` del puerto, del doble y del adaptador. La batería cubre solo lo que queda: `save` y `find_by_slug`.
- **Precondición:** R18 hecha *(cumplida en a603969)*.
- **Línea base: −3:**
  - `hex_5_outbound` y `hex_5_adapters` de `ports/outbound/community_repository.rs`;
  - `hex_5_adapters` de `in_memory/community_repository.rs`.
- **Rojo:**
  - batería `saves_and_finds_by_slug` contra el doble actual, que no guarda nada;
  - test de handler `fails_with_unexpected_when_repository_fails`.
- **Contrato HTTP: cambia (sin cobertura en caracterización).** El 500 por falla del repositorio deja de llevar `COMMUNITIES_REPOSITORY_ERROR` y pasa al cuerpo interino de D7 (`INTERNAL_ERROR`).
- **Tamaño:** M.
- **Resultado:**
  - Commit `692bc7c`: `CommunityRepositoryPort` pasa a `CommunityRepository`, con `find_by_slug -> Result<Option<Community>, Fault>` y `save -> Result<(), Fault>`. Se borran `CommunityRepositoryError` y el código `COMMUNITIES_REPOSITORY_ERROR` (constante y brazo del mapper). El handler, todavía con `SystemError` (R17), puentea con `.map_err(SystemError::Unexpected)?`. Rojo: `fails_with_unexpected_when_repository_fails` falló con `Err(Application(CommunityRepositoryError("Unexpected error")))`. Reemplaza a `fails_when_repository_fails`, que afirmaba el código borrado. En el test del mapper, `technical_and_unknown_errors_map_to_internal_server_error` pasa a `unknown_errors_map_to_internal_server_error`: el código técnico ya no existe, y un `for` de un solo elemento no pasa clippy.
  - Commit `9eacddc`: batería en `application/testing/contracts/community_repository.rs` (macro `community_repository_contract!`) y el doble en `application/testing.rs`, con `#[cfg(any(test, feature = "testing"))]` y la feature `testing = []` nueva en `contexts/communities/Cargo.toml`. `FakeCommunityRepository` (con flags, y un `save` que no guardaba) pasa a `FakeCommunityAdapter` con estado (`empty()`, `failing()`). Rojo contra el doble viejo: `saves_and_finds_by_slug`, `finds_each_community_by_its_own_slug` y `save_replaces_the_community_with_the_same_id` fallaron con `left: None`. Los cinco casos son los del template. Son síncronos, sin `run`, porque los puertos son síncronos hasta R24. `fails_when_slug_already_exists` ya no usa un flag: crea la primera comunidad con el mismo handler. `Community::dummy_community` y `dummy_private_community` se quedaron sin llamador y se borraron.
  - Commit `6b808a2`: `InMemoryCommunityRepository` pasa a `InMemoryCommunityAdapter`, en `in_memory/in_memory_community.rs`, que reemplaza a `community_repository.rs` (el mismo nombre de archivo que en el template). Corre la batería. El `expect` del mutex dice `"invariante: mutex sin envenenar"`. El composition root cablea el nombre nuevo. Rojo: con su entrada borrada de la línea base, `hex_5_adapters` lo reportó como violación nueva (`debe llamarse {Tecnología}CommunityAdapter`).
  - Commit `ba15189`, a pedido del revisor: `fails_with_unexpected_when_save_fails`, con un `StubCommunityAdapter` local que responde `find_by_slug` y falla solo en `save`. `FakeCommunityAdapter::failing()` falla antes, en `find_by_slug`, así que ningún test llegaba al puente de `save`. Se comprobó que el test pasa a rojo (con `Ok(CreateCommunityResponse {…})`) si se descarta el `Fault` con `let _ = save(…)`.
  - **Desviación del encargo:** el encargo pedía `InMemoryCommunityRepositoryAdapter`. Se usó `InMemoryCommunityAdapter`, como dicen HEX-5 y la fila HEX-5.2: el Concepto va sin el rol, y `hex_5_adapters` habría reportado el otro nombre como violación nueva, que la línea base no admite. El revisor lo confirmó.
  - `FakeCommunityAdapter` e `InMemoryCommunityAdapter` quedan casi iguales. Es lo esperable mientras no haya persistencia real, y no se fusionan: uno es un doble de application y el otro, un adaptador de infraestructura. Los dos lo dicen en su doc.
  - **Línea base: −3 (46 → 43 entradas, 64 → 61 violaciones).** Desaparecen:
    - `hex_5_outbound_ports_are_named_by_role contexts/communities/src/application/ports/outbound/community_repository.rs` (692bc7c);
    - `hex_5_adapters_are_named_by_technology_and_concept contexts/communities/src/application/ports/outbound/community_repository.rs` (9eacddc);
    - `hex_5_adapters_are_named_by_technology_and_concept contexts/communities/src/infrastructure/persistence/in_memory/community_repository.rs` (6b808a2).
    Las tres vienen de R1 (`ad558fc`). S1 (`79185ff`) y S2 (`75e262d`) las conservaron con la misma clave, y ninguna de las dos agregó entradas sobre estos archivos. Las entradas de `contexts/communities/Cargo.toml` (`hex_1` de `iam` y `hex_2` de `axum`, `http`, `serde` y `uuid`) no cambian: agregar `[features]` no modifica ninguna dependencia.
  - `app/tests/` y `account_lifecycle.rs` quedaron intactos. 244 tests, fmt y clippy en verde localmente.
  - Revisión con `craft-backend:architecture-reviewer`: aplicado el hallazgo de `save` (arriba). Lo demás queda para otras rebanadas:
    - la batería construye el agregado con `iam::AccountId::generate` (`testing/contracts/community_repository.rs:14`), porque `Community::create` lo exige. No cambia la entrada de `hex_1`, pero es un archivo más de application que importa `iam` y que se compila con la feature `testing`. Se agrega a R16;
    - el puerto recibe `slug: &str` y no `&CommunitySlug`, como en el template. Ningún principio lo exige; puede cambiar con R24, cuando la firma pase a `BoxFuture`;
    - `SystemError`, los asertos sobre `code()` y `CreateCommunity` sin el sufijo `Handler` (R17, R19); `CommunityId::generate` en la batería (R22).

**R8. `AccountRepository`.** *Hecha (ver §5.8).*
- **IDs:**
  - HEX-5.1 y HEX-5.2 (iam);
  - ERR-1.7, ERR-3.2 y ERR-3.3 (iam);
  - TDD-2.1, TDD-2.2 y TDD-2.3 (iam);
  - TDD-2.4 (el email del doble);
  - TDD-1.2 y RS-2.4 (iam).
- **Línea base: −3:** `hex_5_outbound` y `hex_5_adapters` de `ports/outbound/account_repository.rs`, y `hex_5_adapters` de `in_memory/account_repository.rs`.
- **Rojo:** batería `finds_by_email_what_was_saved`. Hoy falla contra el doble: `save` no guarda y `find_by_email` devuelve otra cuenta.
- **Caracterización:** lifecycle se adapta (nombre del puerto y del adaptador, `find_by_email` con `Result`).
- **Contrato HTTP:** desaparece `IAM_ACCOUNT_REPOSITORY_ERROR`, igual que en R7.
- **Tamaño:** M.
- **Resultado:**
  - Commit `ebd0819`: `AccountRepositoryPort` pasa a `AccountRepository`, con `find_by_username` y `find_by_email -> Result<Option<Account>, Fault>` y `save -> Result<(), Fault>`. Se borran `AccountRepositoryError` y el código `IAM_ACCOUNT_REPOSITORY_ERROR` (constante, brazo del mapper y entrada de la lista del test del mapper). Los handlers, todavía con `SystemError` (R11 a R19), puentean cada llamada con `.map_err(SystemError::Unexpected)?`. Se agrega la feature `testing = []` a `contexts/iam/Cargo.toml` y `application/testing.rs`.
    - Un test `fails_with_unexpected_when_<operación>_fails` por cada llamada al puerto: `RegisterAccount` (`find_by_email`, `find_by_username`, `save`), `VerifyAccount` (`find_by_email`, `save`), `AuthenticateAccount` (`find_by_username`) e `IdentifyAccount` (`find_by_email`, `find_by_username`). Son 8.
    - Rojo primero: `fails_with_unexpected_when_save_fails` (register) reemplaza a `fails_when_repository_fails`, que afirmaba `IAM_ACCOUNT_REPOSITORY_ERROR`, y falló contra el código de `main` con `Err(Application(AccountRepositoryError("FakeAccountRepository error")))`. Después se cambió la firma con los `find_*` provisoriamente tragados (`.ok().flatten()`), y los seis tests de `find_*` fallaron con `Ok(…)`, `Err(Application(LoginFailed))` o `Err(Domain(AccountNotFound))`; con el `?` pasaron a verde. `fails_with_unexpected_when_save_fails` (verify) llegó con el puente ya escrito: se comprobó por mutación (`let _ = save(…)` da `Ok(())`).
  - Commit `de6c308`: batería en `application/testing/contracts/account_repository.rs` (macro `account_repository_contract!`), con 8 casos: buscar por email y por username lo guardado, `None` para lo inexistente (uno por búsqueda), comparar el valor entero (uno por búsqueda), varias cuentas y que `save` reemplace por id. `FakeAccountRepository` (flags, un `save` que no guardaba y un `find_by_email` que devolvía otra cuenta) pasa a `FakeAccountAdapter` en `application/testing.rs`, con estado y `empty()`/`containing()`/`failing()`. Los tests de los handlers guardan cuentas reales en el Fake.
    - Rojo contra el doble viejo (`FakeAccountRepository::success()`): fallaron 4 de 8 con `left: None`, todos porque `save` no guardaba: `finds_by_email_what_was_saved`, `finds_by_username_what_was_saved`, `finds_each_account_by_its_own_email_and_username` y `save_replaces_the_account_with_the_same_id`. Los otros 4 (`unknown_*`, `*_matches_the_whole_*`) pasaban, porque un doble que nunca encuentra nada los cumple.
    - La otra mitad de TDD-2.4 no aparece con la fábrica vacía. Se provocó aparte, sin commitearla, con `with_existing_email("ana@example.com")`: `finds_by_email_what_was_saved` devolvió `john_doe`/`john@example.com`.
  - Commit `9a90d8a`: `InMemoryAccountRepository` pasa a `InMemoryAccountAdapter`, en `in_memory/in_memory_account.rs`, que reemplaza a `account_repository.rs`. Corre la batería, y el `expect` del mutex dice `"invariante: mutex sin envenenar"`. El composition root cablea el nombre nuevo. Rojo: con su entrada borrada de la línea base, `hex_5_adapters` lo reportó como violación nueva (`debe llamarse {Tecnología}AccountAdapter`).
  - Commit `cc3d60c`, a pedido del revisor: las fallas de una sola operación pasan a `FakeAccountAdapter::failing_on(AccountOperation)`, y se borra el `StubAccountAdapter` de `ebd0819`. El Stub devolvía la cuenta configurada para cualquier valor buscado (el defecto de TDD-2.4) y no pasaba la batería. `failing()` sigue haciendo fallar todo, como en el template. Por mutación, tragar el `Fault` de `find_by_username` (register) o de `save` (verify) sigue poniendo sus tests en rojo.
  - **Fallas en los dobles.** El `FakeCommunityAdapter` del template tiene `failing()`, así que se siguió, y se anotó la fricción en §6: con `failing()` solo, ninguna llamada posterior a la primera tiene test. En R7 eso se resolvió con un Stub local (ba15189).
  - `account_lifecycle.rs`, adaptación mecánica: `AccountRepositoryPort` → `AccountRepository` en el import (ebd0819); un `.expect("repository answers")` y un `.unwrap()` más sobre el `Result` de `find_by_email`, en `delivered_code` y `password_is_not_stored_in_plain_text` (ebd0819); `InMemoryAccountRepository` → `InMemoryAccountAdapter`, con su ruta nueva (9a90d8a). Ningún escenario ni aserto cambia.
  - **Línea base: −3 (43 → 40 entradas, 61 → 58 violaciones).** Desaparecen:
    - `hex_5_outbound_ports_are_named_by_role contexts/iam/src/application/ports/outbound/account_repository.rs` (ebd0819);
    - `hex_5_adapters_are_named_by_technology_and_concept contexts/iam/src/application/ports/outbound/account_repository.rs` (de6c308);
    - `hex_5_adapters_are_named_by_technology_and_concept contexts/iam/src/infrastructure/persistence/in_memory/account_repository.rs` (9a90d8a).
    Agregar `[features]` a `contexts/iam/Cargo.toml` no cambia ninguna dependencia ni sus entradas.
  - `app/tests/` quedó intacto. 267 tests, fmt y clippy en verde localmente.
  - Revisión con `craft-backend:architecture-reviewer`: aplicado el hallazgo del Stub (arriba). Confirmó que cada uno de los 8 tests llega a su llamada y que ningún `Fault` se traga. Lo demás queda para otras rebanadas:
    - la batería fija la coincidencia exacta, sin mayúsculas ni espacios; cuando R12 normalice con `Email`/`Username`, conviene sumarle un caso (HEX-7.1);
    - el brazo `IAM_TOKEN_GENERATOR_ERROR => 500` del mapper sobra, porque el comodín ya da 500 (ERR-4.5, R10);
    - `SpyPasswordHasherPortAdapter`, `FakePasswordHasher` y la contraseña igual al hash en los tests de authenticate (R9); `TokenGeneratorPort` (R10); `SystemError`, los asertos sobre `code()` y los nombres sin `Handler` (R11 a R19); `if !account.can_authenticate()` (HEX-7.2, R14).

**R9. `PasswordHashingService`.** *Hecha (ver §5.8).*
- **IDs:** HEX-5.1, HEX-5.2 (Argon2), RS-2.4 (Argon2, a `Fault`), TDD-2.3, TDD-2.4 y TDD-1.3 (Argon2).
- **Cambios:**
  - `hash` pasa a `Result<String, Fault>` y `verify` a `Result<bool, Fault>`;
  - un `FixedPasswordHashingAdapter` coherente, con el hash derivado de la contraseña;
  - el `SpyPasswordHasherPortAdapter` de R2 sale de los tests de `authenticate_account.rs` a `application/testing.rs` como `SpyPasswordHashingAdapter`;
  - el señuelo de R2: con `hash` falible, el composition root propaga el `Fault` al arrancar. Hay que decidir qué hace el handler si el `verify` del señuelo devuelve `Fault`.
- **Línea base: −2:** `hex_5_outbound` y `hex_5_adapters` de `password_hasher.rs`. Y **−1:** `hex_5_adapters` de `argon2_password_hasher.rs`.
- **Rojo:**
  - batería `verify_accepts_the_hashed_password_and_rejects_another`, contra el doble actual (compara texto plano con el hash) y contra Argon2;
  - `corrupt_hash_is_a_fault` en Argon2.
- **Tests de handler:** el test de éxito de authenticate deja de mandar el hash como contraseña. Se corrige un doble infiel, no se ajusta un test para que pase.
- **Tamaño:** M.
- **Resultado:**
  - Commit `73f9c44`: `PasswordHasherPort` pasa a `PasswordHashingService`, con `hash -> Result<String, Fault>` y `verify -> Result<bool, Fault>`. `RegisterAccount` propaga el `Fault` de `hash`, y `AuthenticateAccount` el de los dos `verify` con `.map_err(SystemError::Unexpected)?`. En el del señuelo (D6) solo se descarta el `bool`, dentro de `black_box` (decidido por el usuario). El `SpyPasswordHasherPortAdapter` de R2 pasa a `SpyPasswordHashingAdapter`. Los adaptadores solo se envolvieron en `Ok`: sin cambio observable, porque ninguno podía fallar todavía.
    - **Arranque:** si el `hash` del señuelo da `Fault`, el servidor no arranca. `IamContainer::initialize`, `AppContainer::build` y `build_app` devuelven `Result`, y `main` termina con `exit(1)`, sin `expect`. **Excepción a la red de seguridad (decidida por el usuario):** `app()` de `app/tests/common/mod.rs` agrega `.expect("app builds")`. Ningún escenario ni aserto cambia. Esa propagación no tiene test: `initialize` construye Argon2 por dentro y no admite un doble que falle.
  - Commit `c04f69d`: `FakePasswordHasher` (hash constante, y `verify` que comparaba el texto plano con el hash) pasa a `FixedPasswordHashingAdapter` en `application/testing.rs`. El hash se deriva de la contraseña, `verify` solo acepta la contraseña con la que se generó y un hash que no produjo es `Fault`. Tiene `failing()` y `failing_on(PasswordHashingOperation)`, como `FakeAccountAdapter`. `SpyPasswordHashingAdapter` registra cada `verify` y responde como el Fixed. Batería `application/testing/contracts/password_hashing_service.rs` (macro `password_hashing_service_contract!`), contra los dos dobles. El puerto pasa a `ports/outbound/password_hashing_service.rs`.
    - Rojo contra el doble viejo: `verify_accepts_the_hashed_password_and_rejects_another` ("rechazó su contraseña", el previsto) y **además** `corrupt_hash_is_a_fault` (`Ok(false)`: el doble viejo nunca fallaba). `hash_is_not_the_password` y `hash_is_a_valid_hashed_password` pasaban.
    - Los tests de authenticate guardan un hash del Fixed y mandan la contraseña real. **Se corrige un doble infiel, no se ajusta un test para que pase.** `fails_with_login_failed_when_account_does_not_exist` sigue probando que se descarta el veredicto del señuelo: el señuelo es el hash de la contraseña enviada, así que `verify` responde `true`.
    - `fails_with_unexpected_when_hash_fails` (register), `fails_with_unexpected_when_verify_fails` y `fails_with_unexpected_when_decoy_verify_fails` (authenticate). El `?` ya estaba desde `73f9c44`, así que el rojo se vio tragando el `Fault` en el árbol de trabajo, sin commitearlo: `Err(Domain(Invalid))` y dos veces `Err(Application(LoginFailed))`. El revisor señala que el historial muestra la producción antes que el test (ver §6).
  - Commit `17ba68e`: `Argon2PasswordHasher` pasa a `Argon2PasswordHashingAdapter`, en `security/argon2_password_hashing.rs`, que reemplaza a `security/password_hasher/`. Los errores de Argon2 pasan a `Fault` con contexto (RS-2.4), y solo `password_hash::Error::Password` es `Ok(false)`. Corre la batería (TDD-1.3). Rojo: `corrupt_hash_is_a_fault` panicó en el `expect` (`PhcStringField`). `password_hash::Error` no implementa `std::error::Error` sin la feature `std`: se envuelve con `Fault::new`, y los manifiestos no cambian.
  - Commit `dddc146`, a pedido del revisor: `verify_password` también responde `Error::Password` cuando el hash guardado no tiene sal o salida. Un hash truncado (que pasa `HashedPassword`) daba un 401 permanente sin `Fault`. Nuevo caso `truncated_hash_is_a_fault`: el hash del propio adaptador sin su última sección `$`. Rojo contra Argon2: `$argon2id$v=19$m=19456,t=2,p=1$<sal>: Ok(false)`. Los dobles ya lo pasaban.
  - `account_lifecycle.rs`, adaptación mecánica: el import `password_hasher::PasswordHasherPort` → `password_hashing_service::PasswordHashingService` (73f9c44, c04f69d); `.expect("decoy hash")` sobre el `hash` del señuelo (73f9c44); `Argon2PasswordHasher` → `Argon2PasswordHashingAdapter`, con su ruta nueva (17ba68e). Ningún escenario ni aserto cambia.
  - **Línea base: −3 (40 → 37 entradas, 58 → 55 violaciones).** Desaparecen:
    - `hex_5_outbound_ports_are_named_by_role contexts/iam/src/application/ports/outbound/password_hasher.rs` (73f9c44);
    - `hex_5_adapters_are_named_by_technology_and_concept contexts/iam/src/application/ports/outbound/password_hasher.rs` (c04f69d);
    - `hex_5_adapters_are_named_by_technology_and_concept contexts/iam/src/infrastructure/security/password_hasher/argon2_password_hasher.rs` (17ba68e).
  - 285 tests, fmt y clippy en verde localmente.
  - Revisión con `craft-backend:architecture-reviewer`: aplicado el hallazgo del hash truncado (arriba). Confirmó las restricciones del encargo (señuelo al arrancar y en authenticate, un test por llamada que llega a su llamada, lifecycle mecánico, nombres). Lo demás queda para otras rebanadas:
    - `RegisterAccount` hashea antes de que la factory valide la política de contraseña (§4, "Orden en `RegisterAccount`"); encaja en R11 o en una rebanada de HEX-7;
    - ningún test de arquitectura exige que cada `expect` de producción empiece con `"invariante:"` (VER-1, se propone al harness);
    - `SystemError`, `code()`, los nombres sin `Handler` y `if !account.can_authenticate()` (R11 a R19, HEX-7.2); `TokenGeneratorPort` (R10).

**R10. `AccessTokenService` y la query `VerifyAccessToken` en iam.** *Hecha en dos partes, R10a y R10b (ver §5.8).*
- **IDs:**
  - HEX-1.5, HEX-8.5, HEX-5.1 y HEX-5.2 (JWT);
  - ERR-3.1, ERR-3.3 (token);
  - TDD-2.1, TDD-1.3 (JWT) y TDD-2.2.
- **Cambios:**
  - `TokenGeneratorPort` y `TokenVerifier` se fusionan en un puerto `AccessTokenService`: `issue(&str) -> Result<String, Fault>` y `verify(&str) -> Result<AccessTokenVerdict, Fault>`, con `ports/outbound/dtos/access_token_payloads.rs`;
  - se crean `VerifyAccessTokenUseCase` y `VerifyAccessTokenHandler` en `queries/verify_access_token/`;
  - el middleware de `app` depende de `Arc<dyn VerifyAccessTokenUseCase>` (sigue en `app` hasta R28);
  - se borran `shared/src/application/ports/outbound/{token_verifier,error}.rs`;
  - `jwt_service/` pasa a `jwt_access_token.rs` y se quita el `module_inception`.
- **Línea base: −4:**
  - `hex_5_outbound` y `hex_5_adapters` de `token_generator.rs`;
  - `hex_5_adapters` de `jwt_service.rs`;
  - `hex_8_kernel_traits` de `token_verifier.rs`.
- **Rojo:**
  - `expired_token_is_an_invalid_verdict`: hoy es `Err`;
  - tests del handler (`fails_…`, `returns_account_id_for_valid_token`).
- **Caracterización:** sin cambios. Los 401 siguen iguales. Lifecycle se adapta.
- **Contrato HTTP:** desaparece `IAM_TOKEN_GENERATOR_ERROR`.
- **Tamaño:** M/L.
- **División (decidida por el usuario: si el diff pasa de unas 500 líneas, dos PRs, cada uno en verde y saliendo de `main`).** La parte (a) pasaba de 700 líneas.
  - **R10a:** puerto, adaptador y batería, con el middleware de `app` usando `AccessTokenService` directamente. *Hecha (ver §5.8).*
  - **R10b:** `VerifyAccessTokenUseCase` y `VerifyAccessTokenHandler` en `queries/verify_access_token/`, el middleware sobre la query con un 500 por `Fault` (`a_fault_verifying_the_token_returns_500`, ver §5.5), `fails_with_unexpected_when_verify_fails` en el handler, y el borrado de `shared/src/application/ports/outbound/{token_verifier,error}.rs` (con `shared::application`, que queda vacío). Línea base: −1 (`hex_8_kernel_traits` de `token_verifier.rs`). Sale de `main` cuando R10a esté fusionada.
- **Resultado (R10a):**
  - **Secuencia de commits (propuesta del revisor en R9, §6):** el commit que cambia la firma a `Result<_, Fault>` descarta el `Fault` de forma explícita en los llamadores, y el `?` llega en un commit posterior, junto con su test en rojo. El rojo se reproduce contra el commit padre.
  - Commit `517e4cd`: `TokenGeneratorPort` y `TokenVerifier` se fusionan en `AccessTokenService` (`issue -> Result<String, Fault>`, `verify -> Result<AccessTokenVerdict, Fault>`). El veredicto va en `ports/outbound/dtos/access_token_payloads.rs`: `Valid { account_id }` e `Invalid`, sin el motivo. `AuthenticateAccount` traduce el `Fault` de `issue` al `TokenGeneratorError` de antes. El middleware responde 401 a `Invalid` y a `Err`, como antes. `JwtService` es una traducción mecánica: todo token rechazado sigue siendo `Err`. Rojo: el trinquete (`hex_5_outbound` de `token_generator.rs`).
  - Commit `ce6d64b`: batería `application/testing/contracts/access_token_service.rs` (macro `access_token_service_contract!`), con 6 casos: un token emitido es válido para su cuenta, cada token trae su cuenta, y uno mal formado, alterado, de otro emisor o vencido es `Ok(Invalid)`. Además de la fábrica, la macro recibe `expired_token` (el puerto no puede emitir un token vencido) y `another_issuer`. `FakeTokenGenerator` (devolvía la constante `"valid_token"` y nunca aceptaba nada) pasa a `FakeAccessTokenAdapter` en `application/testing.rs`, con estado, `issue_expired`, `failing()` y `failing_on(AccessTokenOperation)`. El puerto pasa a `ports/outbound/access_token_service.rs`.
    - Rojo contra el doble viejo: 2 de 6 con `left: Some(Invalid)` (`issued_token_is_valid_for_its_account` y `each_token_carries_its_own_account`). Los 4 casos de `Invalid` pasaban: un doble que rechaza todo los cumple.
    - El test de éxito de authenticate deja de afirmar la constante del doble viejo y afirma que el token identifica a la cuenta. **Se corrige un doble infiel, no se ajusta un test para que pase.**
  - Commit `65f455d`: `fails_with_unexpected_when_issue_fails`, con `failing_on(AccessTokenOperation::Issue)`. Rojo contra `ce6d64b`: `Err(Application(TokenGeneratorError("")))`. Ahora `.map_err(SystemError::Unexpected)?` (el puente de R5) da el 500 de D7. Se borran `TokenGeneratorError` y `IAM_TOKEN_GENERATOR_ERROR` (la constante, el brazo del mapper y la entrada del test del mapper). Con solo `"UNKNOWN"`, `technical_and_unknown_errors_map_to_internal_server_error` pasa a `unknown_errors_map_to_internal_server_error`, como en R7.
  - Commit `c9db5fa`: `JwtService` pasa a `JwtAccessTokenAdapter`, en el archivo plano `infrastructure/services/jwt_access_token.rs`. Desaparecen `jwt_service/` y su `module_inception`, y `Claims` queda privado. Corre la batería. **Rojo: 4 de 6, no solo el previsto.** Todos son `Err(Fault)`, porque el `TokenVerifier` viejo rechazaba cualquier token con `Err` y la traducción mecánica lo conservó:
    - `expired_token_is_an_invalid_verdict`: `ExpiredSignature` (el previsto);
    - `malformed_token_is_an_invalid_verdict`: `InvalidToken`;
    - `tampered_token_is_an_invalid_verdict`: `InvalidSignature`;
    - `token_from_another_issuer_is_an_invalid_verdict`: `InvalidSignature`.
    `issue_is_a_fault_when_the_expiry_overflows_the_clock` fija el `Fault` de `issue` (un `ttl` que desborda el reloj). Ya se cumplía.
  - Commit `1b08cea`, a pedido del revisor: los tokens de `FakeAccessTokenAdapter` son únicos entre instancias (un contador del proceso), y se quita `with_key`. La "clave" era solo un prefijo, y cada instancia numeraba desde 0. Un verificador que había emitido `k.0` para Bob respondía `Valid { Bob }` al `k.0` que otra instancia había emitido para Alice. `token_from_another_issuer_is_an_invalid_verdict` ahora hace que el verificador emita antes un token propio. Se comprobó por mutación: con la numeración por instancia, el test falla con `Ok(Valid { account_id: <Bob> })`.
  - Commit `62d5456`, a pedido del revisor: `verify` distingue por `ErrorKind`. Los errores del token (formato, firma, algoritmo, vencimiento, claims, codificación) dan `Ok(Invalid)`; cualquier otro, `Fault`. Así una clave o una validación mal configurada da un 500, no un 401 para todos. Rojo: `a_misconfigured_validation_is_a_fault_not_an_invalid_verdict` (una validación sin algoritmos, `MissingAlgorithm`) dio `Ok(Invalid)`.
  - `account_lifecycle.rs`, adaptación mecánica:
    - imports: `TokenVerifier` → `AccessTokenService`, con su ruta (517e4cd, ce6d64b);
    - `JwtService` → `JwtAccessTokenAdapter`, con su ruta (c9db5fa);
    - el aserto `iam.jwt.verify(&token).unwrap().account_id == account_id` pasa a comparar el veredicto, `== AccessTokenVerdict::Valid { account_id }` (517e4cd).
    Ningún escenario ni aserto cambia de significado. `app/tests/`, incluido `common/mod.rs`, quedó intacto.
  - **Línea base: −3 (37 → 34 entradas, 55 → 52 violaciones).** Desaparecen:
    - `hex_5_outbound_ports_are_named_by_role contexts/iam/src/application/ports/outbound/token_generator.rs` (517e4cd);
    - `hex_5_adapters_are_named_by_technology_and_concept contexts/iam/src/application/ports/outbound/token_generator.rs` (ce6d64b);
    - `hex_5_adapters_are_named_by_technology_and_concept contexts/iam/src/infrastructure/services/jwt_service/jwt_service.rs` (c9db5fa).
    La cuarta del plan (`hex_8_kernel_traits` de `token_verifier.rs`) es de R10b.
  - 300 tests, fmt y clippy en verde localmente.
  - Revisión con `craft-backend:architecture-reviewer`: ningún hallazgo bloqueante. Se aplicaron sus dos hallazgos (1b08cea, 62d5456). Lo demás queda para otras rebanadas:
    - el middleware de `app` importa `iam::application::ports::outbound` (un puerto saliente y su DTO) desde un adaptador driving. Es lo que se pidió para R10a, y R10b lo pone sobre la query;
    - el middleware en `app` y `VerifiedIdentity` en `shared::domain` (R27, R28);
    - `SystemError`, `code()`, `AuthenticateAccount` sin `Handler` y `if !account.can_authenticate()` (R11 a R19, HEX-7.2);
    - los nombres heredados `SharedContainer.jwt_service` y `jwt_service` en `app/src/containers/` y `server.rs`. Ningún principio los exige; R10b los toca al cablear la query;
    - VER-1, se proponen al harness: un test que impida que un adaptador driving importe `ports::outbound`, y otro que exija que los DTOs de un puerto saliente vivan en `ports/outbound/dtos/`.
  - **Desviaciones del encargo:**
    - el rojo de JWT fueron 4 casos, no 1 (arriba, con su causa);
    - la query no validará el `account_id` con `AccountId` (R10b): `communities` valida el `sub` y responde 400 `IAM_INVALID_ACCOUNT_ID(_FORMAT)`, que fija la caracterización. Validarlo en la query lo convertiría en 401.
- **Resultado (R10b):**
  - Commit `101f652`: `VerifyAccessTokenUseCase` (`ports/inbound/verify_access_token_use_case.rs`) y `VerifyAccessTokenHandler` en `queries/verify_access_token/`, con `query.rs`, `response.rs`, `error.rs` y `handler.rs`, sin `module_inception`. Es el primer caso de uso con la forma de la fase 3: `Result<VerifyAccessTokenResponse, Failure<VerifyAccessTokenError>>`, con una sola hoja, `Unauthenticated`, en `application/errors/unauthenticated.rs`. `Valid` responde la cuenta e `Invalid` es `Unauthenticated`. No valida el `account_id` (ver la desviación de R10a). El `Fault` de `verify` se descarta de forma explícita como `Unauthenticated`. Rojo: con un esqueleto que siempre respondía `Unauthenticated`, `returns_account_id_for_valid_token` falló con `left: None`. Los dos tests de `Unauthenticated` (token inválido y vencido) pasaban contra el esqueleto.
  - Commit `2aeeec0`: `fails_with_unexpected_when_verify_fails`, con `failing_on(AccessTokenOperation::Verify)`. Rojo contra `101f652`: `Err(Expected(Unauthenticated(Unauthenticated)))`. Ahora el `?` lo propaga como `Failure::Unexpected`.
  - Commit `9be8d19`: el middleware depende de `Arc<dyn VerifyAccessTokenUseCase>`, ya no del puerto saliente ni de su DTO. Sigue en `app` (R28) e inserta `VerifiedIdentity` (R27). `Unauthenticated` da 401, y `Failure::Unexpected` también da 401, descartado de forma explícita. `IamContainer` cablea `VerifyAccessTokenHandler` sobre JWT. Se borra `SharedContainer`, que solo tenía el verificador, y los nombres `jwt_service` pasan a `access_tokens`. Sin cambio de comportamiento: lo cubre la caracterización.
  - Commit `c65817c`: `a_fault_verifying_the_token_returns_500`, con un `StubVerifyAccessTokenUseCase` que responde `Failure::Unexpected`. Rojo contra `9be8d19`: `left: 401, right: 500`. `Failure::Unexpected` pasa por `map_application_error(SystemError::Unexpected(..))` del mapper de iam: el 500 con el cuerpo de D7, y el `Fault` en stderr (interino de R5). El test afirma el cuerpo exacto. Cambio de contrato sin cobertura previa (§5.5).
  - Commit `3c1a6bb`: se borran `shared/src/application/ports/outbound/{token_verifier,error}.rs` y, como queda vacío, todo `shared::application`. Antes de borrar se buscó cada nombre en todo el workspace, tests incluidos. Rojo: el trinquete (`hex_8_kernel_traits` de `token_verifier.rs`).
  - `account_lifecycle.rs` y `app/tests/`, incluido `common/mod.rs`, quedaron intactos.
  - **Línea base: −1 (34 → 33 entradas, 52 → 51 violaciones):** `hex_8_kernel_traits_are_model_markers_or_listed_capabilities shared/src/application/ports/outbound/token_verifier.rs` (3c1a6bb). R10 cierra las 4 entradas que le asignaba el plan.
  - 305 tests, fmt y clippy en verde localmente.
  - Revisión con `craft-backend:architecture-reviewer`: sin hallazgos. Validó las dos decisiones:
    - no validar el `account_id` en la query (ERR-2: el significado lo decide el caso de uso);
    - reusar el mapper de iam para el 500, como interino. Vive en `infrastructure/http`, la capa `api` de iam en legacy, adonde se muda el middleware en R28.
    Lo demás queda para otras rebanadas:
    - R26 y R30 tienen que cambiar también la llamada al mapper de `auth.rs`;
    - si un `Fault` futuro de `verify` se arma con el error de la librería, no debe incluir el token (ERR-4);
    - VER-1, se propone al harness: `hex_4_driving_adapters_never_depend_on_outbound_ports`, sobre `*/infrastructure/http` y `app/src/middleware`. Habría detectado lo que R10a toleró a propósito.
    - El brazo `IAM_TOKEN_GENERATOR_ERROR => 500` que señaló el revisor de R8 lo borró R10a (65f455d).

> **Punto de replanificación P1 (después de R10).** Ver §5.9. Termina la fase de puertos: se revisa si el patrón de las fases 2 y 3 sigue sirviendo antes de migrar los contratos.

#### Fase 3: contratos por caso de uso, en el lugar

Patrón de cada rebanada:
- `commands|queries/<caso>/{command,response,error,handler}.rs`, con `{Verb}{Noun}Handler` y sin `module_inception`;
- hojas planas en `application/errors.rs` (`Validation { field }`, `Conflict { resource }`, `NotFound { resource }`…), con `From<Leaf>` hacia el contrato;
- el `UseCase` devuelve `Result<_, Failure<{Verb}{Noun}Error>>`;
- el handler traduce a mano los errores de dominio a hojas (ERR-2.2);
- en el HTTP, `infrastructure/http/<caso>.rs` hace un `match` exhaustivo, sin comodín, con los códigos y mensajes de `infrastructure/http/codes.rs` (byte a byte los de hoy);
- los tests de handler pasan a `matches!` sobre la variante.

En todas, el contrato HTTP se mantiene.

*(Revalidado en P1 contra `bab68c5`.)* **El `?` y el puente `SystemError::Unexpected`.** Cada llamada a un puerto ya propaga su `Fault` con `.map_err(SystemError::Unexpected)?` (fase 2) y tiene su test `fails_with_unexpected_when_<operación>_fails`. La secuencia de R10a (el commit de la firma descarta el `Fault` y el `?` llega después, con su test en rojo) **no aplica en la fase 3**: no aparece ninguna propagación nueva. En el commit que cambia el contrato a `Failure<{Verb}{Noun}Error>`, `.map_err(SystemError::Unexpected)?` pasa a `?` (`From<Fault> for Failure<E>`), y esos tests pasan de `Err(SystemError::Unexpected(_))` a `Err(Failure::Unexpected(_))`. Es el mismo commit, porque sin él no compilan. Ningún `Fault` se descarta en ningún momento. La secuencia de R10a vuelve a regir si una rebanada agrega una llamada nueva a un puerto (R20: `IdGenerator`, `VerificationCodeService`).

**R11. `RegisterAccount`.**
- **IDs:**
  - HEX-4.1 (register), ERR-2.1 (register), ERR-2.2 (parcial);
  - ERR-2.6 (register), ERR-2.7 (lo que queda: `fails_when_password_is_too_short`);
  - ERR-4.4 y ERR-4.5 (register);
  - ERR-4.10 (`sign_up.rs`, en el lugar).
- **Cambios:**
  - `RegisterAccount` pasa a `RegisterAccountHandler` en `commands/register_account/handler.rs`, con `error.rs` y sin `module_inception`;
  - el `HashedPasswordError` pasa a `Fault` (§4);
  - el `unexpected()` privado de `infrastructure/http/errors/error_mapper.rs` sale a `infrastructure/http/unexpected.rs`, junto con su test `unexpected_fault_maps_to_internal_server_error`. Lo usan el `fold` de register, el mapper (hasta R19) y el middleware de `app`, que deja de llamar a `map_application_error(SystemError::Unexpected(..))` (ver R19);
  - **CI:** `ARCH_REQUIRE_MATERIAL=1` en el paso de tests, junto con el primer contrato de comando. **Condición verificada en P1:** hoy la activación pone en rojo dos reglas que no encuentran sobre qué actuar, y el contrato de R11 no cambia ninguna de las dos: `err_1_application_errors_do_not_implement_std_error` (0 errores declarados de puertos; desde R7–R10 los puertos devuelven `Fault`) y `hex_4_event_subscribers_are_named_and_placed` (0 `EventSubscriber`; con D4 no habrá). Antes de activarla hay que resolver eso con el harness (§6). Si no está resuelto al abrir R11, R11 no la activa y lo dice en el PR.
- **Línea base: −1** (`hex_4_handlers` de `register_account.rs`).
- **Rojo:**
  - `fails_with_conflict_on_taken_email` y `…_username`, sobre la variante. Reemplazan a `fails_when_email_already_exists` y `fails_when_username_already_exists`, que afirman `code()` (§5.1, ERR-2.6);
  - `fails_when_password_is_too_short` (falta hoy);
  - test de endpoint con `StubRegisterAccountUseCase` que devuelve `Failure::Unexpected` y responde 500 con el cuerpo de D7.
- **Sin rojo nuevo:** los 4 `fails_with_unexpected_when_*` (`find_by_email`, `find_by_username`, `save`, `hash`) ya existen; pasan a `Failure::Unexpected` (ver arriba). Los dobles ya tienen `failing_on(AccountOperation)` y `failing_on(PasswordHashingOperation)`.
- **Caracterización:** lifecycle se adapta (nombre del handler).
- **Tamaño:** M.

**R12. Unicidad sobre el valor normalizado.**
- **IDs:** HEX-7.1.
- **Cambios:** se construyen `Email` y `Username` antes de consultar al repositorio.
- **Caracterización previa (primer commit del PR, sin tocar producción):** en `app/tests/auth_api.rs`, `register_with_invalid_username_and_taken_email_returns_409`, marcado `CURRENT BEHAVIOR`. Registra `ana`/`ana@example.com` y después manda `ab` (username inválido) con `ana@example.com`. Afirma 409 `IAM_ACCOUNT_EMAIL_ALREADY_EXISTS`, porque hoy `RegisterAccount` busca el conflicto con el texto crudo antes de que la factory valide el formato (register_account.rs:38-60). Tiene que pasar en verde contra el código actual.
- **Rojo:**
  - `fails_with_conflict_when_email_differs_only_in_case`, en el handler y en HTTP;
  - el cambio de producción pone en rojo el test de caracterización anterior. Se reemplaza por `register_rejects_invalid_username_before_checking_taken_email`, que afirma 400 `IAM_INVALID_USERNAME`: primero el formato, después el conflicto.
- **Batería (desde R8):** `account_repository_contract!` fija la coincidencia exacta. Se le suma un caso que guarde con el valor normalizado y busque con el mismo valor normalizado; la normalización es del value object, no del adaptador.
- **Contrato HTTP: cambia.** Se reemplazan `register_accepts_same_email_with_different_casing` y `register_accepts_same_username_with_surrounding_spaces` por los mismos casos con 409. También cambia "username inválido + email tomado", de 409 a 400 (ver §5.5). Hay que mantener el orden "contraseña, después formato, después conflicto" (`register_validates_password_before_username_and_email` y `register_checks_email_before_username_when_both_are_taken`).
- **Riesgo:** cambia el orden entre validar el formato y detectar el conflicto. Ahora lo fija el test de caracterización de arriba, así que el cambio queda a la vista en el diff de `app/tests/`.
- **Fuera de alcance:** el orden "hashear antes de validar la política" (§4). Ningún test de caracterización lo distingue; si se cambia, va en su propio commit, con el test que lo exija.
- **Tamaño:** S.

**R13. `VerifyAccount`.**
- **IDs:** HEX-4.1, ERR-2.1, ERR-2.6, ERR-2.9 (lo que queda: `fails_when_code_is_out_of_range`) y ERR-1.4 (`expect("invariante: Registered→Active")`).
- **Contrato (D1 decidida):** `VerifyAccountError { NotFound, Validation, InvalidVerification }`. Verificar una cuenta ya verificada es `InvalidVerification`: sigue respondiendo 400 `IAM_ACCOUNT_INVALID_VERIFICATION`. R13 no cambia el contrato. Pasar a 409 se revisa con R30.
- **Línea base: −1** (`hex_4_handlers` de `verify_account.rs`).
- **Rojo:**
  - `fails_when_code_is_out_of_range` (handler);
  - test de endpoint con `StubVerifyAccountUseCase` que devuelve `Failure::Unexpected` y responde 500.
- **Sin rojo nuevo:** `fails_with_unexpected_when_find_by_email_fails` y `…_save_fails` ya existen desde R8 (con `failing_on(AccountOperation::Save)`); el `fails_when_repository_fails` del plan original ya no hace falta.
- **Contrato HTTP:** sin cambios. La búsqueda por `Email` (HEX-7.10) va en R12b.
- **Caracterización:** lifecycle se adapta (de `code()` a la variante, incluido `verifying_twice_fails_with_invalid_verification`).
- **Tamaño:** S (antes S/M: los tests de repositorio ya están).

**R14. `AuthenticateAccount`.**
- **IDs:** HEX-4.1, HEX-4.4, ERR-2.1, ERR-2.3, ERR-2.6, HEX-7.2, ERR-4.10 (`sign_in.rs`) y ERR-4.11.
- **Cambios:**
  - `account.authenticate() -> Result<(), CannotAuthenticate>` en el agregado;
  - `AuthenticateAccountError { LoginFailed, CannotAuthenticate }`: ya existe en `application/errors/authenticate_account.rs` desde R2/R10a. Se mueve a `commands/authenticate_account/error.rs`, sin `ErrorLayout`;
  - `Debug` a mano en `SignInRequest`, `AuthenticateAccountResponse` y `VerifyAccessTokenQuery`, sin el secreto.
- **Línea base: −1.**
- **Rojo:**
  - dominio: `registered_account_cannot_authenticate`;
  - un test de endpoint 500 con `StubAuthenticateAccountUseCase`;
  - `debug_does_not_reveal_the_password` y `…_the_token`.
- **Sin rojo nuevo:** `fails_when_token_generation_fails` ya existe como `fails_with_unexpected_when_issue_fails` (R10a), con `failing_on(AccessTokenOperation::Issue)`. Tampoco los de `find_by_username`, `verify` y el `verify` del señuelo (R8, R9).
- **Contrato HTTP:** sin cambios. La búsqueda por `Username` (HEX-7.11) va en R12b.
- **Tamaño:** M.

**R12b. Búsquedas normalizadas en verify y authenticate (nueva en P1).**
- **IDs:** HEX-7.10 y HEX-7.11. Son la misma causa que HEX-7.1 (R12), en otros dos handlers. Va aparte porque cambia el contrato, y las rebanadas de la fase 3 no lo cambian.
- **Precondición:** R12, R13 y R14 hechas (toca los handlers que esas dos migran).
- **Cambios:**
  - `VerifyAccountHandler` construye `Email` antes de `find_by_email`;
  - `AuthenticateAccountHandler` construye `Username` antes de `find_by_username`. Si no se puede construir, es `LoginFailed` y verifica contra el señuelo, igual que una cuenta inexistente (D6).
- **Línea base:** sin cambios.
- **Rojo:**
  - handler y HTTP: `verifies_with_an_email_that_differs_only_in_case`;
  - handler y HTTP: `authenticates_with_surrounding_spaces_in_the_username`;
  - handler: `invalid_username_fails_with_login_failed_and_verifies_against_the_decoy`.
- **Contrato HTTP: cambia (sin cobertura previa).** Verificar con el email en otras mayúsculas pasa de 404 a 200; un username con espacios alrededor pasa de 401 `IAM_LOGIN_FAILED` a 200. Ningún test de caracterización fija ninguno de los dos (§5.5).
- **Fuera de alcance:** `identify` (D10).
- **Tamaño:** S.

**R15. `IdentifyAccount`.**
- **IDs:** HEX-4.1, ERR-2.1 y ERR-2.6.
- **Cambios:** `IdentifyAccount` pasa a `IdentifyAccountHandler` en `queries/identify_account/handler.rs`. Con R11, R13, R14 y R15, ningún handler de iam usa `map_application_error`.
- **Línea base: −1.**
- **Rojo:** `fails_with_not_found_for_unknown_identifier`, sobre la variante. Reemplaza a `fails_when_account_not_found`.
- **Sin rojo nuevo:** los `fails_with_unexpected_when_find_by_{email,username}_fails` ya existen (R8).
- **Caracterización:** `identify_with_different_email_casing_returns_404` no cambia (D10). La auditoría de P1 lo volvió a señalar (§4).
- **Tamaño:** S.

**R16. Cortar `communities → iam`.**
- **IDs:** HEX-1.1, HEX-1.2, HEX-1.3, HEX-1.4 y TDD-2.6.
- **Cambios:**
  - un `OwnerId` propio en `domain/model/community/owner_id.rs`. **D2 decidida, opción a):** valida el formato UUID y rechaza el nil, con los códigos `IAM_INVALID_ACCOUNT_ID(_FORMAT)`;
  - los códigos propios de communities, y esos dos strings, pasan a `infrastructure/http/codes.rs`;
  - se quita `iam` de `contexts/communities/Cargo.toml`;
  - *(desde R7)* la batería `application/testing/contracts/community_repository.rs:14` y el módulo de tests de `create_community.rs` (línea 86) construyen el agregado con `AccountId`: pasan a `OwnerId`;
  - `FakeCommunityAdapter` gana `failing_on(CommunityOperation)`, como `FakeAccountAdapter` (§5.1), y se borra el `StubCommunityAdapter` local de `ba15189`. `fails_with_unexpected_when_save_fails` usa `failing_on(CommunityOperation::Save)`.
- **Línea base: −1** (`hex_1` de `contexts/communities/Cargo.toml`): **cierra `hex_1_dependencies_point_inward`.**
- **Rojo:**
  - tests de dominio `owner_id_rejects_non_uuid` y `owner_id_rejects_nil`;
  - `FakeCommunityAdapter::failing_on`: en el rojo, por mutación, tragar el `Fault` de `save` pone en rojo `fails_with_unexpected_when_save_fails`. La batería sigue en verde contra el Fake con `failing_on` sobre otra operación.
- **Contrato HTTP: ninguno (D2 a).**
- **Riesgo:** `create_with_non_uuid_subject_returns_400` y `create_with_nil_uuid_subject_returns_400`, que no cambian.
- **Tamaño:** S/M.

**R17. `CreateCommunity`.**
- **IDs:**
  - HEX-4.1, HEX-4.2 (`owner_id: String` en el comando);
  - ERR-2.1, ERR-2.4, ERR-2.6 y ERR-2.10 (lo que queda: `fails_when_owner_id_is_invalid`);
  - HEX-1.8 (los códigos salen de domain);
  - ERR-4.4 y ERR-4.5 (communities).
- **Cambios:** como R11, el `unexpected()` del mapper de communities sale a `infrastructure/http/unexpected.rs`, con su test.
- **Línea base: −1** (`create_community.rs`).
- **Rojo:**
  - `fails_when_owner_id_is_invalid` (handler);
  - un test de endpoint 500 con `StubCreateCommunityUseCase`.
- **Sin rojo nuevo:** `fails_with_unexpected_when_repository_fails` y `…_save_fails` ya existen (R7; desde R16, con `failing_on`).
- **Caracterización:** `create_checks_slug_uniqueness_before_subject_format` fija el orden: primero el slug, después el owner (hoy: slug, conflicto, owner, nombre; create_community.rs:38-50).
- **Tamaño:** M.

**R18. Eliminar `ListPublicCommunities` (D3 decidida: eliminar).** *Hecha (ver §5.8).*
- **IDs:** TDD-1.1, HEX-4.1 (list), HEX-4.3, ERR-1.6, HEX-6.2, TDD-2.5 y RS-2.2 (`get_public_list`).
- **Cambios:** se borran el caso (`queries/list_public_communities/`), su puerto de entrada, `get_public_list` (con su `todo!()`) y `find_by_id` del puerto `CommunityRepositoryPort`, del doble y del adaptador en memoria, los helpers del doble que solo servían a `find_by_id` (TDD-2.5), y `AuthContext` (parte de HEX-8.6). *(`find_by_id` estaba asignado a R7; D3 lo incluye y no tenía usos.)*
- **Orden:** ya no depende de la fase 3. Se adelanta y va **antes de R7**, después de R1 (ver §5.4). Conserva el número para no renumerar las referencias.
- **Línea base: −1** (`hex_4_handlers` de `list_public_communities.rs`). Ya no cierra `hex_4_use_case_implementations_are_handlers`: lo cierra la última de R11, R13 a R15 y R17.
- **Tests:** sin rojo nuevo; el rojo es la línea de la línea base. Se borran los tests comentados (TDD-1.1).
- **Caracterización:** `there_is_no_route_to_list_communities` no cambia.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** S.
- **Resultado:**
  - Commit `a603969`. Se borraron `queries/list_public_communities/` entero (handler, query, response y el bloque de tests comentado), `queries/mod.rs` (quedaba vacío), el puerto `list_public_communities_use_case.rs`, y `get_public_list` y `find_by_id` de `CommunityRepositoryPort`, de `FakeCommunityRepository` y de `InMemoryCommunityRepository`.
  - Antes de borrar se buscó cada nombre con `rg` en todo el workspace, tests incluidos. Ni `get_public_list` ni `find_by_id` tenían otro llamador, así que `find_by_id` se borra aquí y no en R7. `AuthContext` solo lo usaban el caso y su puerto: se borró `shared/src/application/auth_context.rs` y su `pub mod`. De HEX-8.6 queda `VerifiedIdentity` (R27).
  - Al quitar `find_by_id` del doble, quedaron sin lectura `existing_id`, `with_existing_id` y `active_with_existing_username` (los helpers muertos de TDD-2.5), y `_activated` quedaba siempre en `false`. Se quitaron los cuatro: si no, clippy falla con `-D warnings`. El doble queda con `should_fail` y `existing_slug`; R7 lo reescribe.
  - Ningún test de `app/tests/` ni de `account_lifecycle.rs` ejercitaba el caso: no estaba cableado. `there_is_no_route_to_list_communities` no cambió.
  - El rojo fue el trinquete: `hex_4_use_case_implementations_are_handlers` falló con "borrar de baseline.txt" para `list_public_communities.rs`. Es la única entrada que desaparece (47 → 46). S1 no había agregado ninguna entrada sobre estos archivos: la de `list_public_communities.rs` venía de R1 con la misma clave (§5.10 a), y `community_repository.rs` (puerto y adaptador) conserva sus tres entradas `hex_5`, que siguen en R7.
  - 233 tests, fmt y clippy en verde localmente.
  - Revisión con `craft-backend:architecture-reviewer`: sin hallazgos en el diff, sin referencias rotas y sin trabajo de R7 adelantado. Lo que señaló de otras rebanadas ya figura en el inventario y no se corrigió aquí:
    - `expect("mutex poisoned")` en `find_by_slug` y `save` del adaptador (RS-2.4, R7);
    - `FakeCommunityRepository` sin estado: `save` no guarda (TDD-2.3, R7);
    - `CommunityId::as_uuid` en `create_community.rs:55` (HEX-2.6, R22);
    - `shared/src/application/mod.rs` queda solo con `ports` (`token_verifier`). Cuando R10 lo borre, `shared::application` queda vacío y hay que quitar el módulo (HEX-8.5, R10).

**R19. Retirar `SystemError`.**
- **IDs:** ERR-1.1, RS-2.1, ERR-2.2, HEX-8.3, ERR-4.4 (resto) y HEX-7.12.
- **Cambios:**
  - se borran `shared/src/error.rs` y `ErrorLayout`, y con ellos el puente `SystemError::Unexpected` de R5. Desde P1, sus usuarios son los handlers todavía no migrados, los dos mappers y, desde R10b, el middleware de `app`. R11 y R17 sacan `unexpected()` a `infrastructure/http/unexpected.rs` y el middleware pasa a usarlo, así que al llegar a R19 el puente solo queda en los mappers;
  - se borra `CommunityError`, que no tiene uso, con sus códigos (HEX-7.12);
  - los errores de dominio quedan solo con `Debug` y `Display`;
  - se borran los `error_codes.rs` de domain y de application;
  - se borran los dos `error_mapper.rs` y sus tests, que ya quedaron reemplazados por los `fold`. `unexpected_fault_maps_to_internal_server_error` ya vive con `unexpected.rs` desde R11/R17.
- **Línea base: −1:** **cierra `hex_8_kernel_traits_are_model_markers_or_listed_capabilities`.**
- **Tests:** refactor puro, sin rojo nuevo.
- **Caracterización:** lifecycle queda sin `SystemError`.
- **Tamaño:** M.

> **Punto de replanificación P2 (después de R19).** Ver §5.9. Ya no queda `SystemError`: se confirma que el árbol de errores quedó como pide ERR antes de tocar el dominio.

#### Fase 4: dominio

**R20. Alta y restauración de `Account`.**
- **IDs:**
  - HEX-7.5, HEX-7.6 (iam), HEX-2.2 y HEX-2.3;
  - ERR-1.5, HEX-7.7 y ERR-2.5 (se borran `reconstitute` y `from_str`, que no tienen uso, junto con sus tests);
  - RS-2.2 (`account_id.rs` y `account_status.rs`).
- **Cambios:**
  - `Account::create`/`restore` con value objects, y `new` privado;
  - `AccountId` sobre `String`, con el id que llega de `IdGenerator` (`UuidIdGeneratorAdapter` en infra);
  - el código de verificación sale de un puerto `VerificationCodeService` (`RandVerificationCodeAdapter`);
  - se quitan los `should_implement_trait` y el `module_inception` de `model/account`;
  - *(P1)* `AccountId` y `AccountStatus` dejan de implementar `ValueObject`, y con eso desaparecen sus 4 `todo!()` (HEX-8.10, parcial). El trait se ajusta en R21.
- **Línea base:** sin cambios.
- **Rojo:**
  - `create_uses_the_given_id_and_code` (dominio);
  - handler con `FixedIdGeneratorAdapter`: `registers_with_the_generated_id`;
  - batería del generador de códigos: `generates_six_digit_codes`.
- **Riesgo:** `register_returns_201_with_uuid…` y `verification_code_is_six_digits`.
- **Tamaño:** M/L.

**R21. Agregado inmutable y eventos.**
- **IDs:**
  - HEX-7.3, HEX-7.4, HEX-8.9, HEX-8.10, HEX-8.2 y HEX-2.4;
  - VER-1.7 (`todo = "deny"` y `dbg_macro = "deny"`, cuando ya no queda ningún `todo!()`);
  - *(S1)* HEX-2.1 (parcial: `chrono` en `contexts/iam/Cargo.toml`) y HEX-8.1 (parcial: `chrono` en `shared/Cargo.toml`);
  - RS-2.2 (`account.rs` y `account_event.rs`).
- **Cambios:**
  - `confirm(&self, …) -> Result<Self, …>`;
  - `AggregateRoot` y `ValueObject` como marcadores (HEX-8.9, HEX-8.10), con `model.rs` del template; se borra `EventRecorder`;
  - **D4 decidida: se retira el andamiaje muerto.** Se borran `AccountEvent` (`model/account/events/`), `record_event`/`pull_events` (sus 3 `todo!()`) y `version`, porque no tienen comportamiento ni uso. No se agrega `take_events`. `DomainEvent` se borra del kernel si nada lo implementa; si el template lo trae como marcador en `model.rs`, queda vacío, sin `chrono`;
  - ~~el trait `Clock` en `shared/src/clock.rs`~~ **(P1, por D4) `Clock` no entra en R21.** Su primer uso iba a ser el instante de los eventos, que se retiran. Se copia del template (con `SystemClockAdapter`) en la primera rebanada que necesite la hora, con su test;
  - *(S1)* `chrono` sale de `contexts/iam/Cargo.toml` y de `shared/Cargo.toml`: después de este cambio, ninguno de los dos lo usa.
- **Línea base (S1): −3:** `hex_2_core` `contexts/iam/Cargo.toml#dependencies.chrono` y `shared/Cargo.toml#dependencies.chrono`, y `hex_8_kernel_depends` `shared/Cargo.toml#dependencies.chrono`.
- **Rojo:**
  - `confirm_returns_the_active_account_and_leaves_the_original_untouched`;
  - el trinquete de las 3 entradas de `chrono`. Sin eventos (D4) no hay `create_records_registered`.
- **Tamaño:** M.

**R22. Dominio de `communities`.**
- **IDs:** HEX-2.6, HEX-7.6 (communities), HEX-7.8 y HEX-7.9.
- **Cambios:**
  - `CommunityId` sobre `String`, con `IdGenerator`;
  - **D4 decidida:** se borran `MembershipPolicy` (`domain/services/`, que queda vacío), el campo `membership_policy` de `Community` con su getter, y `MembershipPolicyChanged` (`model/community/events/`). No tienen comportamiento ni uso. HEX-7.8 se resuelve borrando, no moviendo;
  - se quitan los `allow`.
- **Rojo:** handler `creates_with_the_generated_id`.
- **Riesgo:** `create_returns_200_with_id…`, que exige un UUID de 36 caracteres; lo cumple el adaptador UUID.
- **Tamaño:** S (antes S/M: D4 retira en lugar de implementar).

#### Fase 5: puertos async (paralelizables entre sí)

**R23. iam async.** **R24. communities async.**
- **IDs:** RS-3.1.
- **Cambios:**
  - `BoxFuture` en `shared/src/future.rs`;
  - todos los puertos de entrada y de salida del contexto;
  - `block_on` de std en `application/testing.rs` para los tests de application;
  - *(desde R7)* las baterías de `application/testing/contracts/` pasan a `async fn`, y sus macros reciben `run` (`block_on` para el Fake y el adaptador en memoria), como en el template;
  - Argon2 con `spawn_blocking` en el adaptador.
- **Rojo:** batería async del adaptador Argon2 bajo `#[tokio::test]` (infra).
- **Tests:** refactor sin cambio de comportamiento.
- **Caracterización:** lifecycle se adapta (`.await`).
- **Tamaño:** M cada una.

#### Fase 6 y 7: kernel y estructura

**R25. `shared` sin dependencias.**
- **IDs:** HEX-8.1 (el resto: `serde`; `chrono` sale en R21, desde S1). HEX-8.2 ya quedó resuelto en R21.
- **Cambios:** `ApiErrorResponse` se copia temporalmente a cada `infrastructure/http`; R30 lo elimina.
- **Línea base: −2.** *(S1: con las claves por dependencia, son `hex_8_kernel_depends` y `hex_2_core` de `shared/Cargo.toml#dependencies.serde`. Las de `chrono` las cierra R21.)* **Cierra `hex_8_kernel_depends_only_on_std`.**
- **Tamaño:** S.

**R26. `edge/http` e `infrastructure/stderr`.**
- **IDs:** ERR-4.7 (parcial), ERR-1.8 y ERR-4.12.
- **Cambios:**
  - crates nuevos con `Problem`, `unexpected()`, `AuthenticatedAccount` y `StderrFaultReporterAdapter`, más los tests del template;
  - `CatchPanicLayer` en `app/src/server.rs`;
  - *(P1)* `AppConfig::load()` propaga el `ConfigError` con `?` en lugar de `process::exit` (ERR-1.8), y `main` reporta el `Fault` de arranque con el `FaultReporter`, que muestra la causa (ERR-4.12).
- **Línea base:** sin cambios. Solo depende de ellos `app`, que es Bootstrap.
- **Rojo:**
  - `serializes_as_problem_json_with_extension_code`;
  - `unexpected_reports_the_fault_and_hides_the_detail`;
  - un test de `app`: `a_panicking_route_returns_500`.
- **Contrato HTTP: cambia (sin cobertura previa).** Un panic responde 500 en lugar de cortar la conexión.
- **Tamaño:** M.

> **Punto de replanificación P3 (después de R26, antes de R27).** Ver §5.9. R26 corre en paralelo desde R5, así que P3 se hace cuando están hechas R22, R24, R25 y R26, las precondiciones de la separación. Es la última revisión antes de las rebanadas L.

**R27. Separar `communities` en crates por capa.**
- **IDs:**
  - RS-1.1 (parcial), HEX-2.5;
  - HEX-1.9 (communities);
  - HEX-6.1 (communities: `CreateCommunityHttpRequest/Response`);
  - ERR-4.8 (`Option<Extension<AuthenticatedAccount>>`);
  - HEX-8.6 (se borra `VerifiedIdentity`; el middleware de `app` inserta `AuthenticatedAccount`).
- **Estructura:** `contexts/communities/{domain,application,infrastructure,api}`, con `CommunitiesApi`.
- **Precondición del trinquete:** de `contexts/communities/src`, en la línea base solo quedan `requests/create.rs` y `responses/created.rs`. Se renombran en este mismo PR.
- **Línea base: −7** *(S1; antes −4)*: las 4 `hex_2_core` de `contexts/communities/Cargo.toml` (`axum`, `http`, `serde`, `uuid`), la `hex_2_frameworks` (`axum`, n = 3), porque el manifiesto desaparece, y los dos `hex_6`.
- **Rojo:** `communities-api/tests/create_community.rs`: `responds_401_without_authenticated_account`.
- **Riesgo:** ~~el test de contratos HEX-4 se activa en este PR~~. Desde S1 ya actúa en legacy. El riesgo que queda es que un crate nuevo (`communities-application`) declare más dependencias de las que pide su capa.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** L (sobre todo, archivos movidos).

**R28. Separar `iam` y crear `IamApi::protect`.**
- **IDs:**
  - RS-1.1, HEX-2.1;
  - HEX-1.6, HEX-1.7, HEX-1.9;
  - HEX-6.1 (iam);
  - TDD-1.8 (test de arranque del señuelo).
- **Cambios:**
  - el middleware sale de `app` y va a `iam-api`;
  - `app` compone `iam.protect(communities.protected())`;
  - lifecycle se muda a `contexts/iam/infrastructure/tests/`;
  - *(P1)* `IamApi` recibe sus adaptadores desde `app`, así que el arranque admite un hasher que falle: test `startup_fails_when_the_decoy_hash_fails` (TDD-1.8).
- **Precondición:** de `contexts/iam/src`, en la línea base solo quedan los 7 `hex_6`.
- **Línea base: −14** *(S1; antes −9)*: las 6 `hex_2_core` de `contexts/iam/Cargo.toml` (`argon2`, `axum`, `jsonwebtoken`, `rand`, `serde`, `uuid`; `chrono` sale en R21), la `hex_2_frameworks` (`axum`, n = 3) y los 7 `hex_6`. **Cierra `hex_2_core…`, `hex_2_server_frameworks…` y `hex_6_transport…`.**
- **Rojo:** `iam-api/tests/authenticate.rs`: `protect_rejects_missing_token_with_401` y `protect_inserts_authenticated_account`.
- **Riesgo:** `unknown_route_without_token_returns_401`. Si `protect` deja de envolver el fallback, la respuesta pasa a 404 y es un cambio de contrato que el usuario decide.
- **Tamaño:** L.

**R29. `shared` pasa a ser `kernel/` y se borra la adaptación legacy.**
- **IDs:** RS-1.2.
- **Cambios:**
  - `git mv shared kernel`;
  - en `tools/architecture/tests` se borra todo lo marcado `legacy:`, más `tests/legacy.rs` y `tests/fixtures/legacy/` *(S1: así lo indica el template desde `v1.0.0-rc.1`; antes eran solo `legacy_layers` y el fallback a `shared/`)*.
- **Precondición:** no queda ninguna entrada de `shared/` (se cumple desde R25).
- **Línea base:** vacía.
- **Tamaño:** S.

**R30. Problem Details.**
- **IDs:** ERR-4.6, ERR-4.7 (resto: los `fold` usan `unexpected(fault, reporter)`) y HEX-8.8.
- **Cambios:** todas las respuestas de error y los 401 pasan a `application/problem+json`, con `code` como miembro de extensión. Se borra el `ApiErrorResponse` interino.
- **Rojo:** tests HTTP con el formato nuevo.
- **Contrato HTTP: cambia todo el cuerpo de error.** Se modifican `assert_error` (`app/tests/common/mod.rs`) y las aserciones `raw_body == ""` de los 401 (ver D5).
- **D1:** aquí se revisa si verificar una cuenta ya verificada pasa de 400 `IAM_ACCOUNT_INVALID_VERIFICATION` a 409. Si cambia, es una decisión del usuario y una fila más de §5.5.
- **Tamaño:** M.

### 5.4 Dependencias y paralelismo

```
R1 ─┬─ R2 ─────────────────────────────┐
    ├─ R3, R4, R6, R18   (paralelas)   │
    └─ R5 ─┬─ R7 (+R18) ──────┬─ R17 ──┤
           │        R16 ──────┘        │
           ├─ R8 ─ R9 ─ R10 ─[P1]─ R11 ─ R12
           │             └─ R13, R14(+R2), R15 ┴─ R19 ─[P2]─┬─ R20 ─ R21 ─ R23 ─┐
           │                                                └─ R22 ─ R24 ───────┤
           └─ R26 (desde R5, paralela a las fases 3 a 5)                        │
                                     R25 (tras R19, R21) ───────────────────────┤
                                           [P3] ─ R27 (R22, R24, R25, R26) ─ R28 (R10, R23) ─ R29, R30
```

- R8, R9 y R10 se pueden hacer en paralelo, pero tocan los mismos handlers de iam. Conviene ir en serie para no tener conflictos.
- R18 (eliminar `ListPublicCommunities`, D3) no depende de nada más que R1 y va antes de R7.
- R7 y R16 son paralelas a toda la rama iam.
- `[P1]`, `[P2]` y `[P3]` son los puntos de replanificación de §5.9. No se arranca la rebanada siguiente sin cerrar el punto.
- R13, R14 y R15 son paralelas entre sí después de R11. R14 necesita R2, R9 y R10.
- *(P1)* R12b va después de R12, R13 y R14, y antes de R19. No bloquea a R15 ni a R16.
- R23 y R24 son paralelas. R26 se puede hacer desde R5 en adelante.

### 5.5 Cambios de contrato HTTP (todos explícitos)

| Rebanada | Cambio | Tests de caracterización que cambian |
|---|---|---|
| R2 (hecho) | Sign-in de un usuario inexistente: de 404 a 401 `IAM_LOGIN_FAILED` | `authenticate_unknown_username_returns_404` → `authenticate_unknown_username_returns_401_login_failed`; `authenticate_by_email_returns_404` → `authenticate_by_email_returns_401` |
| R5, R10 | 500 de infraestructura con `{"code":"INTERNAL_ERROR",…}` (D7), sin `IAM_TOKEN_GENERATOR_ERROR` | ninguno (solo los tests del mapper) |
| R10a (hecho) | `POST /auth/authenticate` con falla al emitir el token: el 500 pierde `IAM_TOKEN_GENERATOR_ERROR` y pasa al cuerpo de D7 (`INTERNAL_ERROR`). Ningún test de caracterización lo cubre: JWT solo falla si el `ttl` desborda el reloj. Lo fijan `fails_with_unexpected_when_issue_fails` (handler) y `unexpected_fault_maps_to_internal_server_error` (mapper). Los 401 de las rutas protegidas no cambian: un token mal formado, de otra clave o vencido sigue dando 401, ahora como veredicto `Invalid` | ninguno. En el test del mapper, `technical_and_unknown_errors_map_to_internal_server_error` pasa a `unknown_errors_map_to_internal_server_error` |
| R10b (hecho) | Ruta protegida con un `Fault` al verificar el token (clave o validación mal configurada): de 401 con cuerpo vacío a 500 con el cuerpo de D7. Es un cambio de contrato sin cobertura previa: ningún test de caracterización lo cubre, porque JWT no falla con la configuración de los tests. Lo fija `a_fault_verifying_the_token_returns_500` (middleware). `Invalid` sigue dando 401, y la caracterización no cambia | ninguno |
| R7 (hecho) | `POST /communities/create` con falla del repositorio: el 500 pierde `COMMUNITIES_REPOSITORY_ERROR` y pasa al cuerpo de D7 (`INTERNAL_ERROR`). Ningún test de caracterización lo cubre: los adaptadores en memoria no fallan. Lo fijan `fails_with_unexpected_when_repository_fails`, `fails_with_unexpected_when_save_fails` (handler) y `unexpected_fault_maps_to_internal_server_error` (mapper) | ninguno. En los tests del mapper, `technical_and_unknown_errors_map_to_internal_server_error` pasa a `unknown_errors_map_to_internal_server_error` |
| R9 (hecho) | Ninguno en las respuestas. Una falla de Argon2 (al hashear, o un hash guardado ilegible o incompleto) da el 500 de D7 en lugar de cortar la conexión con un pánico, o de un 401 si el hash estaba incompleto. Ningún test de caracterización lo cubre. Si el hash del señuelo falla al arrancar, el servidor no arranca | `app/tests/common/mod.rs`: `app()` agrega `.expect("app builds")` porque `build_app` devuelve `Result` (decidido por el usuario). Ningún escenario ni aserto cambia |
| R8 (hecho) | `POST /auth/register`, `/auth/verify`, `/auth/authenticate` e `/auth/identify` con falla del repositorio: el 500 pierde `IAM_ACCOUNT_REPOSITORY_ERROR` y pasa al cuerpo de D7 (`INTERNAL_ERROR`). Antes solo `save` podía fallar; ahora también `find_by_email` y `find_by_username`, con el mismo 500. Ningún test de caracterización lo cubre: `InMemoryAccountAdapter` no falla. Lo fijan los 8 `fails_with_unexpected_when_<operación>_fails` de los handlers y `unexpected_fault_maps_to_internal_server_error` (mapper) | ninguno. En el test del mapper, `technical_and_unknown_errors_map_to_internal_server_error` pierde `IAM_ACCOUNT_REPOSITORY_ERROR` de su lista (queda `IAM_TOKEN_GENERATOR_ERROR` y `UNKNOWN`) |
| R12 | Duplicado por mayúsculas o espacios: de 201 a 409 | `register_accepts_same_email_with_different_casing`, `register_accepts_same_username_with_surrounding_spaces` |
| R12 | Username inválido + email tomado: de 409 `IAM_ACCOUNT_EMAIL_ALREADY_EXISTS` a 400 `IAM_INVALID_USERNAME` | `register_with_invalid_username_and_taken_email_returns_409` (lo agrega R12 en su primer commit) pasa a `register_rejects_invalid_username_before_checking_taken_email` |
| R12b | Verificar con el email en otras mayúsculas: de 404 a 200. Autenticar con espacios alrededor del username: de 401 `IAM_LOGIN_FAILED` a 200. Sin cobertura previa | ninguno (se agregan los tests HTTP de R12b) |
| R16 | Ninguno: D2 decidida, opción a) (se conservan `IAM_INVALID_ACCOUNT_ID(_FORMAT)`) | ninguno |
| R13 | Ninguno: D1 decidida (verificar dos veces sigue siendo 400 `IAM_ACCOUNT_INVALID_VERIFICATION`; el 409 se revisa con R30) | ninguno |
| PR #40 (P1) | Arranque: un `JWT_SECRET` vacío o de menos de 32 bytes ya no arranca el servidor (ERR-4.9). No es una respuesta HTTP | ninguno |
| R26 | Panic: 500 en lugar de conexión cortada | ninguno |
| R28 | Solo si cambia el fallback: ruta desconocida sin token, de 401 a 404 | `unknown_route_without_token_returns_401` |
| R30 | Problem Details en todos los errores y en los 401 | `assert_error`, los `raw_body == ""` |

### 5.6 Cierre de cada test de arquitectura

Desde S1 la columna cuenta entradas con el formato nuevo (una por dependencia en los `Cargo.toml`). Entre paréntesis va el total de violaciones que admiten, sumando las `n`.

**P1 (2026-11-23): sin cambios.** Las 33 entradas (51 violaciones) que quedan tienen, cada una, un ID del inventario y una rebanada: `hex_1` → HEX-1.1 (R16); `hex_2_core` de communities → HEX-2.5 (R27); `hex_2_core` de iam → HEX-2.1 (`chrono` en R21, el resto en R28); `hex_2_core` y `hex_8_kernel_depends` de `shared` → HEX-8.1 (`chrono` en R21, `serde` en R25); `hex_2_frameworks` → HEX-2.5 (R27) y HEX-2.1 (R28); `hex_4_handlers` → HEX-4.1 (R11, R13, R14, R15, R17); `hex_6` → HEX-6.1 (R27, R28); `hex_8_kernel_traits` de `shared/src/error.rs` → HEX-8.3 (R19). Ninguna incumple. Los hallazgos nuevos de P1 no tienen entrada: ninguna regla de `tools/architecture` los cubre (VER-1.3 a VER-1.6).

**S2 (`v1.0.0-rc.2`, 2026-11-23): sin cambios.** La línea base vuelta a llenar tiene las mismas 47 entradas, con las mismas claves y conteos (§5.10). Ninguna rebanada cambia de alcance: los cierres, los −n por rebanada y la columna "Queda en cero en" siguen como los dejó S1.

| Test | Entradas hoy (S2 = S1) | Justo antes de S1 | Queda en cero en |
|---|---|---|---|
| `err_1_domain_errors_do_not_implement_std_error` | 0 | 0 | R3: en cero (eb30039) |
| `err_1_application_errors_do_not_implement_std_error` | 0 | 0 | R3: en cero (eb30039) |
| `hex_1_dependencies_point_inward` | 1 (4) | 1 | R16 |
| `hex_5_outbound_ports_are_named_by_role` | 0 | 4 | R10a: en cero (R7 a R10); R7: −1 (692bc7c); R8: −1 (ebd0819); R9: −1 (73f9c44); R10a: −1 (517e4cd) |
| `hex_5_adapters_are_named_by_technology_and_concept` | 0 | 8 | R10a: en cero (R7 a R10); R7: −2 (9eacddc, 6b808a2); R8: −2 (de6c308, 9a90d8a); R9: −2 (c04f69d, 17ba68e); R10a: −2 (ce6d64b, c9db5fa) |
| `hex_4_use_case_implementations_are_handlers` | 5 | 6 | R17 (R18, R11, R13 a R15, R17); R18: −1 (a603969), quedan 5 |
| `hex_8_kernel_traits_are_model_markers_or_listed_capabilities` | 1 | 2 | R19 (R4, R10, R19); R4: −2 (80c00e4); R10b: −1 (3c1a6bb), queda 1 |
| `hex_8_kernel_depends_only_on_std` | 2 | 1 | R25 (R21 −1 `chrono`, R25 −1 `serde`) |
| `hex_2_core_depends_only_on_std_and_the_kernel` | 13 (24) | 3 | R28 (R21 −2 `chrono`, R25 −1, R27 −4, R28 −6) |
| `hex_2_server_frameworks_only_at_the_edge` | 2 (6) | 2 | R28 (R27, R28) |
| `hex_6_transport_types_carry_the_transport_name` | 9 | 9 | R28 (R27, R28) |
| `hex_4_use_case_contracts_do_not_expose_domain_types` | 0 | 0 | Desde S1 actúa también en legacy (17 tipos de contrato) |

En total quedan 33 entradas (51 violaciones) después de R10b (34 y 52 después de R10a; 37 y 55 después de R9; 40 y 58 después de R8; 43 y 61 después de R7; 46 y 64 después de R18; 47 y 65 en S1), que desaparecen entre R10 y R28. Después de R29, `baseline.txt` queda solo con la cabecera.

Suma por rebanada desde S1: R7 −3, R8 −3, R9 −3, R10 −4 (R10a −3, R10b −1), R11 −1, R13 −1, R14 −1, R15 −1, R16 −1, R17 −1, R18 −1, R19 −1, **R21 −3**, R25 −2, **R27 −7** y **R28 −14**: 47. Las 36 de antes de S1 se repartían entre las mismas rebanadas, con R25 −2, R27 −4 y R28 −9, y sin R21.

**Conciliación (R1).** El encargo hablaba de 56 entradas, pero `tools/architecture/baseline.txt` tiene 55 (63 líneas, de las que 8 son comentario). La columna "Entradas hoy" coincide test por test con `grep -v '^#' baseline.txt | cut -d' ' -f1 | sort | uniq -c`. No falta ninguna entrada: se corrige el conteo, no se asigna ninguna. La suma por rebanada también da 55: R3 −17, R4 −2, R7 −3, R8 −3, R9 −3, R10 −4, R11 −1, R13 −1, R14 −1, R15 −1, R16 −1, R17 −1, R18 −1, R19 −1, R25 −2, R27 −4 y R28 −9.

Los hallazgos que no son de código no tienen rebanada:
- ERR-4.1: rotar el secreto, una acción del usuario.
- TDD-1.5, TDD-1.6 y TDD-1.7: descartados por D8.
- VER-1.3 a VER-1.6: son tests de `tools/architecture`, que solo cambia en una sincronización. Se proponen al harness (§6).

### 5.7 Decisiones del usuario

Decididas (2026-11-20, en el PR de R1):

- **D3. `ListPublicCommunities` (R18, R7): decidido, eliminar.** Se borran el caso, su puerto de entrada, `get_public_list`, `find_by_id` y `AuthContext`. No se agrega ninguna ruta. R18 se adelanta y va antes de R7.
- **D6. Tiempo de respuesta del sign-in (R2): decidido, sí, dentro de R2.** Cuando el usuario no existe, se verifica Argon2 contra un hash señuelo generado al arrancar, así el 401 no permite enumerar usuarios por tiempo. Test rojo: `verifies_against_the_decoy_hash_when_account_does_not_exist`.
- **D7. Cuerpo interino del 500 por `Fault` (R5 a R30): decidido, el cuerpo que propone el plan.** `{"code":"INTERNAL_ERROR","message":"We couldn't complete your request right now. Please try again."}`. Rige hasta que R30 lo reemplaza por Problem Details (D5).

Decididas en P1 (2026-11-23):

- **D1. Verificar dos veces (R13): decidido, sigue 400.** Verificar una cuenta ya verificada responde 400 `IAM_ACCOUNT_INVALID_VERIFICATION`. R13 no cambia el contrato. Pasar a 409 se revisa junto con R30.
- **D2. Identidad del dueño en communities (R16): decidido, opción a).** `OwnerId` valida el formato UUID y rechaza el nil, y conserva los códigos `IAM_INVALID_ACCOUNT_ID(_FORMAT)`. R16 no cambia el contrato. Se descartan b) (códigos propios) y c) (`OwnerId` opaco).
- **D4. Eventos (R21, R22): decidido, se retira el andamiaje muerto.** `AccountEvent`, `take_events` (que nunca se escribió: hoy son `record_event`/`pull_events` con `todo!()`) y `MembershipPolicy`, con `MembershipPolicyChanged`, se borran porque no tienen comportamiento ni uso (HEX-7.4, HEX-7.9). Se implementarán cuando algo consuma los eventos. Efecto: R21 sin `create_records_registered` y sin `Clock`; R22 borra en lugar de mover (HEX-7.8).
- **D8. Hallazgos históricos (TDD-1.5 a TDD-1.7): decidido.** Quedan `descartado (histórico; TDD-1 rige desde R1)`.

Abiertas:

- **D5. Forma de Problem Details (R30).** Si se conserva `code` con los mismos valores, qué texto va en `detail`, y si los 401 llevan cuerpo.
- **D9. Mover `app/` a `apps/server`, como en el template.** Fuera del inventario; opcional después de R29.
- **D10. `identify` no normaliza el email.** Lo fija `identify_with_different_email_casing_returns_404`. No es un hallazgo; se mantiene salvo que se decida lo contrario.

### 5.8 Registro de rebanadas

Una fila por rebanada fusionada. **Tokens** es el consumo de la sesión que hizo la rebanada (lo que informa `/cost` al cerrarla). **Revisor** dice si se corrió `craft-backend:architecture-reviewer` y por qué no, si no se corrió.

| Rebanada | PR | Commit | Toolchain | Tokens | Revisor | Notas |
|---|---|---|---|---|---|---|
| R1 | #24 | `046c174` | `nightly-2026-11-19` | no registrado: ni #24 ni #25 traen el `/cost`, y la sesión ya no existe (revisado en P1) | no: no toca código de producción | Apilado sobre #23, que lleva #21 y #22 a `main`. fmt, clippy y 187 tests en verde localmente. #24 se fusionó en `ci/rust-checks`, no en `main`; lo lleva a `main` #25. |
| R2 | #26 | `ad2f129`, `4149660`, `8a70093` | `nightly-2026-11-19` | $2.67 (opus-5-5: 3.7k input, 39.0k output, 4.2m cache read, 143.1k cache write; 8m 19s API, 26m 9s wall) | sí (ver PR) | Apilado sobre #25. 188 tests, fmt y clippy en verde localmente. Sin test de tiempos: el Spy fija la llamada al señuelo. |
| R3 | #27 | `eb30039`, `f552921` | `nightly-2026-11-19` | $1.48 (opus-5-5: 1.8k input, 18.8k output, 1.9m cache read, 106.8k cache write; 4m 27s API, 4m 54s wall) | sí (ver PR) | Línea base −17 (todas las `err_1_*`). 188 tests, fmt y clippy en verde localmente. |
| R4 | #28 | `80c00e4` | `nightly-2026-11-19` | $0.80 (opus-5-5: 1.3k input, 10.5k output, 778.9k cache read, 59.8k cache write; 2m 34s API, 2m 40s wall) | sí (ver PR) | Línea base −2 (`hex_8_kernel_traits`: `domain_error.rs`, `infrastructure_error.rs`). 188 tests, fmt y clippy en verde localmente. |
| R5 | #30 | `e76c162`, `29890cb`, `446726b` | `nightly-2026-11-19` | $3.06 (opus-5-5: 4.6k input, 27.6k output, 4.1m cache read, 220.0k cache write; 6m 32s API, 12h 21m wall, casi todo inactividad entre turnos) | sí (ver PR) | Línea base sin cambios. Copia de ai-harness `97a8f29`; `Clock` pasa a R21. 194 tests, fmt y clippy en verde localmente. |
| R6 | #29 | `991e672` | `nightly-2026-11-19` | $1.37 (opus-5-5: 447 input, 16.6k output, 1.6m cache read, 98.0k cache write; 3m 17s API, 4m 26s wall) | sí (ver PR) | Línea base sin cambios. `Cargo.lock` solo pierde `anyhow`; features efectivas idénticas. 188 tests, fmt y clippy en verde localmente. Commit aparte `45fe532` con `/slice`. |
| S1 | #31 | `79185ff`, `c8dbed3` | `nightly-2026-11-19` | $3.81 (opus-5-5: 5.6k input, 56.1k output, 6.1m cache read, 207.1k cache write; 10m 3s API, 23m 29s wall) | sí (ver PR) | Fuera del plan: `tools/architecture` desde ai-harness `v1.0.0-rc.1`. Re-línea base 36 → 47 entradas, solo por cambio de formato (§5.10). Sin código de producción. |
| S2 | #33 | `75e262d` | `nightly-2026-11-19` | no registrado: #33 no trae el `/cost` (revisado en P1) | no: no toca código de producción | Fuera del plan: `tools/architecture` desde ai-harness `v1.0.0-rc.2`. Línea base 47 → 47, idéntica (§5.10). 233 tests, fmt y clippy en verde localmente. |
| R18 | #34 | `a603969` | `nightly-2026-11-19` | $1.80 (opus-5-5: 2.0k input, 20.9k output, 2.0m cache read, 131.8k cache write; 3m 54s API, 4m 46s wall) | sí (ver PR) | Línea base −1 (`hex_4_handlers` de `list_public_communities.rs`): 47 → 46. `find_by_id` y `AuthContext` se borran aquí (sin usos). 233 tests, fmt y clippy en verde localmente. Primera rebanada con `/craft-backend:adoption`. |
| R7 | #35 | `692bc7c`, `9eacddc`, `6b808a2`, `ba15189` | `nightly-2026-11-19` | $3.98 (opus-5-5: 5.2k input, 51.8k output, 6.9m cache read, 211.8k cache write; 9m 11s API, 11m 37s wall) | sí (ver PR) | Línea base −3 (`hex_5` del puerto, del doble y del adaptador en memoria): 46 → 43. Adaptador `InMemoryCommunityAdapter` y no `InMemoryCommunityRepositoryAdapter` como pedía el encargo (HEX-5). 244 tests, fmt y clippy en verde localmente. |
| R8 | #36 | `ebd0819`, `de6c308`, `9a90d8a`, `cc3d60c` | `nightly-2026-11-19` | $5.53 (opus-5-5: 27.3k input, 72.8k output, 10.0m cache read, 276.3k cache write; 12m 41s API, 42m 2s wall) | sí (ver PR) | Línea base −3 (`hex_5` del puerto, del doble y del adaptador en memoria): 43 → 40. Fallas por operación con `FakeAccountAdapter::failing_on` (el primer Stub, de respuesta fija, no pasaba la batería). 267 tests, fmt y clippy en verde localmente. |
| R9 | #37 | `73f9c44`, `c04f69d`, `17ba68e`, `dddc146` | `nightly-2026-11-19` | $4.46 (opus-5-5: 21.2k input, 62.4k output, 7.5m cache read, 226.0k cache write; 10m 36s API, 1h 16m wall) | sí (ver PR) | Línea base −3 (`hex_5` del puerto, del doble y de Argon2): 40 → 37. El señuelo propaga su `Fault` al arrancar y en authenticate; `build_app` devuelve `Result` y `app/tests/common/mod.rs` gana un `.expect` (decidido por el usuario). Hash truncado como `Fault`, a pedido del revisor. 285 tests, fmt y clippy en verde localmente. |
| R10a | #38 | `517e4cd`, `ce6d64b`, `65f455d`, `c9db5fa`, `1b08cea`, `62d5456` | `nightly-2026-11-19` | $5.54 (opus-5-5: 21.2k input, 73.1k output, 10.7m cache read, 256.7k cache write; 13m 14s API, 15m 45s wall) | sí (ver PR) | Primera de dos partes de R10 (el diff pasaba de 500 líneas). Línea base −3 (`hex_5` del puerto, del doble y de JWT): 37 → 34. Rojo de JWT en 4 casos, no solo en el vencido. El `Fault` se descarta de forma explícita en el commit de la firma y el `?` llega con su test. 300 tests, fmt y clippy en verde localmente. |
| R10b | #39 | `101f652`, `2aeeec0`, `9be8d19`, `c65817c`, `3c1a6bb` | `nightly-2026-11-19` | $2.25 (opus-5-5: 7.9k input, 30.0k output, 5.6m cache read, 78.6k cache write; 5m 31s API, 8m 50s wall). Misma sesión que R10a: es la diferencia entre el `/cost` acumulado ($7.79) y los $5.54 de R10a (confirmado en P1) | sí (ver PR) | Segunda parte de R10. Línea base −1 (`hex_8_kernel_traits` de `token_verifier.rs`): 34 → 33. Primer caso de uso con `Failure<E>`. Un `Fault` al verificar el token pasa de 401 a 500 (D7). 305 tests, fmt y clippy en verde localmente. |
| P1 | #41 (docs) y #40 (ERR-4.9) | ver PRs | `nightly-2026-11-19` | $4.32 hasta abrir #41, incluye #40 | sí: auditoría completa (ver §5.9) | Punto de replanificación. Fuera del plan, #40: un `JWT_SECRET` vacío o corto ya no arranca (alta, ERR-4.9). 16 hallazgos nuevos, R12b nueva, D1, D2, D4 y D8 decididas. 308 tests en #40. |

### 5.9 Puntos de replanificación

Hay tres: **P1** después de R10 *(hecho, 2026-11-23: ver abajo)*, **P2** después de R19 y **P3** después de R26 (antes de R27). En cada uno, antes de abrir la rebanada siguiente:

1. Auditoría completa con `craft-backend:architecture-reviewer` y actualización de la cabecera del documento (fecha, commit, conteos).
2. `cargo test -p architecture`, y comparar las entradas que quedan en `baseline.txt` con la columna "Entradas hoy" de §5.6 menos lo ya cerrado. Si no coinciden, explicar la diferencia antes de seguir.
3. Revisar las decisiones abiertas de §5.7 que bloquean las rebanadas siguientes: D1 (R13) y D2 (R16, si todavía no se hizo) en P1; D4 (R21, R22) en P2, y D5 (R30) en P3.
4. Comparar tamaño y tokens reales de §5.8 con lo estimado, y partir las rebanadas que vienen si alguna quedó más grande de lo previsto.
5. Reescribir en este documento lo que cambie del plan (secuencia, IDs, línea base) en un PR de solo documentación, antes de seguir.

**P1 (2026-11-23, sobre `bab68c5`).**
1. Auditoría completa de todo el repo con `craft-backend:architecture-reviewer`. Encontró un hallazgo de severidad alta (ERR-4.9: `JWT_SECRET` vacío), que se corrigió antes que este documento, en un PR propio (#40). Además, 15 hallazgos nuevos de severidad media o baja (§2). Pasó a resuelto ERR-2.8, y se actualizaron la ubicación o el estado de unos 25 IDs (ERR-2.7, ERR-2.9 y ERR-2.10 ya tenían los tests de repositorio desde R7/R8; RS-3.1 nombraba `token_verifier.rs`).
2. `cargo test -p architecture` en verde. Las 33 entradas coinciden con §5.6, cada una con su ID y su rebanada.
3. D1, D2, D4 y D8 decididas (§5.7) y propagadas a R13, R16, R21, R22 y R30.
4. Tamaños: R10 se partió en dos (R10a pasó de 700 líneas) y costó $7.79 en total, frente a $3.98–$5.53 de R7 a R9. Con los tests de repositorio ya escritos, R13 baja a S y R22 a S. R12b es nueva (S). Ninguna de las que vienen se parte.
5. R11 a R17 revalidadas contra el código: nombres, `failing_on`, el `?` sin la secuencia de R10a y el puente `SystemError::Unexpected` (texto antes de R11). `ARCH_REQUIRE_MATERIAL=1` queda condicionado en R11 (ver allí).

### 5.10 Sincronizaciones con el harness

Una por PR propio, sin cambios de código (`/craft-backend:adoption sincronizar <tag>`). El análisis completo de `compare_baseline.py` va en el PR; aquí, el resumen.

| Fecha | Tag | PR | Entradas antes → después | Se mantienen | Cambian de clave | Nuevas (IDs) |
|---|---|---|---|---|---|---|
| 2026-11-22 | `v1.0.0-rc.1` | #31 | 36 → 47 | 29 | 7 → 18 (formato `#sección.paquete`, `n` por capa virtual) | ninguna |
| 2026-11-23 | `v1.0.0-rc.2` | #33 | 47 → 47 | 47 | ninguna | ninguna |

#### S2: `v1.0.0-rc.2`

Sincroniza `tools/architecture` con `stacks/rust/template/tools/architecture` de ai-harness `v1.0.0-rc.2` (`451fec9`), sin tocar código de producción. Entre `v1.0.0-rc.1` y `v1.0.0-rc.2` la plantilla cambia dos archivos:
- `baseline.txt`: solo la cabecera. Ya no dice "se llena una sola vez": vuelve a llenarse al sincronizar con una versión que cambia las claves. Se conserva la línea propia del proyecto ("Los IDs de hallazgo… están en docs/harness-adoption.md").
- `tests/legacy.rs:45`: `iter().any(|d| d == "axum")`, la misma corrección que S1 aplicó a mano. La divergencia de S1 desaparece.

El resto de la copia queda idéntica, salvo los `use` que reagrupa `cargo fmt` con el `rustfmt.toml` del proyecto (`common/mod.rs`, `errors.rs`, `output.rs`, `ports.rs` y `ratchet.rs`, igual que en S1). Fuera de `tools/architecture/baseline.txt` el diff está vacío.

El CHANGELOG de `v1.0.0-rc.2` no cambia ni las claves ni los conteos. Igual se volvió a llenar, para confirmarlo: con `CRAFT_BACKEND_BASELINE_INIT=1`, la línea base se vació hasta la cabecera, se corrió `cargo test -p architecture --no-fail-fast` y se escribió una entrada por cada violación reportada.

Resultado de `compare_baseline.py` (salida 0):
- **Se mantienen: 47.** Son todas, con la misma clave y la misma `n`.
- **Cambian de clave: ninguna.**
- **Desaparecen sin un cambio de formato que lo explique: ninguna.** Tampoco baja ningún conteo.
- **Nuevas: ninguna.** Tampoco sube ningún conteo.

**Rebanadas que cambian: ninguna.** §5.6 queda igual.

#### S1: `v1.0.0-rc.1`

Rebanada fuera del plan. Sincroniza `tools/architecture` con `stacks/rust/template/tools/architecture` de ai-harness `v1.0.0-rc.1` (`3b5dc80`), sin tocar código de producción. Los archivos se copiaron tal cual: `cargo fmt` con el `rustfmt.toml` del proyecto solo reagrupa los `use` de 5 archivos. Única divergencia de contenido: en `tests/legacy.rs:45`, `contains(&"axum".to_string())` pasa a `iter().any(|d| d == "axum")`, porque el clippy de `nightly-2026-11-19` lo rechaza (`unnecessary_to_owned`) con `-D warnings`. Hay que reportarlo al template. `tests/common/mod.rs` es el del template, sin adaptación local. Su modo legacy oficial ya coincide con este repo: `LEGACY_KERNEL_DIR = "shared"`, y `iam` y `communities` pasan a legacy porque tienen `Cargo.toml` propio. Fuera de `tools/`, solo cambia el `Cargo.toml` raíz, que gana `publish = false` en `[workspace.package]`. Lo exige `publish.workspace = true` del template.

Línea base vieja: 36 entradas. Nueva: 47 entradas (65 violaciones con las `n`). La nueva se llenó con `CRAFT_BACKEND_BASELINE_INIT=1` a partir del informe de los tests, con la línea base vaciada a la cabecera. La cabecera del template dice que la línea base "se llena una sola vez". Este segundo llenado es una excepción del proyecto, que §5.1 acota a las sincronizaciones con el harness: cambia el formato de las claves, no las violaciones que se toleran.

**a) Se mantienen (29).** Todas las entradas de ruta de archivo, con la misma clave:
- `hex_4_use_case_implementations_are_handlers`: 6;
- `hex_5_adapters_are_named_by_technology_and_concept`: 8;
- `hex_5_outbound_ports_are_named_by_role`: 4;
- `hex_6_transport_types_carry_the_transport_name`: 9;
- `hex_8_kernel_traits_are_model_markers_or_listed_capabilities`: 2 (`token_verifier.rs`, `error.rs`).

**b) Cambian de clave por el formato nuevo (7 → 18).** La clave pasa de `ruta/Cargo.toml` a `ruta/Cargo.toml#sección.paquete`, y `n` cuenta las capas virtuales del contexto legacy que ven la dependencia: domain y application en `hex_2_core` (2); domain, application e infrastructure en `hex_2_frameworks` (3, porque Api puede usar axum), y las cuatro capas de communities en `hex_1` (4).

| Test | Clave vieja | Claves nuevas | ID |
|---|---|---|---|
| `hex_1_dependencies_point_inward` | `contexts/communities/Cargo.toml` | `…#dependencies.iam` 4 | HEX-1.1 |
| `hex_2_core_depends_only_on_std_and_the_kernel` | `contexts/communities/Cargo.toml` | `…#dependencies.axum` 2, `.http` 2, `.serde` 2, `.uuid` 2 | HEX-2.5 |
| `hex_2_core_depends_only_on_std_and_the_kernel` | `contexts/iam/Cargo.toml` | `…#dependencies.argon2` 2, `.axum` 2, `.chrono` 2, `.jsonwebtoken` 2, `.rand` 2, `.serde` 2, `.uuid` 2 | HEX-2.1 |
| `hex_2_core_depends_only_on_std_and_the_kernel` | `shared/Cargo.toml` | `…#dependencies.chrono`, `.serde` | HEX-8.1 |
| `hex_2_server_frameworks_only_at_the_edge` | `contexts/communities/Cargo.toml` | `…#dependencies.axum` 3 | HEX-2.5 |
| `hex_2_server_frameworks_only_at_the_edge` | `contexts/iam/Cargo.toml` | `…#dependencies.axum` 3 | HEX-2.1 |
| `hex_8_kernel_depends_only_on_std` | `shared/Cargo.toml` | `…#dependencies.chrono`, `.serde` | HEX-8.1 |

Cada paquete nuevo ya figuraba en el hallazgo de la columna ID. La única excepción es `http` en communities: HEX-2.5 nombra `uuid`, `axum` y `serde`, y `http` (que usa `infrastructure/http`) es el mismo hallazgo, con un paquete más. Se agrega a su fila.

**c) Desaparecen sin un cambio de formato que lo explique: ninguna.** Las 7 claves viejas que ya no están corresponden, una por una, a las claves nuevas de b).

**d) Entradas nuevas: ninguna.** Ninguna regla nueva del template encuentra violaciones en este repo. Los tests nuevos (`legacy.rs`, `ratchet.rs`, `hex_3_sees_the_legacy_kernel`, `hex_4_contracts_are_checked_in_legacy_contexts`) y los que ahora actúan en legacy (`hex_4_use_case_contracts_do_not_expose_domain_types`, con 17 tipos de contrato) pasan en verde.

**Rebanadas que cambian:**
- **R21:** copia `Clock` del template en lugar de escribirlo, y quita `chrono` de `contexts/iam/Cargo.toml` y de `shared/Cargo.toml`, que dejan de usarlo. Línea base: de 0 a −3.
- **R25:** −2, pero ahora son las dos entradas de `serde` de `shared/Cargo.toml`.
- **R27:** de −4 a −7. Deja de tener el riesgo de "HEX-4 se activa en este PR".
- **R28:** de −9 a −14.
- **R29:** borra todo lo marcado `legacy:` en `tools/architecture/tests`, más `tests/legacy.rs` y `tests/fixtures/legacy/`.
- **R11–R18:** ya no necesitan el `grep "crate::domain"` a mano (§5.1).
- Cualquier rebanada que quite un paquete de un `Cargo.toml` borra sus entradas en el mismo PR (§5.1).

**Kernel del template desde `97a8f29` (no se copia aquí).** Commits `ec3d9ea` y `e1cfa7d`:
- `kernel/src/clock.rs` (`trait Clock: Send + Sync { fn now(&self) -> SystemTime; }`) y `pub use clock::Clock` en `kernel/src/lib.rs` → ~~**R21**~~ *(P1, D4: sin eventos no tiene uso; entra con la primera rebanada que necesite la hora)*.
- Crate nuevo `infrastructure/system` con `SystemClockAdapter` y su test → **R21**, junto con `Clock`, si R21 cablea el reloj real. Si no, en R26, junto con `infrastructure/stderr`.
- `infrastructure/stderr/src/fault_reporter.rs`: el reporte recorre la cadena `source()` del `Fault` (+64 líneas) → **R26**, que crea ese crate. Hay que copiarlo desde `v1.0.0-rc.1`, no desde `97a8f29`.
- `failure.rs`, `fault.rs` e `id.rs` del template coinciden con los de `shared/src` (solo cambia la agrupación de `use` por `cargo fmt`). `future.rs` y `model.rs` siguen asignados a R23/R24 y R21.

## 6. Fricciones con el harness

Lo que el harness no cubrió o cubrió mal durante la adopción. Cada fricción se reporta al harness. Al resolverse, se anota la versión que la cierra.

| Fecha | Rebanada | Fricción | Efecto en la adopción | Estado en el harness |
|---|---|---|---|---|
| 2026-11-22 | S1 | `tools/architecture/tests/legacy.rs:45` de `v1.0.0-rc.1` no pasa clippy con `-D warnings` en `nightly-2026-11-19` (`unnecessary_to_owned`). | La copia "tal cual" diverge del template en una línea (§5.10). | Corregida en craft-backend 1.0.0-rc.2 (v1.0.0-rc.2, PR #17) |
| 2026-11-22 | S1 | `CRAFT_BACKEND_BASELINE_INIT=1` solo se lee al abrir la sesión, y nada lo advierte al empezar una sincronización. | Hubo que cortar S1 a la mitad y reabrir con `claude --continue`. | Corregida en craft-backend 1.0.0-rc.2 (v1.0.0-rc.2, PR #17) |
| 2026-11-22 | S1 | La cabecera del template dice que la línea base "se llena una sola vez", pero un cambio de formato de clave obliga a re-llenarla. El formato no cambió en `v1.0.0-rc.1`: cambió en 0.17.0 (`ruta/Cargo.toml` pasa a `ruta/Cargo.toml#sección.paquete`) y en 0.18.0 (en legacy, `n` cuenta las capas virtuales que ven la dependencia). S1 recibió los dos cambios juntos porque la copia anterior del proyecto era previa a 0.17.0. | Regla propia en §5.1 y análisis en §5.10. | Corregida en craft-backend 1.0.0-rc.2 (v1.0.0-rc.2, PR #17) |
| 2026-11-23 | — | `/craft-backend:adoption` abre el PR solo si se le pide, y no trae las convenciones del proyecto. | Se pasaron a §5.1 al retirar `/slice`. | Corregida en craft-backend 1.0.0-rc.2 (v1.0.0-rc.2, PR #17) |
| 2026-11-23 | R7 | Las baterías del template (`contracts/*.rs`) son `async fn` con un parámetro `run`, porque los puertos del template ya son `BoxFuture`. En una adopción con puertos todavía síncronos (RS-3 pendiente), la skill no dice qué forma darle a la batería. | La batería de R7 se escribió síncrona, sin `run`. R24 tiene que pasarla a `async` y agregar `run` a la macro y a sus dos invocaciones. | sin reportar |
| 2026-11-24 | R8 | El `Fake…Adapter` del template simula las fallas con `failing()`, que hace fallar **todas** las operaciones. Un handler que llama al puerto más de una vez (`find_by_email`, `find_by_username`, `save`) nunca llega a la segunda llamada con ese doble, así que el `?` de las llamadas posteriores queda sin test. El template no trae un doble que falle en una sola operación, y la skill `tdd` no dice cómo cubrir cada llamada. | En R7 se resolvió con un `StubCommunityAdapter` local (ba15189), a pedido del revisor. En R8, `FakeAccountAdapter` conserva el `failing()` del template y gana `failing_on(AccountOperation)`, que hace fallar una sola operación mientras las demás siguen el contrato (cc3d60c). Un primer intento con un `StubAccountAdapter` de respuesta fija devolvía la cuenta configurada para cualquier valor buscado, el mismo defecto de TDD-2.4, y no pasaba la batería; el revisor lo marcó. | sin reportar |
| 2026-11-23 | R9 | Al volver falible la firma de un puerto, el handler necesita el `?` para compilar, pero el doble que falla (y su test) llega después, con la batería. La skill `tdd` no dice cómo ordenar los commits: con `?` en el refactor de firma, el historial muestra la producción antes que el test, y el rojo solo se ve tragando el `Fault` a mano. El revisor propone descartar el `Fault` de forma explícita (o `todo!`) en el commit de la firma y traer el `?` con el test rojo. | En R8 y R9 el rojo se vio por mutación en el árbol de trabajo y se anotó en el mensaje del commit. R10a siguió la propuesta: el commit de la firma (517e4cd) descarta el `Fault` de forma explícita, y el `?` llega con su test (65f455d), así que el rojo se reproduce contra el commit padre. | sin reportar |
| 2026-11-23 | P1 | `ARCH_REQUIRE_MATERIAL=1` convierte en fallo un conjunto vacío, y la plantilla la activa porque ejercita cada regla. En una adopción, dos conjuntos quedan vacíos de forma legítima y permanente: los errores declarados de puertos (`err_1_application_errors_…`), que el propio harness elimina al pasar los puertos a `Fault`, y los `EventSubscriber` (`hex_4_event_subscribers_…`), si el proyecto no tiene eventos. La variable no admite excluir una regla. | R11 no puede activarla tal cual (§5.3, R11). | sin reportar |
| 2026-11-23 | P1 | Reglas expresables como test que `tools/architecture` no tiene: contratos con el dominio del kernel o un segundo argumento en `execute` (VER-1.3), `api → domain` en legacy (VER-1.4), firmas de los puertos entrantes (VER-1.5), marcadores sin ítems (VER-1.6). Antes: `expect("invariante: …")` (R9), adaptadores driving sin `ports::outbound` y DTOs en `ports/outbound/dtos/` (R10a, R10b), `package =` y `workspace = true` en los manifiestos (R6). | Las violaciones quedan sin trinquete y dependen del revisor. | sin reportar |
| 2026-11-23 | S2 | El paso 4 de `sincronizar` pide escribir "una entrada por cada violación reportada", pero nada vuelca el informe al formato de `baseline.txt`. Además, `cargo test -p architecture` se corta en el primer binario que falla, y sin `--no-fail-fast` el informe queda incompleto. | Hubo que correrlo con `--no-fail-fast` y pasar el informe a entradas con un script propio, que también cuenta las `n` (§5.10). | sin reportar |

## Método

- Se leyó entero cada archivo `.rs` y `Cargo.toml` versionado, y cada `archivo:línea` se verificó contra su contenido en `d0b0eeb`.
- No se ejecutaron `cargo test` ni `clippy` (auditoría en solo lectura). El repo no tiene tests de arquitectura que correr.
- El orden de los commits se revisó con `git log --reverse`.
