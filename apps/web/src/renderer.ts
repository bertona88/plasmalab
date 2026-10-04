import type { Frame } from './lib/contracts';

export interface ViewOptions {
  contours: boolean;
  candidates: boolean;
  cursor: { x: number; y: number } | null;
  cursorRadius: number;
}

// This projection reads copied observation samples. Pixel values, contour
// interpolation, and contrast never feed back into the authoritative model.
export function createFieldRenderer(canvas: HTMLCanvasElement) {
  const context = canvas.getContext('2d');
  if (!context)
    throw new Error('A 2D canvas is required to view this experiment.');
  const ctx = context;
  const raster = document.createElement('canvas');
  const rasterContext = raster.getContext('2d');
  if (!rasterContext) throw new Error('The field renderer could not start.');
  const rasterCtx = rasterContext;
  let pixelWidth = 0;
  let pixelHeight = 0;

  function draw(frame: Frame, options: ViewOptions) {
    const rect = canvas.getBoundingClientRect();
    if (!rect.width || !rect.height) return;
    const ratio = Math.min(window.devicePixelRatio || 1, 2);
    const width = rect.width;
    const height = rect.height;
    if (
      pixelWidth !== Math.round(width * ratio) ||
      pixelHeight !== Math.round(height * ratio)
    ) {
      pixelWidth = Math.round(width * ratio);
      pixelHeight = Math.round(height * ratio);
      canvas.width = pixelWidth;
      canvas.height = pixelHeight;
    }
    ctx.setTransform(ratio, 0, 0, ratio, 0, 0);
    const w = frame.width;
    const h = frame.height;
    const { flux, current, activity } = frame.field;
    const sx = width / (w - 1);
    const sy = height / (h - 1);
    const bx = new Float32Array(w * h);
    let peakField = 0.0001;
    let peakCurrent = 0.0001;
    let minimum = Infinity;
    let maximum = -Infinity;
    for (let y = 0; y < h; y++) {
      for (let x = 0; x < w; x++) {
        const i = y * w + x;
        bx[i] =
          flux[Math.min(h - 1, y + 1) * w + x] -
          flux[Math.max(0, y - 1) * w + x];
        peakField = Math.max(peakField, Math.abs(bx[i]));
        peakCurrent = Math.max(peakCurrent, Math.abs(current[i]));
        minimum = Math.min(minimum, flux[i]);
        maximum = Math.max(maximum, flux[i]);
      }
    }
    if (raster.width !== w || raster.height !== h) {
      raster.width = w;
      raster.height = h;
    }
    const pixels = rasterCtx.createImageData(w, h);
    for (let i = 0; i < flux.length; i++) {
      const field = Math.sqrt(Math.min(1, Math.abs(bx[i]) / peakField));
      const stress = Math.pow(
        Math.min(1, Math.abs(current[i]) / peakCurrent),
        0.65,
      );
      const glow = Math.min(1, Math.max(0, activity[i]));
      const upper = bx[i] >= 0;
      const color = upper ? [36, 177, 198] : [217, 131, 53];
      const base = 0.08 + field * 0.2;
      pixels.data[i * 4] = Math.min(
        255,
        7 + color[0] * base + stress * 20 + glow * 182,
      );
      pixels.data[i * 4 + 1] = Math.min(
        255,
        14 + color[1] * base + stress * 13 + glow * 74,
      );
      pixels.data[i * 4 + 2] = Math.min(
        255,
        22 + color[2] * base + stress * 23 + glow * 100,
      );
      pixels.data[i * 4 + 3] = 255;
    }
    rasterCtx.putImageData(pixels, 0, 0);
    ctx.imageSmoothingEnabled = true;
    ctx.drawImage(raster, 0, 0, width, height);

    ctx.strokeStyle = 'rgba(152, 193, 207, 0.055)';
    ctx.lineWidth = 1;
    ctx.beginPath();
    for (let i = 1; i < 12; i++) {
      ctx.moveTo((width * i) / 12, 0);
      ctx.lineTo((width * i) / 12, height);
    }
    for (let i = 1; i < 8; i++) {
      ctx.moveTo(0, (height * i) / 8);
      ctx.lineTo(width, (height * i) / 8);
    }
    ctx.stroke();

    if (options.contours && maximum > minimum) {
      // Marching squares is a display-only contour of the supplied flux field.
      const spacing = (maximum - minimum) / 26;
      for (
        let level = minimum + spacing / 2;
        level < maximum;
        level += spacing
      ) {
        for (const sign of [-1, 1]) {
          ctx.beginPath();
          for (let y = 0; y < h - 1; y++) {
            for (let x = 0; x < w - 1; x++) {
              const i = y * w + x;
              if ((bx[i] >= 0 ? 1 : -1) !== sign) continue;
              const below =
                Number(flux[i] < level) +
                Number(flux[i + 1] < level) +
                Number(flux[i + 1 + w] < level) +
                Number(flux[i + w] < level);
              if (below === 0 || below === 4) continue;
              const values = [
                flux[i],
                flux[i + 1],
                flux[i + 1 + w],
                flux[i + w],
              ];
              const corners = [
                [x, y],
                [x + 1, y],
                [x + 1, y + 1],
                [x, y + 1],
              ];
              const crossings: number[][] = [];
              for (let edge = 0; edge < 4; edge++) {
                const next = (edge + 1) % 4;
                const a = values[edge];
                const b = values[next];
                if (a < level === b < level) continue;
                const t = (level - a) / (b - a);
                crossings.push([
                  (corners[edge][0] +
                    t * (corners[next][0] - corners[edge][0])) *
                    sx,
                  (corners[edge][1] +
                    t * (corners[next][1] - corners[edge][1])) *
                    sy,
                ]);
              }
              if (crossings.length === 4) {
                // Resolve the saddle with the bilinear interpolant's
                // asymptotic decider instead of arbitrary edge pairing.
                const determinant =
                  (values[0] - level) * (values[2] - level) -
                  (values[1] - level) * (values[3] - level);
                if (determinant < 0)
                  [crossings[1], crossings[3]] = [crossings[3], crossings[1]];
              }
              for (let index = 0; index + 1 < crossings.length; index += 2) {
                ctx.moveTo(crossings[index][0], crossings[index][1]);
                ctx.lineTo(crossings[index + 1][0], crossings[index + 1][1]);
              }
            }
          }
          ctx.strokeStyle =
            sign > 0 ? 'rgba(83, 210, 223, 0.55)' : 'rgba(234, 166, 93, 0.55)';
          ctx.lineWidth = 0.9;
          ctx.stroke();
        }
      }
    }

    if (options.candidates) {
      ctx.font = '10px ui-monospace, monospace';
      for (const candidate of frame.observations.candidates.slice(0, 16)) {
        const x = candidate.x * width;
        const y = candidate.y * height;
        const radius = Math.max(6, candidate.radius * height);
        ctx.strokeStyle = 'rgba(203, 230, 223, 0.62)';
        ctx.lineWidth = 1;
        ctx.setLineDash([3, 4]);
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, Math.PI * 2);
        ctx.stroke();
        ctx.setLineDash([]);
        ctx.fillStyle = 'rgba(219, 242, 235, 0.85)';
        ctx.fillText(
          String(candidate.id).padStart(2, '0'),
          x + radius + 4,
          y + 3,
        );
      }
    }

    const shade = ctx.createRadialGradient(
      width / 2,
      height / 2,
      height * 0.22,
      width / 2,
      height / 2,
      width * 0.72,
    );
    shade.addColorStop(0, 'rgba(2, 7, 13, 0)');
    shade.addColorStop(1, 'rgba(2, 7, 13, 0.45)');
    ctx.fillStyle = shade;
    ctx.fillRect(0, 0, width, height);
    if (options.cursor) {
      const x = options.cursor.x * width;
      const y = options.cursor.y * height;
      const radius = options.cursorRadius * height;
      ctx.strokeStyle = 'rgba(220, 244, 237, 0.8)';
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.arc(x, y, radius, 0, Math.PI * 2);
      ctx.moveTo(x - 4, y);
      ctx.lineTo(x + 4, y);
      ctx.moveTo(x, y - 4);
      ctx.lineTo(x, y + 4);
      ctx.stroke();
    }
  }
  return { draw };
}
