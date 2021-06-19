//
// Adapted the insertion exapmle from https://c2rust.com/
//

#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
//#![register_tool(c2rust)]
//#![feature(register_tool)]
#[no_mangle]
pub unsafe extern "C" fn insertion_sort(n: libc::c_int, p: *mut libc::c_int) {
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < n {
        let tmp: libc::c_int = *p.offset(i as isize);
        let mut j: libc::c_int = i;
        while j > 0 as libc::c_int &&
                  *p.offset((j - 1 as libc::c_int) as isize) > tmp {
            *p.offset(j as isize) =
                *p.offset((j - 1 as libc::c_int) as isize);
            j -= 1
        }
        *p.offset(j as isize) = tmp;
        i += 1
    };
}

fn main() {
    let mut array = vec![3,1,4,6,3,9];
    println!("before = {:?}", array);
    unsafe {
      insertion_sort(array.len() as i32, array.as_mut_ptr());
    }
    println!("after  = {:?}", array);
}
