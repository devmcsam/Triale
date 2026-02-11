// baseline lints, general purpose
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// compatibility lints, will switch to deny in first major release
#![warn(warnings)]
#![deny(rust_2018_idioms)]
#![deny(rust_2021_compatibility)]
#![deny(rust_2024_compatibility)]
// code quality lints
#![warn(clippy::module_name_repetitions)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::return_self_not_must_use)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::single_match_else)]
#![allow(clippy::similar_names)]
#![warn(clippy::struct_excessive_bools)]
#![warn(clippy::too_many_lines)]
#![warn(clippy::too_many_arguments)]
#![warn(clippy::type_repetition_in_bounds)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::match_bool)]
#![warn(clippy::match_same_arms)]
#![warn(clippy::unnested_or_patterns)]
// denies, mainly just ensure no panics possible
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![allow(dead_code)]

mod errors;
mod geometry;
mod io;
mod point;
mod triangle;

use crate::errors::AppError;
use crate::geometry::compute_summary;
use crate::io::read_point_with_retries;
use crate::point::Point;
use crate::triangle::build_triangle;

const POINT_LABELS: [&str; 3] = ["A", "B", "C"];

fn main() -> Result<(), AppError> {
    let mut points: [Point; 3] = [Point::zero(); 3];

    for (idx, point_slot) in points.iter_mut().enumerate() {
        let label = POINT_LABELS.get(idx).unwrap_or(&"?");
        *point_slot = read_point_with_retries(label)?;
    }

    let triangle = build_triangle(points)?;
    println!("Successfully created triangle: {triangle}");

    let summary = compute_summary(&triangle);
    println!("{summary}");

    Ok(())
}
