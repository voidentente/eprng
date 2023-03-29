extern crate alloc;

use core::fmt;
use alloc::collections::BTreeMap;
use alloc::string::String;

type DistributionMap<T> = BTreeMap<T, usize>;

#[derive(Debug)]
pub struct Distribution<T>(pub DistributionMap<T>);

/// Calculates the distribution of the slice.
/// Can be visualized with `Display`:
///
/// ```
/// let mut buf = [0u8; 16384];
///
/// eprng::bytes(&mut buf, eprng::initial_offset());
///
/// let distribution = eprng::distribution::distribution(&buf);
///
/// println!("{distribution}");
/// ```
pub fn distribution<T: Ord + Copy>(buf: &[T]) -> Distribution<T> {
    let mut tree = BTreeMap::<T, usize>::new();
    for val in buf  {
        if let Some(entry) = tree.get_mut(val) {
            *entry += 1;
        } else {
            tree.insert(*val, 1);
        }
    }
    Distribution(tree)
}

fn extremes<T>(tree: &DistributionMap<T>) -> (usize, usize) {
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for v in tree.values().cloned() {
        min = min.min(v);
        max = max.max(v);
    }
    (min, max)
}

impl<T: fmt::Display> fmt::Display for Distribution<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min, max) = extremes(&self.0);
        let mut buf = String::new();
        buf.push_str("Minimum Value: ");
        buf.push_str(&alloc::string::ToString::to_string(&min));
        buf.push('\n');
        buf.push_str("Maximum Value: ");
        buf.push_str(&alloc::string::ToString::to_string(&max));
        buf.push_str("\n\n");
        for i in (0..10).rev() {
            for (k, v) in &self.0 {
                let width = alloc::format!("{k}").len().max(alloc::format!("{v}").len());
                match v {
                    _ if *v as f32 >= (max as f32 / 100.) * (i * 10 + 5) as f32 => {
                        for _ in 0..width {
                            buf.push('█');
                        }
                    }
                    _ if *v as f32 >= (max as f32 / 100.) * (i * 10) as f32 => {
                        for _ in 0..width {
                            buf.push('▄');
                        }
                    }
                    _ => {
                        for _ in 0..width {
                            buf.push(' ');
                        }
                    }
                }
                buf.push(' ');
            }
            buf.push('\n');
        }
        for (k, v) in &self.0 {
            let mut k = alloc::format!("{k}");
            let v = alloc::format!("{v}");
            let width = k.len().max(v.len());
            while k.len() < width {
                k.push(' ');
            }
            buf.push_str(&k);
            buf.push(' ');
        }
        buf.push('\n');
        for (k, v) in &self.0 {
            let k = alloc::format!("{k}");
            let mut v = alloc::format!("{v}");
            let width = k.len().max(v.len());
            while v.len() < width {
                v.push(' ');
            }
            buf.push_str(&v);
            buf.push(' ');
        }
        f.write_str(&buf)
    }
}
