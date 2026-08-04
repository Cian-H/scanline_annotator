use ndarray::{Array2, ArrayView2, Axis};
use rayon::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnnotatorError {
    #[error("Shape error: {0}")]
    ShapeError(#[from] ndarray::ShapeError),

    #[error("Miscellaneous Error: {0}")]
    MiscError(String),
}

pub type Result<T> = std::result::Result<T, AnnotatorError>;

#[derive(Clone, Debug)]
struct FilteredPoint {
    x: f64,
    y: f64,
    z: f64, // mean sensor (temperature)
    start_idx: usize,
    end_idx: usize,
}

#[derive(Clone, Debug)]
struct TurningPoint {
    y_idx: usize,
    dist_prev_sq: f64,
    hatch_dist_sq: f64,
    x: f64,
    y: f64,
    start_idx: usize,
    end_idx: usize,
}

fn collapse_x_sequential(input: &ArrayView2<f64>) -> Vec<FilteredPoint> {
    let n = input.nrows();
    let ncols = input.ncols();
    let mut x_collapsed = Vec::with_capacity(n / 2);
    if n == 0 {
        return x_collapsed;
    }

    if let Some(slice) = input.as_slice() {
        let mut current_x = slice[0];
        let mut sum_y = slice[1];
        let mut sum_temp = if ncols > 2 { slice[2] } else { 0.0 };
        let mut count = 1.0;
        let mut start_idx = 0;

        for i in 1..n {
            let offset = i * ncols;
            let x = slice[offset];
            if (x - current_x).abs() < f64::EPSILON {
                sum_y += slice[offset + 1];
                if ncols > 2 {
                    sum_temp += slice[offset + 2];
                }
                count += 1.0;
            } else {
                x_collapsed.push(FilteredPoint {
                    x: current_x,
                    y: sum_y / count,
                    z: sum_temp / count,
                    start_idx,
                    end_idx: i - 1,
                });
                current_x = x;
                sum_y = slice[offset + 1];
                sum_temp = if ncols > 2 { slice[offset + 2] } else { 0.0 };
                count = 1.0;
                start_idx = i;
            }
        }
        x_collapsed.push(FilteredPoint {
            x: current_x,
            y: sum_y / count,
            z: sum_temp / count,
            start_idx,
            end_idx: n - 1,
        });

        return x_collapsed;
    }

    let mut current_x = input[[0, 0]];
    let mut sum_y = input[[0, 1]];
    let mut sum_temp = if input.ncols() > 2 {
        input[[0, 2]]
    } else {
        0.0
    };
    let mut count = 1.0;
    let mut start_idx = 0;

    for i in 1..n {
        let x = input[[i, 0]];
        if (x - current_x).abs() < f64::EPSILON {
            sum_y += input[[i, 1]];
            if input.ncols() > 2 {
                sum_temp += input[[i, 2]];
            }
            count += 1.0;
        } else {
            x_collapsed.push(FilteredPoint {
                x: current_x,
                y: sum_y / count,
                z: sum_temp / count,
                start_idx,
                end_idx: i - 1,
            });
            current_x = x;
            sum_y = input[[i, 1]];
            sum_temp = if input.ncols() > 2 {
                input[[i, 2]]
            } else {
                0.0
            };
            count = 1.0;
            start_idx = i;
        }
    }
    x_collapsed.push(FilteredPoint {
        x: current_x,
        y: sum_y / count,
        z: sum_temp / count,
        start_idx,
        end_idx: n - 1,
    });

    x_collapsed
}

fn collapse_x(input: &ArrayView2<f64>) -> Vec<FilteredPoint> {
    let n = input.nrows();
    let ncols = input.ncols();
    if n == 0 {
        return Vec::new();
    }

    let num_threads = rayon::current_num_threads();
    if num_threads <= 1 || n < 500_000 {
        return collapse_x_sequential(input);
    }

    let in_slice = match input.as_slice() {
        Some(s) => s,
        None => return collapse_x_sequential(input),
    };

    let chunk_size = n.div_ceil(num_threads);
    let chunks: Vec<(usize, usize)> = (0..n)
        .step_by(chunk_size)
        .map(|start| (start, (start + chunk_size).min(n)))
        .collect();

    let sub_results: Vec<Vec<FilteredPoint>> = chunks
        .into_par_iter()
        .map(|(start_row, end_row)| {
            let mut x_collapsed = Vec::with_capacity((end_row - start_row) / 2);
            if start_row >= end_row {
                return x_collapsed;
            }

            let mut current_x = in_slice[start_row * ncols];
            let mut sum_y = in_slice[start_row * ncols + 1];
            let mut sum_temp = if ncols > 2 {
                in_slice[start_row * ncols + 2]
            } else {
                0.0
            };
            let mut count = 1.0;
            let mut start_idx = start_row;

            for i in (start_row + 1)..end_row {
                let offset = i * ncols;
                let x = in_slice[offset];
                if (x - current_x).abs() < f64::EPSILON {
                    sum_y += in_slice[offset + 1];
                    if ncols > 2 {
                        sum_temp += in_slice[offset + 2];
                    }
                    count += 1.0;
                } else {
                    x_collapsed.push(FilteredPoint {
                        x: current_x,
                        y: sum_y / count,
                        z: sum_temp / count,
                        start_idx,
                        end_idx: i - 1,
                    });
                    current_x = x;
                    sum_y = in_slice[offset + 1];
                    sum_temp = if ncols > 2 { in_slice[offset + 2] } else { 0.0 };
                    count = 1.0;
                    start_idx = i;
                }
            }
            x_collapsed.push(FilteredPoint {
                x: current_x,
                y: sum_y / count,
                z: sum_temp / count,
                start_idx,
                end_idx: end_row - 1,
            });

            x_collapsed
        })
        .collect();

    let total_len: usize = sub_results.iter().map(|v| v.len()).sum();
    let mut merged: Vec<FilteredPoint> = Vec::with_capacity(total_len);

    for sub in sub_results {
        for pt in sub {
            if let Some(last) = merged.last_mut()
                && (pt.x - last.x).abs() < f64::EPSILON {
                    let total_count = (last.end_idx - last.start_idx + 1) as f64;
                    let pt_count = (pt.end_idx - pt.start_idx + 1) as f64;
                    let new_count = total_count + pt_count;

                    last.y = (last.y * total_count + pt.y * pt_count) / new_count;
                    last.z = (last.z * total_count + pt.z * pt_count) / new_count;
                    last.end_idx = pt.end_idx;
                    continue;
                }
            merged.push(pt);
        }
    }

    merged
}

fn collapse_y_sequential(x_collapsed: &[FilteredPoint]) -> Vec<FilteredPoint> {
    let n = x_collapsed.len();
    let mut y_collapsed = Vec::with_capacity(n);
    if n == 0 {
        return y_collapsed;
    }

    let mut current_y = x_collapsed[0].y;
    let mut sum_x = x_collapsed[0].x;
    let mut sum_temp = x_collapsed[0].z;
    let mut count = 1.0;
    let mut start_idx = x_collapsed[0].start_idx;

    for i in 1..n {
        let y = x_collapsed[i].y;
        if (y - current_y).abs() < f64::EPSILON {
            sum_x += x_collapsed[i].x;
            sum_temp += x_collapsed[i].z;
            count += 1.0;
        } else {
            y_collapsed.push(FilteredPoint {
                x: sum_x / count,
                y: current_y,
                z: sum_temp / count,
                start_idx,
                end_idx: x_collapsed[i - 1].end_idx,
            });
            current_y = y;
            sum_x = x_collapsed[i].x;
            sum_temp = x_collapsed[i].z;
            count = 1.0;
            start_idx = x_collapsed[i].start_idx;
        }
    }
    y_collapsed.push(FilteredPoint {
        x: sum_x / count,
        y: current_y,
        z: sum_temp / count,
        start_idx,
        end_idx: x_collapsed[n - 1].end_idx,
    });

    y_collapsed
}

fn collapse_y(x_collapsed: &[FilteredPoint]) -> Vec<FilteredPoint> {
    let n = x_collapsed.len();
    if n == 0 {
        return Vec::new();
    }

    let num_threads = rayon::current_num_threads();
    if num_threads <= 1 || n < 250_000 {
        return collapse_y_sequential(x_collapsed);
    }

    let chunk_size = n.div_ceil(num_threads);
    let chunks: Vec<(usize, usize)> = (0..n)
        .step_by(chunk_size)
        .map(|start| (start, (start + chunk_size).min(n)))
        .collect();

    let sub_results: Vec<Vec<FilteredPoint>> = chunks
        .into_par_iter()
        .map(|(start_row, end_row)| {
            let mut y_collapsed = Vec::with_capacity(end_row - start_row);
            if start_row >= end_row {
                return y_collapsed;
            }

            let mut current_y = x_collapsed[start_row].y;
            let mut sum_x = x_collapsed[start_row].x;
            let mut sum_temp = x_collapsed[start_row].z;
            let mut count = 1.0;
            let mut start_idx = x_collapsed[start_row].start_idx;

            for i in (start_row + 1)..end_row {
                let y = x_collapsed[i].y;
                if (y - current_y).abs() < f64::EPSILON {
                    sum_x += x_collapsed[i].x;
                    sum_temp += x_collapsed[i].z;
                    count += 1.0;
                } else {
                    y_collapsed.push(FilteredPoint {
                        x: sum_x / count,
                        y: current_y,
                        z: sum_temp / count,
                        start_idx,
                        end_idx: x_collapsed[i - 1].end_idx,
                    });
                    current_y = y;
                    sum_x = x_collapsed[i].x;
                    sum_temp = x_collapsed[i].z;
                    count = 1.0;
                    start_idx = x_collapsed[i].start_idx;
                }
            }
            y_collapsed.push(FilteredPoint {
                x: sum_x / count,
                y: current_y,
                z: sum_temp / count,
                start_idx,
                end_idx: x_collapsed[end_row - 1].end_idx,
            });

            y_collapsed
        })
        .collect();

    let total_len: usize = sub_results.iter().map(|v| v.len()).sum();
    let mut merged: Vec<FilteredPoint> = Vec::with_capacity(total_len);

    for sub in sub_results {
        for pt in sub {
            if let Some(last) = merged.last_mut()
                && (pt.y - last.y).abs() < f64::EPSILON {
                    // Need to merge
                    let total_count = (last.end_idx - last.start_idx + 1) as f64; // Approximated weight for x collapse
                    let pt_count = (pt.end_idx - pt.start_idx + 1) as f64;
                    let new_count = total_count + pt_count;

                    last.x = (last.x * total_count + pt.x * pt_count) / new_count;
                    last.z = (last.z * total_count + pt.z * pt_count) / new_count;
                    last.end_idx = pt.end_idx;
                    continue;
                }
            merged.push(pt);
        }
    }

    merged
}

fn direction(a: &FilteredPoint, b: &FilteredPoint, c: f64) -> f64 {
    if (a.x - b.x).abs() < f64::EPSILON || (a.y - b.y).abs() < f64::EPSILON {
        c
    } else if a.x < b.x {
        (b.y - a.y) / (b.x - a.x)
    } else {
        -(b.y - a.y) / (b.x - a.x)
    }
}

fn dist_fp_sq(a: &TurningPoint, b: &FilteredPoint) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

fn hatch_dist_sq(a: &TurningPoint, b: &TurningPoint, c: &FilteredPoint) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    if dx.abs() < f64::EPSILON {
        let dx_c = c.x - a.x;
        dx_c * dx_c
    } else {
        let m = dy / dx;
        let c_line = b.y - m * b.x;
        let num = m * c.x - c.y + c_line;
        (num * num) / (m * m + 1.0)
    }
}

fn segment_vector(h: &[TurningPoint], i: usize) -> (f64, f64) {
    if i == 0 {
        (0.0, 0.0)
    } else {
        (h[i].x - h[i - 1].x, h[i].y - h[i - 1].y)
    }
}

fn is_180_turn_vec(v1: (f64, f64), v2: (f64, f64)) -> bool {
    let (dx1, dy1) = v1;
    let (dx2, dy2) = v2;

    let dp = dx1 * dx2 + dy1 * dy2;
    if dp >= 0.0 {
        return false;
    } // Must be pointing opposite directions

    let l1_sq = dx1 * dx1 + dy1 * dy1;
    let l2_sq = dx2 * dx2 + dy2 * dy2;

    // Require a tight 180 flip (angle > 150 deg). cos(150)^2 = 0.75
    (dp * dp) > 0.75 * (l1_sq * l2_sq)
}

fn is_rastering(h: &[TurningPoint], i: usize, median_hatch_sq: f64) -> bool {
    let tp = &h[i];

    // 1. Must not be an excessively long jump (e.g., crossing the whole plate).
    // Bound dynamically by the median hatch spacing (minimum of 1mm^2 to prevent over-constraining).
    let max_jump_sq = (100.0 * median_hatch_sq).max(1.0);
    if tp.dist_prev_sq > max_jump_sq {
        return false;
    }

    // 2. The turnaround must be roughly perpendicular to the scanline.
    // For contours, they trace end-to-end, making them collinear (hatch_dist approaches 0).
    if tp.hatch_dist_sq < 0.5 * tp.dist_prev_sq {
        return false;
    }

    let mut flip_prev = false;
    let mut flip_next = false;
    
    if i > 1 || i + 1 < h.len() {
        let v_curr = segment_vector(h, i);
        if i > 1 {
            let v_prev = segment_vector(h, i - 1);
            flip_prev = is_180_turn_vec(v_prev, v_curr);
        }
        if i + 1 < h.len() {
            let v_next = segment_vector(h, i + 1);
            flip_next = is_180_turn_vec(v_curr, v_next);
        }
    }

    flip_prev || flip_next
}

/// Annotates in-memory PBF raster scanline data.
///
/// Accepts a 2D array view from Python/NumPy and returns processed scanline data.
pub fn annotate_scanlines(input: ArrayView2<f64>) -> Result<Array2<f64>> {
    if input.ndim() != 2 {
        return Err(AnnotatorError::MiscError(format!(
            "Expected 2D array, got {}D array",
            input.ndim()
        )));
    }

    let x_collapsed = collapse_x(&input);
    let y_collapsed = collapse_y(&x_collapsed);

    let n = y_collapsed.len();
    let mut set_a = Vec::new();
    let mut set_b = Vec::new();

    if n > 1 {
        let mut d = vec![0.0; n - 1];
        d[0] = direction(&y_collapsed[0], &y_collapsed[1], 1.0);

        let tp_init = TurningPoint {
            y_idx: 0,
            dist_prev_sq: 0.0,
            hatch_dist_sq: 0.0,
            x: y_collapsed[0].x,
            y: y_collapsed[0].y,
            start_idx: y_collapsed[0].start_idx,
            end_idx: y_collapsed[0].end_idx,
        };
        set_a.push(tp_init.clone());
        set_b.push(tp_init);

        for index in 1..(n - 1) {
            d[index] = direction(
                &y_collapsed[index],
                &y_collapsed[index + 1],
                d[index - 1],
            );

            if d[index].signum() != d[index - 1].signum() {
                let current_fp = &y_collapsed[index];

                if d[index] < 0.0 {
                    let last_a = set_a.last().unwrap();
                    let last_b = set_b.last().unwrap();
                    let dist_sq = dist_fp_sq(last_a, current_fp);
                    let h_dist_sq = hatch_dist_sq(last_b, last_a, current_fp);

                    set_a.push(TurningPoint {
                        y_idx: index,
                        dist_prev_sq: dist_sq,
                        hatch_dist_sq: h_dist_sq,
                        x: current_fp.x,
                        y: current_fp.y,
                        start_idx: current_fp.start_idx,
                        end_idx: current_fp.end_idx,
                    });
                } else {
                    let last_a = set_a.last().unwrap();
                    let last_b = set_b.last().unwrap();
                    let dist_sq = dist_fp_sq(last_b, current_fp);
                    let h_dist_sq = hatch_dist_sq(last_b, last_a, current_fp);

                    set_b.push(TurningPoint {
                        y_idx: index,
                        dist_prev_sq: dist_sq,
                        hatch_dist_sq: h_dist_sq,
                        x: current_fp.x,
                        y: current_fp.y,
                        start_idx: current_fp.start_idx,
                        end_idx: current_fp.end_idx,
                    });
                }
            }
        }
    }

    if set_a.len() > 2 {
        set_a.remove(0);
        set_a.remove(0);
    } else {
        set_a.clear();
    }
    if set_b.len() > 1 {
        set_b.remove(0);
    } else {
        set_b.clear();
    }

    let mut h = Vec::new();
    h.extend(set_a);
    h.extend(set_b);
    h.sort_by_key(|tp| tp.y_idx);

    let median_hatch_sq = if h.is_empty() {
        0.01 // Safe fallback
    } else {
        let mut hatch_dists: Vec<f64> = h
            .iter()
            .map(|tp| tp.hatch_dist_sq)
            .filter(|&d| d > 1e-8) // Exclude nearly perfectly collinear noise
            .collect();

        if hatch_dists.is_empty() {
            0.01
        } else {
            let mid = hatch_dists.len() / 2;
            let (_, median, _) = hatch_dists
                .select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            *median
        }
    };

    let mut scanline_assignments = vec![-1.0; input.nrows()];
    let mut scanline_id = 1.0;
    let mut in_raster_block = false;
    let mut last_turn_start = 0;

    for i in 1..h.len() {
        if is_rastering(&h, i, median_hatch_sq) {
            let mid = h[i].start_idx + (h[i].end_idx - h[i].start_idx) / 2;

            if !in_raster_block {
                in_raster_block = true;
                let start = h[i - 1].end_idx + 1;
                let end = mid;
                if start <= end {
                    for j in start..=end {
                        if j < scanline_assignments.len() {
                            scanline_assignments[j] = scanline_id;
                        }
                    }
                }
                scanline_id += 1.0;
                last_turn_start = mid + 1;
            } else {
                let start = last_turn_start;
                let end = mid;
                if start <= end {
                    for j in start..=end {
                        if j < scanline_assignments.len() {
                            scanline_assignments[j] = scanline_id;
                        }
                    }
                }
                scanline_id += 1.0;
                last_turn_start = mid + 1;
            }
        } else {
            if in_raster_block {
                let start = last_turn_start;
                let end = h[i].start_idx;
                if start < end {
                    for j in start..end {
                        if j < scanline_assignments.len() {
                            scanline_assignments[j] = scanline_id;
                        }
                    }
                }
                scanline_id += 1.0;
                in_raster_block = false;
            }
        }
    }

    // Allocate output array with M+1 columns
    let nrows = input.nrows();
    let ncols = input.ncols();
    let mut out_array = Array2::<f64>::zeros((nrows, ncols + 1));

    let in_slice = input.as_slice();
    out_array
        .axis_iter_mut(Axis(0))
        .into_par_iter()
        .enumerate()
        .for_each(|(i, mut row)| {
            if let (Some(slice), Some(row_slice)) = (in_slice, row.as_slice_mut()) {
                let src = &slice[i * ncols..(i + 1) * ncols];
                row_slice[..ncols].copy_from_slice(src);
            } else {
                for j in 0..ncols {
                    row[j] = input[[i, j]];
                }
            }
            row[ncols] = scanline_assignments[i];
        });

    Ok(out_array)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;

    #[test]
    fn test_annotate_scanlines_stub() {
        let input = Array2::<f64>::zeros((100, 4));
        let result = annotate_scanlines(input.view()).unwrap();
        assert_eq!(result.shape(), &[100, 5]);
    }
}
