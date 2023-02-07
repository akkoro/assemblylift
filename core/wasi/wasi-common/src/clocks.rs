use cap_std::time::Duration;

use crate::Error;

pub trait WasiWallClock: Send + Sync {
    fn resolution(&self) -> Duration;
    fn now(&self) -> Duration;
    fn dup(&self) -> Box<dyn WasiWallClock + Send + Sync>;
}

pub trait WasiMonotonicClock: Send + Sync {
    fn resolution(&self) -> u64;
    fn now(&self) -> u64;
    fn dup(&self) -> Box<dyn WasiMonotonicClock + Send + Sync>;
}

pub struct WasiClocks {
    pub default_wall_clock: Box<dyn WasiWallClock + Send + Sync>,
    pub default_monotonic_clock: Box<dyn WasiMonotonicClock + Send + Sync>,
}

pub trait TableWallClockExt {
    fn get_wall_clock(&self, fd: u32) -> Result<&(dyn WasiWallClock + Send + Sync), Error>;
    fn get_wall_clock_mut(
        &mut self,
        fd: u32,
    ) -> Result<&mut Box<dyn WasiWallClock + Send + Sync>, Error>;
    fn delete_wall_clock(&mut self, fd: u32) -> Result<(), Error>;
}
impl TableWallClockExt for crate::table::Table {
    fn get_wall_clock(&self, fd: u32) -> Result<&(dyn WasiWallClock + Send + Sync), Error> {
        self.get::<Box<dyn WasiWallClock + Send + Sync>>(fd)
            .map(|f| f.as_ref())
    }
    fn get_wall_clock_mut(
        &mut self,
        fd: u32,
    ) -> Result<&mut Box<dyn WasiWallClock + Send + Sync>, Error> {
        self.get_mut::<Box<dyn WasiWallClock + Send + Sync>>(fd)
    }
    fn delete_wall_clock(&mut self, fd: u32) -> Result<(), Error> {
        self.delete::<Box<dyn WasiWallClock + Send + Sync>>(fd)
            .map(|_old| ())
    }
}

pub trait TableMonotonicClockExt {
    fn get_monotonic_clock(
        &self,
        fd: u32,
    ) -> Result<&(dyn WasiMonotonicClock + Send + Sync), Error>;
    fn get_monotonic_clock_mut(
        &mut self,
        fd: u32,
    ) -> Result<&mut Box<dyn WasiMonotonicClock + Send + Sync>, Error>;
    fn delete_monotonic_clock(&mut self, fd: u32) -> Result<(), Error>;
}
impl TableMonotonicClockExt for crate::table::Table {
    fn get_monotonic_clock(
        &self,
        fd: u32,
    ) -> Result<&(dyn WasiMonotonicClock + Send + Sync), Error> {
        self.get::<Box<dyn WasiMonotonicClock + Send + Sync>>(fd)
            .map(|f| f.as_ref())
    }
    fn get_monotonic_clock_mut(
        &mut self,
        fd: u32,
    ) -> Result<&mut Box<dyn WasiMonotonicClock + Send + Sync>, Error> {
        self.get_mut::<Box<dyn WasiMonotonicClock + Send + Sync>>(fd)
    }
    fn delete_monotonic_clock(&mut self, fd: u32) -> Result<(), Error> {
        self.delete::<Box<dyn WasiMonotonicClock + Send + Sync>>(fd)
            .map(|_old| ())
    }
}
