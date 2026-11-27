# Adopción del harness craft-backend: archivo

Lo cerrado de [`../harness-adoption.md`](../harness-adoption.md), movido sin cambios. Los números de sección son los del documento vivo. Los costos están en [`costs.md`](costs.md).

## 1. Resumen

| Regla | Alta | Media | Baja | Total |
|---|---|---|---|---|
| HEX-1 Dependencias hacia adentro | 6 | 3 | 0 | 9 |
| HEX-2 Núcleo sin framework | 3 | 4 | 0 | 7 |
| HEX-4 Puertos entrantes | 0 | 2 | 3 | 5 |
| HEX-5 Puertos por rol | 0 | 2 | 0 | 2 |
| HEX-6 Vocabulario | 0 | 0 | 2 | 2 |
| HEX-7 El dominio decide | 1 | 7 | 6 | 14 |
| HEX-8 Kernel delgado | 3 | 5 | 4 | 12 |
| VER-1 Verificación automática | 2 | 1 | 6 | 9 |
| ERR-1 Dos árboles | 2 | 4 | 2 | 8 |
| ERR-2 Contrato por caso de uso | 2 | 5 | 5 | 12 |
| ERR-3 Semántica de adaptadores | 0 | 3 | 1 | 4 |
| ERR-4 Transporte en el borde | 3 | 6 | 5 | 14 |
| TDD-1 Test primero | 0 | 4 | 4 | 8 |
| TDD-2 Tests por capa y dobles | 1 | 3 | 7 | 11 |
| RS-1 Crate por capa | 1 | 1 | 4 | 6 |
| RS-2 `Fault` y `panic!` | 1 | 4 | 1 | 6 |
| RS-3 Puertos async | 0 | 1 | 0 | 1 |
| RS-4 Tests de arquitectura | 1 | 0 | 0 | 1 |
| **Total** | **26** | **55** | **50** | **131** |

Cada fila del inventario cuenta como un hallazgo. Cuando una misma violación se repite igual en varios archivos, la fila lista todas las ubicaciones.

`shared/` se audita como si fuera el kernel, porque hoy cumple ese papel aunque no tenga la forma que pide RS-1. Ver la sección 4.

## 2. Inventario

### HEX (hexagonal-architecture) y VER

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| HEX-1.1 | HEX-1 Dependencias | contexts/communities/Cargo.toml:16 | `communities` declara `iam` como dependencia: un contexto depende de otro. | alta | Quitar `iam`; `communities` modela su propio `OwnerId`. | resuelto (f6bc1fd) |
| HEX-1.2 | HEX-1 Dependencias | contexts/communities/src/domain/model/community/community.rs:7 | El dominio de `communities` importa `iam::…::AccountId` y lo guarda como `_owner_id` (línea 12). | alta | Value object `OwnerId` propio del contexto. | resuelto (3646f25, f6bc1fd) |
| HEX-1.3 | HEX-1 Dependencias | contexts/communities/src/application/commands/create_community/create_community.rs:17; application/testing/contracts/community_repository.rs:14 | El handler importa `AccountId` de `iam` y valida con él el sujeto del token (línea 47). También lo importan la batería de contrato (desde R7) y el módulo de tests del handler (línea 86). | alta | Validar con `OwnerId::new` del propio contexto. | resuelto (f6bc1fd): `OwnerId::from_str` |
| HEX-1.4 | HEX-1 Dependencias | contexts/communities/src/infrastructure/http/errors/error_mapper.rs:12 | El borde HTTP de `communities` importa códigos de error del dominio de `iam`. | media | Códigos propios en `communities-api/src/codes.rs`. | resuelto (f6bc1fd): el mapper toma `IAM_INVALID_ACCOUNT_ID(_FORMAT)` de `infrastructure/http/codes.rs`. Los demás códigos de communities salen de domain con R17 (HEX-1.8). *(P2)* El mapper ya no existe (11f8c92); esos dos códigos los usa el `fold` (`create_community.rs:79-91`) |
| HEX-1.5 | HEX-1 Autenticación | shared/src/application/ports/outbound/token_verifier.rs:6 | `TokenVerifier` vive en un `shared` que ven todos los contextos: es la violación típica que nombra la regla. | alta | `VerifyAccessTokenUseCase` en `iam/application/queries` + puerto `AccessTokenService`. | resuelto (517e4cd, 101f652, 9be8d19, 3c1a6bb): `AccessTokenService` en R10a; la query `VerifyAccessToken`, el middleware sobre ella y el borrado de `TokenVerifier` en R10b |
| HEX-1.7 | HEX-1 Dependencias | contexts/iam/src/infrastructure/http/errors/error_mapper.rs:3 | El adaptador driving (HTTP) importa `crate::domain::errors::error_codes`: `api` ve `domain`. | media | `api` solo ve el contrato `{Verb}{Noun}Error` de `application`. | resuelto (11f8c92): R19 borró el mapper con sus 9 códigos de dominio; la entrada `hex_1_driving_adapters_see_only_use_cases` (n = 9) se borró y el test quedó en cero. La separación del crate `iam-api` sigue en R28 (HEX-1.9) |
| HEX-1.8 | HEX-1 Dependencias | contexts/communities/src/infrastructure/http/errors/error_mapper.rs:3 | Igual que HEX-1.7: el `api` de `communities` importa códigos de `domain`. | media | Igual que HEX-1.7. | resuelto (838b923): R17. El `fold` de `CreateCommunity` traduce por variante, así que los `impl ErrorLayout` de `InvalidCommunityName`, `InvalidCommunitySlug` e `InvalidOwnerId` quedaron sin llamadores y se borraron con sus constantes de dominio; sin productores, se borró el brazo 400 del mapper. El mapper sigue en su ruta hasta R19 y ya no nombra `crate::domain`: con la entrada borrada, el test la reportó en el mismo archivo y pasó a verde al quitar el import. Tampoco queda el acoplamiento por valor de R16: ningún error de communities responde un código de cliente por `ErrorLayout` |
| HEX-2.2 | HEX-2 Núcleo sin framework | contexts/iam/src/domain/model/account/account_id.rs:3 | El dominio usa `uuid::Uuid`. | alta | El id llega de `kernel::IdGenerator`; `uuid` solo en infraestructura. | resuelto (bae89e9, d461bbc): `AccountId` sobre `String`; `uuid` salió de `contexts/iam/Cargo.toml` |
| HEX-4.1 | HEX-4 Casos de uso | iam/…/authenticate_account.rs:17; register_account.rs:18; verify_account.rs:14; identify_account.rs:16; communities/…/create_community.rs:21; list_public_communities.rs:14 | Las implementaciones no se llaman `{Verb}{Noun}Handler` (`AuthenticateAccount`, `RegisterAccount`…). | media | Renombrar a `AuthenticateAccountHandler` y demás, en `handler.rs`. | resuelto (4a04532): R18 (a603969) borró `list_public_communities.rs`; R11 (1482280) movió register a `RegisterAccountHandler` en `commands/register_account/handler.rs`; R13 (2d16725), verify a `VerifyAccountHandler` en `commands/verify_account/handler.rs`; R14 (09fb976), authenticate a `AuthenticateAccountHandler` en `commands/authenticate_account/handler.rs`; R15 (cf2b905), identify a `IdentifyAccountHandler` en `queries/identify_account/handler.rs`; R17 (4a04532), create a `CreateCommunityHandler` en `commands/create_community/handler.rs`. Resuelto |
| HEX-4.2 | HEX-4 Casos de uso | contexts/communities/src/application/ports/inbound/create_community_use_case.rs:10 | `execute` recibe un segundo argumento, `VerifiedIdentity`, del dominio del kernel: el contrato expone un tipo de `domain` fuera del `Command`. | media | Campo `owner_id: String` en `CreateCommunityCommand`. | resuelto (58958bc): R17. `execute(command)`, y el `fold` de `infrastructure/http/create_community.rs` copia el sujeto de `VerifiedIdentity` a `owner_id`. Se borraron las dos entradas (`hex_4_use_case_signatures_…`, n = 2, y `hex_4_use_case_contracts_…`) |
| HEX-4.3 | HEX-4 Casos de uso | contexts/communities/src/application/ports/inbound/list_public_communities_use_case.rs:6 | Le falta `Send + Sync`, recibe un `AuthContext` que no usa (línea 10) y no está cableado. | baja | Firma `execute(query)`, con `Send + Sync`, o eliminarlo. | resuelto (a603969): eliminado (D3) |
| HEX-4.4 | HEX-4 Casos de uso | contexts/iam/src/application/errors/authenticate_account.rs:6 | El `Error` del caso de uso vive en `errors/`, no junto al handler. | baja | `commands/authenticate_account/error.rs`. | resuelto (7932c45) |
| HEX-4.5 | HEX-4 Casos de uso | contexts/iam/src/application/ports/inbound/verify_account_use_case.rs:5 | `execute` responde `()`: no hay `VerifyAccountResponse` en `commands/verify_account/`. R13 lo decidió así (el caso no devuelve datos, y el revisor lo confirmó), pero `hex_4_use_case_signatures_are_their_own_contract` exige un `{Verb}{Noun}Response` y cuenta 2 violaciones (el tipo que falta y la firma). Hallado en S3. | baja | `VerifyAccountResponse` vacío en `commands/verify_account/response.rs`; el `fold` de `infrastructure/http/verify_account.rs` sigue respondiendo 200 con cuerpo `null`, sin serializarlo. O, si el harness acepta `()` para un caso sin datos (§6), borrar la entrada. | resuelto (d37fe10): R13b (D11 a) |
| HEX-5.1 | HEX-5 Rol | iam/…/ports/outbound/account_repository.rs:6; password_hasher.rs:1; token_generator.rs:3; communities/…/community_repository.rs:6 | Sufijo `Port`, prohibido. En `PasswordHasher` y `TokenGenerator`, al quitarlo, no queda rol. | media | `AccountRepository`, `CommunityRepository`, `PasswordHashingService`, `AccessTokenService`. | resuelto (692bc7c, ebd0819, 73f9c44, 517e4cd): `CommunityRepository` en R7, `AccountRepository` en R8, `PasswordHashingService` en R9 y `AccessTokenService` (fusión con `TokenVerifier`) en R10a |
| HEX-5.2 | HEX-5 Adaptadores | iam/…/in_memory/account_repository.rs:14; communities/…/in_memory/community_repository.rs:13; iam/…/argon2_password_hasher.rs:8; iam/…/jwt_service.rs:15 | Los adaptadores no siguen `{Tecnología}{Concepto}Adapter`: repiten el rol o terminan en `Service`. | media | `InMemoryAccountAdapter`, `InMemoryCommunityAdapter`, `Argon2PasswordHashingAdapter`, `JwtAccessTokenAdapter`. | resuelto (6b808a2, 9a90d8a, 17ba68e, c9db5fa): el último, `JwtAccessTokenAdapter`, en R10a |
| HEX-6.2 | HEX-6 Vocabulario | contexts/communities/src/application/queries/list_public_communities/response.rs:5 | `CommunityResult` usa el sufijo `Result`, reservado para control de flujo. | baja | `PublicCommunity` o `CommunitySummary`. | resuelto (a603969): eliminado (D3) |
| HEX-7.1 | HEX-7 Lógica en handler | contexts/iam/src/application/commands/register_account/register_account.rs:40 | La unicidad se consulta con el texto crudo (`cmd.email`, y `cmd.username` en la línea 48), no con el value object normalizado. Resultado: se registran duplicados por mayúsculas o espacios, como confirma el test `register_accepts_same_email_with_different_casing`. | alta | Construir `Email`/`Username` antes y buscar por el valor normalizado (como hace `CreateCommunity` con el slug). | resuelto (77e0ca4, 06a5676): `AccountFactory::draft` construye los value objects y el handler busca con sus valores |
| HEX-7.2 | HEX-7 Lógica en handler | contexts/iam/src/application/commands/authenticate_account/authenticate_account.rs:70 | `if !account.can_authenticate()` es un `if` de negocio en el handler. | media | `account.authenticate() -> Result<_, CannotAuthenticate>` en el agregado. | resuelto (c6c3281, 8adb31f): `Account::authenticate() -> Result<(), CannotAuthenticate>`; `fails_with_login_failed_when_an_inactive_account_gets_a_wrong_password` fija que la contraseña se verifica antes |
| HEX-7.10 | HEX-7 Lógica en handler | contexts/iam/src/application/commands/verify_account/verify_account.rs:28 | `find_by_email` recibe `cmd.email` crudo, no el `Email` normalizado. El alta guarda el email en minúsculas, así que verificar con `Ana@Example.com` responde 404 a una cuenta que existe. Misma causa que HEX-7.1. Hallado en P1. | media | Construir `Email` y buscar por su valor. | resuelto (8460492): un email mal formado es `NotFound`, como uno desconocido |
| HEX-7.11 | HEX-7 Lógica en handler | contexts/iam/src/application/commands/authenticate_account/authenticate_account.rs:47 | `find_by_username` recibe `cmd.username` sin el recorte que aplica `Username`. Misma causa que HEX-7.1. Hallado en P1. | media | Buscar por `Username`; si no se puede construir, `LoginFailed` con la verificación del señuelo (D6). | resuelto (5359976) |
| HEX-7.12 | HEX-7 Comportamiento | contexts/communities/src/domain/errors/community_error.rs:10 | `CommunityError` está muerto: 6 variantes, con sus códigos en `domain/errors/error_codes.rs:4-9`, que nada produce. Residuo de la membresía, como HEX-7.9. Hallado en P1. | baja | Borrarlo con sus códigos. | resuelto (11f8c92): R19 borró `CommunityError` y sus 6 códigos |
| HEX-8.3 | HEX-8 Kernel | shared/src/error.rs:10 | `ErrorLayout` y `SystemError` (línea 17), un error global por capas, viven en el kernel. | alta | `Fault` + `Failure<E>` (RS-2). | resuelto (11f8c92): R19 borró `shared/src/error.rs`; con él, `hex_8_kernel_traits_are_model_markers_or_listed_capabilities` quedó en cero |
| HEX-8.4 | HEX-8 Marcadores de error | shared/src/domain/domain_error.rs:3; shared/src/infrastructure/infrastructure_error.rs:3 | Traits `DomainError` e `InfrastructureError`: Rust no los necesita (y aquí nadie los usa). | media | Eliminarlos. | resuelto (80c00e4) |
| HEX-8.5 | HEX-8 Sin puertos | shared/src/application/ports/outbound/token_verifier.rs:6 | Un puerto de negocio (`TokenVerifier`) y su `TokenError` (error.rs:2) están en el kernel; no es una capacidad de la lista. | alta | Moverlo a `iam` (ver HEX-1.5). | resuelto (3c1a6bb): R10b borró `TokenVerifier`, `TokenError` y `shared::application` |
| HEX-8.7 | HEX-8 Kernel | shared/src/application/common_application_error.rs:2 | `CommonApplicationError` está muerto y es un error compartido de aplicación. | baja | Eliminarlo. | resuelto (80c00e4) |
| VER-1.1 | VER-1 Tests de arquitectura | Cargo.toml:2-5 | No existe `tools/architecture`: ninguna regla de dependencias o de nombres tiene test. | alta | Agregar `tools/architecture` (la plantilla de rust-stack). | resuelto (ad558fc) |
| VER-1.2 | VER-1 CI | .github/workflows/sync-development.yml:22-58 | El único workflow sincroniza ramas; no corre `fmt`, `clippy` ni `test`. | alta | Workflow con el CI mínimo de RS-4. | resuelto (c54f6ae) |
| VER-1.3 | VER-1 Tests de arquitectura | tools/architecture/tests/naming.rs:537 | El test de contratos no trata `shared::domain` como dominio ni cuenta los argumentos de `execute`: HEX-4.2 (`VerifiedIdentity` como segundo argumento) pasa en verde. Hallado en P1. | baja | Proponer al harness: el dominio del kernel cuenta como dominio, y un solo parámetro además de `&self`. | resuelto en el harness (S3, `v1.0.0-rc.3`): `hex_4_use_case_signatures_are_their_own_contract` cuenta los argumentos, y `hex_4_use_case_contracts_do_not_expose_domain_types` trata `shared::domain` como dominio. HEX-4.2 tiene sus entradas |
| VER-1.4 | VER-1 Tests de arquitectura | tools/architecture/tests/naming.rs | Ningún test prohíbe que el adaptador driving legacy (`infrastructure/http`) importe `crate::domain`: HEX-1.4, HEX-1.7 y HEX-1.8 no tienen entrada en la línea base. Hallado en P1. | baja | Proponer al harness un test de imports `api → domain` sobre la capa virtual. | resuelto en el harness (S3, `v1.0.0-rc.3`): `hex_1_driving_adapters_see_only_use_cases`. HEX-1.7 y HEX-1.8 tienen sus entradas; HEX-1.4 sigue en la de HEX-1.1 |
| VER-1.5 | VER-1 Tests de arquitectura | tools/architecture/tests/naming.rs | Nada prohíbe `shared::error::SystemError` en `ports/inbound` (ERR-2.1) ni exige que `{Verb}{Noun}Error` viva junto al handler (HEX-4.4, ERR-2.4). Hallado en P1. | baja | Proponer al harness un test de firmas de los puertos entrantes. | resuelto en el harness (S3, `v1.0.0-rc.3`): `hex_4_use_case_signatures_are_their_own_contract` exige `Failure<{Verb}{Noun}Error>` y los tres tipos junto al handler. ERR-2.1 tiene sus entradas |
| VER-1.6 | VER-1 Tests de arquitectura | tools/architecture/tests/kernel.rs:69 | El test acepta como marcadores `AggregateRoot` y `ValueObject` con métodos (HEX-8.9, HEX-8.10). Hallado en P1. | baja | Proponer al harness: un marcador es un trait sin ítems. | resuelto en el harness (S3, `v1.0.0-rc.3`): `hex_8_model_markers_have_no_items`. HEX-8.9, HEX-8.10, HEX-8.11 y HEX-8.12 tienen sus entradas |
| VER-1.8 | VER-1 Tests de arquitectura | tools/architecture/tests/dependencies.rs:87 | En legacy, `hex_2_core_depends_only_on_std_and_the_kernel` agrupa las violaciones por manifiesto: un `use` nuevo de un crate prohibido en un archivo de domain o application no cambia ninguna entrada. Así entró `use uuid::Uuid` en `owner_id.rs` en R16 (HEX-2.7). Hasta R27 y R28, cualquier `use chrono`, `rand`, `uuid` o `serde` nuevo en el núcleo pasa en verde. Hallado en P2. | media | Proponer al harness (§6) un test de imports por capa virtual en legacy, con línea base por archivo. Mientras tanto, el revisor lo busca en cada rebanada. | resuelto en el harness (S4, `v1.0.0-rc.4`): `hex_2_core_imports_only_std_and_the_kernel` (1.0.0-rc.3.1) falla por archivo. Sus 6 entradas tienen ID y rebanada: HEX-2.3 (R20b), HEX-2.4 y HEX-8.2 (R21), HEX-2.6 y HEX-2.7 (R22), HEX-8.8 (R25) |
| VER-1.9 | VER-1 Tests de arquitectura | tools/architecture/tests/naming.rs | Ningún test exige que el `fold` de un `{Verb}{Noun}Error` sea exhaustivo y sin `_ =>`. ERR-4.5 se cerró borrando los mappers, pero nada impide volver a meter un comodín. Hoy ningún `fold` lo tiene, y los brazos que enlazan un `field` o `resource` desconocido van a `unexpected` (500). Es la fricción de R17 en §6. Hallado en P2. | baja | Proponer al harness un test: un `match` sobre `Failure::Expected(e)` o un `…Error` del contrato no admite `Pat::Wild`, y un brazo que enlaza un `&str` desconocido solo llama a `unexpected`. | resuelto en el harness (S4, `v1.0.0-rc.4`): `err_4_driving_folds_are_exhaustive` (1.0.0-rc.3.2) pasa en verde, sin entradas. Los `fold` que eligen por el texto de `field` o `resource` no son un enum y la regla no los ve: es ERR-4.13 |

### ERR (error-handling)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| ERR-1.1 | ERR-1 Dos árboles | shared/src/error.rs:17 | `SystemError` mezcla los fallos de negocio con los técnicos (`Infrastructure`) en un tipo. El `impl<T: ErrorLayout> From<T>` (línea 40) hace que `?` los absorba implícitamente. | alta | `Failure<{Verb}{Noun}Error>` y `Fault` separados. | resuelto (11f8c92): R19. Los casos de uso devuelven `Failure<{Verb}{Noun}Error>` desde R17 y el árbol B es `Fault`; `SystemError` y su `From` genérico se borraron |
| ERR-1.2 | ERR-1 Errores de dominio | iam/src/domain/errors/account.rs:27, account_id.rs:20, account_status.rs:20, code_validation.rs:18, email.rs:18, hashed_password.rs:18, password_policy.rs:20, username.rs:18; communities/src/domain/errors/community_error.rs:38, invalid_community_id.rs:14, invalid_community_name.rs:14, invalid_community_slug.rs:14 | Los errores de dominio implementan `std::error::Error`: un `?` los manda al canal técnico. | alta | Solo `Debug` + `Display`. | resuelto (eb30039) |
| ERR-1.3 | ERR-1 Errores de application | iam/src/application/errors/account_repository.rs:14, authenticate_account.rs:24, token_generator.rs:14; communities/src/application/errors/community_creation.rs:20, community_repository.rs:14 | Las hojas y los errores de puerto implementan `std::error::Error`. | media | Quitar el `impl`. | resuelto (eb30039) |
| ERR-1.4 | ERR-1 Fallo silenciado | contexts/iam/src/domain/model/account/account.rs:127-129 | `let _ = self.transition_status(…).map_err(…)` descarta el `Result` y responde `Ok`, aunque la transición falle. | media | Propagar con `?`, o `expect("invariante: Registered→Active siempre válida")`. | resuelto (ef94042): `expect("invariante: Registered→Active siempre es válida")`, con `confirmed_registration_leaves_the_account_active` |
| ERR-1.5 | ERR-1 Conversión implícita | contexts/iam/src/domain/model/account/account_factory.rs:39-43 | En `reconstitute`, un dato persistido corrupto se convierte en error de negocio (400) con `?`. | media | En el adaptador, conversión a `Fault` a mano y con contexto. | resuelto (0a3c49a): R19, adelantado de R20 a pedido del usuario porque era el último llamador de `SystemError` fuera de los mappers. `reconstitute` se borró (sin uso fuera de los tests); los helpers de test construyen la cuenta con `Account::new` y los value objects. El adaptador que traduzca datos persistidos a `Fault` llega con la persistencia real |
| ERR-1.6 | ERR-1 Sin árbol A | contexts/communities/src/application/ports/inbound/list_public_communities_use_case.rs:11 | El caso de uso no tiene fallos de negocio, pero declara un tipo de error (`SystemError`). | baja | `Result<Response, Fault>`. | resuelto (a603969): eliminado (D3) |
| ERR-1.7 | ERR-1 Firma del puerto | iam/…/ports/outbound/account_repository.rs:7-9; password_hasher.rs:2-3; communities/…/community_repository.rs:7-10 | `find_*` devuelve `Option` sin `Result`, `hash` devuelve `String` y `verify`, `bool`: un adaptador real no puede reportar una falla técnica y tiene que entrar en pánico. | media | `Result<Option<T>, Fault>`, `Result<String, Fault>`. | resuelto (692bc7c, ebd0819, 73f9c44): `CommunityRepository` en R7, `AccountRepository` en R8 y `PasswordHashingService` en R9 |
| ERR-2.1 | ERR-2 Contrato por caso de uso | iam/…/ports/inbound/authenticate_account_use_case.rs:10, identify_account_use_case.rs:7, register_account_use_case.rs:10, verify_account_use_case.rs:5; communities/…/create_community_use_case.rs:11 | Ningún caso de uso tiene su `{Verb}{Noun}Error` cerrado: todos devuelven el global `SystemError`. | alta | `RegisterAccountError { Validation, Conflict }` y demás, junto al handler. | resuelto (58958bc): R10b (`VerifyAccessTokenError`) y R11 (3ff0ae1, `RegisterAccountError { Validation, Conflict }`) y R13 (2d91c4a, `VerifyAccountError { NotFound, Validation, InvalidVerification }`) y R14 (7932c45, `AuthenticateAccountError { LoginFailed, CannotAuthenticate }`) y R15 (2c37b03, `IdentifyAccountError { NotFound }`) ya tienen su contrato; R17 (58958bc), `CreateCommunityError { Validation, Conflict, InvalidOwner }`. Desde S3, `hex_4_use_case_signatures_are_their_own_contract` lo verifica; con R17 queda en cero |
| ERR-2.2 | ERR-2 Sin envolver | shared/src/error.rs:18-20 | Los errores se envuelven (`Domain(Box<dyn ErrorLayout>)`), y el error de dominio llega tal cual al contrato (p. ej. `AccountError::EmailAlreadyExists.into()` en register_account.rs:43). | alta | Traducir a una hoja plana (`Conflict { resource }`). | resuelto (11f8c92): R11 (3f25daf, 3ff0ae1, 7e51295) en register: `AccountFactory::create` devuelve `AccountCreationError` y el handler lo traduce por variante a `Validation { field }`; los conflictos son `Conflict { resource }` y se borraron `AccountError::{Email,Username}AlreadyExists`. R13 (2d91c4a) en verify: `CodeValidationError` → `Validation { field: "code" }` y `AccountError::InvalidVerification` → `InvalidVerification`, por variante. R14 (7932c45) en authenticate: el `CannotAuthenticate` de dominio → la hoja `CannotAuthenticate`. R15 (2c37b03) en identify: la cuenta que falta es la hoja `NotFound { resource: "account" }`, no `AccountError::AccountNotFound`, que se borró (6b09abd). R17 (58958bc) en create: `InvalidCommunitySlug`/`InvalidCommunityName` → `Validation { field }`, el slug tomado → `Conflict { resource: "slug" }` e `InvalidOwnerId` → `InvalidOwner`, por variante, y se borró `CommunityCreationError`. Queda `SystemError` (R19). R19 (11f8c92) lo borró: ningún error se envuelve | 
| ERR-2.3 | ERR-2 Contrato incompleto | contexts/iam/src/application/errors/authenticate_account.rs:6 | `AuthenticateAccountError` no cubre `NotFound` (viene de `AccountError`) ni la falla de token: hay contratos parciales mezclados. | media | Un enum cerrado con todas las variantes que el caso produce. | resuelto (ad2f129, 65f455d, 7932c45): R2 quitó `NotFound` del caso; R10a hizo de la falla de token un `Fault` (árbol B), que no va en el contrato; R14 cerró el enum, `AuthenticateAccountError { LoginFailed, CannotAuthenticate }` en `commands/authenticate_account/error.rs` |
| ERR-2.4 | ERR-2 Nombre | contexts/communities/src/application/errors/community_creation.rs:6 | `CommunityCreationError` en vez de `CreateCommunityError`, y fuera del directorio del caso. | baja | `commands/create_community/error.rs`. | resuelto (58958bc): `CreateCommunityError` en `commands/create_community/error.rs`; `CommunityCreationError` y `application/errors/error_codes.rs` se borraron |
| ERR-2.6 | ERR-2 Tests por variante | contexts/iam/src/application/commands/register_account/register_account.rs:147 (patrón en todos los tests de handler) | Los tests afirman sobre `err.code()` (un string), no sobre la variante. | baja | `matches!(r, Err(Failure::Expected(…)))`. | resuelto (58958bc): R11 (3ff0ae1) pasó los tests de register a `matches!` sobre la variante, R13 (2d91c4a) los de verify, R14 (7932c45) los de authenticate, R15 (2c37b03) los de identify y R17 (58958bc) los de create |
| ERR-2.7 | ERR-2 Test por variante | contexts/iam/src/application/commands/register_account/register_account.rs:65 | Falta el test del handler para contraseña corta (`fails_when_password_is_too_short`). Falla de repositorio sin test de endpoint (solo el unit test del mapper). | media | Agregar ambos. | resuelto (3ff0ae1): `fails_when_password_is_too_short`; las fallas del repositorio ya estaban desde R8 (ebd0819, cc3d60c), y el 500 por endpoint lo fija `a_fault_returns_500_with_the_d7_body` |
| ERR-2.8 | ERR-2 Test por variante | contexts/iam/src/application/commands/authenticate_account/authenticate_account.rs:77 | La falla del generador de tokens no tiene test de handler (`fails_when_token_generation_fails`) ni de endpoint. | media | Doble que falle + test. | resuelto (65f455d, c65817c): R10a agrega `fails_with_unexpected_when_issue_fails`. La falla es árbol B, no una variante del contrato, así que el test por variante de ERR-2 no aplica; el 500 lo fijan `unexpected_fault_maps_to_internal_server_error` y `a_fault_verifying_the_token_returns_500`. El resto del manejador global sigue en ERR-4.7 |
| ERR-2.9 | ERR-2 Test por variante | contexts/iam/src/application/commands/verify_account/verify_account.rs:41 | Faltan el test de handler para código fuera de rango (`fails_when_code_is_out_of_range`) y los de handler y endpoint para la falla del repositorio (`fails_when_repository_fails`). | media | Agregar los tests. | resuelto (ebd0819, cc3d60c, 2d91c4a): R8 agregó los de handler para la falla del repositorio (árbol B); R13, `fails_when_code_is_out_of_range` y el 500 por endpoint (`a_fault_returns_500_with_the_d7_body` en `infrastructure/http/verify_account.rs`). El de endpoint del código ya existía (`verify_with_code_out_of_6_digit_range_returns_400`) |
| ERR-2.10 | ERR-2 Test por variante | contexts/communities/src/application/commands/create_community/create_community.rs:62 | El owner con formato inválido o nil no tiene test de handler (`fails_when_owner_id_is_invalid`), solo de endpoint. La falla de repositorio no tiene test de endpoint. | media | Agregar los tests. | resuelto (58958bc): R7 (692bc7c, ba15189) agrega los de handler para la falla del repositorio, que es árbol B; R17, `fails_when_owner_id_is_invalid` y `fails_when_owner_id_is_nil`, y el 500 por endpoint (`a_fault_returns_500_with_the_d7_body` en `infrastructure/http/create_community.rs`) |
| ERR-2.11 | ERR-2 Variante imposible | contexts/iam/src/application/commands/verify_account/handler.rs:57 | `confirm_registration` devuelve `AccountError`, así que el handler tiene que traducir `AccountNotFound`, que nunca produce, con una rama a `Fault` que ningún test alcanza. El `match` exhaustivo deja de decir algo. Hallado en la revisión de R13. | baja | Un error propio de `confirm_registration` con solo `InvalidVerification`. O, cuando R15 deje a `AccountError::AccountNotFound` sin productor, borrar la variante y la rama. | resuelto (6b09abd): R15 dejó `AccountError::AccountNotFound` sin productor; se borraron la variante, la rama de `VerifyAccountHandler`, su código de dominio y el brazo del mapper |
| ERR-3.1 | ERR-3 Respondió vs no pudo | contexts/iam/src/infrastructure/services/jwt_service/jwt_service.rs:85-92 | Un token inválido o vencido se devuelve como `Err(TokenError)`; es un veredicto, no un fallo. | media | `Ok(AccessTokenVerdict::Invalid)`. | resuelto (c9db5fa, 62d5456): `JwtAccessTokenAdapter` responde `Invalid` a todo error del token, y `Fault` a uno de clave o de validación |
| ERR-3.2 | ERR-3 Errores para transporte | iam/src/application/errors/account_repository.rs:16-27, token_generator.rs:16-27; communities/src/application/errors/community_repository.rs:16-27 | Los errores de puerto traen un `code()` y un `message()` pensados para el cliente HTTP. | media | Sin texto de usuario; el borde decide. | resuelto (692bc7c, ebd0819, 65f455d): R10a borró `TokenGeneratorError` |
| ERR-3.3 | ERR-3 Falla técnica = árbol B | iam/src/application/errors/account_repository.rs:6, token_generator.rs:6; communities/src/application/errors/community_repository.rs:6 | Fallas técnicas no declarables (persistencia, codificación JWT) modeladas como errores tipados del contrato. | media | `Fault`, propagado con `?`. | resuelto (692bc7c, ebd0819, 65f455d): R10a borró `TokenGeneratorError` |
| ERR-4.2 | ERR-4 Logging en el núcleo | contexts/iam/src/application/commands/register_account/register_account.rs:55 | `dbg!(&account)` escribe en stderr la cuenta completa (email y hash de la contraseña). | alta | Eliminarlo. | resuelto (57e814e) |
| ERR-4.3 | ERR-4 Secretos | app/src/config/jwt.rs:3; app/src/config/app.rs:3 | `#[derive(Debug)]` sobre una configuración que contiene el secreto. | media | Sin `Debug`. | resuelto (8195600) |
| ERR-4.4 | ERR-4 Códigos en el borde | iam/src/domain/errors/error_codes.rs:1-12; iam/src/application/errors/error_codes.rs:1-2; communities/src/domain/errors/error_codes.rs:1-13; communities/src/domain/errors/invalid_owner_id.rs:33; communities/src/application/errors/error_codes.rs:1-2 | Los códigos estables y los mensajes de cliente viven en el núcleo. | media | `{ctx}-api/src/codes.rs`. | resuelto (11f8c92): R11 (7655856, 3ff0ae1) creó `infrastructure/http/codes.rs` con los códigos y mensajes de register y el de D7, byte a byte; R13 (2d91c4a) sumó los tres de verify (`IAM_ACCOUNT_NOT_FOUND`, `IAM_INVALID_CODE_VALIDATION`, `IAM_ACCOUNT_INVALID_VERIFICATION`); R14 (7932c45), los dos de authenticate (`IAM_LOGIN_FAILED`, `IAM_CANNOT_AUTHENTICATE`), y borró `application/errors/error_codes.rs`; R15 (2c37b03, 6b09abd) usa en el `fold` de identify los de verify, y borró `IAM_ACCOUNT_NOT_FOUND` de `domain/errors/error_codes.rs`; los demás de ese archivo siguen alimentando el mapper (R17, R19); R16 (3646f25) sumó al dominio de communities `IAM_INVALID_ACCOUNT_ID(_FORMAT)` (D2 a) y los mensajes de `InvalidOwnerId::message()`, puente del mapper legacy, y los dos códigos en `communities/src/infrastructure/http/codes.rs` (f6bc1fd); R17 (58958bc, 838b923) pasó a `codes.rs` los códigos y mensajes de create (`COMMUNITIES_INVALID_COMMUNITY_{SLUG,NAME}`, `COMMUNITIES_SLUG_ALREADY_EXISTS` y los mensajes `IAM_`), y borró `communities/src/application/errors/error_codes.rs`, las constantes `IAM_` y las de nombre y slug del dominio y los `impl ErrorLayout` de `InvalidOwnerId`, `InvalidCommunityName` e `InvalidCommunitySlug`. Quedan en `communities/src/domain/errors/error_codes.rs` el de `InvalidCommunityId` y los 6 de `CommunityError` (HEX-7.12), y los de `iam/src/domain/errors/error_codes.rs` (R19). R19 (11f8c92) borró los dos `error_codes.rs` de domain (los de application ya no estaban desde R14 y R17) y todos los `impl ErrorLayout`: los códigos y mensajes de cliente viven solo en `infrastructure/http/codes.rs`. La forma `{ctx}-api/src/codes.rs` llega con la separación de crates (R27, R28) |
| ERR-4.5 | ERR-4 `fold` exhaustivo | contexts/iam/src/infrastructure/http/errors/error_mapper.rs:60-77; contexts/communities/src/infrastructure/http/errors/error_mapper.rs:58-66 | El status HTTP se decide comparando strings, con un comodín `_ => 500` (líneas 76 y 65). | media | `match` exhaustivo sobre el enum del contrato. | resuelto (11f8c92): R11 (3ff0ae1, d0dd8cb) `POST /auth/register` ya no usa el mapper: `match` exhaustivo en `infrastructure/http/register_account.rs`, con un test de endpoint por brazo; un `field` o `resource` desconocido es árbol B. R13 (2d91c4a) hizo lo mismo con `POST /auth/verify` en `infrastructure/http/verify_account.rs`, y R14 (7932c45) con `POST /auth/authenticate` en `infrastructure/http/authenticate_account.rs` (se borró el brazo 401 del mapper, sin productor). R15 (2c37b03) con `POST /auth/identify` en `infrastructure/http/identify_account.rs`; con él, ningún endpoint de iam usa el mapper. R17 (58958bc) con `POST /communities/create` en `infrastructure/http/create_community.rs`; con él, ningún endpoint usa un mapper. R19 (11f8c92) los borró, con sus comodines |
| ERR-4.9 | ERR-4 Secretos | app/src/config/jwt.rs:26 | `from_vars` aceptaba un `JWT_SECRET` vacío: copiando `.env.example` (`JWT_SECRET=`), el servidor arrancaba firmando HS256 con clave vacía y cualquiera podía falsificar un token. Hallado en P1. | alta | `ConfigError::Invalid("JWT_SECRET")` por debajo de 32 bytes. | resuelto (e1b613b, PR #40) |
| ERR-4.10 | ERR-4 Secretos | contexts/iam/src/infrastructure/http/requests/sign_in.rs:4; sign_up.rs:4 | `SignInRequest` y `SignUpRequest` derivan `Debug` con `password` en claro: un `{:?}` futuro (log, `TraceLayer` con cuerpo) filtra la contraseña. Igual que ERR-4.3. Hallado en P1. | media | Sin `Debug`, o a mano con `<redacted>`. Se corrige en el lugar: las entradas `hex_6` de esos archivos prohíben moverlos, no editarlos. | resuelto (9819983, e370462): `sign_up.rs` en R11 y `sign_in.rs` en R14, los dos con `Debug` a mano y `<redacted>`. R12 revisó los demás DTOs de entrada y los `Command`: ningún otro lleva la contraseña en `Debug` (el código de verificación, en ERR-4.14) |
| ERR-4.11 | ERR-4 Secretos | contexts/iam/src/application/queries/verify_access_token/query.rs:1; commands/authenticate_account/response.rs:1 | `VerifyAccessTokenQuery` y `AuthenticateAccountResponse` derivan `Debug` con el token. Hallado en P1. | baja | `Debug` a mano con el token redactado. | resuelto (542fa7b) |
| ERR-4.14 | ERR-4 Secretos | contexts/iam/src/infrastructure/http/requests/verify.rs:4 | `VerifyRequest` deriva `Debug` con `validation_code`: un `{:?}` futuro (log, `TraceLayer` con cuerpo) filtra el código que verifica la cuenta, que además no tiene límite de intentos (§4). Igual que ERR-4.10, pero con el código y no con la contraseña. Hallado en R12, al buscar otros DTOs de entrada como `SignUpRequest`. | baja | `Debug` a mano con `<redacted>`, en el lugar (como `sign_up.rs`). | resuelto (f0089c4): incluido en R13 a pedido del usuario, con `debug_does_not_reveal_the_validation_code` |

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
| TDD-2.1 | TDD-2 Batería de contrato | iam/…/ports/outbound/account_repository.rs:6, password_hasher.rs:1, token_generator.rs:3; communities/…/community_repository.rs:6; shared/…/token_verifier.rs:6 | Ningún puerto tiene batería de contrato; dobles y adaptadores no comparten comportamiento verificado. | alta | `testing/contracts/{puerto}.rs` + macro, contra el doble y el adaptador. | resuelto (9eacddc, de6c308, c04f69d, ce6d64b, 3c1a6bb): R7 (692bc7c, 9eacddc, 6b808a2) creó la de `CommunityRepository` y R8 (de6c308, 9a90d8a) la de `AccountRepository`; R9 (c04f69d, 17ba68e) la de `PasswordHashingService`; R10a (ce6d64b, c9db5fa) la de `AccessTokenService`, que reemplaza a `TokenGeneratorPort` y a `TokenVerifier`; R10b (3c1a6bb) borró `TokenVerifier` |
| TDD-2.2 | TDD-2 Nombre de dobles | iam/…/account_repository.rs:23, password_hasher.rs:10, token_generator.rs:14; communities/…/community_repository.rs:24 | Dobles sin el sufijo `Adapter` (`FakeAccountRepository`…). | baja | `{Prefijo}{Concepto}Adapter`. | resuelto (9eacddc, de6c308, c04f69d, ce6d64b): el último, `FakeAccessTokenAdapter`, en R10a |
| TDD-2.3 | TDD-2 Semántica del prefijo | iam/…/password_hasher.rs:10; token_generator.rs:14; account_repository.rs:23; communities/…/community_repository.rs:24 | `Fake` sin estado. `FakePasswordHasher` y `FakeTokenGenerator` devuelven respuestas fijas. Los repositorios se configuran por flags y `save` no guarda nada (account_repository.rs:98-106). | media | `FixedPasswordHashingAdapter`, `FixedAccessTokenAdapter`, `StubAccountAdapter`, o hacerlos `Fake` con estado real. | resuelto (9eacddc, de6c308, c04f69d, ce6d64b): R7 y R8 resolvieron los repositorios (`FakeCommunityAdapter` y `FakeAccountAdapter`, con estado); R9 (c04f69d) cambió `FakePasswordHasher` por `FixedPasswordHashingAdapter`; R10a (ce6d64b, 1b08cea) cambió `FakeTokenGenerator` por `FakeAccessTokenAdapter`, con estado |
| TDD-2.4 | TDD-2 Doble fiel | contexts/iam/src/application/ports/outbound/password_hasher.rs:19-21 | `verify` compara texto plano contra el hash, y el repositorio devuelve una cuenta cuyo email no es el consultado (account_repository.rs:95). El test de éxito tiene que mandar el hash como contraseña (authenticate_account.rs:163). | media | Doble coherente + batería de contrato. | resuelto (de6c308, cc3d60c, c04f69d): R8 resolvió el repositorio; R9, `verify` (`FixedPasswordHashingAdapter` solo acepta la contraseña con la que generó el hash) |
| TDD-2.5 | TDD-2 Doble completo | contexts/communities/src/application/ports/outbound/community_repository.rs:80 | El doble tiene `todo!()` en `get_public_list`, helpers muertos copiados de iam (líneas 59 y 68) y el campo `_activated` sin uso (línea 26). | baja | Implementar el método o quitarlo del puerto. | resuelto (a603969): `get_public_list`, los helpers muertos y `_activated` quitados |
| TDD-2.6 | TDD-2 Doble completo | contexts/communities/src/application/commands/create_community/create_community.rs:169 | El `StubCommunityAdapter` local (ba15189) falla siempre en `save` y no puede pasar la batería de contrato. Fricción ya anotada en §6. Hallado en P1. | baja | `FakeCommunityAdapter::failing_on(CommunityOperation::Save)`, como `FakeAccountAdapter`, y borrar el Stub. | resuelto (c7cc244) |
| TDD-2.8 | TDD-2 Prueba por llamada | contexts/communities/src/application/commands/create_community/create_community.rs:146 | `fails_with_unexpected_when_repository_fails` usa `FakeCommunityAdapter::failing()`, así que falla en la primera llamada, `find_by_slug`, pero el nombre no dice cuál. El de `save` sí la nombra (`fails_with_unexpected_when_save_fails`), y §5.1 pide `fails_with_unexpected_when_<operación>_fails` por llamada. Hallado en la revisión de R16. | baja | Renombrar a `fails_with_unexpected_when_find_by_slug_fails`, con `failing_on(CommunityOperation::FindBySlug)`, como iam desde R12b. | resuelto (60caf6f) |

### RS (rust-stack)

| ID | Regla | archivo:línea | Violación | Sev. | Corrección | Estado |
|---|---|---|---|---|---|---|
| RS-1.3 | RS-1 Herencia | Cargo.toml:1-11 | Sin `[workspace.package]` ni `[workspace.lints]`; versiones repetidas por crate (`uuid` en contexts/iam/Cargo.toml:7 y contexts/communities/Cargo.toml:7, `serde` en cuatro crates). | baja | Herencia del workspace. | resuelto (991e672) |
| RS-1.4 | RS-1 Dependencias | app/Cargo.toml:18 | `anyhow` está declarado y no se usa: cargo avisa `unused dependency`. Hallado en R2, fuera de la auditoría. | baja | Quitarlo, junto con RS-1.3 (R6). | resuelto (991e672) |
| RS-2.1 | RS-2 Árbol B = `Fault` | shared/src/error.rs:17 | No existen `Fault` ni `Failure<E>`; el árbol B viaja como `SystemError::Infrastructure`. | alta | `kernel::Fault` y `kernel::Failure`. | resuelto (11f8c92): R5 agregó `Fault` y `Failure<E>`; R19 borró `SystemError` con su puente `Unexpected` |
| RS-2.3 | RS-2 `unwrap` prohibido | iam/…/authenticate_account.rs:55; verify_account.rs:30; identify_account.rs:44; account_status.rs:79 (dos veces), 81; app/src/config/jwt.rs:15; app/src/main.rs:12,14 | `unwrap` en producción. En `account_status.rs` panica ante un dato persistido; en `jwt.rs`, ante un `JWT_EXPIRATION_TIME` no numérico. | media | `let Some(..) else`, `?` o `ConfigError`. | resuelto (0415e3e, 4355c9f, ebcd593, 9315004) |
| RS-2.4 | RS-2 `expect("invariante: …")` | iam/…/argon2_password_hasher.rs:26,32; iam/…/in_memory/account_repository.rs:28,36,44; communities/…/in_memory/community_repository.rs:31,39,47 | Los mensajes no empiezan con "invariante:". Argon2 usa `expect` para una falla técnica (línea 26) y para un hash persistido corrupto (línea 32), que deberían ser `Fault`. | media | `Fault` en Argon2; "invariante: mutex sin envenenar" en los mutex. | resuelto (6b808a2, 9a90d8a, 17ba68e, dddc146): los mutex en R7 y R8; Argon2 en R9, con sus errores como `Fault` |
| RS-2.5 | RS-2 Lints | Cargo.toml:1-11 | Sin `[workspace.lints]` con `unwrap_used = "deny"` ni `unsafe_code`. | media | Agregarlos. | resuelto (9315004) |
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

## 4. Dudas resueltas

- ~~**Enumeración de usuarios (ERR-4).** `authenticate` responde 404 a un usuario inexistente y 401 a una contraseña incorrecta, así que se puede saber qué nombres de usuario existen.~~ Confirmada por el usuario y resuelta en R2 (ad2f129, 4149660): 401 `IAM_LOGIN_FAILED` y verificación contra un hash señuelo (D6).
- ~~**Orden en `RegisterAccount`.** Hashea con Argon2 (register_account.rs:49) antes de validar la política de contraseña dentro de la factory. Es orquestación cuestionable (costo y contraseña en claro pasada al dominio), no claramente un `if` de negocio en el handler.~~ Resuelto en R12 (06a5676) como consecuencia del orden pedido (contraseña, formato, conflicto y después el hash). Lo fija `rejects_the_password_before_hashing_it` (e4b270b).

## 5. Plan de rebanadas

### 5.2 Errores y estructura: recomendación

**Migrar los errores antes de separar los crates, en la disposición legacy.** La hipótesis "corregir en el lugar antes de mover" se sostiene, con estos ajustes:

1. **La excepción del HTTP es más chica de lo propuesto.** ERR-4.4 y ERR-4.5 se pueden hacer en el lugar. Cada caso de uso migrado trae su `fold` exhaustivo, en `infrastructure/http/<caso>.rs`, y sus códigos en `infrastructure/http/codes.rs`. Esos archivos no violan nada y se mudan intactos a `{ctx}-api/src`. Lo que sí espera a la separación es lo que necesita `edge/http`: ERR-4.6, ERR-4.7 (la parte de `unexpected()` en los handlers), ERR-4.8, HEX-1.6, HEX-8.6 y HEX-8.8. La razón es la del contexto: un contexto legacy no puede depender de `edge-http` sin agregar una entrada en `contexts/iam/Cargo.toml` o sin bloquear la de `contexts/communities/Cargo.toml`.
2. **Lo estructural por definición.** HEX-1.7, HEX-1.9, HEX-2.1, HEX-2.5 y RS-1.1 se resuelven solo separando crates. Las entradas `hex_2_*` de los `Cargo.toml` de contexto no desaparecen mientras el HTTP comparta manifiesto con el dominio.
3. **Lo que la separación verifica por primera vez.** La separación es la primera verificación real de HEX-4: el test de contratos solo empieza a actuar ahí. Por eso conviene que R27 y R28 muevan código que ya se revisó a mano (§5.1).
4. **Antes de separar.** Hay que sacar del dominio `uuid`, `rand` y `chrono` (R20–R22). Además, los puertos ya tienen que ser async con un `block_on` de std (R23–R24). Si no, los manifiestos nuevos de domain y application nacen con violaciones de `hex_2_core` que la línea base no admite.

Si se hiciera al revés (separar primero), habría que mover las 17 entradas ERR-1 y las 6 HEX-4 a rutas nuevas, todavía violando, y el trinquete lo prohíbe. Las alternativas serían arreglarlas en el PR de la mudanza, que deja de ser chico, o escribir dos veces el mapeo de `SystemError`.

### 5.3 Secuencia: rebanadas hechas

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

*(Revalidado en P1 contra `bab68c5`.)* **El `?` y el puente `SystemError::Unexpected`.** Cada llamada a un puerto ya propaga su `Fault` con `.map_err(SystemError::Unexpected)?` (fase 2) y tiene su test `fails_with_unexpected_when_<operación>_fails`. La secuencia de R10a (el commit de la firma descarta el `Fault` y el `?` llega después, con su test en rojo) **no aplica en la fase 3**: no aparece ninguna propagación nueva. En el commit que cambia el contrato a `Failure<{Verb}{Noun}Error>`, `.map_err(SystemError::Unexpected)?` pasa a `?` (`From<Fault> for Failure<E>`), y esos tests pasan de `Err(SystemError::Unexpected(_))` a `Err(Failure::Unexpected(_))`. Es el mismo commit, porque sin él no compilan. Ningún `Fault` se descarta en ningún momento. ~~La secuencia de R10a vuelve a regir si una rebanada agrega una llamada nueva a un puerto (R20: `IdGenerator`, `VerificationCodeService`).~~ *(S3)* Si una rebanada agrega una llamada nueva a un puerto falible, rige la regla de §5.1 ("Firma que pasa a ser falible"): `.expect("pendiente: …")` en el commit de la firma y el `?` después, con su test. En R20, `IdGenerator::next_id` devuelve `String` y no falla; la regla solo aplica a `VerificationCodeService` si su firma es falible.

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
- **Resultado (hecho, PR #42):** commits `1482280`, `7655856`, `3f25daf`, `3ff0ae1`, `9819983`, `7e51295` y `d0dd8cb`. Línea base −1 (33 → 32). 324 tests, fmt y clippy en verde localmente; `app/tests/` sin cambios.
  - Como pedía el plan: `RegisterAccountHandler` en `handler.rs` (rojo: la entrada borrada de la línea base); `unexpected()` en `infrastructure/http/unexpected.rs`, con `codes.rs`, usado por el `fold`, el mapper y el middleware de `app`; `Failure<RegisterAccountError { Validation, Conflict }>` con `?` en las 4 llamadas a puertos; `fold` exhaustivo en `infrastructure/http/register_account.rs`; `Debug` de `SignUpRequest` con `<redacted>`.
  - **`ARCH_REQUIRE_MATERIAL=1`: no se activa.** La fricción de §6 (P1) sigue sin resolver en el harness (`craft-backend` 1.0.0-rc.2), y la activación pondría en rojo `err_1_application_errors_…` y `hex_4_event_subscribers_…`.
  - *(S3)* **Ya se puede activar.** `v1.0.0-rc.3` trae `material-exempt.txt` (P1, §6). Con las tres exenciones que escribe el usuario (§5.10, S3: `err_1_application_errors_…`, `hex_4_event_subscribers_…` y `hex_8_capability_adapters_…`, este último hasta R20), `ARCH_REQUIRE_MATERIAL=1` pasa en verde. Activada en el PR de S3 (commit `53d6bd3`, `ci: require architecture material`), en el paso `cargo test --workspace --locked` de `.github/workflows/ci.yml`.
  - **Agregado:** `AccountFactory::create` devuelve `AccountCreationError` (una variante por regla), para que el handler traduzca por variante y no por `code()` (ERR-2.2). Con sus tests de dominio, incluido el orden contraseña → username → email.
  - **Agregado:** se borran `AccountError::{Email,Username}AlreadyExists`, sus constantes de dominio, el brazo `CONFLICT` del mapper y su test (`already_existing_account_maps_to_conflict`): quedaron sin productor.
  - **Desvío:** el mapper conserva `unexpected_system_error_maps_to_the_d7_body` para su brazo legacy (lo siguen usando verify, authenticate e identify hasta R19). El test original se mudó con `unexpected()`, como pedía el plan.
  - **Desvío respecto del template:** un `field` o `resource` que el `fold` no conoce responde el 500 de D7 (árbol B: handler y borde desincronizados es un bug), en lugar de un código de cliente genérico inventado. Lo fijan `an_unmapped_validation_field_returns_500` y `…_conflict_resource_returns_500`.
  - **Rojo:** en el commit del contrato, primero de compilación (el contrato no existía); después cada test nuevo falló contra una mutación de producción en el árbol de trabajo, anotada en el mensaje del commit. Los tests de endpoint por brazo y los de la factory fijan comportamiento existente: su rojo también se vio por mutación (d0dd8cb).
  - **Revisor:** sin bloqueantes. Aplicados en la rebanada: tests de endpoint por brazo, tests de dominio de la factory y el borrado de las variantes muertas. Anotados para después: ERR-3.4, ERR-4.13 y TDD-2.7 (nuevos), HEX-7.1 (R12), el orden "hashear antes de la política" (§4), y `unexpected()` en `edge/http` (R26, R30).

**R12. Unicidad sobre el valor normalizado.** *Hecha (ver §5.8).*
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
- **Resultado (hecho, PR #43):** commits `3146a43`, `77e0ca4`, `06a5676`, `d1417a4`, `e4b270b` y `46020e1`. Línea base sin cambios (32). 333 tests, fmt y clippy en verde localmente.
  - Commit `3146a43` (`test(app)`): `register_with_invalid_username_and_taken_email_returns_409`, marcado `CURRENT BEHAVIOR`, en verde contra el código de `main`, como pedía el plan.
  - Commit `77e0ca4` (`refactor`): `AccountFactory` se parte en `draft(username, email, raw_password) -> Result<AccountDraft, AccountCreationError>` (política, username, email, en ese orden) y `create(draft, hashed_password) -> Result<Account, HashedPasswordError>`. `AccountDraft` lleva el `Username` y el `Email` normalizados y es la única entrada a `create`. `AccountCreationError` pierde la variante `HashedPassword`. Sin cambio de comportamiento: el handler conserva el orden de antes. Los tests de dominio conservan sus asertos sobre los dos pasos, más `create_keeps_the_drafted_username_and_email`.
  - Commit `06a5676` (`fix`): el handler pide el `draft` primero y busca los conflictos con sus valores. El orden queda contraseña → formato → conflicto (email, después username) → hash. Rojo contra `77e0ca4`, antes de tocar producción:
    - handler: `fails_with_conflict_when_email_differs_only_in_case` y `…_when_username_differs_only_in_surrounding_spaces` (`Ok(..)`), `fails_with_validation_on_invalid_username_before_checking_taken_email` y `…_on_short_password_…` (`Err(Expected(Conflict { resource: "email" }))`);
    - HTTP: `register_rejects_same_email_with_different_casing_with_409` y `register_rejects_same_username_with_surrounding_spaces_with_409` (`left: 201`), y `register_rejects_invalid_username_before_checking_taken_email` (`left: 409`).
    Con el cambio, los tres tests reemplazados fallaron y se borraron (§5.5). `register_validates_password_before_username_and_email` y `register_checks_email_before_username_when_both_are_taken` no cambian. Ningún otro test de `app/tests/` cambió.
  - Commit `d1417a4`: `finds_by_the_normalized_email_and_username` en `account_repository_contract!`, contra el Fake y el adaptador en memoria. Fija comportamiento existente: su rojo se vio por mutación del Fake.
  - Commit `e4b270b`, **desvío respecto de "fuera de alcance"**: el orden pedido (contraseña antes del conflicto) y el que ya había (conflicto antes del hash) dejan el hash después de la política, sin elegirlo aparte (§4, "Orden en `RegisterAccount`"). Como ningún test lo distinguía, lo fija `rejects_the_password_before_hashing_it`. Rojo por mutación (el hash antes del `draft`): `Err(Unexpected(..))`. **El revisor lo marca (TDD-1):** el test llegó en un commit posterior a `06a5676`, así que el historial muestra la producción antes que el test; debió ir en `06a5676`, con su rojo contra `77e0ca4`. No se reescribe la rama: queda dicho aquí y en el PR.
  - **Cambio de contrato sin cobertura previa:** con el orden nuevo, una contraseña corta o un email inválido junto a un valor ya tomado pasan de 409 a 400. Los fijan los tests de handler de `06a5676`, no `app/tests/` (§5.5).
  - **Revisión de DTOs de entrada (pedida en el encargo):** `SignInRequest` (ERR-4.10, R14) sigue siendo el único con la contraseña en `Debug`. Los `Command` no derivan `Debug`. Hallazgo nuevo: `VerifyRequest` deriva `Debug` con `validation_code` (ERR-4.14). No se corrige aquí.
  - **Revisor:** sin bloqueantes. Dos hallazgos de la rebanada: el orden de `e4b270b` (arriba, anotado) y el comentario de `AccountDraft`, que prometía que ninguna cuenta se crea sin validar aunque `Account::new` sigue público (HEX-7.5) y el hash no está atado a la contraseña validada (TDD-2.7). Se acotó en `46020e1`. Observó que el caso nuevo de la batería suma poco, porque el adaptador recibe valores ya normalizados y lo que prueba en realidad son `Email::new` y `Username::new`. Se deja porque lo pedía el plan. Fuera de la rebanada no encontró nada que no esté en el inventario (HEX-7.10 y HEX-7.11 en R12b, ERR-4.14).

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
- **Resultado (hecho, PR #44):** commits `2d16725`, `f0089c4`, `2d91c4a` y `ef94042`. Línea base −1 (32 → 31 entradas, 50 → 49 violaciones). 345 tests, fmt y clippy en verde localmente; `app/tests/` sin cambios.
  - Commit `2d16725` (`refactor`): `VerifyAccount` pasa a `VerifyAccountHandler` en `commands/verify_account/handler.rs`, sin `module_inception`; el módulo reexporta el comando y el handler, como register. Rojo: con su entrada borrada de la línea base, `hex_4_use_case_implementations_are_handlers` reportó `` `VerifyAccount` implementa un UseCase y debe llamarse *Handler ``.
  - Commit `f0089c4` (`fix`, ERR-4.14, incluido a pedido del usuario): `Debug` a mano en `VerifyRequest`, con `validation_code` como `<redacted>`, igual que `SignUpRequest` en R11. Rojo contra el `derive`: `debug_does_not_reveal_the_validation_code` falló con `VerifyRequest { email: "ana@example.com", validation_code: 481516 }`.
  - Commit `2d91c4a` (`refactor`): `Result<(), Failure<VerifyAccountError>>`, con `VerifyAccountError { NotFound, Validation, InvalidVerification }` en `error.rs` y dos hojas nuevas en `application/errors/`: `NotFound { resource }` e `InvalidVerification`. El handler traduce por variante: email desconocido → `NotFound { resource: "account" }`; `CodeValidationError::Invalid` → `Validation { field: "code" }`; `AccountError::InvalidVerification` (código incorrecto o cuenta ya verificada, D1) → `InvalidVerification`. `AccountError::AccountNotFound`, que `confirm_registration` no produce, es árbol B. Las dos llamadas al repositorio pasan de `.map_err(SystemError::Unexpected)?` a `?`. `POST /auth/verify` deja el mapper: `fold` exhaustivo en `infrastructure/http/verify_account.rs`, con los códigos y mensajes de hoy en `codes.rs`, y el 200 con cuerpo `null`. Un `resource` o `field` desconocido da el 500 de D7, como en register.
    - Tests de handler sobre la variante (ERR-2.6). Nuevos: `fails_when_code_is_out_of_range` (ERR-2.9), `fails_with_not_found_before_checking_the_code` (fija el orden de hoy: primero la cuenta, después el código), `fails_with_invalid_verification_when_account_is_already_verified` (D1), y el test de éxito afirma que la cuenta guardada queda `active`. Un test de endpoint por brazo con `StubVerifyAccountUseCase`, incluidos `a_fault_returns_500_with_the_d7_body`, `a_verified_account_returns_200_with_a_null_body` y los dos `an_unmapped_…_returns_500`.
    - Rojo: primero de compilación (el contrato no existía). Después, cada test nuevo falló contra una mutación de producción en el árbol de trabajo, anotada en el commit: el código como `InvalidVerification`, el `Fault` como 400 (`left: 400`), el éxito como 204 (`left: 204`), sin `save` (`left: "registered"`) y el código antes que la cuenta (`Validation { field: "code" }`). Es la fricción de R11 (§6).
    - `account_lifecycle.rs`, adaptación mecánica: `verify_with` devuelve el contrato nuevo, y los dos `code() == "IAM_ACCOUNT_INVALID_VERIFICATION"` pasan a `matches!` sobre `VerifyAccountError::InvalidVerification` (`verifying_twice_fails_with_invalid_verification` y `wrong_code_keeps_account_registered_and_unable_to_sign_in`). Ya en `2d16725`: el import y el nombre del handler. Ningún escenario ni aserto cambia de significado.
  - Commit `ef94042` (`fix`, ERR-1.4): `confirm_registration` deja de descartar el `Result` de la transición: `expect("invariante: Registered→Active siempre es válida")`, porque `can_transition_to` siempre la admite. Sin cambio observable. Antes, el test de dominio `confirmed_registration_leaves_the_account_active`, que fija comportamiento existente: rojo por mutación (sin la transición), `left: Registered { code_validation: CodeValidation(123123) }`.
  - **Sin `response.rs`:** el caso de uso no devuelve datos, así que responde `()`. Un `VerifyAccountResponse` vacío no agrega nada al contrato y cambiaría el cuerpo `null` si se serializara.
  - **Revisor:** sin bloqueantes. Confirmó los cinco puntos pedidos: la rama `AccountNotFound` → `Fault` (ERR-1: una respuesta imposible del dominio es árbol B, y el `Fault` da el 500 de D7 en vez de cortar la conexión, porque todavía no hay `CatchPanicLayer`); las hojas planas; `()` sin `response.rs`; ningún `Fault` tragado, con un test de falla por llamada; y lifecycle solo mecánico. Un hallazgo de la rebanada, que pide no cambiar aquí: esa rama no tiene test porque `confirm_registration` devuelve el `AccountError` compartido (ERR-2.11, nuevo, R15). Señala que `confirmed_registration_leaves_the_account_active` también pasa contra el commit padre, porque `let _ =` igual hacía la transición: es caracterización, y su rojo solo se ve por mutación (así lo dice el commit). Lo demás ya está inventariado: HEX-7.10 (R12b), ERR-4.4 y ERR-4.5 (R14, R15, R19), ERR-4.6 (R30), HEX-7.3, HEX-6.1 (`VerifyRequest`), VER-1.4 y VER-1.5.

**R13b. `VerifyAccountResponse` (nueva en S3).**
- **IDs:** HEX-4.5.
- **Precondición:** D11 decidida (§5.7): opción a), así que R13b existe.
- **Cambios (D11 a):** `VerifyAccountResponse` vacío en `commands/verify_account/response.rs`, y `execute -> Result<VerifyAccountResponse, Failure<VerifyAccountError>>`. El `fold` de `infrastructure/http/verify_account.rs` no lo serializa: sigue respondiendo 200 con cuerpo `null`.
- **Línea base: −1** (`hex_4_use_case_signatures_are_their_own_contract` de `verify_account_use_case.rs`, n = 2).
- **Rojo:** el trinquete. `a_verified_account_returns_200_with_a_null_body` fija que el cuerpo no cambia.
- **Caracterización:** lifecycle se adapta (`verify_with` devuelve la respuesta nueva).
- **Contrato HTTP:** sin cambios.
- **Tamaño:** XS.
- **Resultado (hecho, PR #46):** commit `d37fe10`. Línea base −1 (42 → 41 entradas, 72 → 70 violaciones). 388 tests, fmt y clippy en verde localmente, con `ARCH_REQUIRE_MATERIAL=1`; `app/tests/` sin cambios.
  - `VerifyAccountResponse` es un unit struct (`Debug`, `PartialEq`, `Eq`) en `commands/verify_account/response.rs`, reexportado por el módulo como register. El handler responde `Ok(VerifyAccountResponse)`.
  - El `fold` hace `Ok(VerifyAccountResponse) => (StatusCode::OK, Json(()))`: no lo serializa, y el cuerpo sigue `null`. `a_verified_account_returns_200_with_a_null_body` pasa sin cambios en su aserto; solo el `Ok(())` del stub pasa a `Ok(VerifyAccountResponse)`.
  - Rojo: con su entrada borrada de la línea base, `hex_4_use_case_signatures_are_their_own_contract` reportó las 2 violaciones de `verify_account_use_case.rs` (`` responde `un tipo que no es una ruta` `` y `` `VerifyAccountResponse` no está definido en commands/verify_account/ ``).
  - `account_lifecycle.rs`, adaptación mecánica: el import de `VerifyAccountResponse` y el tipo de retorno de `verify_with`. Ningún escenario ni aserto cambia.

**R14. `AuthenticateAccount`.**
- **IDs:** HEX-4.1, HEX-4.4, ERR-2.1, ERR-2.3, ERR-2.6, HEX-7.2, ERR-4.10 (`sign_in.rs`) y ERR-4.11.
- **Cambios:**
  - `account.authenticate() -> Result<(), CannotAuthenticate>` en el agregado;
  - `AuthenticateAccountError { LoginFailed, CannotAuthenticate }`: ya existe en `application/errors/authenticate_account.rs` desde R2/R10a. Se mueve a `commands/authenticate_account/error.rs`, sin `ErrorLayout`;
  - `Debug` a mano en `SignInRequest`, `AuthenticateAccountResponse` y `VerifyAccessTokenQuery`, sin el secreto.
- **Línea base: −2** *(S3; antes −1)*: `hex_4_handlers` de `authenticate_account.rs` y `hex_4_use_case_signatures_are_their_own_contract` de `authenticate_account_use_case.rs` (`SystemError`, ERR-2.1).
- **Rojo:**
  - dominio: `registered_account_cannot_authenticate`;
  - un test de endpoint 500 con `StubAuthenticateAccountUseCase`;
  - `debug_does_not_reveal_the_password` y `…_the_token`.
- **Sin rojo nuevo:** `fails_when_token_generation_fails` ya existe como `fails_with_unexpected_when_issue_fails` (R10a), con `failing_on(AccessTokenOperation::Issue)`. Tampoco los de `find_by_username`, `verify` y el `verify` del señuelo (R8, R9).
- **Contrato HTTP:** sin cambios. La búsqueda por `Username` (HEX-7.11) va en R12b.
- **Tamaño:** M.
- **Resultado (hecho, PR #47):** commits `09fb976`, `e370462`, `542fa7b`, `c6c3281`, `7932c45` y `8adb31f`. Línea base −2 (41 → 39 entradas, 70 → 68 violaciones). 398 tests, fmt y clippy en verde localmente, con `ARCH_REQUIRE_MATERIAL=1`; `app/tests/` sin cambios.
  - Commit `09fb976` (`refactor`): `AuthenticateAccount` pasa a `AuthenticateAccountHandler` en `commands/authenticate_account/handler.rs`, sin `module_inception`; el módulo reexporta comando, respuesta y handler. Rojo: con su entrada borrada, `hex_4_use_case_implementations_are_handlers` reportó `` `AuthenticateAccount` implementa un UseCase y debe llamarse *Handler ``.
  - Commit `e370462` (`fix`, ERR-4.10): `Debug` a mano en `SignInRequest`, con la contraseña como `<redacted>`, como `SignUpRequest` en R11. Rojo contra el `derive`: `debug_does_not_reveal_the_password` falló con `SignInRequest { username: "ana", password: "s3cret-password" }`.
  - Commit `542fa7b` (`fix`, ERR-4.11): lo mismo con el token en `AuthenticateAccountResponse` y `VerifyAccessTokenQuery` (que conserva `Clone`). Rojo contra los `derive`: los dos `debug_does_not_reveal_the_token` mostraron el token.
  - Commit `c6c3281` (`refactor`, HEX-7.2): `Account::authenticate() -> Result<(), CannotAuthenticate>`, con `CannotAuthenticate` como error de dominio (solo `Debug` y `Display`). El handler lo usa en lugar de `if !account.can_authenticate()`, que se borra. Rojo contra un esqueleto que respondía `Ok(())`: `registered_account_cannot_authenticate` y `non_active_accounts_cannot_authenticate` (`left: Ok(())`). `active_account_can_authenticate` fija el otro lado.
  - Commit `7932c45` (`refactor`): `Result<_, Failure<AuthenticateAccountError>>`, con `AuthenticateAccountError { LoginFailed, CannotAuthenticate }` en `error.rs`, sin `ErrorLayout`, y dos hojas nuevas en `application/errors/` (`LoginFailed`, `CannotAuthenticate`). Las cuatro llamadas a puertos pasan a `?`; ningún `Fault` se descarta. `POST /auth/authenticate` deja el mapper: `fold` exhaustivo en `infrastructure/http/authenticate_account.rs`, con los códigos y mensajes de hoy en `codes.rs`, y un test de endpoint por brazo con `StubAuthenticateAccountUseCase`, incluido `a_fault_returns_500_with_the_d7_body`. Tests de handler sobre la variante (ERR-2.6).
    - Rojo: primero de compilación; después, cada test nuevo contra una mutación en el árbol de trabajo (el `Fault` como 401, el éxito como 201, cada hoja con el código de la otra, y el handler traduciendo `CannotAuthenticate` a `LoginFailed`). Con su entrada borrada, `hex_4_use_case_signatures_…` pidió borrarla.
    - **Agregado:** se borran el brazo `IAM_LOGIN_FAILED | IAM_CANNOT_AUTHENTICATE => 401` del mapper legacy, su test `failed_authentication_maps_to_unauthorized` y `application/errors/error_codes.rs`: quedaron sin productor (como el brazo `CONFLICT` en R11). La entrada `hex_1_driving_…` del mapper sigue con n = 10: esos códigos eran de `application`, no de `crate::domain`.
    - `account_lifecycle.rs`, adaptación mecánica: el import del error, el tipo de retorno de `sign_in` y sus dos `code()` como `matches!` sobre la variante equivalente. Ya en `09fb976`: el import y el nombre del handler. Ningún escenario ni aserto cambia de significado.
  - Commit `8adb31f` (`test`, a pedido del revisor): `fails_with_login_failed_when_an_inactive_account_gets_a_wrong_password`. Fija que la contraseña se verifica antes del estado: con el orden invertido, una cuenta no activa con contraseña incorrecta respondería `CannotAuthenticate` y revelaría que existe (D6). Fija comportamiento existente; rojo por mutación (`authenticate()` antes de `verify`): `Err(Expected(CannotAuthenticate(CannotAuthenticate)))`.
  - **Revisor:** sin bloqueantes. Un hallazgo de la rebanada, el orden (arriba, aplicado). Confirmó que el `if !verify` es orquestación (el puerto responde un veredicto), D6 intacto, un test de falla por cada llamada, el borrado del brazo del mapper y lifecycle solo mecánico. Lo demás ya está inventariado: HEX-7.11 (R12b), el mapper legacy para `identify` (ERR-4.4, ERR-4.5, HEX-1.7; R15, R19) y ERR-4.6 (R30).

**R12b. Búsquedas normalizadas en verify, authenticate e identify (nueva en P1; identify desde D10).**
- **IDs:** HEX-7.10, HEX-7.11 y D10. Son la misma causa que HEX-7.1 (R12), en los otros tres handlers. Va aparte porque cambia el contrato, y las rebanadas de la fase 3 no lo cambian.
- **Precondición:** R12, R13 y R14 hechas (toca los handlers que esas dos migran). D10 decidida (§5.7).
- **Cambios:**
  - `VerifyAccountHandler` construye `Email` antes de `find_by_email`. Si no se puede construir, es `NotFound`, igual que un email desconocido;
  - `AuthenticateAccountHandler` construye `Username` antes de `find_by_username`. Si no se puede construir, es `LoginFailed` y verifica contra el señuelo, igual que una cuenta inexistente (D6);
  - `IdentifyAccount` (D10) busca por `Email` si el identificador se puede construir como email, y si no encuentra, por `Username`. Si no se puede construir como ninguno de los dos, es `NotFound`. El nombre del handler y su contrato siguen para R15.
- **Línea base:** sin cambios.
- **Caracterización previa (primer commit de código, sin tocar producción):** un test por endpoint que fija que un identificador mal formado responde como hoy, en verde contra `main`: `verify_with_malformed_email_returns_404`, `authenticate_with_malformed_username_returns_401_login_failed` e `identify_with_malformed_identifier_returns_404`.
- **Rojo:**
  - handler: `verifies_with_an_email_that_differs_only_in_case_or_surrounding_spaces`, `authenticates_with_surrounding_spaces_in_the_username`, `identifies_with_an_email_that_differs_only_in_case_or_surrounding_spaces` e `identifies_with_surrounding_spaces_in_the_username`;
  - HTTP (`app/tests/auth_api.rs`): los de §5.5;
  - handler, ya en verde contra `main` (fijan lo que no cambia; rojo por mutación): `verifies_against_the_decoy_hash_when_identifier_is_malformed`, `fails_with_login_failed_when_identifier_is_malformed`, y los `NotFound` por formato inválido de verify e identify.
- **Contrato HTTP: cambia (§5.5).** El email en otras mayúsculas o con espacios alrededor encuentra la cuenta en verify e identify, y el username con espacios alrededor la encuentra en authenticate e identify. Autenticar con el email sigue siendo 401 `IAM_LOGIN_FAILED` (`authenticate_by_email_returns_401` no cambia): authenticate busca solo por username.
- **Tamaño:** S.
- **Resultado (hecho, PR #48):** commits `e79a25f`, `38b36b6`, `8460492`, `5359976`, `1d143b2` y `3efd113`. Línea base sin cambios (39). 413 tests, fmt y clippy en verde localmente, con `ARCH_REQUIRE_MATERIAL=1`.
  - Commit `e79a25f` (`docs`): D10 decidida (§5.7) y propagada a §5.3, §5.5 y §4, con la lista de los tests de `app/tests/` que cambian, antes de tocar código.
  - Commit `38b36b6` (`test(app)`): los tres tests de caracterización del identificador mal formado, en verde contra `main` (el revisor lo comprobó en un worktree).
  - Commit `8460492` (HEX-7.10): verify construye `Email`; mal formado → `NotFound`. Rojo antes de producción: el test de handler (`Err(Expected(NotFound { resource: "account" }))`) y los dos HTTP (`left: 404, right: 400`). Como el código real no lo expone la API, los HTTP prueban que encontraron la cuenta con un código fuera de rango: la cuenta se busca antes que el código, así que el 400 `IAM_INVALID_CODE_VALIDATION` solo sale si la encontró. `fails_with_not_found_when_email_is_malformed` fija lo que no cambia; rojo por mutación (mal formado → `Validation`).
  - Commit `5359976` (HEX-7.11): authenticate construye `Username`; mal formado → el mismo camino que la cuenta inexistente, con el señuelo. Rojo: el de handler (`Err(Expected(LoginFailed(LoginFailed)))`) y el HTTP (`IAM_LOGIN_FAILED` en lugar de `IAM_CANNOT_AUTHENTICATE`, que prueba la cuenta encontrada con la contraseña correcta y sin verificar). `verifies_against_the_decoy_hash_when_identifier_is_malformed` y `fails_with_login_failed_when_identifier_is_malformed` ya pasaban contra `main`; rojo por mutación (mal formado → `LoginFailed` sin el señuelo: `left: []`).
  - Commit `1d143b2` (D10): identify construye `Email` y después `Username`; ninguno de los dos → `NotFound`. Rojo: los dos de handler (`left: None`) y los dos HTTP (`left: 404`). `fails_with_unexpected_when_find_by_email_fails` pasa a un identificador con forma de email (uno que no lo es ya no llega a `find_by_email`) y a `failing_on(FindByEmail)`: con `failing()`, el `find_by_username` que sigue ocultaba un `Fault` tragado, también en `main` (visto por mutación). El nombre del handler y su `SystemError` siguen para R15.
  - Commit `3efd113` (`refactor`, a pedido del revisor, ERR-2): `Err(EmailError::Invalid)` y `Err(UsernameError::Invalid)` en lugar del comodín, para que una variante nueva obligue a decidir su significado en cada búsqueda.
  - **Lectura del encargo:** "authenticate con el email en otras mayúsculas" no aplica tal cual: authenticate busca solo por username, y `Username` recorta pero no cambia las mayúsculas (como el alta). Se fijó el username con espacios alrededor; autenticar con el email sigue en 401 (`authenticate_by_email_returns_401` no cambia, y no estaba en la lista).
  - `app/tests/` cambia exactamente lo listado en §5.5; `account_lifecycle.rs` y `app/tests/common/` quedaron intactos.
  - **Revisor:** sin bloqueantes. Confirmó las cinco restricciones del encargo (sin 400 nuevos, señuelo, lista de §5.5, ningún `Fault` tragado y cada llamada con su test), que construir el value object en el handler es orquestación (HEX-7: la regla vive en `Email::new`/`Username::new`) y que traducir el error de formato a `NotFound`/`LoginFailed` es legítimo (ERR-2, D6, D10). Un hallazgo de la rebanada, el comodín (aplicado en `3efd113`). Fuera de la rebanada, ya inventariado: `IdentifyAccount` sin `Handler` y con `SystemError` (R15, R19), Problem Details (R30) y la enumeración por identify y verify (§4). Observación sin principio: `Email::new` acepta espacios internos (`"a b@x.co"`), igual en el alta y en las búsquedas.

**R15. `IdentifyAccount`.** *Hecha (ver §5.8).*
- **IDs:** HEX-4.1, ERR-2.1, ERR-2.6 y ERR-2.11 (si `AccountError::AccountNotFound` queda sin productor, se borra con la rama de `VerifyAccountHandler`).
- **Cambios:** `IdentifyAccount` pasa a `IdentifyAccountHandler` en `queries/identify_account/handler.rs`. Con R11, R13, R14 y R15, ningún handler de iam usa `map_application_error`.
- **Línea base: −2** *(S3; antes −1)*: `hex_4_handlers` de `identify_account.rs` y `hex_4_use_case_signatures_are_their_own_contract` de `identify_account_use_case.rs` (`SystemError`, ERR-2.1).
- **Rojo:** `fails_with_not_found_for_unknown_identifier`, sobre la variante. Reemplaza a `fails_when_account_not_found`.
- **Sin rojo nuevo:** los `fails_with_unexpected_when_find_by_{email,username}_fails` ya existen (R8).
- **Caracterización:** ~~`identify_with_different_email_casing_returns_404` no cambia (D10).~~ Desde R12b (D10 decidida) es `identify_with_different_email_casing_returns_200`, y R15 no lo cambia.
- **Tamaño:** S.
- **Resultado (hecho, PR #49):** commits `cf2b905`, `2c37b03` y `6b09abd`. Línea base −2 entradas y la `n` del mapper de iam de 10 a 9 (39 → 37 entradas, 68 → 65 violaciones). 417 tests, fmt y clippy en verde localmente, con `ARCH_REQUIRE_MATERIAL=1`; `app/tests/` sin cambios, como pedía el encargo (la fase 3 no cambia el contrato HTTP).
  - Commit `cf2b905` (`refactor`): `IdentifyAccount` pasa a `IdentifyAccountHandler` en `queries/identify_account/handler.rs`, sin `module_inception`; el módulo reexporta query, respuesta y handler, como `verify_access_token`. Rojo: con su entrada borrada, `hex_4_use_case_implementations_are_handlers` reportó `` `IdentifyAccount` implementa un UseCase y debe llamarse *Handler ``.
  - Commit `2c37b03` (`refactor`): `Result<IdentifyAccountResponse, Failure<IdentifyAccountError>>`, con `IdentifyAccountError { NotFound }` en `error.rs` y la hoja compartida `NotFound { resource }`. Un identificador desconocido o mal formado es `NotFound { resource: "account" }`. Las dos búsquedas pasan a `?`; ningún `Fault` se descarta. `POST /auth/identify` deja el mapper: `fold` exhaustivo en `infrastructure/http/identify_account.rs`, con el código y el mensaje de hoy en `codes.rs` (los de verify), y un `resource` desconocido como el 500 de D7. Se borra `infrastructure/http/handlers/`, que queda vacío. Con R11, R13, R14 y R15, ningún endpoint de iam usa el mapper.
    - Tests de handler sobre la variante (ERR-2.6): `fails_with_not_found_for_unknown_identifier` reemplaza a `fails_when_account_not_found`, y el test de éxito (que solo afirmaba `is_ok`) se parte en `identifies_by_email_with_username_and_status` e `identifies_by_username_with_username_and_status`. Un test de endpoint por brazo con `StubIdentifyAccountUseCase`, incluidos `a_fault_returns_500_with_the_d7_body` y `an_unmapped_not_found_resource_returns_500`.
    - Rojo: el trinquete (`hex_4_use_case_signatures_…` reportó `` `IdentifyAccountUseCase::execute` falla con `SystemError` ``), después de compilación (el contrato no existía) y cada test nuevo contra una mutación de producción, anotada en el commit: el `resource`, el `Fault` tragado en cada búsqueda (`Err(Expected(NotFound(..)))`), el estado fijo (`left: "active"`) y cuatro mutaciones del `fold` (`left: 400`, `201`, `404`). Es la fricción de R11 (§6).
  - Commit `6b09abd` (`refactor`, ERR-2.11): sin productor, se borran `AccountError::AccountNotFound`, la rama de `VerifyAccountHandler` que lo traducía a `Fault` (el `match` sobre el error de `confirm_registration` solo nombra `InvalidVerification`), su código de dominio `IAM_ACCOUNT_NOT_FOUND`, y el brazo `IAM_ACCOUNT_NOT_FOUND => 404` del mapper con su test `account_not_found_maps_to_not_found`. El código de cliente sigue en `codes.rs`. Rojo: el trinquete, con la `n` bajada a 9 (`10 violaciones y baseline.txt admite 9`).
  - `account_lifecycle.rs`, adaptación mecánica: el import y el nombre del handler (`cf2b905`). Ningún escenario ni aserto cambia.
  - **Revisor:** sin hallazgos, ni de la rebanada ni nuevos. Confirmó las cinco restricciones del encargo (contrato HTTP byte a byte, ningún `Fault` tragado, lifecycle mecánico, nada adelantado y el borrado de `AccountNotFound` según ERR-1/ERR-2). Fuera de la rebanada, ya inventariado: el mapper de iam y el `impl ErrorLayout for AccountError` quedan sin llamadores de producción (HEX-1.7, ERR-4.4, ERR-4.5, HEX-8.3; R19), y `IdentifyRequest`/`IdentifiedResponse` (HEX-6.1, R28).
  - **D13 (registrada, abierta):** el username distingue mayúsculas (§5.7). No cambia nada en R15.

**R16. Cortar `communities → iam`.** *Hecha (ver §5.8).*
- **IDs:** HEX-1.1, HEX-1.2, HEX-1.3, HEX-1.4 y TDD-2.6.
- **Cambios:**
  - un `OwnerId` propio en `domain/model/community/owner_id.rs`. **D2 decidida, opción a):** valida el formato UUID y rechaza el nil, con los códigos `IAM_INVALID_ACCOUNT_ID(_FORMAT)`;
  - ~~los códigos propios de communities, y~~ esos dos strings pasan a `infrastructure/http/codes.rs` *(R16: los propios de communities, con el `fold` de R17; ver Resultado)*;
  - se quita `iam` de `contexts/communities/Cargo.toml`;
  - *(desde R7)* la batería `application/testing/contracts/community_repository.rs:14` y el módulo de tests de `create_community.rs` (línea 86) construyen el agregado con `AccountId`: pasan a `OwnerId`;
  - `FakeCommunityAdapter` gana `failing_on(CommunityOperation)`, como `FakeAccountAdapter` (§5.1), y se borra el `StubCommunityAdapter` local de `ba15189`. `fails_with_unexpected_when_save_fails` usa `failing_on(CommunityOperation::Save)`. *(S3)* Desde craft-backend 1.0.0-rc.3 es también la forma de la plantilla (`FakeCommunityAdapter::failing_on(CommunityOperation)`, con una prueba de falla por cada llamada): se toma de ahí, como ya hizo iam en R8.
- **Línea base: −1** (`hex_1` de `contexts/communities/Cargo.toml`): **cierra `hex_1_dependencies_point_inward`.**
- **Rojo:**
  - tests de dominio `owner_id_rejects_non_uuid` y `owner_id_rejects_nil`;
  - `FakeCommunityAdapter::failing_on`: en el rojo, por mutación, tragar el `Fault` de `save` pone en rojo `fails_with_unexpected_when_save_fails`. ~~La batería sigue en verde contra el Fake con `failing_on` sobre otra operación.~~ *(R16: no se puede tal cual, porque cada caso de la batería usa las dos operaciones. La batería corre contra `empty()`, y dos tests propios del doble fijan que `failing_on` hace fallar solo la operación marcada.)*
- **Contrato HTTP: ninguno (D2 a).**
- **Riesgo:** `create_with_non_uuid_subject_returns_400` y `create_with_nil_uuid_subject_returns_400`, que no cambian.
- **Tamaño:** S/M.
- **Resultado (hecho, ver §5.8):**
  - Commit `3646f25` (`feat`, HEX-1.2): `OwnerId` en `domain/model/community/owner_id.rs` e `InvalidOwnerId { Nil, WrongFormat }` en `domain/errors/invalid_owner_id.rs`, con los códigos y mensajes `IAM_INVALID_ACCOUNT_ID(_FORMAT)` (D2 a). Sin `generate()`: el id llega del token, y los tests usan un UUID fijo para no sumar otro `Uuid::new_v4` al dominio (R22). Rojo: `owner_id_accepts_a_uuid`, `owner_id_rejects_non_uuid` y `owner_id_rejects_nil` contra un `from_str` que respondía `Ok(OwnerId(nil))`.
  - Commit `f6bc1fd` (`refactor`, HEX-1.1, HEX-1.3, HEX-1.4): `Community`, `CreateCommunity` y sus tests, y la batería de `CommunityRepository`, usan `OwnerId`. El mapper toma los dos códigos `IAM_` de `infrastructure/http/codes.rs`. `iam` sale de `contexts/communities/Cargo.toml` (y de `Cargo.lock`). `rg 'iam::' contexts/communities` no encuentra nada, tests incluidos. Rojo: el trinquete, con la entrada borrada y `iam` todavía en el manifiesto.
  - Commit `c7cc244` (`test`, TDD-2.6): `FakeCommunityAdapter` con `failing_all` y `failing_on(CommunityOperation)`, la forma de la plantilla rc.3 con el puerto síncrono. Se borra el `StubCommunityAdapter`. Rojo contra un `failing_on` que ignoraba la operación: `fails_with_unexpected_when_save_fails` (`Ok(CreateCommunityResponse {…})`), `failing_on_save_fails_only_save` y `failing_on_find_by_slug_fails_only_find_by_slug`. Mutación con el `failing_on` real: `let _ = save(…)` en el handler pone en rojo `fails_with_unexpected_when_save_fails`.
  - **Desviación del plan, de acuerdo con el revisor:** solo los dos strings `IAM_` pasan a `codes.rs`. Mover también los códigos propios de communities dejaría en cero la entrada `hex_1_driving_…` del mapper (HEX-1.8) mientras el mapper siguiera despachando por `code()` del dominio, un cierre en falso. Pasan con el `fold` de R17. Por la misma razón, los dos strings `IAM_` quedan duplicados en el dominio (`InvalidOwnerId::code()`) y en `codes.rs`: importarlos de `crate::domain` habría subido la `n` del mapper a 4.
  - `app/tests/` intacto: `create_with_non_uuid_subject_returns_400` y `create_with_nil_uuid_subject_returns_400` no cambian y siguen en verde.
  - **Línea base: −1** (`hex_1_dependencies_point_inward contexts/communities/Cargo.toml#dependencies.iam 4`): 37 → 36 entradas, 65 → 61 violaciones. **Cierra `hex_1_dependencies_point_inward`.**
  - **Revisor:** sin hallazgos bloqueantes. Aplicados en este documento: las filas ERR-4.4 y HEX-1.8 nombran las constantes y los mensajes `IAM_` del dominio de communities, y la `n` del mapper que subestima el acoplamiento; R17 los cierra con el `fold`. Fuera de la rebanada: `OwnerId::from_str` copia el `#[allow(clippy::should_implement_trait)]` de `AccountId`, y ninguna rebanada lo quita en communities (se puede implementar `FromStr`); `fails_with_unexpected_when_repository_fails` debería llamarse `…_find_by_slug_fails`. *(R16b)* Inventariados como RS-2.6 (R22) y TDD-2.8 (R17).

**R16b. Baterías con la forma async, con los puertos síncronos (nueva en S3; D12).** *Hecha (ver §5.8).*
- **IDs:** ninguno nuevo; adelanta la parte de las baterías de RS-3.1 (R23, R24).
- **Precondición:** D12 decidida con la opción b) (§5.7). R16 hecha, para no tocar dos veces la batería de `CommunityRepository` (R16 cambia `AccountId` por `OwnerId`). Antes de R20, que crea una batería nueva con esa forma.
- **Cambios:** las 4 baterías (`community_repository`, `account_repository`, `password_hashing_service`, `access_token_service`) pasan a la forma final de la receta "Puerto todavía síncrono": casos `pub async fn`, helpers `async fn` sin `.await`, macro con `run`, invocada con `run: block_on` en el doble, el adaptador en memoria, Argon2 y JWT, y `#![expect(clippy::unused_async, reason = "puerto todavía síncrono: se quita en su rebanada de asincronía")]`. `block_on` de std en `application/testing.rs`.
- **Línea base:** sin cambios.
- **Tests:** refactor de tests sin cambio de comportamiento: los mismos casos, en verde antes y después. El rojo se ve por mutación del doble, como en R7 a R10.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** S.
- **Resultado (hecho, ver §5.8):** commits `d3ab538` (communities), `fdc1941` (iam) y `c0d0102` (notas del revisor). Línea base sin cambios (36 entradas, 61 violaciones). 422 tests, los mismos que después de R16; fmt y clippy en verde localmente, también con `--features testing`.
  - Las 4 baterías tienen la forma de la receta: casos `pub async fn`, helpers `async fn` que llaman al puerto sin `.await`, macro con `run` (el último parámetro, como en la plantilla) y `#![expect(clippy::unused_async, reason = "puerto todavía síncrono: se quita en su rebanada de asincronía")]`. `block_on` de std, copiado de la plantilla, en `application/testing.rs` de cada contexto. Las 9 invocaciones pasan `run: block_on`: `FakeCommunityAdapter`, `InMemoryCommunityAdapter`, `FakeAccountAdapter`, `InMemoryAccountAdapter`, `FixedPasswordHashingAdapter`, `SpyPasswordHashingAdapter`, `Argon2PasswordHashingAdapter`, `FakeAccessTokenAdapter` y `JwtAccessTokenAdapter`. Argon2 bloquea dentro del future: con `block_on` no importa, y R23 lo pasa a un runtime real con `spawn_blocking`.
  - La forma encajó sin tocar producción: ni puertos, ni adaptadores (fuera de su `mod tests`), ni `app/`. El `#![expect]` se cumple con el clippy por defecto (`unused_async` dispara), como dice la receta.
  - `access_token_service_contract!` conserva sus parámetros propios: `expired_token` sigue siendo una closure síncrona (`Fn(&S, &str) -> String`), porque es un gancho del adaptador y no una llamada al puerto, y `another_issuer` no cambia. `run` va después de los dos.
  - **Refactor puro:** los mismos 55 tests de batería, por nombre, antes y después (`cargo test --workspace -- --list`, `diff` vacío): community 5 × 2, account 9 × 2, password hashing 5 × 3 y access token 6 × 2. Ningún aserto cambia; solo se agregan los `.await` a los helpers.
  - **Rojo por mutación del doble** (sin commitear; reproduce el rojo de la rebanada que creó cada batería):
    - `FakeCommunityAdapter::save` sin guardar: `saves_and_finds_by_slug`, `finds_each_community_by_its_own_slug` y `save_replaces_the_community_with_the_same_id`, como en R7;
    - `FakeAccountAdapter::save` sin guardar: los 4 de R8 y `finds_by_the_normalized_email_and_username` (R12);
    - `FixedPasswordHashingAdapter::verify` comparando el texto plano con el hash, como el doble viejo: `verify_accepts_the_hashed_password_and_rejects_another`, `corrupt_hash_is_a_fault` y `truncated_hash_is_a_fault`, en Fixed y en Spy (R9);
    - `FakeAccessTokenAdapter::verify` siempre `Invalid`: `issued_token_is_valid_for_its_account` y `each_token_carries_its_own_account`, como en R10a.
  - `app/tests/` y `account_lifecycle.rs` intactos.
  - Inventariadas las dos notas del revisor de R16: RS-2.6 (`OwnerId` con `FromStr`, R22) y TDD-2.8 (el nombre del test de `find_by_slug`, R17).
  - **Revisor:** sin hallazgos en la rebanada. Confirmó las cinco restricciones del encargo: los mismos casos y asertos, cada batería contra el doble y el adaptador, producción y puertos intactos, la forma de la plantilla, y que `run` ejecuta cada caso (lo probó rompiendo un aserto en una copia desechable, también en un caso sin ningún `.await`). Aplicadas dos notas de texto en `c0d0102`: el doc de la macro dice "como `async fn` arriba", como la plantilla, y el comentario de módulo nombra también las llamadas directas al puerto dentro de los casos, que ganan `.await` en R23/R24. Fuera de la rebanada no encontró nada nuevo.

**R17. `CreateCommunity`.** *Hecha (ver §5.8).*
- **IDs:**
  - HEX-4.1, HEX-4.2 (`owner_id: String` en el comando);
  - ERR-2.1, ERR-2.4, ERR-2.6 y ERR-2.10 (lo que queda: `fails_when_owner_id_is_invalid`);
  - *(R16b)* TDD-2.8: `fails_with_unexpected_when_repository_fails` pasa a `…_find_by_slug_fails`, con `failing_on(CommunityOperation::FindBySlug)`;
  - HEX-1.8 (los códigos salen de domain);
  - ERR-4.4 y ERR-4.5 (communities).
- **Cambios:** como R11, el `unexpected()` del mapper de communities sale a `infrastructure/http/unexpected.rs`, con su test.
- **Línea base: −4** *(S3; antes −1)*:
  - `hex_4_handlers` de `create_community.rs`;
  - `hex_4_use_case_signatures_are_their_own_contract` de `create_community_use_case.rs` (n = 2: `SystemError`, ERR-2.1, y el segundo argumento, HEX-4.2). Con R13b, R14 y R15 hechas, **cierra `hex_4_use_case_signatures_are_their_own_contract`**;
  - `hex_4_use_case_contracts_do_not_expose_domain_types` de `create_community_use_case.rs` (`VerifiedIdentity`, HEX-4.2): **cierra el test**;
  - `hex_1_driving_adapters_see_only_use_cases` del mapper de communities (HEX-1.8), si deja de nombrar `crate::domain`. Si el mapper sigue hasta R19, esta pasa a R19 (R17 −3).
- **Rojo:**
  - `fails_when_owner_id_is_invalid` (handler);
  - un test de endpoint 500 con `StubCreateCommunityUseCase`.
- **Sin rojo nuevo:** `fails_with_unexpected_when_repository_fails` y `…_save_fails` ya existen (R7; desde R16, con `failing_on`).
- *(R16)* El `fold` cierra también los brazos `IAM_INVALID_ACCOUNT_ID(_FORMAT)`, que hoy casan por valor con `InvalidOwnerId::code()`, y borra esas constantes del dominio y `InvalidOwnerId::message()` (ERR-4.4, HEX-1.8).
- **Caracterización:** `create_checks_slug_uniqueness_before_subject_format` fija el orden: primero el slug, después el owner (hoy: slug, conflicto, owner, nombre; create_community.rs:38-50).
- **Tamaño:** M.
- **Resultado (hecho, ver §5.8):** commits `4a04532`, `60caf6f`, `9614aaa`, `58958bc` y `838b923`. Línea base −4: 36 → 32 entradas (61 → 55 violaciones). **Cierra `hex_4_use_case_implementations_are_handlers`, `hex_4_use_case_signatures_are_their_own_contract` y `hex_4_use_case_contracts_do_not_expose_domain_types`.** 434 tests, fmt y clippy en verde localmente, con `ARCH_REQUIRE_MATERIAL=1`; `app/tests/` sin cambios (la fase 3 no cambia el contrato HTTP).
  - Commit `4a04532` (`refactor`, HEX-4.1): `CreateCommunityHandler` en `commands/create_community/handler.rs`, con reexports y sin `module_inception`. Rojo: con la entrada borrada, `hex_4_use_case_implementations_are_handlers` reportó `` `CreateCommunity` implementa un UseCase y debe llamarse *Handler ``.
  - Commit `60caf6f` (`test`, TDD-2.8): `fails_with_unexpected_when_find_by_slug_fails`, con `failing_on(CommunityOperation::FindBySlug)`. Rojo por mutación: `.unwrap_or(None)` sobre `find_by_slug` da `Ok(CreateCommunityResponse {…})`.
  - Commit `9614aaa` (`refactor`, ERR-4.7): `unexpected()` en `infrastructure/http/unexpected.rs`, con el código y el mensaje de D7 en `codes.rs` y `unexpected_fault_maps_to_internal_server_error`; el mapper delega en él (`unexpected_system_error_maps_to_the_d7_body`). `infrastructure/http/testing.rs` es el de iam.
  - Commit `58958bc` (`refactor`, HEX-4.2, ERR-2.1, ERR-2.4, ERR-2.6, ERR-2.10, ERR-4.4, ERR-4.5): `execute(command)` con `owner_id` en el comando y `Failure<CreateCommunityError { Validation, Conflict, InvalidOwner }>`. `Validation` y `Conflict`, copiados de iam; el slug tomado es `Conflict { resource: "slug" }`. Se borran `CommunityCreationError`, `application/errors/error_codes.rs` y `handlers/`. `fold` exhaustivo en `infrastructure/http/create_community.rs`, con un test de endpoint por brazo, el 500 de D7, dos claves desconocidas que van al 500 (como iam) y `SpyCreateCommunityUseCase`, que fija que el sujeto de la identidad es el `owner_id`. Se borra el brazo `CONFLICT` del mapper, sin productor, con `duplicated_slug_maps_to_conflict`.
    - **Decisión de diseño (no estaba en el plan):** D2 a) responde un código distinto para el owner nil y para uno mal formado, y `Validation { field }` no puede llevar esa diferencia. Se agregó la hoja `InvalidOwner { Nil, WrongFormat }` en `application/errors/invalid_owner.rs`, que el handler traduce por variante desde `InvalidOwnerId`. El revisor la confirmó según ERR-2, como el conjunto cerrado de motivos que propone ERR-4.13.
    - Orden sin cambios: slug, conflicto, owner y nombre. Lo fijan `fails_with_conflict_on_taken_slug_before_checking_the_owner` y `fails_when_owner_id_is_invalid_before_validating_the_name`, además de `create_checks_slug_uniqueness_before_subject_format`.
    - Rojo: el trinquete (los dos `hex_4_*` reportaron violaciones nuevas), después compilación (el contrato no existía) y cada test nuevo contra una mutación de producción, anotada en el commit: 15 mutaciones (las dos variantes del owner, el `resource`, el `field` del nombre, los dos órdenes y nueve del `fold`), cada una con su test en rojo.
  - Commit `838b923` (`refactor`, HEX-1.8, ERR-4.4): sin llamadores, se borran los `impl ErrorLayout` de `InvalidCommunityName`, `InvalidCommunitySlug` e `InvalidOwnerId` (con `message()`) y sus constantes de dominio, incluidos los dos strings `IAM_` duplicados desde R16. Quedan solo en `codes.rs`. Sin productores, se borra el brazo 400 del mapper con `invalid_input_maps_to_bad_request`. **Cierre de HEX-1.8 por la dependencia:** el mapper sigue en `infrastructure/http/errors/error_mapper.rs`. Con la entrada borrada, el test lo reportó en esa ruta (`error_mapper.rs:1: nombra el dominio (crate::domain::errors::error_codes::COMMUNITIES_INVALID_COMMUNITY_{NAME,SLUG})`) y pasó a verde al quitar el import, sin mover ni renombrar nada. Quedan en `domain/errors/error_codes.rs` el código de `InvalidCommunityId` y los 6 de `CommunityError` (R19).
  - **Revisor:** sin hallazgos en la rebanada. Confirmó las cinco restricciones del encargo: el orden, el contrato HTTP byte a byte, el cierre de HEX-1.8 por la dependencia (sin acoplamiento por valor con el dominio), ningún `Fault` tragado y nada adelantado. Aplicados en este documento: ERR-4.13 suma las ubicaciones de communities y la nota sobre `InvalidOwner` para R30; ERR-4.8 apunta a `create_community.rs:27`, porque R17 borró `community_handlers.rs`. Propuesta de VER-1, anotada en §6: ningún test prohíbe un comodín `_ =>` sobre el enum del contrato en un `fold`.

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
- **Línea base: −2** *(S3; antes −1)*: `hex_8_kernel_traits` de `shared/src/error.rs`, y `hex_1_driving_adapters_see_only_use_cases` del mapper de iam (n = 10, HEX-1.7), que desaparece con el mapper. *(R17)* La del mapper de communities ya la cerró R17 (838b923): R19 queda en −2. **Cierra `hex_8_kernel_traits_are_model_markers_or_listed_capabilities` y `hex_1_driving_adapters_see_only_use_cases`.**
- **Tests:** refactor puro, sin rojo nuevo.
- **Caracterización:** lifecycle queda sin `SystemError`.
- **Tamaño:** M.
- **Resultado (hecho, ver §5.8):** commits `0a3c49a` y `11f8c92`. Línea base −2: 32 → 30 entradas (55 → 45 violaciones). **Cierra `hex_1_driving_adapters_see_only_use_cases` y `hex_8_kernel_traits_are_model_markers_or_listed_capabilities`.** 429 tests (434 − los 5 de los mappers), fmt y clippy en verde localmente; `app/tests/` sin cambios, y lifecycle sin adaptación: ya no nombraba `SystemError`.
  - **Comprobaciones previas (pedidas por el usuario):** el registro interino del `Fault` a stderr (R5, ERR-4.7) ya estaba en los dos `unexpected()` (`iam/…/http/unexpected.rs:17` y `communities/…/http/unexpected.rs:17`), no solo en los mappers: no hizo falta el commit previo. `rg` no encontró llamadores de producción de los mappers, pero sí uno de `SystemError`: `AccountFactory::reconstitute` (`account_factory.rs:77`). La rebanada se detuvo y el usuario eligió adelantarlo desde R20.
  - Commit `0a3c49a` (`refactor`, ERR-1.5, adelantado de R20): se borra `AccountFactory::reconstitute`, sin uso fuera de los tests. Los helpers `AccountFactory::dummy_account(_with_status)` y `dummy_account` de los tests de authenticate construyen la cuenta con `Account::new` y los value objects, con los mismos strings de estado (todavía por `AccountStatus::from_str`, que borra R20). Sin rojo: refactor de tests; los 190 de iam siguen en verde.
  - Commit `11f8c92` (`refactor`, ERR-1.1, ERR-2.2, RS-2.1, HEX-8.3, HEX-1.7, HEX-7.12, ERR-4.4, ERR-4.5): se borran `shared/src/error.rs` (`ErrorLayout`, `ArchitectureLayer`, `SystemError` y el puente `Unexpected`), `iam/src/domain/errors/error_codes.rs`, `communities/src/domain/errors/error_codes.rs` (el de `InvalidCommunityId` y los 6 de `CommunityError`), `CommunityError`, los dos `infrastructure/http/errors/` (mapper y `mod.rs`) con sus 5 tests (`invalid_input_maps_to_bad_request`, `unknown_errors_map_to_internal_server_error` ×2, `unexpected_system_error_maps_to_the_d7_body` ×2) y el doble `ErrorWithCode` (R3), y los 9 `impl ErrorLayout` de los errores de dominio, que quedan con `Debug` y `Display`. Los `error_codes.rs` de application ya no existían (R14, R17). Rojo: el trinquete. Con las dos entradas borradas, `hex_8_kernel_traits_…` reportó `` shared/src/error.rs:10: `ErrorLayout` no es un marcador … `` y `hex_1_driving_…`, los 9 códigos de `crate::domain::errors::error_codes` del mapper de iam; los dos pasaron a verde con el borrado.
  - **Revisor:** sin hallazgos en la rebanada. Confirmó los cinco puntos del encargo: el `Fault` sigue llegando a stderr por los dos `unexpected()` (los mappers solo delegaban en él); ningún código o mensaje que un `fold` responda se perdió (todos están en los `codes.rs`, y lo borrado no llegaba a ninguna respuesta); los helpers reescritos conservan escenarios y asertos; `app/tests/` intacto; y de otra rebanada solo se adelantó `reconstitute`. Aplicado en este documento: ERR-1.5 resuelto y fuera de los IDs de R20, y la nota de R20 sobre los helpers que usan `AccountStatus::from_str`. Queda para R20: `reconstituted_user_preserves_status` (`account.rs:265`) conserva el nombre aunque ya no exista `reconstitute`. Sin propuesta de VER-1.

> **Punto de replanificación P2 (después de R19).** Ver §5.9. Ya no queda `SystemError`: se confirma que el árbol de errores quedó como pide ERR antes de tocar el dominio.

#### Fase 4: dominio (hechas)

**R20a. El id de `Account` llega de `IdGenerator` (P2: primera parte de R20).**
- **IDs:** HEX-7.6 (iam), HEX-2.2, RS-2.2 (`account_id.rs`) y HEX-7.14 (iam: se borran `from_uuid`, `from_str` y `AccountIdError`, sin llamadores de producción).
- **Cambios:**
  - `AccountId` sobre `String` (`new(String)`, `as_str()`), sin `uuid`, sin `generate`, sin `from_uuid`, sin `from_str` y sin `as_uuid`. `from_str` y `as_uuid` no tienen llamadores de producción fuera del id: `as_uuid().to_string()` en `register_account/handler.rs:95` y `authenticate_account/handler.rs:87` pasa a `as_str()`;
  - `AccountId` deja de implementar `ValueObject`, y desaparecen sus 2 `todo!()` (HEX-8.10, parcial). El trait se ajusta en R21;
  - `RegisterAccountHandler` recibe `Arc<dyn IdGenerator>` y pasa el id a `AccountFactory::create`; el composition root cablea `UuidIdGeneratorAdapter`;
  - `FixedIdGeneratorAdapter` (y, si hace falta, uno con secuencia) en `application/testing.rs`;
  - los usos de `AccountId::generate()` en tests (la batería de `AccountRepository`, los helpers de `AccountFactory` y de authenticate) pasan a ids fijos;
  - *(S3)* `UuidIdGeneratorAdapter` se copia de `stacks/rust/template/infrastructure/uuid` (crate `infrastructure-uuid`, con `next_id_is_a_uuid_v4` y `next_id_does_not_repeat`) a `infrastructure/uuid/` en la raíz del workspace: infraestructura compartida, no `contexts/iam/src/infrastructure`. Lo exige `hex_8_capability_adapters_are_shared_infrastructure`. En legacy depende de `shared` (`shared.workspace = true`, `use shared::IdGenerator`) en lugar de `kernel`; R29 lo renombra. `members` del `Cargo.toml` raíz suma `infrastructure/*`. *(P2, verificado)* `tools/architecture` ya clasifica `infrastructure/*` como Infrastructure compartida (`tests/common/mod.rs`, `crates_at`), también en legacy;
  - con este primer adaptador de producción, el conjunto `implementaciones de producción de capacidades del kernel` deja de estar vacío, y el test pide **borrar la tercera exención de `material-exempt.txt`** (`hex_8_capability_adapters_are_shared_infrastructure`). La exención la escribió el usuario, y el hook `pre-edit` bloquea al agente sobre ese archivo. Si también bloquea el borrado, la rebanada se detiene ahí y le pide al usuario que la borre, en el mismo PR;
  - se quita el `should_implement_trait` de `account_id.rs`.
- **Línea base: −1** *(P2; antes sin cambios)*: `hex_2_core_depends_only_on_std_and_the_kernel contexts/iam/Cargo.toml#dependencies.uuid 2`. Con `AccountId` sobre `String`, nada de iam usa `uuid` (verificado en P2: el único archivo es `account_id.rs`), y sale del manifiesto (§5.1, "Una dependencia, una entrada").
- **Rojo:**
  - handler con `FixedIdGeneratorAdapter`: `registers_with_the_generated_id`;
  - los dos tests de la plantilla en `infrastructure-uuid` (no compilan sin el crate);
  - el trinquete de `uuid` y la exención.
- **Riesgo:** `register_returns_201_with_uuid…` (36 caracteres), que cumple el adaptador UUID.
- **Contrato HTTP:** sin cambios.
- **Tamaño:** M.
- **Resultado (R20a):** `bae89e9`, `5b49e36`, `d461bbc`. Línea base −1 (`hex_2_core … contexts/iam/Cargo.toml#dependencies.uuid 2`): 30 → 29 entradas. La tercera exención de `material-exempt.txt` se borró en `5b49e36`, a pedido del usuario (el hook `pre-edit` no bloqueó el borrado; §6). 428 tests, fmt, clippy y `ARCH_REQUIRE_MATERIAL=1` en verde localmente.
  - **Orden:** primero el refactor de `AccountId` sobre `String`, sin cambio de comportamiento (`bae89e9`), para que el rojo de `registers_with_the_generated_id` fuera de aserción y no de compilación (`left` un UUID aleatorio, `right` `"acc-1"`). Los dos tests de `infrastructure-uuid` llegan con el crate (rojo de compilación, como preveía el plan, más la mutación anotada en el revisor).
  - **Lifecycle (adaptación mecánica):** un `FixedIdGeneratorAdapter` local para construir el handler, porque el test de integración no ve `application::testing`. Cada escenario registra una sola cuenta: ningún escenario ni aserto cambia.
  - **Tests:** la batería de `AccountRepository` usa `id-{username}` para que las cuentas de un caso tengan ids distintos; `save_replaces_the_account_with_the_same_id`, `acc-1`.
  - **Revisor:** sin hallazgos, de esta rebanada ni de otra. Confirmó que no entra ningún `use` prohibido en el núcleo (VER-1.8), que lifecycle solo cambia la construcción, los nombres de los dobles y del adaptador, los ids distintos de la batería y que no se adelantó nada de R20b, R21 ni R22. Aplicado: el comentario del hash volvió encima de `AccountFactory::create` (`register_account/handler.rs`). Sugirió evidencia de mutación para `infrastructure-uuid`: con `next_id` constante, `next_id_does_not_repeat` falla (`left: 1`, `right: 100`); revertido. Sin propuesta de VER-1.

**S4. Sincronizar con `v1.0.0-rc.4` (nueva en P2; se adelanta en S4 a antes de R20b).**
- **Objetivo:** `/craft-backend:adoption sincronizar v1.0.0-rc.4`, el primer release de ai-harness posterior a `v1.0.0-rc.3`. Trae las correcciones de las nueve fricciones de P2 y la de `unexpected()` sin `code` (§6).
- **Cuándo:** *(S4)* aquí, después de R20a y antes de R20b. *(P2: después de R26 y antes de P3.)* Se adelanta porque el release salió antes de lo previsto, y porque `hex_2_core_imports_only_std_and_the_kernel` (VER-1.8) pone trinquete por archivo a los imports del núcleo que R20b, R21, R22 y R25 sacan: conviene que esas rebanadas lo vean en rojo. Ya no espera a R26, y P3 ya no espera a S4.
- **Regla (§5.1, "La línea base solo crece en una sincronización con el harness"):** PR propio, sin código de producción, con el análisis de la línea base vieja frente a la nueva en cuatro grupos (se mantienen, cambian de clave, desaparecen y nuevas), cada entrada con su ID del inventario y su rebanada. Si una entrada desaparece sin un cambio de formato que lo explique, el PR se detiene.
- **Resultado (S4):** ver §5.10. Medido sobre `main` en `027813a` (después de R20a), no sobre la medición de ai-harness. 29 se mantienen, ninguna cambia de clave, ninguna desaparece, 6 nuevas (29 → 35). `material-exempt.txt` no cambia, y `ARCH_REQUIRE_MATERIAL=1` no pide ninguna exención nueva. Comparación de la plantilla con lo copiado: `infrastructure/uuid` no cambia entre rc.3 y rc.4 (R20a sigue al día); `kernel/src/model.rs` solo suma el comentario que admite `Entity` e `Identifier` (R21); `edge/http` exporta `UnexpectedProblem` y `unexpected()` admite `.with_code(…)` (R26, R30).
- **Tamaño:** S (solo `tools/architecture` y este documento).

### 5.5 Cambios de contrato HTTP: filas hechas

|---|---|---|
| R2 (hecho) | Sign-in de un usuario inexistente: de 404 a 401 `IAM_LOGIN_FAILED` | `authenticate_unknown_username_returns_404` → `authenticate_unknown_username_returns_401_login_failed`; `authenticate_by_email_returns_404` → `authenticate_by_email_returns_401` |
| R5, R10 | 500 de infraestructura con `{"code":"INTERNAL_ERROR",…}` (D7), sin `IAM_TOKEN_GENERATOR_ERROR` | ninguno (solo los tests del mapper) |
| R10a (hecho) | `POST /auth/authenticate` con falla al emitir el token: el 500 pierde `IAM_TOKEN_GENERATOR_ERROR` y pasa al cuerpo de D7 (`INTERNAL_ERROR`). Ningún test de caracterización lo cubre: JWT solo falla si el `ttl` desborda el reloj. Lo fijan `fails_with_unexpected_when_issue_fails` (handler) y `unexpected_fault_maps_to_internal_server_error` (mapper). Los 401 de las rutas protegidas no cambian: un token mal formado, de otra clave o vencido sigue dando 401, ahora como veredicto `Invalid` | ninguno. En el test del mapper, `technical_and_unknown_errors_map_to_internal_server_error` pasa a `unknown_errors_map_to_internal_server_error` |
| R10b (hecho) | Ruta protegida con un `Fault` al verificar el token (clave o validación mal configurada): de 401 con cuerpo vacío a 500 con el cuerpo de D7. Es un cambio de contrato sin cobertura previa: ningún test de caracterización lo cubre, porque JWT no falla con la configuración de los tests. Lo fija `a_fault_verifying_the_token_returns_500` (middleware). `Invalid` sigue dando 401, y la caracterización no cambia | ninguno |
| R7 (hecho) | `POST /communities/create` con falla del repositorio: el 500 pierde `COMMUNITIES_REPOSITORY_ERROR` y pasa al cuerpo de D7 (`INTERNAL_ERROR`). Ningún test de caracterización lo cubre: los adaptadores en memoria no fallan. Lo fijan `fails_with_unexpected_when_repository_fails`, `fails_with_unexpected_when_save_fails` (handler) y `unexpected_fault_maps_to_internal_server_error` (mapper) | ninguno. En los tests del mapper, `technical_and_unknown_errors_map_to_internal_server_error` pasa a `unknown_errors_map_to_internal_server_error` |
| R9 (hecho) | Ninguno en las respuestas. Una falla de Argon2 (al hashear, o un hash guardado ilegible o incompleto) da el 500 de D7 en lugar de cortar la conexión con un pánico, o de un 401 si el hash estaba incompleto. Ningún test de caracterización lo cubre. Si el hash del señuelo falla al arrancar, el servidor no arranca | `app/tests/common/mod.rs`: `app()` agrega `.expect("app builds")` porque `build_app` devuelve `Result` (decidido por el usuario). Ningún escenario ni aserto cambia |
| R8 (hecho) | `POST /auth/register`, `/auth/verify`, `/auth/authenticate` e `/auth/identify` con falla del repositorio: el 500 pierde `IAM_ACCOUNT_REPOSITORY_ERROR` y pasa al cuerpo de D7 (`INTERNAL_ERROR`). Antes solo `save` podía fallar; ahora también `find_by_email` y `find_by_username`, con el mismo 500. Ningún test de caracterización lo cubre: `InMemoryAccountAdapter` no falla. Lo fijan los 8 `fails_with_unexpected_when_<operación>_fails` de los handlers y `unexpected_fault_maps_to_internal_server_error` (mapper) | ninguno. En el test del mapper, `technical_and_unknown_errors_map_to_internal_server_error` pierde `IAM_ACCOUNT_REPOSITORY_ERROR` de su lista (queda `IAM_TOKEN_GENERATOR_ERROR` y `UNKNOWN`) |
| R11 (hecho) | Ninguno en `app/tests/`. Si el hasher devuelve un hash que no es un `HashedPassword`, `POST /auth/register` responde el 500 de D7 en lugar de 400 `IAM_INVALID_HASHED_PASSWORD`. Sin cobertura previa: Argon2 nunca produce uno. Lo fija `fails_with_unexpected_when_the_hasher_returns_a_malformed_hash` | ninguno |
| R12 (hecho) | Duplicado por mayúsculas o espacios: de 201 a 409 | `register_accepts_same_email_with_different_casing` → `register_rejects_same_email_with_different_casing_with_409`; `register_accepts_same_username_with_surrounding_spaces` → `register_rejects_same_username_with_surrounding_spaces_with_409` |
| R12 (hecho) | Username inválido + email tomado: de 409 `IAM_ACCOUNT_EMAIL_ALREADY_EXISTS` a 400 `IAM_INVALID_USERNAME`. Es el cambio de orden anotado en P1: primero el formato, después el conflicto | `register_with_invalid_username_and_taken_email_returns_409` (lo agrega R12 en su primer commit, `3146a43`) pasa a `register_rejects_invalid_username_before_checking_taken_email` |
| R12 (hecho) | Contraseña corta o email inválido + un valor ya tomado: de 409 a 400 (la validación va antes que el conflicto). Sin cobertura previa | ninguno. Lo fijan `fails_with_validation_on_short_password_before_checking_taken_email` y `fails_with_validation_on_invalid_username_before_checking_taken_email` (handler) |
| R12b | D10: los tres endpoints que buscan una cuenta normalizan el identificador como el alta. `POST /auth/verify` con el email en otras mayúsculas o con espacios alrededor encuentra la cuenta (de 404 `IAM_ACCOUNT_NOT_FOUND` a la respuesta de esa cuenta: 200, o 400 si el código no sirve). `POST /auth/authenticate` con espacios alrededor del username, de 401 `IAM_LOGIN_FAILED` a la respuesta de esa cuenta. `POST /auth/identify` con el email en otras mayúsculas o con espacios alrededor, o con el username con espacios alrededor, de 404 a 200. Sin cambio: un identificador mal formado sigue respondiendo como una cuenta inexistente (404 en verify e identify, 401 `IAM_LOGIN_FAILED` con el señuelo en authenticate), y autenticar con el email sigue siendo 401 | **Cambia:** `identify_with_different_email_casing_returns_404` → `identify_with_different_email_casing_returns_200`. **Nuevos, del cambio:** `verify_finds_the_account_with_different_email_casing`, `verify_finds_the_account_with_surrounding_spaces_in_the_email`, `authenticate_finds_the_account_with_surrounding_spaces_in_the_username` e `identify_with_surrounding_spaces_returns_200`. **Nuevos, de lo que no cambia** (verdes contra `main`): `verify_with_malformed_email_returns_404`, `authenticate_with_malformed_username_returns_401_login_failed` e `identify_with_malformed_identifier_returns_404`. Ningún otro test de `app/tests/` cambia |
| R15 (hecho) | Ninguno: el 404 `IAM_ACCOUNT_NOT_FOUND`, el 200 `{username,status}` y el 500 de D7 se conservan byte a byte | ninguno |
| R16 (hecho) | Ninguno: D2 decidida, opción a) (se conservan `IAM_INVALID_ACCOUNT_ID(_FORMAT)`) | ninguno |
| R16b (hecho) | Ninguno: solo cambian las baterías de contrato | ninguno |
| R17 (hecho) | Ninguno: los códigos, mensajes y status de `POST /communities/create` se conservan byte a byte, con el orden slug, conflicto, owner y nombre. Un `Fault` sigue dando el 500 de D7 | ninguno |
| R13 (hecho) | Ninguno: D1 decidida (verificar dos veces sigue siendo 400 `IAM_ACCOUNT_INVALID_VERIFICATION`; el 409 se revisa con R30). Los códigos, mensajes y el 200 con cuerpo `null` se conservan byte a byte | ninguno |
| R13b (hecho) | Ninguno: `VerifyAccountResponse` vacío no se serializa; sigue el 200 con cuerpo `null` (D11 a) | ninguno |
| PR #40 (P1) | Arranque: un `JWT_SECRET` vacío o de menos de 32 bytes ya no arranca el servidor (ERR-4.9). No es una respuesta HTTP | ninguno |

### 5.9 Puntos de replanificación: P1 y P2

**P1 (2026-11-23, sobre `bab68c5`).**
1. Auditoría completa de todo el repo con `craft-backend:architecture-reviewer`. Encontró un hallazgo de severidad alta (ERR-4.9: `JWT_SECRET` vacío), que se corrigió antes que este documento, en un PR propio (#40). Además, 15 hallazgos nuevos de severidad media o baja (§2). Pasó a resuelto ERR-2.8, y se actualizaron la ubicación o el estado de unos 25 IDs (ERR-2.7, ERR-2.9 y ERR-2.10 ya tenían los tests de repositorio desde R7/R8; RS-3.1 nombraba `token_verifier.rs`).
2. `cargo test -p architecture` en verde. Las 33 entradas coinciden con §5.6, cada una con su ID y su rebanada.
3. D1, D2, D4 y D8 decididas (§5.7) y propagadas a R13, R16, R21, R22 y R30.
4. Tamaños: R10 se partió en dos (R10a pasó de 700 líneas) y costó $7.79 en total, frente a $3.98–$5.53 de R7 a R9. Con los tests de repositorio ya escritos, R13 baja a S y R22 a S. R12b es nueva (S). Ninguna de las que vienen se parte.
5. R11 a R17 revalidadas contra el código: nombres, `failing_on`, el `?` sin la secuencia de R10a y el puente `SystemError::Unexpected` (texto antes de R11). `ARCH_REQUIRE_MATERIAL=1` queda condicionado en R11 (ver allí).

**P2 (2026-11-26, sobre `0eaab25`).** PR propio, solo documentación: no cambia código, `baseline.txt` ni `material-exempt.txt`.
1. Auditoría completa de todo el repo con `craft-backend:architecture-reviewer` (no sobre un diff). **Ningún hallazgo de severidad alta.** 11 hallazgos nuevos (§2): 3 de severidad media (HEX-2.7, VER-1.8, TDD-2.9) y 8 de baja (HEX-7.13, HEX-7.14, VER-1.9, ERR-2.12, TDD-2.10, TDD-2.11, RS-1.5, RS-1.6). Ninguno abierto quedó resuelto del todo de paso. Parciales: HEX-7.5 (R12 y R19 dejaron `create` con un `AccountDraft`), HEX-2.5 (el uso de `http` desapareció en R17, 58958bc; queda el manifiesto, RS-1.5) y HEX-8.6 (`AuthContext`, a603969). IDs desactualizados que se corrigieron: HEX-1.4 (el mapper ya no existe), HEX-7.3, HEX-7.5, HEX-7.6, VER-1.7, ERR-3.4 (líneas), RS-2.2 (el `todo!()` de communities ya no existe; quedan 10), TDD-2.7 (sin rebanada: R20b), HEX-8.10 (11 archivos más 6 `impl`), HEX-1.7 en los IDs de R28 (ya resuelto), y la cabecera, las prioridades y la nota de §5 sobre `reconstitute`. Se asignó rebanada a los 12 hallazgos que solo decían `abierto`.
2. `cargo test -p architecture` en verde, también con `ARCH_REQUIRE_MATERIAL=1`. Las 30 entradas coinciden con §5.6, y cada una tiene un ID y una rebanada (tabla en §5.6). Ninguna incumplía de forma, pero tres tenían una rebanada que no las resolvía o las resolvía tarde (`uuid` y `http` de communities, `uuid` de iam). Pasan a R22 y R20a.
3. R20 a R26 revalidadas contra el código y contra la plantilla `v1.0.0-rc.3` (§5.3):
   - **R20** se parte en R20a (el id, `infrastructure/uuid`, la tercera exención y `uuid` de iam, −1) y R20b (`VerificationCodeService`, `create`/`restore`, `from_str` con sus 26 llamadas de los helpers, `restored_account_preserves_status`, TDD-2.7). ERR-1.5 ya salió en R19;
   - **R21** conserva su alcance (−7). `Entity` se borra, `DomainEvent` queda como el marcador de la plantilla, y `Clock` sigue fuera por D4, aunque rc.3 lo trae en el kernel. Suma HEX-7.13 (se borran 6 métodos sin uso, así que HEX-7.3 se achica) y ERR-2.12. Si pasa de 500 líneas, se parte en R21a y R21b;
   - **R22** pasa de S a S/M y de 0 a −2: `OwnerId` sin `uuid` (HEX-2.7), `http` (RS-1.5), el dueño en la batería (TDD-2.9) y el borrado de `CommunityId::from_str` (HEX-7.14). Depende de R20a;
   - **R23 y R24:** después de R16b les quedan los puertos, los `.await` y el `#![expect]`. `spawn_blocking` y el runtime real de Argon2 y JWT pasan a R28, porque `tokio` en legacy sería una entrada nueva. R24 baja a S;
   - **R25** sigue siendo correcta con rc.3 (el kernel de la plantilla no tiene dependencias), y resuelve HEX-8.8, que antes era de R30;
   - **R26** sigue siendo correcta con rc.3: copia `edge/http` e `infrastructure/stderr`, y no `infrastructure/system` (D4). Suma RS-1.6 y la feature `catch-panic`.
4. Precondiciones de R27 y R28 (§5.6): con la secuencia de P2, solo quedan las entradas que esas rebanadas resuelven al mover. R27 baja de −7 a −5 y R28 de −14 a −13.
5. S4 (sincronización con el próximo release de ai-harness) entra antes de P3, con la regla de §5.1. §6 tiene la lista de fricciones sin reportar (9), que se reportan antes de S4.
6. D5, D9 y D13 decididas (§5.7). Efectos: R30 con el formato de D5 (M/L, partible), R29b nueva (XS, D9) y R31 nueva (S/M, D13 b, con cambio de contrato). §5.5 suma R25, R29b, R30 y R31.
7. §5.8: el costo de #32 ($0.85) estaba en su PR y no tenía fila. R1, S2 y la primera sesión de S3 siguen sin registro. Subtotales por fase: $88.91 hasta R19, y $99.52 con P2 ($10.61, la auditoría completa incluida). R10 (M/L) costó $7.79 y R17 (M) $6.12; R20 era M/L, por eso se parte.

### 5.10 Sincronizaciones con el harness

Una por PR propio, sin cambios de código (`/craft-backend:adoption sincronizar <tag>`). El análisis completo de `compare_baseline.py` va en el PR; aquí, el resumen.

| Fecha | Tag | PR | Entradas antes → después | Se mantienen | Cambian de clave | Nuevas (IDs) |
|---|---|---|---|---|---|---|
| 2026-11-22 | `v1.0.0-rc.1` | #31 | 36 → 47 | 29 | 7 → 18 (formato `#sección.paquete`, `n` por capa virtual) | ninguna |
| 2026-11-23 | `v1.0.0-rc.2` | #33 | 47 → 47 | 47 | ninguna | ninguna |
| 2026-11-25 | `v1.0.0-rc.3` | #45 | 31 → 42 | 31 | ninguna | 11: HEX-1.7, HEX-1.8, ERR-2.1, HEX-4.2, HEX-4.5 (nuevo), HEX-8.9, HEX-8.10, HEX-8.11 (nuevo), HEX-8.12 (nuevo) |
| 2026-11-26 | `v1.0.0-rc.4` | #56 | 29 → 35 | 29 | ninguna | 6: HEX-2.3, HEX-2.4, HEX-2.6, HEX-2.7, HEX-8.2, HEX-8.8 (ningún ID nuevo) |

#### S4: `v1.0.0-rc.4`

Sincroniza `tools/architecture` con `stacks/rust/template/tools/architecture` de ai-harness `v1.0.0-rc.4` (`8a2fa95`), sin tocar código de producción. Parte de `v1.0.0-rc.3` (S3), así que recibe los cambios de 1.0.0-rc.3.1 a 1.0.0-rc.3.8. Entre las dos, la plantilla cambia `tests/dependencies.rs`, `tests/errors.rs`, `tests/naming.rs` y `tests/common/mod.rs`, y suma los fixtures `core_imports/` y `mixed/` y casos nuevos en `legacy/`. `Cargo.toml`, `src/`, la cabecera de `baseline.txt` y `material-exempt.txt` no cambian.

**Medición:** sobre `main` del proyecto en `027813a` (después de R20a), no sobre la medición de ai-harness (`241502d`). Con `CRAFT_BACKEND_BASELINE_INIT=1`, la línea base se vació hasta la cabecera y se llenó con `ARCH_BASELINE_PRINT=1 cargo test -p architecture --no-fail-fast -- --nocapture`, volcado a `/tmp/baseline-entries.txt` y escrito con Write.

**Lo que el proyecto adapta:**
- `cargo fmt` con el `rustfmt.toml` del proyecto reagrupa los `use` de `common/mod.rs`, `errors.rs`, `output.rs`, `ports.rs` y `ratchet.rs` (solo los `use`: verificado con `diff` contra la plantilla), como en S1 a S3. Los fixtures no son miembros del workspace y quedan idénticos a la plantilla.
- `baseline.txt` conserva la línea propia de la cabecera ("Los IDs de hallazgo… están en docs/harness-adoption.md").
- `material-exempt.txt` es el del proyecto, sin tocar: las 2 exenciones de hoy (`err_1_application_errors_…` y `hex_4_event_subscribers_…`). Ninguna se renombra y `ARCH_REQUIRE_MATERIAL=1` no pide ninguna nueva.

**Cambios del CHANGELOG que podían mover entradas:** 1.0.0-rc.3.1 (imports del núcleo por archivo: entradas nuevas), 1.0.0-rc.3.3 (driving ve otros contextos legacy: podía sumar violaciones a una entrada), 1.0.0-rc.3.4 (`()` como una sola violación: podía bajar un conteo). Solo la primera mueve algo aquí: R16 quitó la dependencia `communities → iam` y R13b ya responde un `Response` vacío. 1.0.0-rc.3.2 (folds sin comodín) es una regla nueva que pasa en verde. 1.0.0-rc.3.5 a 1.0.0-rc.3.8 no tocan `tools/architecture`.

Resultado de `compare_baseline.py` (salida 0):
- **Se mantienen: 29.** Todas las que quedaban después de R20a, con la misma clave y la misma `n`.
- **Cambian de clave: ninguna.**
- **Desaparecen sin explicación: ninguna. Bajan de conteo: ninguna.**
- **Suben de conteo: ninguna.**
- **Nuevas: 6** (6 violaciones):

| Test | Ruta | Crate | ID | Rebanada |
|---|---|---|---|---|
| `hex_2_core_imports_only_std_and_the_kernel` | `contexts/communities/src/domain/model/community/community_id.rs` | `uuid` | HEX-2.6 | R22 |
| `hex_2_core_imports_only_std_and_the_kernel` | `contexts/communities/src/domain/model/community/owner_id.rs` | `uuid` | HEX-2.7 | R22 |
| `hex_2_core_imports_only_std_and_the_kernel` | `contexts/iam/src/domain/model/account/code_validation.rs` | `rand` | HEX-2.3 | R20b |
| `hex_2_core_imports_only_std_and_the_kernel` | `contexts/iam/src/domain/model/account/events/account_event.rs` | `chrono` | HEX-2.4 | R21 |
| `hex_2_core_imports_only_std_and_the_kernel` | `shared/src/domain/domain_event.rs` | `chrono` | HEX-8.2 | R21 |
| `hex_2_core_imports_only_std_and_the_kernel` | `shared/src/infrastructure/http/api_error_response.rs` | `serde` | HEX-8.8 | R25 |

Ningún ID nuevo: las 6 son violaciones que el inventario ya tenía por archivo y que el trinquete solo veía por manifiesto (VER-1.8, resuelto en el harness). `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --locked` (428 tests) y `ARCH_REQUIRE_MATERIAL=1 cargo test -p architecture` en verde.

**Rebanadas que cambian:**
- **S4:** se adelanta de después de R26 a después de R20a, antes de R20b. P3 ya no la espera.
- **R20b:** −1 (antes sin cambios).
- **R21:** −9 (antes −7). `Entity` queda como marcador vacío y no se borra (HEX-8.12, 1.0.0-rc.3.7).
- **R22:** −4 (antes −2).
- **R22b (nueva, propuesta, S/M):** `Field` cerrado con `Field::Owner(OwnerRejection)` (ERR-4.13, antes en R30). Sin cambios en la línea base ni en el contrato.
- **R25:** −3 (antes −2).
- **R26 y R30:** `unexpected(&f, r).with_code(codes::INTERNAL_ERROR)` de `edge/http` reemplaza a los `unexpected.rs` de cada contexto. El 500 queda sin `detail`: diferencia con D5 que decide el usuario (R30).

#### S3: `v1.0.0-rc.3`

Sincroniza `tools/architecture` con `stacks/rust/template/tools/architecture` de ai-harness `v1.0.0-rc.3` (`16e5255`), sin tocar código de producción (commit `50eb601`). Entre `v1.0.0-rc.2` y `v1.0.0-rc.3` la plantilla cambia `tests/` (con `fixtures/`) y agrega `material-exempt.txt`. `Cargo.toml`, `src/` y la cabecera de `baseline.txt` no cambian.

**Lo que el proyecto adapta:**
- `cargo fmt` con el `rustfmt.toml` del proyecto reagrupa los `use` de `common/mod.rs`, `errors.rs`, `output.rs`, `ports.rs` y `ratchet.rs`, como en S1 y S2. Los fixtures no son miembros del workspace y quedan idénticos a la plantilla.
- `baseline.txt` conserva la línea propia de la cabecera ("Los IDs de hallazgo… están en docs/harness-adoption.md").
- `material-exempt.txt` es el de la plantilla, solo con la cabecera. Las exenciones las escribe el usuario (abajo).

**Reglas nuevas que traen entradas** (CHANGELOG de 1.0.0-rc.2.1 a 1.0.0-rc.2.8): `hex_4_use_case_signatures_are_their_own_contract` y el dominio del kernel en `hex_4_use_case_contracts_…` (rc.2.6), `hex_1_driving_adapters_see_only_use_cases` (rc.2.7) y `hex_8_model_markers_have_no_items` (rc.2.8). Las demás reglas nuevas pasan en verde: `hex_8_capability_adapters_are_shared_infrastructure` (rc.2.5), `hex_6_outbound_port_types_live_in_dtos` (rc.2.7) y `rs_2_expect_names_the_invariant` (rc.2.8).

Con `CRAFT_BACKEND_BASELINE_INIT=1`, la línea base se vació hasta la cabecera y se llenó con `ARCH_BASELINE_PRINT=1 cargo test -p architecture --no-fail-fast -- --nocapture`, volcado a un temporal y escrito con Write. Resultado de `compare_baseline.py` (salida 0):
- **Se mantienen: 31.** Todas las que quedaban después de R13, con la misma clave y la misma `n`.
- **Cambian de clave: ninguna.**
- **Desaparecen: ninguna. Bajan de conteo: ninguna.**
- **Suben de conteo: ninguna.**
- **Nuevas: 11** (23 violaciones):

| Test | Clave | n | Causa | ID | Rebanada |
|---|---|---|---|---|---|
| `hex_1_driving_adapters_see_only_use_cases` | `contexts/iam/src/infrastructure/http/errors/error_mapper.rs` | 10 | nombra 10 códigos de `crate::domain::errors::error_codes` | HEX-1.7 | R19 |
| `hex_1_driving_adapters_see_only_use_cases` | `contexts/communities/src/infrastructure/http/errors/error_mapper.rs` | 2 | nombra `COMMUNITIES_INVALID_COMMUNITY_NAME` y `…_SLUG` de `crate::domain` | HEX-1.8 | R17 (o R19) |
| `hex_4_use_case_signatures_are_their_own_contract` | `contexts/communities/src/application/ports/inbound/create_community_use_case.rs` | 2 | `SystemError`; 2 argumentos además de `&self` | ERR-2.1, HEX-4.2 | R17 |
| `hex_4_use_case_signatures_are_their_own_contract` | `contexts/iam/src/application/ports/inbound/authenticate_account_use_case.rs` | 1 | `SystemError` | ERR-2.1 | R14 |
| `hex_4_use_case_signatures_are_their_own_contract` | `contexts/iam/src/application/ports/inbound/identify_account_use_case.rs` | 1 | `SystemError` | ERR-2.1 | R15 |
| `hex_4_use_case_signatures_are_their_own_contract` | `contexts/iam/src/application/ports/inbound/verify_account_use_case.rs` | 2 | responde `()`: falta `VerifyAccountResponse` en `commands/verify_account/`, y la firma no lo devuelve | HEX-4.5 (nuevo) | R13b (D11) |
| `hex_4_use_case_contracts_do_not_expose_domain_types` | `contexts/communities/src/application/ports/inbound/create_community_use_case.rs` | 1 | expone `VerifiedIdentity` (`shared::domain`) | HEX-4.2 | R17 |
| `hex_8_model_markers_have_no_items` | `shared/src/domain/aggregate_root.rs` | 1 | `type Event`, `record_event`, `pull_events`, `version` | HEX-8.9 | R21 |
| `hex_8_model_markers_have_no_items` | `shared/src/domain/domain_event.rs` | 1 | `event_type`, `occurred_at`, `aggregate_id`, `event_version` | HEX-8.11 (nuevo) | R21 |
| `hex_8_model_markers_have_no_items` | `shared/src/domain/entity.rs` | 1 | `type Id`, `id`, `same_identity_as` | HEX-8.12 (nuevo) | R21 |
| `hex_8_model_markers_have_no_items` | `shared/src/domain/value_object.rs` | 1 | `type Value`, `type Error`, `new`, `value` | HEX-8.10 | R21 |

**Diferencias con lo medido en R12** (sobre una copia de `development` antes de R13):
- Se mantienen 31 y no 32: R13 (2d16725) ya cerró la entrada `hex_4_handlers` de `verify_account.rs`.
- `hex_4_use_case_signatures_…` suma 6 violaciones y no 7, y en `verify_account_use_case.rs` cambia la causa. Antes de R13 eran `SystemError` y `VerifyAccountResponse` fuera de `commands/verify_account/`. R13 cerró el `SystemError` (ERR-2.1) y respondió `()` sin `response.rs`, y la regla cuenta el tipo que falta y la firma (HEX-4.5).
- `hex_1_driving_…` no ve HEX-1.4: el mapper de communities importa `iam::domain::errors::error_codes`, de otro crate, y la regla solo mira `crate::domain`. HEX-1.4 sigue en la entrada `hex_1` de HEX-1.1 (R16), que quita la dependencia.

**Exenciones de material (las escribe el usuario).** Con `ARCH_REQUIRE_MATERIAL=1`, tres conjuntos quedan vacíos (verificado con `--nocapture`: `revisó 0 …`):
- `err_1_application_errors_do_not_implement_std_error | errores declarados de puertos`: los 4 puertos salientes devuelven `Fault` (R7 a R10) y ninguno declara un error. Se borra si un puerto declara uno (ERR-3.4).
- `hex_4_event_subscribers_are_named_and_placed | implementaciones de un EventSubscriber en application (fuera de tests)`: no hay eventos que consumir; D4 retira el andamiaje.
- `hex_8_capability_adapters_are_shared_infrastructure | implementaciones de producción de capacidades del kernel`: nada implementa `IdGenerator`, `Clock` ni `FaultReporter` en producción. Se borra en R20, con `UuidIdGeneratorAdapter`.

Con las tres, `ARCH_REQUIRE_MATERIAL=1` pasa en verde (verificado: ningún otro conjunto vacío), y queda activada en CI en `53d6bd3`.

**Rebanadas que cambian:**
- **R13b (nueva, XS):** `VerifyAccountResponse` vacío (HEX-4.5, D11), −1.
- **R14 y R15:** −2 cada una (antes −1).
- **R16:** `failing_on` de la plantilla.
- **R16b (nueva, S, D12):** las 4 baterías con la forma async.
- **R17:** −4 (antes −1). Cierra `hex_4_use_case_signatures_…` y `hex_4_use_case_contracts_…`.
- **R19:** −2 (antes −1). Cierra `hex_1_driving_…`.
- **R20:** `infrastructure/uuid` de la plantilla; borra la tercera exención.
- **R21:** −7 (antes −3), M/L. Cierra `hex_8_model_markers_…`.
- **R23, R24:** la receta "Puerto todavía síncrono" (D12).
- La regla del commit de una firma falible cambia (§5.1).

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

## 6. Fricciones corregidas

| Fecha | Rebanada | Fricción | Efecto en la adopción | Estado en el harness |
|---|---|---|---|---|
| 2026-11-22 | S1 | `tools/architecture/tests/legacy.rs:45` de `v1.0.0-rc.1` no pasa clippy con `-D warnings` en `nightly-2026-11-19` (`unnecessary_to_owned`). | La copia "tal cual" diverge del template en una línea (§5.10). | Corregida en craft-backend 1.0.0-rc.2 (v1.0.0-rc.2, PR #17) |
| 2026-11-22 | S1 | `CRAFT_BACKEND_BASELINE_INIT=1` solo se lee al abrir la sesión, y nada lo advierte al empezar una sincronización. | Hubo que cortar S1 a la mitad y reabrir con `claude --continue`. | Corregida en craft-backend 1.0.0-rc.2 (v1.0.0-rc.2, PR #17) |
| 2026-11-22 | S1 | La cabecera del template dice que la línea base "se llena una sola vez", pero un cambio de formato de clave obliga a re-llenarla. El formato no cambió en `v1.0.0-rc.1`: cambió en 0.17.0 (`ruta/Cargo.toml` pasa a `ruta/Cargo.toml#sección.paquete`) y en 0.18.0 (en legacy, `n` cuenta las capas virtuales que ven la dependencia). S1 recibió los dos cambios juntos porque la copia anterior del proyecto era previa a 0.17.0. | Regla propia en §5.1 y análisis en §5.10. | Corregida en craft-backend 1.0.0-rc.2 (v1.0.0-rc.2, PR #17) |
| 2026-11-23 | — | `/craft-backend:adoption` abre el PR solo si se le pide, y no trae las convenciones del proyecto. | Se pasaron a §5.1 al retirar `/slice`. | Corregida en craft-backend 1.0.0-rc.2 (v1.0.0-rc.2, PR #17) |
| 2026-11-23 | R7 | Las baterías del template (`contracts/*.rs`) son `async fn` con un parámetro `run`, porque los puertos del template ya son `BoxFuture`. En una adopción con puertos todavía síncronos (RS-3 pendiente), la skill no dice qué forma darle a la batería. | La batería de R7 se escribió síncrona, sin `run`. R24 tiene que pasarla a `async` y agregar `run` a la macro y a sus dos invocaciones. | Resuelta en craft-backend 1.0.0-rc.3 (v1.0.0-rc.3): con el puerto todavía síncrono, la batería nace con su forma final (`async fn`, macro con `run: block_on` y `#![expect(clippy::unused_async)]`), receta "Puerto todavía síncrono" del README del stack (1.0.0-rc.2.3). Las 4 baterías de R7 a R10 siguen síncronas: R16b (D12) o R23/R24 |
| 2026-11-24 | R8 | El `Fake…Adapter` del template simula las fallas con `failing()`, que hace fallar **todas** las operaciones. Un handler que llama al puerto más de una vez (`find_by_email`, `find_by_username`, `save`) nunca llega a la segunda llamada con ese doble, así que el `?` de las llamadas posteriores queda sin test. El template no trae un doble que falle en una sola operación, y la skill `tdd` no dice cómo cubrir cada llamada. | En R7 se resolvió con un `StubCommunityAdapter` local (ba15189), a pedido del revisor. En R8, `FakeAccountAdapter` conserva el `failing()` del template y gana `failing_on(AccountOperation)`, que hace fallar una sola operación mientras las demás siguen el contrato (cc3d60c). Un primer intento con un `StubAccountAdapter` de respuesta fija devolvía la cuenta configurada para cualquier valor buscado, el mismo defecto de TDD-2.4, y no pasaba la batería; el revisor lo marcó. | Resuelta en craft-backend 1.0.0-rc.3 (v1.0.0-rc.3): `FakeCommunityAdapter::failing_on(CommunityOperation)` en la plantilla, y una prueba de falla por cada llamada a un puerto que puede fallar en el principio, la skill `tdd` y el revisor (1.0.0-rc.2.2). R16 lo adopta en communities |
| 2026-11-23 | R9 | Al volver falible la firma de un puerto, el handler necesita el `?` para compilar, pero el doble que falla (y su test) llega después, con la batería. La skill `tdd` no dice cómo ordenar los commits: con `?` en el refactor de firma, el historial muestra la producción antes que el test, y el rojo solo se ve tragando el `Fault` a mano. El revisor propone descartar el `Fault` de forma explícita (o `todo!`) en el commit de la firma y traer el `?` con el test rojo. | En R8 y R9 el rojo se vio por mutación en el árbol de trabajo y se anotó en el mensaje del commit. R10a siguió la propuesta: el commit de la firma (517e4cd) descarta el `Fault` de forma explícita, y el `?` llega con su test (65f455d), así que el rojo se reproduce contra el commit padre. | Resuelta en craft-backend 1.0.0-rc.3 (v1.0.0-rc.3): el commit de la firma falla fuerte con `.expect("pendiente: …")`, nunca descarta el error, y el `?` llega en el siguiente con su test en rojo contra el commit padre (1.0.0-rc.2.2); `rs_2_expect_names_the_invariant` admite `"pendiente:"` (1.0.0-rc.2.8). **Contradice la secuencia de R10a**, que el revisor de R9 propuso y R10a y R10b siguieron: el commit de la firma descartaba el `Fault` de forma explícita (517e4cd lo traducía al `TokenGeneratorError` de antes; 101f652 lo respondía como `Unauthenticated`), justo lo que la regla nueva prohíbe. Es historia y no se reescribe: ningún `Fault` quedó descartado al final de esas rebanadas. Desde S3 rige la regla nueva (§5.1) |
| 2026-11-23 | P1 | `ARCH_REQUIRE_MATERIAL=1` convierte en fallo un conjunto vacío, y la plantilla la activa porque ejercita cada regla. En una adopción, dos conjuntos quedan vacíos de forma legítima y permanente: los errores declarados de puertos (`err_1_application_errors_…`), que el propio harness elimina al pasar los puertos a `Fault`, y los `EventSubscriber` (`hex_4_event_subscribers_…`), si el proyecto no tiene eventos. La variable no admite excluir una regla. | R11 no puede activarla tal cual (§5.3, R11). | Resuelta en craft-backend 1.0.0-rc.3 (v1.0.0-rc.3): `tools/architecture/material-exempt.txt` exime un conjunto vacío para siempre, con motivo obligatorio, y pide borrar la exención si el conjunto deja de estar vacío (1.0.0-rc.2.1). Las escribe el usuario (§5.10, S3) |
| 2026-11-23 | P1 | Reglas expresables como test que `tools/architecture` no tiene: contratos con el dominio del kernel o un segundo argumento en `execute` (VER-1.3), `api → domain` en legacy (VER-1.4), firmas de los puertos entrantes (VER-1.5), marcadores sin ítems (VER-1.6). Antes: `expect("invariante: …")` (R9), adaptadores driving sin `ports::outbound` y DTOs en `ports/outbound/dtos/` (R10a, R10b), `package =` y `workspace = true` en los manifiestos (R6). | Las violaciones quedan sin trinquete y dependen del revisor. | Resuelta en craft-backend 1.0.0-rc.3 (v1.0.0-rc.3), con reglas nuevas: VER-1.3 y VER-1.5, `hex_4_use_case_signatures_are_their_own_contract` y el dominio del kernel en `hex_4_use_case_contracts_…` (1.0.0-rc.2.6); VER-1.4 y el adaptador driving sin `ports::outbound`, `hex_1_driving_adapters_see_only_use_cases`; los DTOs, `hex_6_outbound_port_types_live_in_dtos` (1.0.0-rc.2.7); VER-1.6, `hex_8_model_markers_have_no_items`, y `expect("invariante: …")`, `rs_2_expect_names_the_invariant` (1.0.0-rc.2.8). La de `package =` y `workspace = true` (R6) ya estaba en craft-backend 1.0.0-rc.1 (v1.0.0-rc.1, 85e68ca) |
| 2026-11-23 | S2 | El paso 4 de `sincronizar` pide escribir "una entrada por cada violación reportada", pero nada vuelca el informe al formato de `baseline.txt`. Además, `cargo test -p architecture` se corta en el primer binario que falla, y sin `--no-fail-fast` el informe queda incompleto. | Hubo que correrlo con `--no-fail-fast` y pasar el informe a entradas con un script propio, que también cuenta las `n` (§5.10). | Resuelta en craft-backend 1.0.0-rc.3 (v1.0.0-rc.3): `ARCH_BASELINE_PRINT=1` imprime las entradas con su conteo, y el llenado y `sincronizar` piden `--no-fail-fast` (1.0.0-rc.2.4). S3 las volcó sin script propio |
| 2026-11-24 | R11 | En Rust, cambiar la firma de un caso de uso a `Failure<{Verb}{Noun}Error>` hace que los tests nuevos sobre la variante no compilen hasta que el contrato existe: el único rojo previo es de compilación, que `tdd` excluye ("no por un error de compilación ajeno"). La skill no dice cómo ver el rojo de un cambio de contrato, ni de los tests que fijan comportamiento existente. | El rojo se vio por mutación de producción en el árbol de trabajo y se anotó en cada commit (3ff0ae1, d0dd8cb), como en R8 y R9. | Corregida en craft-backend 1.0.0-rc.4 (v1.0.0-rc.4, 1.0.0-rc.3.6): el contrato entra primero sin productor y con el brazo del `fold` en `pendiente:`, después el test del endpoint y por último el del handler en rojo por aserción; un test que fija comportamiento existente se ve en rojo por mutación anotada en el commit) |
| 2026-11-24 | R11 | El `fold` del template (`create_community.rs`) cae en `_ => codes::INVALID_REQUEST` (400) ante un `field` desconocido: inventa un código de cliente para algo que solo pasa si handler y borde se desincronizan, un bug (ERR-1). | R11 lo manda al 500 de D7, con tests, para no inventar un código en la fase 3. R13, R15 y R17 hicieron lo mismo (`an_unmapped_…_returns_500`). **Argumento (P2):** un `field` o `resource` que el borde no conoce no es un error del cliente. El cliente mandó una petición que el handler tradujo a una hoja con un nombre que el `fold` no mapea: es un defecto del servidor, y le corresponde un 500 (árbol B), que además se reporta. Un 400 genérico le dice al cliente que corrija algo que no puede corregir, y esconde el bug detrás de un código que parece de negocio. | Corregida en craft-backend 1.0.0-rc.4 (v1.0.0-rc.4, 1.0.0-rc.3.2): `Field` cerrado, el `fold` no compila ante un campo nuevo y se borra `codes::INVALID_REQUEST`. En el proyecto, R22b (ERR-4.13)) |
| 2026-11-25 | S3 | `hex_4_use_case_signatures_are_their_own_contract` no contempla un caso de uso sin datos: exige un `{Verb}{Noun}Response` aunque `execute` no devuelva nada, y ni las skills ni el README dicen si se responde `()` o un `Response` vacío. Una sola causa cuenta como 2 violaciones (el tipo que falta y la firma), y el mensaje dice ``responde `un tipo que no es una ruta` `` en lugar de nombrar `()`. | R13 respondió `()`, con el acuerdo del revisor, y S3 suma 2 violaciones en `verify_account_use_case.rs` (HEX-4.5, D11, R13b). | Corregida en craft-backend 1.0.0-rc.4 (v1.0.0-rc.4, 1.0.0-rc.3.4): la forma canónica es `pub struct {Verb}{Noun}Response;`, y `Result<(), …>` cuenta como una violación que nombra `()`. Coincide con D11 a) |
| 2026-11-25 | S3 | `MODEL_MARKERS` de `tools/architecture` incluye `Entity`, pero el `model.rs` del kernel de la plantilla solo define `ValueObject`, `AggregateRoot` y `DomainEvent`. No queda claro si `Entity` es un marcador admitido o un resto que hay que borrar. | HEX-8.12 propone borrarlo en R21, siguiendo la plantilla. | Corregida en craft-backend 1.0.0-rc.4 (v1.0.0-rc.4, 1.0.0-rc.3.7): `Entity` e `Identifier` son marcadores admitidos; un proyecto que usa `Entity` lo conserva sin ítems. R21 lo deja vacío y no lo borra) |
| 2026-11-25 | S3 | En legacy, `hex_1_driving_adapters_see_only_use_cases` solo mira `crate::domain`: un adaptador driving que nombra el dominio de otro contexto (`iam::domain::…` en el mapper de communities) pasa. Lo cubre solo `hex_1_dependencies_point_inward`, por el `Cargo.toml`. | Ninguno hoy: HEX-1.4 sigue en la entrada `hex_1` de HEX-1.1, y R16 quita la dependencia. | Corregida en craft-backend 1.0.0-rc.4 (v1.0.0-rc.4, 1.0.0-rc.3.3): también falla si un adaptador driving nombra el dominio o un puerto saliente de otro contexto legacy. Sin entradas aquí: R16 quitó la dependencia) |
| 2026-11-25 | R13b | El hook `post-edit` corre los tests de arquitectura después de cada archivo editado. Al crear `response.rs` antes de cambiar la firma del puerto, bloqueó con la violación que el paso siguiente iba a cerrar: un cambio que abarca varios archivos pasa por estados intermedios en rojo. | Ninguno: el siguiente paso lo corrigió. Pero el aviso se lee como un error del cambio, no como un estado intermedio. | Corregida en craft-backend 1.0.0-rc.4 (v1.0.0-rc.4, 1.0.0-rc.3.8): el aviso de `post-edit` dice que, si un paso siguiente del mismo cambio cierra la violación, se siga sin hacer commit) |
| 2026-11-26 | R17 | Un value object que el dominio rechaza por varios motivos, cada uno con su código de cliente (D2 a: `OwnerId` nil o mal formado), no cabe en la hoja `Validation { field }` de la plantilla, y las skills no dicen si se agrega un motivo a la hoja o una hoja propia. | R17 agregó la hoja local `InvalidOwner { Nil, WrongFormat }` (ver ERR-4.13). | Corregida en craft-backend 1.0.0-rc.4 (v1.0.0-rc.4, 1.0.0-rc.3.2): el motivo va en la variante del campo (`Field::Owner(OwnerRejection)`), no en una hoja propia. En el proyecto, R22b) |
| 2026-11-26 | R17 | Ningún test de `tools/architecture` verifica que el `fold` de un adaptador driving sea exhaustivo: un comodín `_ =>` sobre el enum del contrato pasaría (propuesta del revisor de R17). | Lo fijan hoy solo los tests de endpoint por brazo. | Corregida en craft-backend 1.0.0-rc.4 (v1.0.0-rc.4, 1.0.0-rc.3.2): `err_4_driving_folds_are_exhaustive`, en verde aquí. VER-1.9 resuelto) |
| 2026-11-26 | P2 | En legacy, `hex_2_core_depends_only_on_std_and_the_kernel` cuenta por manifiesto: un `use` nuevo de un crate prohibido en domain o application no cambia ninguna entrada si el paquete ya figura en la línea base (VER-1.8). | `use uuid::Uuid` entró en `owner_id.rs` en R16 sin que fallara nada (HEX-2.7), y lo encontró recién la auditoría de P2. | Corregida en craft-backend 1.0.0-rc.4 (v1.0.0-rc.4, 1.0.0-rc.3.1): `hex_2_core_imports_only_std_and_the_kernel`, una entrada por archivo. S4 suma 6. VER-1.8 resuelto) |
| 2026-11-26 | P2 | El `unexpected()` de `edge/http` de la plantilla responde `Problem::internal()` sin `code`. Un proyecto que conserva `code` en el 500 (D5: `INTERNAL_ERROR`) tiene que duplicar `unexpected()` en cada contexto, con el riesgo de que alguna copia agregue un detalle del `Fault`. | R30 proponía un `unexpected()` propio en cada `{ctx}-api`, además de los `unexpected.rs` interinos de R11 y R17. | Corregida en craft-backend 1.0.0-rc.4 (v1.0.0-rc.4, 1.0.0-rc.3.5): `unexpected(&fault, reporter)` devuelve `UnexpectedProblem`, que solo admite `.with_code(…)`. R26 y R30 usan `unexpected(&f, r).with_code(codes::INTERNAL_ERROR)` en lugar de los `unexpected.rs` de cada contexto. Queda abierto para el usuario: sin `detail`, el 500 pierde el mensaje de D7 (ver R30) |

### Sin reportar en P2, ya corregidas en craft-backend 1.0.0-rc.4

1. *(Corregida en craft-backend 1.0.0-rc.4.)* **El hook `post-edit` en estados intermedios (R13b).** Corre los tests de arquitectura después de cada archivo editado y bloquea con una violación que el paso siguiente del mismo cambio va a cerrar. Propuesta: que el aviso diga que puede ser un estado intermedio, o que el bloqueo se aplique al cerrar el turno y no en cada edición.
2. *(Corregida en craft-backend 1.0.0-rc.4.)* **`()` en `hex_4_use_case_signatures_are_their_own_contract` (S3).** No contempla un caso de uso sin datos: cuenta 2 violaciones por una sola causa y dice ``responde `un tipo que no es una ruta` `` en lugar de nombrar `()`. El proyecto lo resolvió con un `Response` vacío (D11 a), pero el harness tiene que decir cuál de las dos formas es la canónica.
3. *(Corregida en craft-backend 1.0.0-rc.4.)* **`Entity` sin definir en la plantilla (S3).** `MODEL_MARKERS` lo admite, pero el `model.rs` del kernel no lo trae. O se agrega a la plantilla como marcador, o se saca de `MODEL_MARKERS`.
4. *(Corregida en craft-backend 1.0.0-rc.4.)* **`hex_1_driving_adapters_see_only_use_cases` ciego a otros contextos (S3).** En legacy solo mira `crate::domain`: un adaptador driving que nombra `iam::domain::…` pasa. Propuesta: que también cuente el dominio de otros contextos del workspace.
5. *(Corregida en craft-backend 1.0.0-rc.4.)* **La hoja `InvalidOwner` (R17).** Un value object con varios motivos de rechazo, cada uno con su código de cliente, no cabe en `Validation { field }`, y las skills no dicen si se agrega un motivo cerrado a la hoja o una hoja propia (ver ERR-4.13).
6. *(Corregida en craft-backend 1.0.0-rc.4.)* **Ningún test contra el comodín `_ =>` en los `fold` (R17, VER-1.9).** Un `fold` con comodín sobre el enum del contrato pasa todos los tests de arquitectura; hoy solo lo fijan los tests de endpoint por brazo.
7. *(Corregida en craft-backend 1.0.0-rc.4.)* **El código genérico para nombres desconocidos (R11).** El `fold` de la plantilla responde `_ => codes::INVALID_REQUEST` (400) a un `field` desconocido. Es un defecto del servidor (handler y borde desincronizados), no del cliente: le corresponde un 500 del árbol B, que además se reporta. Ver el argumento en la fila de R11.
8. *(Corregida en craft-backend 1.0.0-rc.4.)* **El rojo de un cambio de contrato (R11).** No estaba en el encargo de P2, pero tampoco se reportó. En Rust, los tests nuevos sobre la variante no compilan hasta que el contrato existe, y `tdd` excluye el rojo de compilación. La skill no dice cómo ver el rojo, ni el de los tests que fijan comportamiento existente. El proyecto lo resuelve por mutación anotada en el commit.
9. *(Corregida en craft-backend 1.0.0-rc.4.)* **`hex_2_core` cuenta por manifiesto en legacy (P2, VER-1.8).** Un `use` nuevo de un crate prohibido en domain o application pasa en verde si el paquete ya tiene entrada. Propuesta: un test de imports por capa virtual en legacy, con línea base por archivo.

## Método

- Se leyó entero cada archivo `.rs` y `Cargo.toml` versionado, y cada `archivo:línea` se verificó contra su contenido en `d0b0eeb`.
- No se ejecutaron `cargo test` ni `clippy` (auditoría en solo lectura). El repo no tiene tests de arquitectura que correr.
- El orden de los commits se revisó con `git log --reverse`.
