use os_test_framework::test;

use crate::bootinfo::bootinfo;

test!(loader_transferred_control_to_test_kernel);
fn loader_transferred_control_to_test_kernel() {
    assert!(true);
}

test!(heap_allocations_work_in_target_environment);
fn heap_allocations_work_in_target_environment() {
    let values = alloc::vec![1_u64, 2, 3, 5, 8];
    assert_eq!(values.as_slice(), &[1, 2, 3, 5, 8]);
}

test!(framebuffer_is_mapped_and_writable);
fn framebuffer_is_mapped_and_writable() {
    let framebuffer = &bootinfo().framebuffer;
    let ptr = framebuffer.ptr();

    assert!(!ptr.is_null());
    assert!(framebuffer.size > 0);
    assert!(framebuffer.stride > 0);

    unsafe {
        let first = ptr;
        let original = first.read_volatile();

        first.write_volatile(original ^ 0xff);
        let updated = first.read_volatile();
        assert_eq!(updated, original ^ 0xff);

        first.write_volatile(original);
        assert_eq!(first.read_volatile(), original);
    }
}
