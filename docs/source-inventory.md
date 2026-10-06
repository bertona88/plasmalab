# Context coverage and missing sources

Inventory date: 2026-10-04. This repository does **not** contain the full project conversation history or all earlier prototype artifacts. The [project context](project-context.md) is a synthesis, not a complete archive.

The assistant preparing the foundation had the two supplied documents, the repository-setup conversation, and partial summaries/excerpts of some earlier project chats. Retrieval of the full earlier chats was unavailable at that time. The 2026-10-04 reviews recovered five turns from “Electron Experience Imagined,” including the complete framework response within that returned window, five available turns from “Plasma Pattern Zoology,” and four turns from “Sensazioni di un elettrone.” The subsequent GPU/PIC conversation and supplied-attachment review extends current coverage as recorded below; it does not retroactively change the earlier reviewers’ retrieval windows.

## Source coverage

| Source                                     | Available basis                                                                      | Preserved context or remaining gap                                                                                                                                                                                                                                                                                                                                |
| ------------------------------------------ | ------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [The ladder](ladder.md)                    | Full supplied document, preserved                                                    | Direction, architectural commitments, rung gates, and deferred scope.                                                                                                                                                                                                                                                                                             |
| [SELFS abstract](SELFS_WOFI_abstract.md)   | Full supplied document, preserved                                                    | Conceptual origin, operational predictive identity, proposed comparison, and limits of the hypothesis.                                                                                                                                                                                                                                                            |
| Repository-setup discussion, 2026-10-04    | Direct conversation used to create this repo                                         | Decisions distilled in [decision 0001](architecture/0001-foundation.md), the [project context](project-context.md), and contributor guidance. The oscillator was an assistant proposal.                                                                                                                                                                           |
| “Computational Architecture Patterns”      | Direct architecture chat plus three supplied excerpts reviewed                       | User choice of Architecture 3 and milestones, initial data/identity patterns, alternatives and original illustrative Rust declarations in [architecture boundaries](architecture/README.md#recovered-alternatives-and-design-sketches). Proposals remain distinct from adopted direction and implemented APIs.                                                    |
| “Plasma Pattern Zoology,” 2026-10-04       | Earlier five turns plus three supplied excerpts reviewed                             | Candidate identity families and diagnostic proposals are distilled in [project context](project-context.md#plasma-pattern-zoology-candidate-identity-not-just-recurring-shapes); toy rules, alternative model routes, and open experimental choices in [plasmoid ecology](experiments/plasmoid-ecology.md). No code or verified results recovered.                |
| “GPU Plasma Simulation Basics,” 2026-10-03 | Visible conversation plus 144-line resonance attachment reviewed                     | User scope and SELF motivation in project context; [driven-plasma tests](experiments/driven-plasma-selfs.md) and [micronozzle/WarpX investigation](experiments/micronozzle.md). No deck, measured runtime or verified speedup recovered.                                                                                                                          |
| “Sensazioni di un elettrone,” 2026-10-03   | Earlier four turns plus 41-line assistant attachment reviewed                        | User framing, assistant interpretations, philosophical questions, and model limits distilled in [the POV context](project-context.md#recovered-conversation-interaction-response-and-reciprocity). Review scope is recorded below; no full transcript is archived.                                                                                                |
| “Electron Experience Imagined,” 2026-10-03 | Earlier five turns plus 378-line framework attachment reviewed                       | IRSF mapping and axes, competing binding hypotheses, conceptual comparison sequence, renderer and validity-limit ideas distilled in [project context](project-context.md#recovered-irsf-proposal-meaning-and-open-definitions). Assistant speculation, with no implementation or numerical results in the returned material. Earlier context remains unavailable. |
| “Develop New HEDP Algorithm,” 2026-10-03   | Complete earlier report text visible in architecture chat; original code unavailable | [PAN-RAD report](evidence/pan-rad-reported.md): model, conservative transfers, adaptive neighborhoods, original pseudocode/run instructions, reported checks, and negative fixed-block comparison. The original artifacts remain missing; no result independently reproduced.                                                                                     |

These are the sources known from the available context, not an exhaustive list of the user’s project chats or documents.

## Electron Experience review coverage

Reviewed conversation ID: `6ac16e66-e678-83ed-9ff9-0aee14c9907f`, title “Electron Experience Imagined.” The conversation reader returned four user/assistant exchanges dated 2026-10-03 (black-hole POV, gravity, richness of experience, and the IRSF request/response) and the 2026-10-04 repository-review request/handoff. It reported no further cursor. This describes the returned window, not proof that the whole original history was recovered; the earlier electron/laser/capillary context alluded to in those turns was not returned.

The latest handoff mentioned SELFS, the ladder, plasma SELF zoology, and a CA-like plasmoid toy idea. It provided references rather than the original zoology discussion or toy rules/code. Those references do not extend the reviewed coverage of “Plasma Pattern Zoology” or “Computational Architecture Patterns.”

The available local project reference files were `sources/ladder.md` and `sources/SELFS_WOFI_abstract.md`. Both were read and compared byte-for-byte with their existing `docs/` copies; they already match and were left intact. They are supplied project documents, not independently recovered attachments to this conversation. No additional attachment content or executable artifact was available through the retrieved conversation. No code or numerical results were recovered or independently reproduced in this review.

The review compared these sources with `AGENTS.md`, the ladder, project context, source inventory, foundation architecture, roadmap, first-experiment brief, evidence guidance, Rust result/requirement definitions, and browser shell. The useful addition is conceptual provenance in the existing project context; it changes neither scientific laws nor implementation status. No new literature or novelty review was performed for IRSF.

## Plasma Pattern Zoology recovery, 2026-10-04

Source: [“Plasma Pattern Zoology”](https://chatgpt.com/c/6ac1f58e-f7a8-83eb-aa9e-ddf220617368), conversation ID `6ac1f58e-f7a8-83eb-aa9e-ddf220617368`. This private source locator preserves provenance; the distilled repository material can be used without access to it.

The conversation retrieval returned five turns, with no further page cursor:

- `c42df8d5-ef8d-4e1e-a4a8-28acdf4746a7`: user asks about a cellular-automaton resemblance; assistant sketches a cascade, local toy rules, and toy versus MHD routes.
- `54a1ba6c-e4ea-45b5-a95c-8b9434ef39eb`: user asks about solar plasmoids; assistant reports observations and discusses recursive sheets/islands and 2D versus 3D interpretation.
- `bc7067c9-b04e-4465-8de4-ad5b3d917f9e`: user invokes “operation sundiver”; assistant assumes Brin's novel as the reference and distinguishes organized structure from life/intelligence. That interpretation was not confirmed by the user.
- `d71d60aa-5952-4af5-a5bc-e751671d785d`: user asks for the zoology of SELFS; assistant proposes identity-carrier families, a diagnostic profile, and an illustrative detector/history display.
- `7b0d6bd2-15e1-409a-9277-ea8094fac70d`: repository-integration request and assistant handoff response. The response alone supplies no evidence of a completed repo contribution.

The first returned turn selects “spawn smaller plasmoids recursively” from an earlier answer whose original text was not returned. Exhausting this retrieval does not establish that the complete original conversation was reviewed. The cached preview was shorter than the retrieved zoology response; the latter supplied the additional families and diagnostic discussion.

The attachment list was empty. No conversation uploads were exposed by this retrieval. The two read-only project files available locally, `sources/ladder.md` and `sources/SELFS_WOFI_abstract.md`, were reviewed and their SHA-256 hashes matched the repository copies; they required no duplicate import. No executable code, prototype, result file, or dataset appeared in the recovered turns.

The assistant's embedded citation placeholders and solar image carousel were visible as references, but their target papers, images, and underlying data were not retrieved or reviewed. In particular, the reported 2025 twisted-coronal-plasmoid observation (at least approximately 55 km/s, less than five minutes) remains an unverified source claim here, not numerical evidence for PlasmaLab. The arbitrary diagnostic tuples, inspector numbers, and particle-to-blob compression illustration were not measurements. Recover citation targets and original artifacts before relying on any of these as scientific evidence.

The review preserves the user's questions separately from assistant proposals. The proposed taxonomy and experiment option do not adopt a new solver, reorder the oscillator-first roadmap, or override the ladder's deferral of a comprehensive taxonomy. The current repository still has no implemented detector or simulation; no chat-reported physical or performance result was independently reproduced by this review.

## Electron conversation review, 2026-10-04

The review read the latest repository's contributor guidance, ladder, project context, and inventory before inspecting the architecture, roadmap, experiment/evidence briefs, Rust core, and browser shell. It fetched and reread `main` immediately before editing. The recovered conversation adds context and provenance; it does not change the ladder, technology ownership, or implementation status.

| Material                                                                                                                                                                                | Access and review outcome                                                                                                                                                                                                                                                                                                        |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Conversation `6ac16ae5-2c08-83ed-acff-65fc2fd2490c`, “Sensazioni di un elettrone”                                                                                                       | Direct retrieval returned four turns: three electron discussion exchanges and one repository-review request/handoff exchange. All returned messages were read; retrieval supplied no older-page cursor and reported no attachments. This describes the accessible record, not proof that no other history or files ever existed. |
| `SELFS_WOFI_abstract.md` and `ladder.md`                                                                                                                                                | Full files read from the read-only project mirror and compared byte-for-byte with the repository copies; both matched. Already preserved, so no duplicate import or source revision was needed.                                                                                                                                  |
| `/mnt/data/Sensazioni di un elettrone.txt`                                                                                                                                              | Named by the review request, but `/mnt/data` is absent on this host and this export was not in the project mirror. Conversation retrieval was used instead; the export was not reviewed or compared with the retrieved record.                                                                                                   |
| `/mnt/data/GPU Plasma Simulation Basics.txt`, `/mnt/data/Electron Experience Imagined.txt`, `/mnt/data/Plasma Pattern Zoology.txt`, `/mnt/data/Computational Architecture Patterns.txt` | Named by the review request, but absent from the supplied project mirror and inaccessible at the stated paths. This contribution did not review their original contents; each conversation’s current repository coverage is recorded above.                                                                                      |
| `/mnt/data/SELFS_WOFI_abstract.md` and `/mnt/data/ladder.md`                                                                                                                            | The stated paths were inaccessible. The matching project-mirror files above were reviewed instead; identity with the inaccessible `/mnt/data` copies was not established.                                                                                                                                                        |

The repository contains the useful conceptual distillation with separate user and assistant attribution. No code, prototype, concrete numerical experiment, or benchmark appears in the retrieved electron exchanges. The later assistant handoff's references to other experiments or attachments do not establish their availability, contents, or results. The physics qualification linked in the project context was added during this review, rather than attributed to the original chat.

## GPU conversation and supplied-attachment review, 2026-10-04

This review used the visible conversation and all supplied text excerpts, rather than claiming access to complete sibling chats. It rechecked the current repository before writing and preserved the concurrent IRSF, zoology and electron-POV contributions. The earlier review-window records above remain historical statements about what those reviewers could access; an empty attachment list in their retrieval does not imply no attachments were supplied here.

### GPU Plasma Simulation Basics and the visible conversation

The visible 2026-10-03 conversation covers GPU/PIC performance questions, the user's ultrashort/small-volume scope choice, micronozzle/HF questions, response communities, rave/crowd analogies, SELFS, resonance, scale/POV discussion and the WOFI abstract request. This is substantially more than the foundation's excerpts, not proof of complete project history. Interrupted responses remain fragments.

The supplied `GPU Plasma Simulation Basics.txt` is only the 144-line assistant **polyrhythm/resonance answer**, opening “Yes—the useful version of this is a polyrhythm.” It is not an export of the whole chat. The visible user turns separately establish attribution of the scope and resonance questions. The [shared-response brief](experiments/driven-plasma-selfs.md), [micronozzle brief](experiments/micronozzle.md) and project context retain the useful reasoning, comparison designs and limits.

No paper, vendor price or literature speedup was independently verified during this review. Retained public links are original research leads, not a completed literature review. The micronozzle settings are earlier assistant-reported values awaiting checking against the source and exact inputs. No EPOCH/WarpX deck, timing log, phase-locking result or simulation output was supplied. Historic cost estimates and conditional scaling arithmetic are not benchmarks or spending authorization.

### Computational Architecture Patterns

Three separately supplied, same-title excerpts were readable:

1. The 284-line “I'd add WebGPU immediately” answer: stack, resident buffers, worker, graph/FFT/audio and deferred CPU threads. This is the mounted `Computational Architecture Patterns.txt`.
2. The 551-line Lean/Serious/Moonshot answer: alternative architectures, illustrative Rust interfaces, multirate sketch and predictive entity state.
3. The 15-line ladder-creation handoff: Architecture 3 as destination and a **then-reported** inability to push. That historical statement does not describe the current repository.

All three excerpts were reviewed, but the full sibling chat and intervening user decisions were not recovered. [Architecture boundaries](architecture/README.md#recovered-alternatives-and-design-sketches) retain alternatives and the useful original Rust declarations with instructions and limitations. They are non-runnable design sketches with undefined types, not an imported crate, GPU kernel or prototype. Earlier dependency and scheduling suggestions do not override the ladder.

### Other supplied attachments and duplicate filenames

The full supplied `ladder.md` and `SELFS_WOFI_abstract.md` match their repository copies by Git blob hash (`91f95f5e336e23c7231d55f35bf3c3eec6f4d886` and `9d40a4a1732b5cd5a6f81e7e2a49e46bae7197f1`). They were left intact. No WOFI submission receipt was recovered; the abstract's preserved draft status is not a live service audit.

The 41-line `Sensazioni di un elettrone.txt` is an assistant excerpt developing interaction/response/feedback, not a complete sibling chat or numerical specification. The 378-line `Electron Experience Imagined.txt` supplies the IRSF framework response already distilled by the concurrent contribution; it adds no executable code or calibrated results. Their conceptual distinctions are preserved in the existing POV/IRSF discussion rather than duplicated in new documents.

Three zoology excerpts were also readable: the 70-line CA/cascade proposal (the mounted `Plasma Pattern Zoology.txt`), 33-line Sundiver response, and 375-line SELF taxonomy response. They corroborate material already integrated by the zoology contribution; no duplicate experiment document was added. Some public citation URLs are now visible, but the source papers, solar images and underlying data were not reviewed. Earlier reported observations and arbitrary example scores remain unverified.

A repeated display filename does not identify interchangeable source bytes. For architecture and zoology, one excerpt per title was mounted, while other excerpts were supplied as readable text. No raw backing-file identity or complete export is claimed for those other excerpts. The repository preserves useful synthesis and original illustrative declarations, not a purported full chat archive or unrelated personal dialogue.

The missing HEDP prototype files listed below were not supplied in this review either. No scientific prototype or reported-result dataset was imported, no physics simulation was run, and no 10×–100× result was independently reproduced. Implementation status and the first-slice priority remain unchanged.

## Architecture chat review coverage

Reviewed on 2026-10-04: the visible architecture discussion from the user's architecture-pattern question through the stack/options discussion, explicit choice of Architecture 3, high-level `ladder.md` request, file-creation handoff, and this repository-review request. This supplies direct evidence of the user's destination/milestone decision. The earlier recommendations, trait/struct sketches, library list, model menus, and numerical schedules are assistant proposals, not individually adopted or implemented choices.

The current Files listing contained 12 entries: the SELFS abstract, two ladder entries (generated and supplied), three same-title architecture-response excerpts, three same-title zoology excerpts, and one response each for GPU Plasma Simulation Basics, Sensazioni di un elettrone, and Electron Experience Imagined. Their complete available text was reviewed through the supplied context, Files, and mounted copies as applicable; repeated display names are not complete parent-chat exports. The generated ladder text agrees with the preserved direction; mounted ladder/abstract Git blob hashes match the existing source documents. No duplicate source import was needed.

This review compared the attachments with the concurrent POV, zoology, [driven-plasma](experiments/driven-plasma-selfs.md), and [micronozzle](experiments/micronozzle.md) contributions and retained them. Its duplicate driven-experiment draft was discarded rather than adding a parallel document. The [architecture addition](architecture/README.md#direct-user-choice-and-the-initial-pattern-proposals) retains the direct choice and initial computation/identity rationale not available to the preceding excerpt-only review. The parent-chat turn counts and private locators above remain those reviewers' recorded scope; this pass did not independently retrieve their complete histories or the full repository-setup chat.

The complete assistant report previously surfaced as `Develop New HEDP Algorithm.txt` remains readable in this architecture conversation's earlier context. It describes PAN-RAD, gives pseudocode and reported tables, and claims an earlier run. A fresh Files read could not retrieve that source file, and it is absent from the current listing and mounted files. The [historical report](evidence/pan-rad-reported.md) therefore distills visible report text; it is **not an import of original code, ZIP, tests, or result JSON**. Its pseudocode and original run commands are preserved with limitations. No numerical claim was independently rerun.

The zoology excerpts also expose citation targets missing from the earlier placeholder-only retrieval: a [coherent edge/SOL structure review](https://www.cambridge.org/core/journals/journal-of-plasma-physics/article/abs/recent-theoretical-progress-in-understanding-coherent-structures-in-edge-and-sol-turbulence/8D63C9858F71611B114B7516FC4CF8EC), a [flux-rope paper](https://agupubs.onlinelibrary.wiley.com/doi/full/10.1029/2008JA013660), a [dusty-plasma vortex review](https://www.cambridge.org/core/journals/journal-of-plasma-physics/article/review-on-the-vortex-and-coherent-structures-in-dusty-plasma-medium/C4F2BEB9F72D502A56682AF0AB599E1C), and an [NSO coronal-observation link](https://nso.edu/blog/30-hours-worth-of-cona-science-data-released/). These are recovered links only, not reviewed papers, images, or data, and do not verify the earlier solar-speed/lifetime claim. No new literature/novelty review or scientific validation was performed. Source direction, first-slice priority, dependencies, and runtime are unchanged.

## Referenced artifacts not imported

- `PAN_RAD_prototype.zip`
- `pan_rad.py`
- `benchmark_results.json`
- `verification_results.json`
- `test_pan_rad.py` and `requirements.txt`, named by the recovered run instructions

The foundation knew the first four names only from a summary. The recovered PAN-RAD response now supplies a reported method, comparisons, and run instructions, but none of these original artifact bytes or a verified result-generation environment was available in the architecture review. The method and negative result are preserved as [reported context](evidence/pan-rad-reported.md); their existence in prose is not independent verification. Recover and inspect the original archive/source/inputs/outputs before attempting reproduction or adopting the method.

## What a new contributor can and cannot recover

The repo is enough to recover the chosen product direction, scientific ownership, staged authority for SELFS, current implementation status, and a proposed first slice. This inventory also preserves the additional source references known to the assistant.

It is not enough to reconstruct all earlier experiments, discarded approaches, numerical choices, prototype results, or the full reasoning in the original chats. Those gaps also limit the assistant working from the available summaries. Adding more confident prose would not restore missing evidence.

## Completing the handoff

Import the missing chat exports and original prototype files when provided. For each source, capture its title/date and extract the useful question, actual decisions, alternatives considered, assumptions, proposed experiments, unresolved issues, and any changes to earlier direction. Distinguish the user’s commitments from assistant suggestions and abandoned sketches.

For prototypes, preserve the original artifact and provenance, identify the model and dependencies, inspect the code and result-generation procedure, and reproduce relevant checks before promoting any claim into [evidence](evidence/README.md). Keep reported results distinguishable from independently reproduced results.

The goal is a self-contained project handoff: enough reasoning and reproducible material to continue the work without access to the original chats. It does not require copying every conversational turn. Keep this inventory current as sources are recovered, and link extracted decisions to their basis.

## Attention and SELFS convergence review, 2026-10-06

The user requested an assessment of commit [c1799d1](https://github.com/bertona88/plasmalab/commit/c1799d183b165e358b9f40a94563f10a90d6430d): what to retain, what can be computed, how current HEDP machine learning relates, whether a generic network should learn the representation, and a possible network design.

The review read that commit's complete [divergent notebook](research/2026-10-05-attention-selfs-divergence.md), repository guidance, the ladder, project context, this inventory, foundation decision, roadmap, driven-plasma brief, Rust requirement/result definitions, and browser shell. The attached SELFS abstract and ladder matched the preserved repository versions by Git blob hash. No additional original chat history, prototype, or result artifact was recovered.

The [convergence assessment](research/2026-10-06-selfs-ml-convergence.md) records a targeted primary-source literature review through 2026-10-06, including publication status and limitations, followed by assistant rankings and proposed experiments. Its recommendations are not user-adopted architecture decisions. The review performed no plasma simulation, network training, or reproduction of external results; it establishes no speedup or novelty. The original divergence notebook remains an unranked historical source.
