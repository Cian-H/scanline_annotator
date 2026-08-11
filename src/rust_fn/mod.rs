use ndarray::{Array1, ArrayView1};
use num_traits::Float;
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
struct FilteredPoint<T> {
    x: T,
    y: T,
    start_idx: usize,
    end_idx: usize,
}

#[derive(Clone, Debug)]
struct TurningPoint<T> {
    y_idx: usize,
    dist_prev_sq: T,
    hatch_dist_sq: T,
    x: T,
    y: T,
    start_idx: usize,
    end_idx: usize,
}

fn collapse_x_sequential<T: Float + num_traits::FromPrimitive>(
    x_slice: &[T],
    y_slice: &[T],
) -> Vec<FilteredPoint<T>> {
    let n = x_slice.len();
    let mut x_collapsed = Vec::with_capacity(n / 2);
    if n == 0 {
        return x_collapsed;
    }

    let mut current_x = x_slice[0];
    let mut sum_y = y_slice[0];
    let mut count: usize = 1;
    let mut start_idx = 0;
    let eps = T::epsilon();

    for i in 1..n {
        let x = x_slice[i];
        if (x - current_x).abs() < eps {
            sum_y = sum_y + y_slice[i];
            count += 1;
        } else {
            let count_t = T::from(count).unwrap();
            x_collapsed.push(FilteredPoint {
                x: current_x,
                y: sum_y / count_t,
                start_idx,
                end_idx: i - 1,
            });
            current_x = x;
            sum_y = y_slice[i];
            count = 1;
            start_idx = i;
        }
    }
    let count_t = T::from(count).unwrap();
    x_collapsed.push(FilteredPoint {
        x: current_x,
        y: sum_y / count_t,
        start_idx,
        end_idx: n - 1,
    });

    x_collapsed
}

fn collapse_x<T: Float + num_traits::FromPrimitive + Send + Sync>(
    x_slice: &[T],
    y_slice: &[T],
) -> Vec<FilteredPoint<T>> {
    let n = x_slice.len();
    if n == 0 {
        return Vec::new();
    }

    let num_threads = rayon::current_num_threads();
    if num_threads <= 1 || n < 500_000 {
        return collapse_x_sequential(x_slice, y_slice);
    }

    let chunk_size = n.div_ceil(num_threads);
    let chunks: Vec<(usize, usize)> = (0..n)
        .step_by(chunk_size)
        .map(|start| (start, (start + chunk_size).min(n)))
        .collect();

    let sub_results: Vec<Vec<FilteredPoint<T>>> = chunks
        .into_par_iter()
        .map(|(start_row, end_row)| {
            let mut x_collapsed = Vec::with_capacity((end_row - start_row) / 2);
            if start_row >= end_row {
                return x_collapsed;
            }

            let mut current_x = x_slice[start_row];
            let mut sum_y = y_slice[start_row];
            let mut count: usize = 1;
            let mut start_idx = start_row;
            let eps = T::epsilon();

            for i in (start_row + 1)..end_row {
                let x = x_slice[i];
                if (x - current_x).abs() < eps {
                    sum_y = sum_y + y_slice[i];
                    count += 1;
                } else {
                    let count_t = T::from(count).unwrap();
                    x_collapsed.push(FilteredPoint {
                        x: current_x,
                        y: sum_y / count_t,
                        start_idx,
                        end_idx: i - 1,
                    });
                    current_x = x;
                    sum_y = y_slice[i];
                    count = 1;
                    start_idx = i;
                }
            }
            let count_t = T::from(count).unwrap();
            x_collapsed.push(FilteredPoint {
                x: current_x,
                y: sum_y / count_t,
                start_idx,
                end_idx: end_row - 1,
            });

            x_collapsed
        })
        .collect();

    let total_len: usize = sub_results.iter().map(|v| v.len()).sum();
    let mut merged: Vec<FilteredPoint<T>> = Vec::with_capacity(total_len);
    let eps = T::epsilon();

    for sub in sub_results {
        for pt in sub {
            if let Some(last) = merged.last_mut() {
                if (pt.x - last.x).abs() < eps {
                    let total_count = (last.end_idx - last.start_idx + 1) as f64;
                    let pt_count = (pt.end_idx - pt.start_idx + 1) as f64;
                    let new_count = total_count + pt_count;

                    let last_y_f = last.y.to_f64().unwrap();
                    let pt_y_f = pt.y.to_f64().unwrap();
                    let new_y = (last_y_f * total_count + pt_y_f * pt_count) / new_count;
                    last.y = T::from(new_y).unwrap();
                    last.end_idx = pt.end_idx;
                    continue;
                }
            }
            merged.push(pt);
        }
    }

    merged
}

fn collapse_y_sequential<T: Float + num_traits::FromPrimitive>(
    x_collapsed: &[FilteredPoint<T>],
) -> Vec<FilteredPoint<T>> {
    let n = x_collapsed.len();
    let mut y_collapsed = Vec::with_capacity(n);
    if n == 0 {
        return y_collapsed;
    }

    let mut current_y = x_collapsed[0].y;
    let mut sum_x = x_collapsed[0].x;
    let mut count: usize = 1;
    let mut start_idx = x_collapsed[0].start_idx;
    let eps = T::epsilon();

    for i in 1..n {
        let y = x_collapsed[i].y;
        if (y - current_y).abs() < eps {
            sum_x = sum_x + x_collapsed[i].x;
            count += 1;
        } else {
            let count_t = T::from(count).unwrap();
            y_collapsed.push(FilteredPoint {
                x: sum_x / count_t,
                y: current_y,
                start_idx,
                end_idx: x_collapsed[i - 1].end_idx,
            });
            current_y = y;
            sum_x = x_collapsed[i].x;
            count = 1;
            start_idx = x_collapsed[i].start_idx;
        }
    }
    let count_t = T::from(count).unwrap();
    y_collapsed.push(FilteredPoint {
        x: sum_x / count_t,
        y: current_y,
        start_idx,
        end_idx: x_collapsed[n - 1].end_idx,
    });

    y_collapsed
}

fn collapse_y<T: Float + num_traits::FromPrimitive + Send + Sync>(
    x_collapsed: &[FilteredPoint<T>],
) -> Vec<FilteredPoint<T>> {
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

    let sub_results: Vec<Vec<FilteredPoint<T>>> = chunks
        .into_par_iter()
        .map(|(start_row, end_row)| {
            let mut y_collapsed = Vec::with_capacity(end_row - start_row);
            if start_row >= end_row {
                return y_collapsed;
            }

            let mut current_y = x_collapsed[start_row].y;
            let mut sum_x = x_collapsed[start_row].x;
            let mut count: usize = 1;
            let mut start_idx = x_collapsed[start_row].start_idx;
            let eps = T::epsilon();

            for i in (start_row + 1)..end_row {
                let y = x_collapsed[i].y;
                if (y - current_y).abs() < eps {
                    sum_x = sum_x + x_collapsed[i].x;
                    count += 1;
                } else {
                    let count_t = T::from(count).unwrap();
                    y_collapsed.push(FilteredPoint {
                        x: sum_x / count_t,
                        y: current_y,
                        start_idx,
                        end_idx: x_collapsed[i - 1].end_idx,
                    });
                    current_y = y;
                    sum_x = x_collapsed[i].x;
                    count = 1;
                    start_idx = x_collapsed[i].start_idx;
                }
            }
            let count_t = T::from(count).unwrap();
            y_collapsed.push(FilteredPoint {
                x: sum_x / count_t,
                y: current_y,
                start_idx,
                end_idx: x_collapsed[end_row - 1].end_idx,
            });

            y_collapsed
        })
        .collect();

    let total_len: usize = sub_results.iter().map(|v| v.len()).sum();
    let mut merged: Vec<FilteredPoint<T>> = Vec::with_capacity(total_len);
    let eps = T::epsilon();

    for sub in sub_results {
        for pt in sub {
            if let Some(last) = merged.last_mut() {
                if (pt.y - last.y).abs() < eps {
                    let total_count = (last.end_idx - last.start_idx + 1) as f64;
                    let pt_count = (pt.end_idx - pt.start_idx + 1) as f64;
                    let new_count = total_count + pt_count;

                    let last_x_f = last.x.to_f64().unwrap();
                    let pt_x_f = pt.x.to_f64().unwrap();
                    let new_x = (last_x_f * total_count + pt_x_f * pt_count) / new_count;
                    last.x = T::from(new_x).unwrap();
                    last.end_idx = pt.end_idx;
                    continue;
                }
            }
            merged.push(pt);
        }
    }

    merged
}

fn direction<T: Float + num_traits::FromPrimitive>(a: &FilteredPoint<T>, b: &FilteredPoint<T>, c: T) -> T {
    let eps = T::epsilon();
    if (a.x - b.x).abs() < eps || (a.y - b.y).abs() < eps {
        c
    } else if a.x < b.x {
        (b.y - a.y) / (b.x - a.x)
    } else {
        -(b.y - a.y) / (b.x - a.x)
    }
}

fn dist_fp_sq<T: Float>(a: &TurningPoint<T>, b: &FilteredPoint<T>) -> T {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

fn hatch_dist_sq<T: Float>(a: &TurningPoint<T>, b: &TurningPoint<T>, c: &FilteredPoint<T>) -> T {
    let eps = T::epsilon();
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    if dx.abs() < eps {
        let dx_c = c.x - a.x;
        dx_c * dx_c
    } else {
        let m = dy / dx;
        let c_line = b.y - m * b.x;
        let num = m * c.x - c.y + c_line;
        (num * num) / (m * m + T::one())
    }
}

fn segment_vector<T: Float>(h: &[TurningPoint<T>], i: usize) -> (T, T) {
    if i == 0 {
        (T::zero(), T::zero())
    } else {
        (h[i].x - h[i - 1].x, h[i].y - h[i - 1].y)
    }
}

fn is_180_turn_vec<T: Float + num_traits::FromPrimitive>(v1: (T, T), v2: (T, T)) -> bool {
    let (dx1, dy1) = v1;
    let (dx2, dy2) = v2;

    let dp = dx1 * dx2 + dy1 * dy2;
    if dp >= T::zero() {
        return false;
    }

    let l1_sq = dx1 * dx1 + dy1 * dy1;
    let l2_sq = dx2 * dx2 + dy2 * dy2;

    let threshold = T::from(0.75).unwrap();
    (dp * dp) > threshold * (l1_sq * l2_sq)
}

fn is_rastering<T: Float + num_traits::FromPrimitive>(h: &[TurningPoint<T>], i: usize, median_hatch_sq: T) -> bool {
    let tp = &h[i];

    let max_jump_sq = (T::from(100.0).unwrap() * median_hatch_sq).max(T::one());
    if tp.dist_prev_sq > max_jump_sq {
        return false;
    }

    let half = T::from(0.5).unwrap();
    if tp.hatch_dist_sq < half * tp.dist_prev_sq {
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
/// Accepts 1D views of x and y coordinates, returning scanline assignments (int32 array).
pub fn annotate_scanlines<T: Float + num_traits::FromPrimitive + Send + Sync>(
    x: ArrayView1<T>,
    y: ArrayView1<T>,
) -> Result<Array1<i32>> {
    if x.len() != y.len() {
        return Err(AnnotatorError::MiscError(format!(
            "Input x and y length mismatch: {} vs {}",
            x.len(),
            y.len()
        )));
    }

    let n_total = x.len();
    if n_total == 0 {
        return Ok(Array1::<i32>::zeros(0));
    }

    let x_slice = x.as_slice().ok_or_else(|| AnnotatorError::MiscError("x array must be contiguous".to_string()))?;
    let y_slice = y.as_slice().ok_or_else(|| AnnotatorError::MiscError("y array must be contiguous".to_string()))?;

    let x_collapsed = collapse_x(x_slice, y_slice);
    let y_collapsed = collapse_y(&x_collapsed);

    let n = y_collapsed.len();
    let mut set_a = Vec::new();
    let mut set_b = Vec::new();

    if n > 1 {
        let mut d = vec![T::zero(); n - 1];
        d[0] = direction(&y_collapsed[0], &y_collapsed[1], T::one());

        let tp_init = TurningPoint {
            y_idx: 0,
            dist_prev_sq: T::zero(),
            hatch_dist_sq: T::zero(),
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

                if d[index] < T::zero() {
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
        T::from(0.01).unwrap()
    } else {
        let thresh = T::from(1e-8).unwrap();
        let mut hatch_dists: Vec<T> = h
            .iter()
            .map(|tp| tp.hatch_dist_sq)
            .filter(|&d| d > thresh)
            .collect();

        if hatch_dists.is_empty() {
            T::from(0.01).unwrap()
        } else {
            let mid = hatch_dists.len() / 2;
            let (_, median, _) = hatch_dists
                .select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            *median
        }
    };

    let mut scanline_assignments = vec![-1i32; n_total];
    let mut scanline_id: i32 = 1;
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
                scanline_id += 1;
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
                scanline_id += 1;
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
                scanline_id += 1;
                in_raster_block = false;
            }
        }
    }

    Ok(Array1::from_vec(scanline_assignments))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array1;

    #[test]
    fn test_annotate_scanlines_stub_f64() {
        let x = Array1::<f64>::zeros(100);
        let y = Array1::<f64>::zeros(100);
        let result = annotate_scanlines(x.view(), y.view()).unwrap();
        assert_eq!(result.len(), 100);
    }

    #[test]
    fn test_annotate_scanlines_stub_f32() {
        let x = Array1::<f32>::zeros(100);
        let y = Array1::<f32>::zeros(100);
        let result = annotate_scanlines(x.view(), y.view()).unwrap();
        assert_eq!(result.len(), 100);
    }
}
