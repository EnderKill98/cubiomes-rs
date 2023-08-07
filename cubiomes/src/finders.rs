use std::mem::MaybeUninit;
use std::ffi::{c_int, c_void};

pub use libcubiomes_sys::BiomeID;
pub use libcubiomes_sys::Dimension;
pub use libcubiomes_sys::MCVersion;

pub struct CubiomesFinder {
    generator: MaybeUninit<libcubiomes_sys::Generator>,
}

impl CubiomesFinder {
    pub fn new(seed: i64, version: MCVersion, dim: Dimension) -> Self {
        unsafe {
            let mut finder = CubiomesFinder {
                generator: MaybeUninit::zeroed(),
            };
            libcubiomes_sys::setupGenerator(finder.generator.as_mut_ptr(), version as c_int, 0);
            libcubiomes_sys::applySeed(finder.generator.as_mut_ptr(), dim, seed as u64);
            finder
        }
    }

    pub fn apply_seed(&mut self, seed: i64, dim: Dimension) {
        unsafe {
            libcubiomes_sys::applySeed(self.generator.as_mut_ptr(), dim, seed as u64);
        }
    }

    pub fn get_biome_at(&self, x: i32, y: i32, z: i32) -> BiomeID {
        unsafe {
            libcubiomes_sys::getBiomeAt(
                self.generator.as_ptr(),
                1, /* = Block coords */
                x,
                y,
                z,
            )
        }
    }
}

pub enum CoordScaling {
    /// 1:1 block scaling
    Block,
    /// 1:4 scaling
    Quad,
    /// 1:16 chunk scaling
    Chunk,
    /// 1:64 scaling
    QuadChunk,
    /// 1:256 scaling (**Overworld only**)
    Region,
}

impl CoordScaling {
    pub fn value(&self) -> i32 {
        match self {
            CoordScaling::Block => 1,
            CoordScaling::Quad => 4,
            CoordScaling::Chunk => 16,
            CoordScaling::QuadChunk => 64,
            CoordScaling::Region => 256,
        }
    }
}

/// https://github.com/Cubitect/cubiomes/tree/master#biome-generation-in-a-range
pub struct BiomeCache {
    x: i32,
    y: i32,
    z: i32,
    sx: i32,
    sy: i32,
    sz: i32,
    biome_id_cache: *mut BiomeID,
}

impl BiomeCache {
    pub fn new(
        finder: &CubiomesFinder,
        scale: CoordScaling,
        range_start: (i32, i32, i32), // (x, y, z)
        range_size: (i32, i32, i32),  // (sx, sy, sz)
    ) -> Self {
        unsafe {
            let r = libcubiomes_sys::Range {
                scale: scale.value(), // Divide your input coordinates by this value
                // Define the position and size for a horizontal area:
                x: range_start.0, // position (x, y, z)
                y: range_start.1,
                z: range_start.2,
                sx: range_size.0, // size (width, vertical range, height)
                sy: range_size.1, // Set the vertical range as a plane near sea level at scale 1:4 (unless Coordscaling == Block [1:1]).
                sz: range_size.2,
            };
            let biome_id_cache: *mut BiomeID =
                libcubiomes_sys::allocCache(finder.generator.as_ptr(), r);
            libcubiomes_sys::genBiomes(finder.generator.as_ptr(), biome_id_cache, r);
            BiomeCache {
                biome_id_cache,
                x: range_start.0,
                y: range_start.1,
                z: range_start.2,
                sx: range_size.0,
                sy: range_size.1,
                sz: range_size.2,
            }
        }
    }

    pub fn is_in_bounds(&self, x: i32, y: i32, z: i32) -> bool {
        x >= self.x
            && x < self.x + self.sx
            && y >= self.y
            && y < self.y + self.sy
            && z >= self.z
            && z < self.z + self.sz
    }

    pub fn get_biome_at(&self, x: i32, y: i32, z: i32) -> BiomeID {
        if !self.is_in_bounds(x, y, z) {
            panic!(
                "Coordinate out of range for cache! Accepted: (x:{}..{}, y:{}..{}, z:{}..{}), Received: (x:{}, y:{}, z:{}).",
                    self.x, self.x + self.sx, self.y, self.y + self.sy, self.z, self.z + self.sz, x, y, z
            );
        }
        let i_x = x - self.x;
        let i_y = y - self.y;
        let i_z = z - self.z;
        let offset = i_y * self.sx * self.sz + i_z * self.sx + i_x;
        unsafe { *self.biome_id_cache.offset(offset as isize) }
    }
}

impl Drop for BiomeCache {
    fn drop(&mut self) {
        unsafe {
            libcubiomes_sys::free(self.biome_id_cache as *mut c_void);
        }
    }
}
