---
description: Implementa una rebanada del plan de adopción de craft-backend
argument-hint: <Rn> [notas específicas]
---

Rebanada e instrucciones adicionales: $ARGUMENTS

## Antes de empezar
1. git switch main && git pull.
2. Lee docs/harness-adoption.md entero (inventario y §5). Confirma en §5.3 que todas
   las dependencias de la rebanada (§5.4) están marcadas como hechas y que su código
   está en main. Si no, detente y avísame.
3. Si la rebanada depende de una decisión D<n> que no está "decidida" en §5.7,
   detente y pregúntame.
4. Crea una rama nueva desde main.

## Durante
- Implementa solo el alcance de la rebanada según §5.3, más las instrucciones
  adicionales de arriba. No adelantes trabajo de otras rebanadas.
- TDD: cada cambio de comportamiento empieza con un test en rojo, y confirmas el rojo
  antes de implementar. En los refactors puros, el rojo es el trinquete ("Ya no
  violan: borrar").
- baseline.txt solo se achica. Si un archivo que mueves o renombras tiene violaciones
  pendientes, o aparece una violación nueva, detente y avísame. Nunca agregues líneas.
- app/tests/ no se modifica, salvo los cambios de contrato de §5.5 para esta
  rebanada. account_lifecycle.rs admite solo adaptación mecánica, enumerada en el PR.
- Si la rebanada lo indica, quita los #[allow(...)] que apuntan a sus hallazgos.
- Commits en inglés con Conventional Commits.

## Al terminar
1. cargo fmt --check, cargo clippy --workspace --all-targets -- -D warnings y
   cargo test --workspace --locked, en verde.
2. Inventario (resuelto (<commit>)), §5.3 (rebanada hecha y su resultado), §5.6 y
   §5.5 si cambió algún contrato.
3. architecture-reviewer sobre el diff. Aplica solo lo que sea de esta rebanada; lo
   demás se anota en §5.3.
4. PR con resumen, tests nuevos con su rojo, entradas eliminadas de la línea base y
   cambios de contrato.
5. gh pr view --json baseRefName debe decir main.
6. Fila de la rebanada en §5.8 con Tokens: pendiente.
7. Repórtame en el formato de siempre: comprobaciones previas, commits, rojo/verde,
   revisor y desviaciones respecto de lo pedido.
