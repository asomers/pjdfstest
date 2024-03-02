/// Create a test case which asserts that the syscall returns EACCES
/// if the operation attempts to write within a directory that denies write
/// permission.
///
/// The supplied expression will be called with one writable directory and one
/// unwritable directory.
macro_rules! eacces_parent_dir_unwritable_test_case {
    ($syscall: ident, $f: expr $(; $attrs:tt )?) => {
        crate::test_case! {
            #[doc = concat!(stringify!($syscall),
                 " returns EACCESS if it requires writing to a directory that",
                 " denies write permission")]
            eaccess_parent_dir_unwritable, serialized, root $(, $attrs )?
        }
        fn eaccess_parent_dir_unwritable(ctx: &mut SerializedTestContext) {
            use nix::errno::Errno;

            let dir1 = ctx
                .new_file(crate::context::FileType::Dir)
                .name("writable_dir")
                .mode(0o777)
                .create()
                .unwrap();
            let dir2 = ctx
                .new_file(crate::context::FileType::Dir)
                .name("unwritable_dir")
                .mode(0o500)
                .create()
                .unwrap();
            ctx.as_user(ctx.get_new_user(), None, || {
                assert_eq!($f(ctx, &dir1, &dir2), Err(Errno::EACCES));
            });
        }
    };
}

pub(crate) use eacces_parent_dir_unwritable_test_case ;


