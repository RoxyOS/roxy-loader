use os_test_framework::test;

test! {
    "loader transferred control to test-kernel" {
        assert!(true);
    }
}

test! {
    "heap allocations work in target environment" {
        let values = alloc::vec![1_u64, 2, 3, 5, 8];
        assert_eq!(values.as_slice(), &[1, 2, 3, 5, 8]);
    }
}
