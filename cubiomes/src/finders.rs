use std::mem::MaybeUninit;
use std::ffi::{c_int, c_void};

pub use libcubiomes_sys::BiomeID;
pub use libcubiomes_sys::MCVersion;


pub struct CubiomesFinder {
    generator: MaybeUninit<libcubiomes_sys::Generator>,
}

impl CubiomesFinder {
    pub fn new(seed: i64, version: MCVersion) -> Self {
        unsafe {
            let mut finder = CubiomesFinder {
                generator: MaybeUninit::zeroed(),
            };
            libcubiomes_sys::setupGenerator(finder.generator.as_mut_ptr(), version as c_int, 0);
            libcubiomes_sys::applySeed(
                finder.generator.as_mut_ptr(),
                libcubiomes_sys::Dimension_DIM_OVERWORLD,
                seed as u64,
            );
            finder
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
    //y: i32,
    z: i32,
    sx: i32,
    //sy: i32,
    sz: i32,
    biome_id_cache: *mut BiomeID,
}

impl BiomeCache {
    pub fn new(
        finder: &CubiomesFinder,
        scale: CoordScaling,
        x: i32,
        z: i32,
        sx: i32,
        sz: i32,
    ) -> Self {
        unsafe {
            let y = 63;
            let sy = 1;
            let r = libcubiomes_sys::Range {
                scale: scale.value(), // Divide your input coordinates by this value
                // Define the position and size for a horizontal area:
                x,
                z,  // position (x,z)
                sx, // size (width,height)
                sz,
                // Set the vertical range as a plane near sea level at scale 1:4.
                y,
                sy,
            };
            let biome_id_cache: *mut BiomeID =
                libcubiomes_sys::allocCache(finder.generator.as_ptr(), r);
            libcubiomes_sys::genBiomes(finder.generator.as_ptr(), biome_id_cache, r);
            BiomeCache {
                biome_id_cache,
                x,
                //y,
                z,
                sx,
                //sy,
                sz,
            }
        }
    }

    pub fn is_in_bounds(&self, x: i32, z: i32) -> bool {
        x >= self.x && x < self.x + self.sx && z >= self.z && z < self.z + self.sz
    }

    pub fn get_biome_at(&self, x: i32, z: i32) -> BiomeID {
        if !self.is_in_bounds(x, z) {
            panic!(
                "Coordinate out of range for cache! Accepted: (x:{}..{}, z:{}..{}), Received: (x:{}, z:{}).",
                    self.x, self.x + self.sx, self.z, self.z + self.sz, x, z
            );
        }
        let i_x = x - self.x;
        let i_y = 0;
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
