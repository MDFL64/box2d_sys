/*#[no_mangle]
unsafe extern "C" fn malloc(size: i32) -> i32 {
    let layout = std::alloc::Layout::from_size_align_unchecked(size as usize,16);
    return std::alloc::alloc( layout ) as i32;
}*/

extern "C" {
    fn log_fixture(a: &str, b: i32, c: i32);
}

// BIOHAZARD WARNING:
//
// In order to deal with the rust allocator api, this stores the allocation's size in the space before
// the allocation. I tried to pull the size out of the allocator's internals, but the validation
// adds in padding which I don't want to bother with. This adds an extra 16 bytes per allocation but at least
// it should be a bit safer if the allocator changes.

// box2d may request alignment of 32 (for 32 byte simd (which we don't use)), hopefully this isn't an issue
const STATIC_ALIGN: i32 = 16;

#[no_mangle]
unsafe extern "C" fn aligned_alloc(_align: i32, size: i32) -> i32 {
    let size = size + STATIC_ALIGN;
    let layout = std::alloc::Layout::from_size_align_unchecked(size as usize,STATIC_ALIGN as usize);
    let ptr = std::alloc::alloc( layout ) as i32;
    (ptr as *mut i32).write(size);
    ptr + STATIC_ALIGN
}

#[no_mangle]
unsafe extern "C" fn free(ptr: i32) {
    let ptr = ptr - STATIC_ALIGN;
    let size = (ptr as *mut i32).read();
    let layout = std::alloc::Layout::from_size_align_unchecked(size as usize,STATIC_ALIGN as usize);
    std::alloc::dealloc(ptr as *mut u8, layout);
}

type compare_fn = extern "C" fn(a: *const void, b: *const void)->i32;
type void = std::ffi::c_void;

#[no_mangle]
unsafe extern "C" fn qsort(base: *mut void, count: i32, size: i32, compare: compare_fn) {
    if size == 8 {
        qsort_inner::<8>(base,count,compare);
    } else {
        panic!("qsort failed {}",size);
    }
}

// another very stupid hack
unsafe fn qsort_inner<const SIZE: usize>(base: *mut void, count: i32, compare: compare_fn) {
    let slice = std::slice::from_raw_parts_mut(base as *mut [u8;SIZE],count as usize);
    slice.sort_by(|a,b| compare(a.as_ptr() as _,b.as_ptr() as _).cmp(&0));
}

#[no_mangle]
unsafe extern "C" fn remainderf(x: f32, y: f32) -> f32 {
    let n = (x / y).round_ties_even();
    x - n * y
}

/*fn compare_ints(a: *const void, b: *const void) -> i32 {
    let arg1 = unsafe{ (a as *const i64).read() };
    let arg2 = unsafe{ (b as *const i64).read() };

    if (arg1 < arg2) { return -1 }
    if (arg1 > arg2) { return 1 }
    0
}

unsafe fn qsort_test() {
    let mut nums: &mut [i64] = &mut [-2, 99, 0, -743, 2, -2147483648, 4];
    qsort(nums as *mut [i64] as *mut void,nums.len() as i32, 8, compare_ints);

    for (i,x) in nums.iter().enumerate() {
        log_fixture("sort",i as i32,*x as i32);
    }
    panic!("stop");
}*/
