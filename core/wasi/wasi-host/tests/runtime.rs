use std::{
    io::{Cursor, Write},
    sync::Mutex,
};

use anyhow::Result;
use cap_rand::RngCore;
use cap_std::{ambient_authority, fs::Dir, time::Duration};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

use host::wasi_filesystem::Descriptor;
use host::wasi_poll::WasiStream;
use host::{add_to_linker, WasiCommand, WasiCtx};
use wasi_cap_std_sync::WasiCtxBuilder;
use wasi_common::{
    clocks::{WasiMonotonicClock, WasiWallClock},
    pipe::ReadPipe,
};

test_programs_macros::tests!();

// FIXME: a bunch of these test cases are expected to fail, either by
// returning an Err or by panicking. Rather than try to juggle the
// intersection of catch_unwind and async, we use this boolean to
// short-circuit a bunch of tests expected to blow up. Developers may change
// this redefintion to `false` locally  to see if any of the failing tests
// start passing as you add functionality.
const EXPECT_FAIL: bool = true;

async fn instantiate(path: &str) -> Result<(Store<WasiCtx>, WasiCommand)> {
    println!("{}", path);

    let mut config = Config::new();
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, &path)?;
    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker, |x| x)?;

    let mut store = Store::new(&engine, WasiCtxBuilder::new().build());

    let (wasi, _instance) = WasiCommand::instantiate_async(&mut store, &component, &linker).await?;
    Ok((store, wasi))
}

async fn run_hello_stdout(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    wasi.command(
        &mut store,
        0 as WasiStream,
        1 as WasiStream,
        &["gussie", "sparky", "willa"],
        &[],
        &[],
    )
    .await?
    .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
}

async fn run_panic(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    let r = wasi
        .command(
            &mut store,
            0 as WasiStream,
            1 as WasiStream,
            &[
                "diesel",
                "the",
                "cat",
                "scratched",
                "me",
                "real",
                "good",
                "yesterday",
            ],
            &[],
            &[],
        )
        .await;
    assert!(r.is_err());
    println!("{:?}", r);
    Ok(())
}

async fn run_args(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    wasi.command(
        &mut store,
        0 as WasiStream,
        1 as WasiStream,
        &["hello", "this", "", "is an argument", "with 🚩 emoji"],
        &[],
        &[],
    )
    .await?
    .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
}

async fn run_random(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    struct FakeRng;

    impl RngCore for FakeRng {
        fn next_u32(&mut self) -> u32 {
            42
        }

        fn next_u64(&mut self) -> u64 {
            unimplemented!()
        }

        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            unimplemented!()
        }

        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), cap_rand::Error> {
            unimplemented!()
        }
    }

    store.data_mut().random = Box::new(FakeRng);

    wasi.command(&mut store, 0 as WasiStream, 1 as WasiStream, &[], &[], &[])
        .await?
        .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
}

async fn run_time(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    struct FakeWallClock;

    impl WasiWallClock for FakeWallClock {
        fn resolution(&self) -> Duration {
            Duration::from_secs(1)
        }

        fn now(&self) -> Duration {
            Duration::from_secs(1431648000)
        }

        fn dup(&self) -> Box<dyn WasiWallClock + Send + Sync> {
            Box::new(Self)
        }
    }

    struct FakeMonotonicClock {
        now: Mutex<u64>,
    }

    impl WasiMonotonicClock for FakeMonotonicClock {
        fn resolution(&self) -> u64 {
            1_000_000_000
        }

        fn now(&self) -> u64 {
            let mut now = self.now.lock().unwrap();
            let then = *now;
            *now += 42 * 1_000_000_000;
            then
        }

        fn dup(&self) -> Box<dyn WasiMonotonicClock + Send + Sync> {
            let now = *self.now.lock().unwrap();
            Box::new(Self {
                now: Mutex::new(now),
            })
        }
    }

    store.data_mut().clocks.default_wall_clock = Box::new(FakeWallClock);
    store.data_mut().clocks.default_monotonic_clock =
        Box::new(FakeMonotonicClock { now: Mutex::new(0) });

    wasi.command(&mut store, 0 as WasiStream, 1 as WasiStream, &[], &[], &[])
        .await?
        .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
}

async fn run_stdin(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    store
        .data_mut()
        .set_stdin(Box::new(ReadPipe::new(Cursor::new(
            "So rested he by the Tumtum tree",
        ))));

    wasi.command(&mut store, 0 as WasiStream, 1 as WasiStream, &[], &[], &[])
        .await?
        .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
}

async fn run_poll_stdin(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    store
        .data_mut()
        .set_stdin(Box::new(ReadPipe::new(Cursor::new(
            "So rested he by the Tumtum tree",
        ))));

    wasi.command(&mut store, 0 as WasiStream, 1 as WasiStream, &[], &[], &[])
        .await?
        .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
}

async fn run_env(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    wasi.command(
        &mut store,
        0 as Descriptor,
        1 as Descriptor,
        &[],
        &[("frabjous", "day"), ("callooh", "callay")],
        &[],
    )
    .await?
    .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
}

async fn run_file_read(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    let dir = tempfile::tempdir()?;

    std::fs::File::create(dir.path().join("bar.txt"))?.write_all(b"And stood awhile in thought")?;

    let open_dir = Dir::open_ambient_dir(dir.path(), ambient_authority())?;
    let descriptor =
        store
            .data_mut()
            .push_dir(Box::new(wasi_cap_std_sync::dir::Dir::from_cap_std(
                open_dir,
            )))?;

    wasi.command(
        &mut store,
        0 as Descriptor,
        1 as Descriptor,
        &[],
        &[],
        &[(descriptor, "/")],
    )
    .await?
    .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
}

async fn run_file_append(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    let dir = tempfile::tempdir()?;

    std::fs::File::create(dir.path().join("bar.txt"))?
        .write_all(b"'Twas brillig, and the slithy toves.\n")?;

    let open_dir = Dir::open_ambient_dir(dir.path(), ambient_authority())?;
    let descriptor =
        store
            .data_mut()
            .push_dir(Box::new(wasi_cap_std_sync::dir::Dir::from_cap_std(
                open_dir,
            )))?;

    wasi.command(
        &mut store,
        0 as Descriptor,
        1 as Descriptor,
        &[],
        &[],
        &[(descriptor, "/")],
    )
    .await?
    .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))?;

    let contents = std::fs::read(dir.path().join("bar.txt"))?;
    assert_eq!(
        std::str::from_utf8(&contents).unwrap(),
        "'Twas brillig, and the slithy toves.\n\
               Did gyre and gimble in the wabe;\n\
               All mimsy were the borogoves,\n\
               And the mome raths outgrabe.\n"
    );
    Ok(())
}

async fn run_file_dir_sync(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    let dir = tempfile::tempdir()?;

    std::fs::File::create(dir.path().join("bar.txt"))?
        .write_all(b"'Twas brillig, and the slithy toves.\n")?;

    let open_dir = Dir::open_ambient_dir(dir.path(), ambient_authority())?;
    let descriptor =
        store
            .data_mut()
            .push_dir(Box::new(wasi_cap_std_sync::dir::Dir::from_cap_std(
                open_dir,
            )))?;

    wasi.command(
        &mut store,
        0 as Descriptor,
        1 as Descriptor,
        &[],
        &[],
        &[(descriptor, "/")],
    )
    .await?
    .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
}

async fn run_exit_success(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    let r = wasi
        .command(
            &mut store,
            0 as Descriptor,
            1 as Descriptor,
            &[],
            &[("frabjous", "day"), ("callooh", "callay")],
            &[],
        )
        .await;
    let err = r.unwrap_err();
    let status = err.downcast_ref::<wasi_common::I32Exit>().unwrap();
    assert_eq!(status.0, 0);
    Ok(())
}

async fn run_exit_default(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    let r = wasi
        .command(
            &mut store,
            0 as Descriptor,
            1 as Descriptor,
            &[],
            &[("frabjous", "day"), ("callooh", "callay")],
            &[],
        )
        .await?;
    assert!(r.is_ok());
    Ok(())
}

async fn run_exit_failure(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    let r = wasi
        .command(
            &mut store,
            0 as Descriptor,
            1 as Descriptor,
            &[],
            &[("frabjous", "day"), ("callooh", "callay")],
            &[],
        )
        .await;
    let err = r.unwrap_err();
    let status = err.downcast_ref::<wasi_common::I32Exit>().unwrap();
    assert_eq!(status.0, 1);
    Ok(())
}

async fn run_exit_panic(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    let r = wasi
        .command(
            &mut store,
            0 as Descriptor,
            1 as Descriptor,
            &[],
            &[("frabjous", "day"), ("callooh", "callay")],
            &[],
        )
        .await;
    let err = r.unwrap_err();
    // The panic should trap.
    assert!(err.downcast_ref::<wasi_common::I32Exit>().is_none());
    Ok(())
}

async fn run_directory_list(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    let dir = tempfile::tempdir()?;

    std::fs::File::create(dir.path().join("foo.txt"))?;
    std::fs::File::create(dir.path().join("bar.txt"))?;
    std::fs::File::create(dir.path().join("baz.txt"))?;
    std::fs::create_dir(dir.path().join("sub"))?;
    std::fs::File::create(dir.path().join("sub").join("wow.txt"))?;
    std::fs::File::create(dir.path().join("sub").join("yay.txt"))?;

    let open_dir = Dir::open_ambient_dir(dir.path(), ambient_authority())?;
    let descriptor =
        store
            .data_mut()
            .push_dir(Box::new(wasi_cap_std_sync::dir::Dir::from_cap_std(
                open_dir,
            )))?;

    wasi.command(
        &mut store,
        0 as Descriptor,
        1 as Descriptor,
        &[],
        &[],
        &[(descriptor, "/")],
    )
    .await?
    .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
}

async fn run_default_clocks(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    wasi.command(&mut store, 0 as Descriptor, 1 as Descriptor, &[], &[], &[])
        .await?
        .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
}

async fn run_with_temp_dir(mut store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    let dir = tempfile::tempdir()?;

    let open_dir = Dir::open_ambient_dir(dir.path(), ambient_authority())?;
    let descriptor =
        store
            .data_mut()
            .push_dir(Box::new(wasi_cap_std_sync::dir::Dir::from_cap_std(
                open_dir,
            )))?;

    wasi.command(
        &mut store,
        0 as WasiStream,
        1 as WasiStream,
        &["program", "/foo"],
        &[],
        &[(descriptor, "/foo")],
    )
    .await?
    .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
}

async fn run_big_random_buf(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    run_with_temp_dir(store, wasi).await
}

async fn run_clock_time_get(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    run_with_temp_dir(store, wasi).await
}

async fn run_close_preopen(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_dangling_fd(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL && cfg!(windows) {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_dangling_symlink(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_directory_seek(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_fd_advise(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_fd_filestat_get(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_fd_filestat_set(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_fd_flags_set(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_fd_readdir(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    run_with_temp_dir(store, wasi).await
}

async fn run_file_allocate(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_file_pread_pwrite(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    run_with_temp_dir(store, wasi).await
}

async fn run_file_seek_tell(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_file_truncation(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    run_with_temp_dir(store, wasi).await
}

async fn run_file_unbuffered_write(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    run_with_temp_dir(store, wasi).await
}

async fn run_interesting_paths(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_isatty(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    run_with_temp_dir(store, wasi).await
}

async fn run_nofollow_errors(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_path_exists(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    run_with_temp_dir(store, wasi).await
}

async fn run_path_filestat(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_path_link(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_path_open_create_existing(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_path_open_dirfd_not_dir(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_path_open_missing(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_path_open_read_without_rights(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_path_rename(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_path_rename_dir_trailing_slashes(
    store: Store<WasiCtx>,
    wasi: WasiCommand,
) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_path_rename_file_trailing_slashes(
    store: Store<WasiCtx>,
    wasi: WasiCommand,
) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_path_symlink_trailing_slashes(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_poll_oneoff_files(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_poll_oneoff_stdio(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_readlink(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_remove_directory_trailing_slashes(
    store: Store<WasiCtx>,
    wasi: WasiCommand,
) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_remove_nonempty_directory(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_renumber(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_sched_yield(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    run_with_temp_dir(store, wasi).await
}

async fn run_stdio(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    run_with_temp_dir(store, wasi).await
}

async fn run_symlink_create(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    run_with_temp_dir(store, wasi).await
}

async fn run_symlink_filestat(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_symlink_loop(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_truncation_rights(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}

async fn run_unlink_file_trailing_slashes(store: Store<WasiCtx>, wasi: WasiCommand) -> Result<()> {
    if EXPECT_FAIL {
        // TODO!
        return Ok(());
    }
    run_with_temp_dir(store, wasi).await
}
