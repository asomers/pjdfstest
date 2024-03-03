/// Create a test case which asserts that the syscall returns EACCES
/// if the operation attempts to write within a directory that denies write
/// permission.
///
/// The supplied expression will be called with one path argument.  The path will lie within an
/// unwritable directory.
macro_rules! eacces_parent_dir_unwritable_test_case {
    ($syscall: ident, $f: expr $(; $attrs:tt )?) => {
        crate::test_case! {
            #[doc = concat!(stringify!($syscall),
                 " returns EACCESS if it requires writing to a directory that",
                 " denies write permission")]
            eacces_parent_dir_unwritable, serialized, root $(, $attrs )?
        }
        fn eacces_parent_dir_unwritable(ctx: &mut crate::SerializedTestContext) {
            use nix::errno::Errno;

            let dir = ctx
                .new_file(crate::context::FileType::Dir)
                .name("unwriteable_dir")
                .mode(0o555)
                .create()
                .unwrap();
            let path = dir.join("foo");

            ctx.as_user(ctx.get_new_user(), None, || {
                assert_eq!($f(ctx, &path), Err(Errno::EACCES));
            });
        }
    };

    ($syscall: ident $( ($( $($before:expr),* ,)? ~path $(, $($after:expr),*)?) )?) => {
        eacces_parent_dir_unwritable_test_case !($syscall, |_ctx: &crate::context::SerializedTestContext,
                                             path: &std::path::Path| {
                $syscall($( $($($before),* ,)? )? path $( $(, $($after),*)? )?)
        });
    };
}

/// Create a test case which asserts that the syscall returns EACCES
/// if the operation attempts to write within a directory that denies write
/// permission.
///
/// The supplied expression will be called with one writable directory and one
/// unwritable directory.
macro_rules! eacces_parent_dir_unwritable_test_case2 {
    ($syscall: ident $(; $attrs:tt )?) => {
        crate::test_case! {
            #[doc = concat!(stringify!($syscall),
                 " returns EACCESS if it requires writing to a directory that",
                 " denies write permission")]
            eacces_parent_dir_unwritable, serialized, root $(, $attrs )?
        }
        fn eacces_parent_dir_unwritable(ctx: &mut crate::SerializedTestContext) {
            use nix::errno::Errno;

            let rwdir = ctx
                .new_file(crate::context::FileType::Dir)
                .name("writable_dir")
                .mode(0o777)
                .create()
                .unwrap();
            let rodir = ctx
                .new_file(crate::context::FileType::Dir)
                .name("unwritable_dir")
                .mode(0o555)
                .create()
                .unwrap();
            let srcpath = rwdir.join("src");
            let dstpath = rodir.join("dst");
            let dstpath2 = rodir.join("dst2");
            let dstpath3 = rodir.join("dst3");
            ::std::fs::File::create(&srcpath).unwrap();
            ::std::fs::File::create(&dstpath2).unwrap();
            ctx.as_user(ctx.get_new_user(), None, || {
                // Destination directory is unwritable
                assert_eq!($syscall(&srcpath, &dstpath), Err(Errno::EACCES));
                // Both directories are the same
                assert_eq!($syscall(&dstpath2, &dstpath3), Err(Errno::EACCES));
            });
        }
    };
}

/// Create a test case which asserts that a syscall fails with EACCES if a component of the path is
/// not searchable.
///
/// The supplied expression will be called with one path argument.
macro_rules! eacces_search_permission_denied_test_case {
    ($syscall: ident, $f: expr $(; $attrs:tt )?) => {
        crate::test_case! {
            #[doc = concat!(stringify!($syscall),
                " returns EACCES when search permission is denied for a",
                " component of the path prefix")]
            eacces_search_permission_denied, serialized, root $(, $attrs )?
        }
        fn eacces_search_permission_denied(ctx: &mut crate::SerializedTestContext) {
            use nix::errno::Errno;

            let dir = ctx
                .new_file(crate::context::FileType::Dir)
                .name("unsearchable_dir")
                .mode(0o666)
                .create()
                .unwrap();
            let path = dir.join("foo");

            ctx.as_user(ctx.get_new_user(), None, || {
                assert_eq!($f(ctx, &path), Err(Errno::EACCES));
            });
        }
    };

    ($syscall: ident $( ($( $($before:expr),* ,)? ~path $(, $($after:expr),*)?) )?) => {
        eacces_search_permission_denied_test_case !($syscall, |_ctx: &crate::context::SerializedTestContext,
                                             path: &std::path::Path| {
                $syscall($( $($($before),* ,)? )? path $( $(, $($after),*)? )?)
        });
    };
}

/// Create a test case which asserts that a syscall fails with EACCES if a component of the path is
/// not searchable.  For syscalls that involve two directories.
macro_rules! eacces_search_permission_denied_test_case2 {
    ($syscall: ident $(; $attrs:tt )?) => {
        crate::test_case! {
            #[doc = concat!(stringify!($syscall),
                " returns EACCES when search permission is denied for a",
                " component of the path prefix")]
            eacces_search_permission_denied2, serialized, root $(, $attrs )?
        }
        fn eacces_search_permission_denied2(ctx: &mut crate::SerializedTestContext) {
            use nix::errno::Errno;

            let rwdir = ctx
                .new_file(crate::context::FileType::Dir)
                .name("writable_dir")
                .mode(0o777)
                .create()
                .unwrap();
            let usdir = ctx
                .new_file(crate::context::FileType::Dir)
                .name("unsearchable_dir")
                .mode(0o666)
                .create()
                .unwrap();
            let srcpath = rwdir.join("src");
            let dstpath = usdir.join("dst");
            let dstpath2 = usdir.join("dst2");
            ::std::fs::File::create(&srcpath).unwrap();
            ctx.as_user(ctx.get_new_user(), None, || {
                // Destination directory is unsearchable
                assert_eq!($syscall(&srcpath, &dstpath), Err(Errno::EACCES));
                // Source directory is unsearchable
                assert_eq!($syscall(&dstpath, &srcpath), Err(Errno::EACCES));
                // Both directories are the same
                assert_eq!($syscall(&dstpath, &dstpath2), Err(Errno::EACCES));
            });
        }
    };
}


pub(crate) use eacces_parent_dir_unwritable_test_case;
pub(crate) use eacces_parent_dir_unwritable_test_case2;
pub(crate) use eacces_search_permission_denied_test_case;
pub(crate) use eacces_search_permission_denied_test_case2;
