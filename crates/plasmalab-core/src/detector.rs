//! Observation-only connected-component detector, not a topology/SELF classifier.
use crate::contract::Candidate;
use std::f64::consts::{PI, TAU};

/// Four-connected, same-sign anomalies |ψ−ψ₀|>.22, within the central 60%.
/// Discard fewer than 10 cells and components occupying >=75% of x columns
/// (a sheet-spanning stripe is not a localized candidate). Horizontal adjacency
/// and centers respect the periodic boundary. IDs are rebuilt each observation.
pub(crate) fn detect(
    flux: &[f64],
    background: &[f64],
    width: usize,
    height: usize,
) -> Vec<Candidate> {
    let mut labels = vec![0i8; width * height];
    for y in 0..height {
        if (y as f64 - (height - 1) as f64 * 0.5).abs() > height as f64 * 0.3 {
            continue;
        }
        for x in 0..width {
            let q = flux[y * width + x] - background[y];
            labels[y * width + x] = if q > 0.22 {
                1
            } else if q < -0.22 {
                -1
            } else {
                0
            };
        }
    }
    let mut candidates = Vec::new();
    let mut stack = Vec::new();
    for origin in 0..labels.len() {
        let sign = labels[origin];
        if sign == 0 {
            continue;
        }
        stack.push(origin);
        labels[origin] = 0;
        let (mut area, mut sy, mut sine, mut cosine) = (0u32, 0.0, 0.0, 0.0);
        let mut columns = vec![false; width];
        while let Some(i) = stack.pop() {
            let x = i % width;
            let y = i / width;
            area += 1;
            sy += y as f64;
            let angle = TAU * x as f64 / width as f64;
            sine += angle.sin();
            cosine += angle.cos();
            columns[x] = true;
            let neighbors = [
                y * width + (x + width - 1) % width,
                y * width + (x + 1) % width,
                if y > 0 { i - width } else { i },
                if y + 1 < height { i + width } else { i },
            ];
            for neighbor in neighbors {
                if labels[neighbor] == sign {
                    labels[neighbor] = 0;
                    stack.push(neighbor);
                }
            }
        }
        if area < 10 || columns.iter().filter(|&&used| used).count() * 4 >= width * 3 {
            continue;
        }
        let x = sine.atan2(cosine).rem_euclid(TAU) / TAU;
        candidates.push(Candidate {
            id: candidates.len() as u32 + 1,
            x,
            y: sy / area as f64 / (height - 1) as f64,
            area,
            radius: (area as f64 / PI).sqrt() / height as f64,
            polarity: sign as i32,
        });
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identifies_known_patches_and_periodic_seam_without_mutation() {
        let (w, h) = (32, 24);
        let background = vec![0.0; h];
        let mut flux = vec![0.0; w * h];
        for y in 9..13 {
            for x in [30, 31, 0, 1] {
                flux[y * w + x] = 1.0;
            }
        }
        for y in 10..14 {
            for x in 13..17 {
                flux[y * w + x] = -1.0;
            }
        }
        let before = flux.clone();
        let candidates = detect(&flux, &background, w, h);
        assert_eq!(candidates.len(), 2);
        assert!(candidates.iter().all(|c| c.area == 16));
        assert!(candidates.iter().any(|c| c.x < 0.05 || c.x > 0.95));
        assert_eq!(flux, before);
    }
    #[test]
    fn rejects_unbroken_sheet_and_subthreshold_noise() {
        let (w, h) = (32, 24);
        let background = vec![0.0; h];
        let mut flux = vec![0.1; w * h];
        assert!(detect(&flux, &background, w, h).is_empty());
        for y in 9..13 {
            for x in 0..w {
                flux[y * w + x] = 1.0;
            }
        }
        assert!(detect(&flux, &background, w, h).is_empty());
    }
}
