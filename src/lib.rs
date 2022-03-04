use std::fmt::Debug;
use binrw::{binread, BinRead, BinReaderExt, BinResult, BinrwNamedArgs, io::Cursor, ReadOptions, until_eof};

#[derive(Debug, Clone)]
#[binread]
pub enum ImgDiffVersion {
    #[br(magic = b"1")] Version1,
    #[br(magic = b"2")] Version2,
    #[br(magic = b"3")] Version3,
    VersionUnknown,
}


#[derive(Debug, Clone)]
#[binread]
#[br(little, magic = b"IMGDIFF")]
pub struct ImgDiffPatchHeader {
    version: ImgDiffVersion,
    chunk_count: u32,
    #[br(little, count = chunk_count, pad_size_to = 0x10)] 
    chunks: Vec<ImgDiffChunk>,
}

#[derive(Debug, Clone)]
#[binread]
#[br(little)]
pub enum ImgDiffChunk {
    #[br(magic = 0)] Normal {
        source_start: u64,
        source_len: u64,
        bsdiff_patch_offset: u64,
    },
    #[br(magic = 1)] Gzip {
        source_start: u64,
        source_len: u64,
        bsdiff_patch_offset: u64, 
        source_expanded_len: u64, // size of uncompressed source
        target_expected_len: u64, // size of uncompressed target
        gzip_level: u32,
        method: u32,
        window_bits: u32,
        mem_level: u32,
        strategy: u32,
        gzip_header_len: u32,
        #[br(little, count = gzip_header_len)]
        gzip_header: Vec<u8>,
        gzip_footer: u64,
    },
    #[br(magic = 2)] Deflate {
        source_start: u64,
        source_len: u64,
        bsdiff_patch_offset: u64,
        source_expanded_len: u64,
        source_expected_len: u64,
        gzip_level: u32,
        method: u32,
        window_bits: u32,
        mem_level: u32,
        strategy: u32,
    },
    #[br(magic = 3)] Raw {
        target_len: u32,
        #[br(little, count = target_len)]
        data: Vec<u8>,
    },
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let src = ::std::fs::read("data/recovery-from-boot.p").unwrap();
        let mut data = Cursor::new(src);
        let diff = ImgDiffPatchHeader::read(&mut data);
        dbg!(diff);
    }
}
