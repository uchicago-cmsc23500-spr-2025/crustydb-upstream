use std::sync::atomic::{AtomicUsize, Ordering};

use common::{ids::ContainerId, PAGE_SIZE};

use crate::{base_file::BaseFileTrait, file_stats::FileStats, page::Page};

#[allow(dead_code)]
/// BaseFileMock is a mock implementation of the BaseFileTrait.
pub struct BaseFileMock {
    num_pages: AtomicUsize,
    spin_time_ms: u64,
    c_id: ContainerId,
    stats: FileStats,
    mock_page: Page,
    direct: bool,
}

pub const SPIN_TIME_MS: u64 = 2;

impl BaseFileMock {
    #[allow(dead_code)]
    pub fn new<P: AsRef<std::path::Path>>(
        _db_dir: P,
        c_id: ContainerId,
    ) -> Result<Self, std::io::Error> {
        let mut mock_page = Page::new_empty();
        // fill the page with c_id as u8
        let data: [u8; PAGE_SIZE] = [c_id as u8; PAGE_SIZE];
        mock_page.data.copy_from_slice(&data);
        Ok(BaseFileMock {
            num_pages: AtomicUsize::new(0),
            c_id,
            spin_time_ms: SPIN_TIME_MS,
            stats: FileStats::new(),
            mock_page,
            direct: true,
        })
    }
}

impl BaseFileTrait for BaseFileMock {
    fn num_pages(&self) -> usize {
        self.num_pages.load(Ordering::Relaxed)
    }

    fn get_stats(&self) -> FileStats {
        self.stats.clone()
    }

    fn prefetch_page(&self, _page_id: u32) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn read_page(&self, page_id: u32, page: &mut Page) -> Result<(), std::io::Error> {
        self.stats.inc_read_count(self.direct);
        // Simulate a delay for the read operation
        if self.spin_time_ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(self.spin_time_ms));
        }
        page.copy_from_slice(&self.mock_page.data);
        page.set_page_id(page_id);
        Ok(())
    }

    fn write_page(&self, page_id: u32, _page: &Page) -> Result<(), std::io::Error> {
        self.stats.inc_write_count(self.direct);
        if page_id as usize > self.num_pages() {
            self.num_pages.store(page_id as usize, Ordering::Relaxed);
        }
        if self.spin_time_ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(self.spin_time_ms));
        }
        Ok(())
    }

    fn flush(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
}
