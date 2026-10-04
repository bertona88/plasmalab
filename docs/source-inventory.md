# Context coverage and missing sources

Inventory date: 2026-10-04. This repository does **not** contain the full project conversation history or all earlier prototype artifacts. The [project context](project-context.md) is a synthesis, not a complete archive.

The assistant preparing the foundation had the two supplied documents, the repository-setup conversation, and partial summaries/excerpts of some earlier project chats. Retrieval of the full earlier chats was unavailable. This inventory transfers those known references without implying that their original contents were reviewed.

## Source coverage

| Source                                     | Available basis                                      | Preserved context or remaining gap                                                                                                                                                                                        |
| ------------------------------------------ | ---------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [The ladder](ladder.md)                    | Full supplied document, preserved                    | Direction, architectural commitments, rung gates, and deferred scope.                                                                                                                                                     |
| [SELFS abstract](SELFS_WOFI_abstract.md)   | Full supplied document, preserved                    | Conceptual origin, operational predictive identity, proposed comparison, and limits of the hypothesis.                                                                                                                    |
| Repository-setup discussion, 2026-10-04    | Direct conversation used to create this repo         | Decisions distilled in [decision 0001](architecture/0001-foundation.md), the [project context](project-context.md), and contributor guidance. The oscillator was an assistant proposal.                                   |
| “Computational Architecture Patterns”      | Referenced by the ladder; full chat unavailable      | The ladder attributes technology ownership and the three-architecture progression to it. Earlier alternatives and detailed reasoning have not been recovered.                                                             |
| “Plasma Pattern Zoology”                   | Referenced by the ladder; full chat unavailable      | The ladder preserves the distinction between physics mode and a deliberately simplified, CA-like toy universe. Specific patterns, rules, and experiments remain unknown here.                                             |
| “GPU Plasma Simulation Basics,” 2026-10-03 | Partial conversation excerpts in project context     | Discussion moved from an electron perspective toward collective identity, continuity, and a SELFS abstract. The title alone does not establish a chosen GPU algorithm or validated implementation.                        |
| “Sensazioni di un elettrone,” 2026-10-03   | Short conversation excerpt in project context        | The electron perspective emphasized receiving a field and responding, extending the action/response analogy across scales. The full discussion is not archived.                                                           |
| “Electron Experience Imagined,” 2026-10-03 | Partial conversation excerpts in project context     | Electron POV and extreme-condition thought experiments motivated a request for a new simulation framework. The framework response is not available in these excerpts; specific numerical capabilities cannot be inferred. |
| “Develop New HEDP Algorithm,” 2026-10-03   | A request and artifact references in project context | A HEDP algorithm exploration produced references to the files below. Their code, methods, outputs, and validation have not been inspected for this repository.                                                            |

These are the sources known from the available context, not an exhaustive list of the user’s project chats or documents.

## Referenced artifacts not imported

- `PAN_RAD_prototype.zip`
- `pan_rad.py`
- `benchmark_results.json`
- `verification_results.json`

These names came from the HEDP conversation summary. They are not present in this repository or among the supplied files used for the foundation. Their names do not establish the model, numerical method, accuracy, performance, or relationship to SELFS. In particular, the relationship between that prototype and the ladder remains to be examined.

## What a new contributor can and cannot recover

The repo is enough to recover the chosen product direction, scientific ownership, staged authority for SELFS, current implementation status, and a proposed first slice. This inventory also preserves the additional source references known to the assistant.

It is not enough to reconstruct all earlier experiments, discarded approaches, numerical choices, prototype results, or the full reasoning in the original chats. Those gaps also limit the assistant working from the available summaries. Adding more confident prose would not restore missing evidence.

## Completing the handoff

Import the missing chat exports and original prototype files when provided. For each source, capture its title/date and extract the useful question, actual decisions, alternatives considered, assumptions, proposed experiments, unresolved issues, and any changes to earlier direction. Distinguish the user’s commitments from assistant suggestions and abandoned sketches.

For prototypes, preserve the original artifact and provenance, identify the model and dependencies, inspect the code and result-generation procedure, and reproduce relevant checks before promoting any claim into [evidence](evidence/README.md). Keep reported results distinguishable from independently reproduced results.

The goal is a self-contained project handoff: enough reasoning and reproducible material to continue the work without access to the original chats. It does not require copying every conversational turn. Keep this inventory current as sources are recovered, and link extracted decisions to their basis.
