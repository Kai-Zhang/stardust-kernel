use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;

pub const PAGE_SIZE: u64 = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysFrame(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtPage(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryRegion {
    pub start: u64,
    pub page_count: u64,
    pub conventional: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameStats {
    pub tracked_total: usize,
    pub free: usize,
    pub allocated: usize,
    pub skipped_regions: usize,
}

pub struct FrameAllocator {
    free_list: Vec<PhysFrame>,
    allocated: BTreeSet<PhysFrame>,
    skipped_regions: usize,
}

impl FrameAllocator {
    pub fn from_memory_map(regions: &[MemoryRegion]) -> Self {
        let mut free_list = Vec::new();
        let mut skipped_regions = 0usize;

        for region in regions {
            if !region.conventional || region.page_count == 0 {
                continue;
            }
            if region.start % PAGE_SIZE != 0 {
                skipped_regions += 1;
                continue;
            }

            for i in 0..region.page_count {
                free_list.push(PhysFrame(region.start + i * PAGE_SIZE));
            }
        }

        free_list.sort();

        Self {
            free_list,
            allocated: BTreeSet::new(),
            skipped_regions,
        }
    }

    pub fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.free_list.pop()?;
        self.allocated.insert(frame);
        Some(frame)
    }

    pub fn free_frame(&mut self, frame: PhysFrame) -> bool {
        if !self.allocated.remove(&frame) {
            return false;
        }
        self.free_list.push(frame);
        true
    }

    pub fn stats(&self) -> FrameStats {
        FrameStats {
            tracked_total: self.free_list.len() + self.allocated.len(),
            free: self.free_list.len(),
            allocated: self.allocated.len(),
            skipped_regions: self.skipped_regions,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapFlags {
    pub writable: bool,
    pub executable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapEntry {
    pub frame: PhysFrame,
    pub flags: MapFlags,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapError {
    AlreadyMapped,
    NotMapped,
}

pub struct VirtualMapper {
    entries: BTreeMap<VirtPage, MapEntry>,
}

impl Default for VirtualMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualMapper {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    pub fn map(
        &mut self,
        page: VirtPage,
        frame: PhysFrame,
        flags: MapFlags,
    ) -> Result<(), MapError> {
        if self.entries.contains_key(&page) {
            return Err(MapError::AlreadyMapped);
        }
        self.entries.insert(page, MapEntry { frame, flags });
        Ok(())
    }

    pub fn unmap(&mut self, page: VirtPage) -> Result<PhysFrame, MapError> {
        self.entries
            .remove(&page)
            .map(|entry| entry.frame)
            .ok_or(MapError::NotMapped)
    }

    pub fn translate(&self, page: VirtPage) -> Option<MapEntry> {
        self.entries.get(&page).copied()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocator_tracks_conventional_regions() {
        let regions = [
            MemoryRegion {
                start: 0x1000,
                page_count: 2,
                conventional: true,
            },
            MemoryRegion {
                start: 0x2003,
                page_count: 1,
                conventional: true,
            },
            MemoryRegion {
                start: 0x9000,
                page_count: 3,
                conventional: false,
            },
        ];

        let mut alloc = FrameAllocator::from_memory_map(&regions);
        assert_eq!(alloc.stats().tracked_total, 2);
        assert_eq!(alloc.stats().skipped_regions, 1);

        let a = alloc.allocate_frame().unwrap();
        let b = alloc.allocate_frame().unwrap();
        assert!(a != b);
        assert!(alloc.allocate_frame().is_none());

        assert!(alloc.free_frame(a));
        assert!(!alloc.free_frame(a));
    }

    #[test]
    fn mapper_detects_overlap_and_unmap() {
        let mut mapper = VirtualMapper::new();
        let page = VirtPage(0x4000);
        let frame = PhysFrame(0x2000);
        let flags = MapFlags {
            writable: true,
            executable: false,
        };

        assert!(mapper.map(page, frame, flags).is_ok());
        assert_eq!(mapper.translate(page).unwrap().frame, frame);
        assert_eq!(mapper.map(page, frame, flags), Err(MapError::AlreadyMapped));

        assert_eq!(mapper.unmap(page).unwrap(), frame);
        assert_eq!(mapper.unmap(page), Err(MapError::NotMapped));
    }
}
