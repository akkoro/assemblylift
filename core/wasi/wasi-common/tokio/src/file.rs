use std::any::Any;
use std::io;

#[cfg(windows)]
use io_extras::os::windows::{AsHandleOrSocket, BorrowedHandleOrSocket};
#[cfg(not(windows))]
use io_lifetimes::AsFd;
use wasi_common::{
    Error,
    file::{Advice, FdFlags, Filestat, FileType, WasiFile},
};

use crate::block_on_dummy_executor;

pub struct File(wasi_cap_std_sync::file::File);

impl File {
    pub(crate) fn from_inner(file: wasi_cap_std_sync::file::File) -> Self {
        File(file)
    }
    pub fn from_cap_std(file: cap_std::fs::File) -> Self {
        Self::from_inner(wasi_cap_std_sync::file::File::from_cap_std(file))
    }
}

pub struct TcpListener(wasi_cap_std_sync::net::TcpListener);

impl TcpListener {
    pub(crate) fn from_inner(listener: wasi_cap_std_sync::net::TcpListener) -> Self {
        TcpListener(listener)
    }
    pub fn from_cap_std(listener: cap_std::net::TcpListener) -> Self {
        Self::from_inner(wasi_cap_std_sync::net::TcpListener::from_cap_std(listener))
    }
}

pub struct TcpStream(wasi_cap_std_sync::net::TcpStream);

impl TcpStream {
    pub(crate) fn from_inner(stream: wasi_cap_std_sync::net::TcpStream) -> Self {
        TcpStream(stream)
    }
    pub fn from_cap_std(stream: cap_std::net::TcpStream) -> Self {
        Self::from_inner(wasi_cap_std_sync::net::TcpStream::from_cap_std(stream))
    }
}

#[cfg(unix)]
pub struct UnixListener(wasi_cap_std_sync::net::UnixListener);

#[cfg(unix)]
impl UnixListener {
    pub(crate) fn from_inner(listener: wasi_cap_std_sync::net::UnixListener) -> Self {
        UnixListener(listener)
    }
    pub fn from_cap_std(listener: cap_std::os::unix::net::UnixListener) -> Self {
        Self::from_inner(wasi_cap_std_sync::net::UnixListener::from_cap_std(listener))
    }
}

#[cfg(unix)]
pub struct UnixStream(wasi_cap_std_sync::net::UnixStream);

#[cfg(unix)]
impl UnixStream {
    fn from_inner(stream: wasi_cap_std_sync::net::UnixStream) -> Self {
        UnixStream(stream)
    }
    pub fn from_cap_std(stream: cap_std::os::unix::net::UnixStream) -> Self {
        Self::from_inner(wasi_cap_std_sync::net::UnixStream::from_cap_std(stream))
    }
}

pub struct Stdin(wasi_cap_std_sync::stdio::Stdin);

pub fn stdin() -> Stdin {
    Stdin(wasi_cap_std_sync::stdio::stdin())
}

pub struct Stdout(wasi_cap_std_sync::stdio::Stdout);

pub fn stdout() -> Stdout {
    Stdout(wasi_cap_std_sync::stdio::stdout())
}

pub struct Stderr(wasi_cap_std_sync::stdio::Stderr);

pub fn stderr() -> Stderr {
    Stderr(wasi_cap_std_sync::stdio::stderr())
}

macro_rules! wasi_file_impl {
    ($ty:ty) => {
        #[wiggle::async_trait]
        impl WasiFile for $ty {
            fn as_any(&self) -> &dyn Any {
                self
            }
            #[cfg(unix)]
            fn pollable(&self) -> Option<rustix::fd::BorrowedFd> {
                Some(self.0.as_fd())
            }

            #[cfg(windows)]
            fn pollable(&self) -> Option<io_extras::os::windows::BorrowedHandleOrSocket> {
                Some(self.0.as_handle_or_socket())
            }

            async fn try_clone(&mut self) -> Result<Box<dyn WasiFile>, Error> {
                block_on_dummy_executor(|| self.0.try_clone())
            }
            async fn datasync(&self) -> Result<(), Error> {
                block_on_dummy_executor(|| self.0.datasync())
            }
            async fn sync(&self) -> Result<(), Error> {
                block_on_dummy_executor(|| self.0.sync())
            }
            async fn get_filetype(&mut self) -> Result<FileType, Error> {
                block_on_dummy_executor(|| self.0.get_filetype())
            }
            async fn get_fdflags(&mut self) -> Result<FdFlags, Error> {
                block_on_dummy_executor(|| self.0.get_fdflags())
            }
            async fn set_fdflags(&mut self, fdflags: FdFlags) -> Result<(), Error> {
                block_on_dummy_executor(|| self.0.set_fdflags(fdflags))
            }
            async fn get_filestat(&mut self) -> Result<Filestat, Error> {
                block_on_dummy_executor(|| self.0.get_filestat())
            }
            async fn set_filestat_size(&mut self, size: u64) -> Result<(), Error> {
                block_on_dummy_executor(move || self.0.set_filestat_size(size))
            }
            async fn advise(&mut self, offset: u64, len: u64, advice: Advice) -> Result<(), Error> {
                block_on_dummy_executor(move || self.0.advise(offset, len, advice))
            }
            async fn allocate(&mut self, offset: u64, len: u64) -> Result<(), Error> {
                block_on_dummy_executor(move || self.0.allocate(offset, len))
            }
            async fn read_vectored_at<'a>(
                &mut self,
                bufs: &mut [io::IoSliceMut<'a>],
                offset: u64,
            ) -> Result<u64, Error> {
                block_on_dummy_executor(move || self.0.read_vectored_at(bufs, offset))
            }
            fn is_read_vectored_at(&self) -> bool {
                self.0.is_read_vectored_at()
            }
            async fn write_vectored_at<'a>(
                &mut self,
                bufs: &[io::IoSlice<'a>],
                offset: u64,
            ) -> Result<u64, Error> {
                block_on_dummy_executor(move || self.0.write_vectored_at(bufs, offset))
            }
            fn is_write_vectored_at(&self) -> bool {
                self.0.is_write_vectored_at()
            }
            async fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64, Error> {
                block_on_dummy_executor(move || self.0.seek(pos))
            }
            async fn set_times(
                &mut self,
                atime: Option<wasi_common::SystemTimeSpec>,
                mtime: Option<wasi_common::SystemTimeSpec>,
            ) -> Result<(), Error> {
                block_on_dummy_executor(move || self.0.set_times(atime, mtime))
            }
            async fn num_ready_bytes(&self) -> Result<u64, Error> {
                block_on_dummy_executor(|| self.0.num_ready_bytes())
            }
            fn isatty(&mut self) -> bool {
                self.0.isatty()
            }

            #[cfg(not(windows))]
            async fn readable(&self) -> Result<(), Error> {
                // The Inner impls OwnsRaw, which asserts exclusive use of the handle by the owned object.
                // AsyncFd needs to wrap an owned `impl std::os::unix::io::AsRawFd`. Rather than introduce
                // mutability to let it own the `Inner`, we are depending on the `&mut self` bound on this
                // async method to ensure this is the only Future which can access the RawFd during the
                // lifetime of the AsyncFd.
                use std::os::unix::io::AsRawFd;
                use tokio::io::{unix::AsyncFd, Interest};
                let rawfd = self.0.as_fd().as_raw_fd();
                match AsyncFd::with_interest(rawfd, Interest::READABLE) {
                    Ok(asyncfd) => {
                        let _ = asyncfd.readable().await?;
                        Ok(())
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                        // if e is EPERM, this file isnt supported by epoll because it is immediately
                        // available for reading:
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                }
            }

            #[cfg(not(windows))]
            async fn writable(&self) -> Result<(), Error> {
                // The Inner impls OwnsRaw, which asserts exclusive use of the handle by the owned object.
                // AsyncFd needs to wrap an owned `impl std::os::unix::io::AsRawFd`. Rather than introduce
                // mutability to let it own the `Inner`, we are depending on the `&mut self` bound on this
                // async method to ensure this is the only Future which can access the RawFd during the
                // lifetime of the AsyncFd.
                use std::os::unix::io::AsRawFd;
                use tokio::io::{unix::AsyncFd, Interest};
                let rawfd = self.0.as_fd().as_raw_fd();
                match AsyncFd::with_interest(rawfd, Interest::WRITABLE) {
                    Ok(asyncfd) => {
                        let _ = asyncfd.writable().await?;
                        Ok(())
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                        // if e is EPERM, this file isnt supported by epoll because it is immediately
                        // available for writing:
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                }
            }

            async fn accept(&mut self, fdflags: FdFlags) -> Result<Box<dyn WasiFile>, Error> {
                block_on_dummy_executor(|| self.0.accept(fdflags))
            }
        }
        #[cfg(windows)]
        impl AsHandleOrSocket for $ty {
            #[inline]
            fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket {
                self.0.as_handle_or_socket()
            }
        }
    };
}

wasi_file_impl!(File);
wasi_file_impl!(TcpListener);
wasi_file_impl!(TcpStream);
#[cfg(unix)]
wasi_file_impl!(UnixListener);
#[cfg(unix)]
wasi_file_impl!(UnixStream);
wasi_file_impl!(Stdin);
wasi_file_impl!(Stdout);
wasi_file_impl!(Stderr);
