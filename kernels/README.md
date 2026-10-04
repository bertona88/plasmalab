# WGSL numerical execution

Reserved for real, tested kernels; no GPU execution exists in the scaffold.

Rust defines the scientific operation, conventions, layout contract, admissible inputs, and reference behavior. WGSL carries out parallel arithmetic. The browser adapter owns device setup, pipeline dispatch, capabilities, and device-loss handling. Svelte consumes observations rather than GPU storage internals.

Add a kernel only with an implemented experiment, an explicit host/shader data layout, a CPU reference comparison across relevant inputs, precision/tolerance decisions, bounds handling, and end-to-end timing. Show unsupported hardware or backend changes clearly. Never silently substitute a different scientific model when WebGPU is unavailable.

Do not add a decorative compute shader just to make this directory look implemented. The first small model can use Rust/WASM CPU execution; acceleration follows measured need.
