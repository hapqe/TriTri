mod math;
mod edges;

use edges::Caster;
use math::{*, triangle::*};

#[no_mangle]
pub extern "C" fn intersect(a: Triangle3, b: Triangle3) -> Line3 {
    if let Some(l) = a.intersect(b) {
        l
    } else {
        Line3::new(Vec3::nan(), Vec3::nan())
    }
}

// #[no_mangle]
// pub extern "C" fn calculate_edges(verts: *const Vec3, vert_count: i32, indices: *const i32, index_count: i32, transform: *const Mat4) -> (*const i32, i32) {
//     unsafe {
//         // let verts = std::slice::from_raw_parts(verts, vert_count as usize).to_vec();
//         // let indices = std::slice::from_raw_parts(indices, index_count as usize).to_vec();
//         // let transform = *transform;

//         // let caster = Caster::new(verts, indices, transform);
//         // let edges = edges::calculate_edges(caster);
//         let edges = vec![Line3::new(Vec3::nan(), Vec3::nan())];

//         (edges.as_ptr() as *const _, edges.len() as _)
//     }
// }

#[no_mangle]
pub extern "C" fn test_sum(arr: *const i32, len: i32) -> i32 {
    unsafe {
        let arr = std::slice::from_raw_parts(arr, len as usize);
        arr.iter().sum()
    }
}

#[no_mangle]
pub extern "C" fn vec_sum(arr: *const Vec3, len: i32) -> Vec3 {
    unsafe {
        let arr = std::slice::from_raw_parts(arr, len as usize);
        arr.iter().fold(Vec3::zero(), |a, b| a + *b)
    }
}

#[no_mangle]
pub extern "C" fn generate_dynamic_array() -> (usize, *mut i32) {
    let mut array = vec![1, 2, 3, 4, 5];
    let array_len = array.len();
    let array_ptr = array.as_mut_ptr();
    std::mem::forget(array); // Prevent Rust from deallocating the array
    return (array_len, array_ptr);
}