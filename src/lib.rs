#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Fix missing types.
pub type HI_VOID = ::std::os::raw::c_void;

// Fix incomplete Eq trait for MPP_BIND_DEST_S.
#[cfg(not(feature = "hi3531v100"))]
impl std::cmp::Eq for MPP_BIND_DEST_S {}

// Fix incomplete Eq trait for MPP_VERSION_S.
impl std::cmp::Eq for MPP_VERSION_S {}

// Fix incomplete Debug trait for MPP_BIND_DEST_S
#[cfg(not(feature = "hi3531v100"))]
impl std::fmt::Debug for MPP_BIND_DEST_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME:
        write!(
            f,
            "MPP_BIND_DEST_S {{ u32Num: {}, astMppChn: [...] }}",
            self.u32Num
        )
    }
}

// Fix incomplete Debug trait for MPP_VERSION_S
impl std::fmt::Debug for MPP_VERSION_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::ffi::CStr::from_bytes_with_nul(&self.aVersion).unwrap_or_default();
        write!(f, "MPP_VERSION_S {{ aVersion: {} }}", s.to_string_lossy())
    }
}

/// Make HI_BOOL can convert to bool.
impl std::convert::Into<bool> for HI_BOOL {
    fn into(self) -> bool {
        match self {
            HI_BOOL::HI_TRUE => true,
            HI_BOOL::HI_FALSE => false,
        }
    }
}

/// Make bool can convert to HI_BOOL.
impl std::convert::Into<HI_BOOL> for bool {
    fn into(self) -> HI_BOOL {
        match self {
            true => HI_BOOL::HI_TRUE,
            false => HI_BOOL::HI_FALSE,
        }
    }
}

// Fix incomplete Debug trait for VENC_ATTR_S
impl std::fmt::Debug for VENC_ATTR_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PAYLOAD_TYPE_E::*;
        let mut fds = f.debug_struct("VENC_ATTR_S");
        fds.field("enType", &self.enType)
            .field("u32MaxPicWidth", &self.u32MaxPicWidth)
            .field("u32MaxPicHeight", &self.u32MaxPicHeight)
            .field("u32BufSize", &self.u32BufSize)
            .field("u32Profile", &self.u32Profile)
            .field("bByFrame", &self.bByFrame)
            .field("u32PicWidth", &self.u32PicWidth)
            .field("u32PicHeight", &self.u32PicHeight);
        unsafe {
            match self.enType {
                PT_H264 => fds.field("stAttrH264e", &self.un1.stAttrH264e).finish(),
                PT_H265 => fds.field("stAttrH265e", &self.un1.stAttrH265e).finish(),
                PT_MJPEG => fds.field("stAttrH265e", &self.un1.stAttrMjpege).finish(),
                PT_JPEG => fds.field("stAttrH265e", &self.un1.stAttrJpege).finish(),
                _ => fds.finish(),
            }
        }
    }
}

// Fix incomplete PartialEq trait for VENC_ATTR_S
impl PartialEq for VENC_ATTR_S {
    fn eq(&self, other: &Self) -> bool {
        self.enType == other.enType
            && self.u32MaxPicWidth == other.u32MaxPicHeight
            && self.u32MaxPicHeight == other.u32MaxPicHeight
            && self.u32BufSize == other.u32BufSize
            && self.u32Profile == other.u32Profile
            && self.bByFrame == other.bByFrame
            && self.u32PicWidth == other.u32PicWidth
            && self.u32PicHeight == other.u32PicHeight
    }
}

// Fix incomplete Eq trait for VENC_ATTR_S
impl Eq for VENC_ATTR_S {}

// Fix incomplete Debug trait for VENC_CHN_ATTR_S
impl std::fmt::Debug for VENC_CHN_ATTR_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VENC_CHN_ATTR_S")
            .field("stVencAttr", &self.stVencAttr)
            .finish()
    }
}

// Fix incomplete PartialEq trait for VENC_CHN_ATTR_S
impl PartialEq for VENC_CHN_ATTR_S {
    fn eq(&self, other: &Self) -> bool {
        self.stVencAttr == other.stVencAttr
    }
}

// Fix incomplete Eq trait for VENC_CHN_ATTR_S
impl Eq for VENC_CHN_ATTR_S {}

cfg_if::cfg_if! {
    if #[cfg(feature = "hi3531v100")] {
        pub use HI_MPI_SYS_Mmap as HI_MPI_SYS_MmapCache;
        pub use HI_MPI_SYS_GetCurPts as HI_MPI_SYS_GetCurPTS;
        pub use HI_MPI_SYS_InitPtsBase as HI_MPI_SYS_InitPTSBase;
        pub use HI_MPI_SYS_SyncPts as HI_MPI_SYS_SyncPTS;
    }
}

cfg_if::cfg_if! {
    if #[cfg(any(feature = "hi3519av100", feature = "hi3559av100"))] {
        use rust_bitfield::{bitfield_fields, BitRange, Bits};

        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aIEy_Bi(u16);

        impl tV56aIEy_Bi {
            bitfield_fields! {
                pub IEDZ, set_IEDZ : u16 [9..0];
                pub _rb_, set__rb_ : u16 [15..10];
            }
        }

        #[test]
        fn test_layout_tV56aIEy_Bi() {
            assert_eq!(
                ::core::mem::size_of::<tV56aIEy_Bi>(),
                2usize,
                concat!("Size of: ", stringify!(tV56aIEy_Bi))
            );
        }

        /// 定义 Hi3519AV100 的 3DNR 图像增强参数。
        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aIEy {
            pub IES0: u8,
            pub IES1: u8,
            pub IES2: u8,
            pub IES3: u8,
            pub bi: tV56aIEy_Bi,
        }

        #[test]
        fn test_layout_tV56aIEy() {
            assert_eq!(
                ::core::mem::size_of::<tV56aIEy>(),
                6usize,
                concat!("Size of: ", stringify!(tV56aIEy))
            );
        }

        /// 定义 Hi3519AV100 的 3DNR 空域滤波参数第 1 组位域。
        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aSFy_Bi1(u8, u8);

        impl tV56aSFy_Bi1 {
            bitfield_fields! {
                pub SPN6, set_SPN6 : u8 [2..0];
                pub SFR, set_SFR : u8 [7..3];
                pub SBN6, set_SBN6: u8 [2..0] in 1;
                pub PBR6, set_PBR6: u8 [7..3] in 1;
            }
        }

        #[test]
        fn test_layout_tV56aSFy_Bi1() {
            assert_eq!(
                ::core::mem::size_of::<tV56aSFy_Bi1>(),
                2usize,
                concat!("Size of: ", stringify!(tV56aSFy_Bi1))
            );
        }

        /// 定义 Hi3519AV100 的 3DNR 空域滤波参数第 2 组位域。
        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aSFy_Bi2(u16, u16, u16);

        impl tV56aSFy_Bi2 {
            bitfield_fields! {
                // 0
                pub STH1, set_STH1 : u16 [8..0];
                pub SFN1, set_SFN1 : u16 [11..9];
                pub SFN0, set_SFN0: u16 [14..12];
                pub NRyEn, set_NRyEn: u16 [15];
                // 1
                pub STH2, set_STH2: u16 [8..0] in 1;
                pub SFN2, set_SFN2: u16 [11..9] in 1;
                pub BWSF4, set_BWSF4: u16 [12] in 1;
                pub kMode, set_kMode: u16 [15..13] in 1;
                // 2
                pub STH3, set_STH3: u16 [8..0] in 2;
                pub SFN3, set_SFN3: u16 [11..9] in 2;
                pub tEdge, set_tEdge: u16 [13..12] in 2;
                pub TriTh, set_TriTh: u16 [14] in 2;
                pub _rb, set__rb: u16 [15] in 2;
            }
        }

        #[test]
        fn test_layout_tV56aSFy_Bi2() {
            assert_eq!(
                ::core::mem::size_of::<tV56aSFy_Bi2>(),
                6usize,
                concat!("Size of: ", stringify!(tV56aSFy_Bi2))
            );
        }

        /// 定义 Hi3519AV100 的 3DNR 空域滤波参数。
        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aSFy {
            pub bi1: tV56aSFy_Bi1,
            pub SFS2: u8,
            pub SFT2: u8,
            pub SBR2: u8,
            pub SFS4: u8,
            pub SFT4: u8,
            pub SBR4: u8,
            pub bi2: tV56aSFy_Bi2,
        }

        #[test]
        fn test_layout_tV56aSFy() {
            assert_eq!(
                ::core::mem::size_of::<tV56aSFy>(),
                14usize,
                concat!("Size of: ", stringify!(tV56aSFy))
            );
        }

        /// 定义 Hi3519AV100 的 3DNR 动静判决参数第 1 组位域。
        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aMDy_Bi1(u16);

        impl tV56aMDy_Bi1 {
            bitfield_fields! {
                pub MADZ, set_MADZ : u16 [9..0];
                pub MAI1, set_MAI1 : u16 [11..10];
                pub MAI2, set_MAI2: u16 [13..12];
                pub MAI3, set_MAI3: u16 [15..14];
            }
        }

        #[test]
        fn test_layout_tV56aMDy_Bi1() {
            assert_eq!(
                ::core::mem::size_of::<tV56aMDy_Bi1>(),
                2usize,
                concat!("Size of: ", stringify!(tV56aMDy_Bi1))
            );
        }

        /// 定义 Hi3519AV100 的 3DNR 动静判决参数第 2 组位域。
        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aMDy_Bi2(u16, u8, u8);

        impl tV56aMDy_Bi2 {
            bitfield_fields! {
                // 0
                pub MATH, set_MATH : u16 [9..0];
                pub MATE, set_MATE : u16 [13..10];
                pub MATW, set_MATW: u16 [15..14];
                // 1
                pub MASW, set_MASW: u8 [3..0] in 1;
                pub MABW, set_MABW: u8 [6..4] in 1;
                pub MAXN, set_MAXN: u8 [7] in 1;
                // 2
                pub _rb, _:u8 [] in 2;
            }
        }

        #[test]
        fn test_layout_tV56aMDy_Bi2() {
            assert_eq!(
                ::core::mem::size_of::<tV56aMDy_Bi2>(),
                4usize,
                concat!("Size of: ", stringify!(tV56aMDy_Bi2))
            );
        }

        /// 定义 Hi3519AV100 的 3DNR 动静判决参数。
        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aMDy {
            pub bi1: tV56aMDy_Bi1,
            pub MADK: u8,
            pub MABR: u8,
            pub bi2: tV56aMDy_Bi2,
        }

        #[test]
        fn test_layout_tV56aMDy() {
            assert_eq!(
                ::core::mem::size_of::<tV56aMDy>(),
                8usize,
                concat!("Size of: ", stringify!(tV56aMDy))
            );
        }

        /// 定义 Hi3519AV100 的 3DNR 时域滤波参数第 1 组位域。
        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aTFy_Bi1(u16);

        impl tV56aTFy_Bi1 {
            bitfield_fields! {
                // 0
                pub TFS, set_TFS: u16 [3..0];
                pub TDZ, set_TDZ: u16 [13..4];
                pub TDX, set_TDX: u16 [15..14];
            }
        }

        #[test]
        fn test_layout_tV56aTFy_Bi1() {
            assert_eq!(
                ::core::mem::size_of::<tV56aTFy_Bi1>(),
                2usize,
                concat!("Size of: ", stringify!(tV56aTFy_Bi1))
            );
        }

        /// 定义 Hi3519AV100 的 3DNR 时域滤波参数第 2 组位域。
        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aTFy_Bi2(u8, u16);

        impl tV56aTFy_Bi2 {
            bitfield_fields! {
                // 0
                pub TSS, set_TSS : u8 [3..0];
                pub TSI, set_TSI : u8 [4];
                pub _rb_, set__rb_: u8 [7..5];
                // 1
                pub SDZ, set_SDZ: u16 [9..0] in 1;
                pub STR, set_STR: u16 [14..10] in 1;
                pub bRef, set_bRef: u16 [15] in 1;
            }
        }

        #[test]
        fn test_layout_tV56aTFy_Bi2() {
            assert_eq!(
                ::core::mem::size_of::<tV56aTFy_Bi2>(),
                3usize,
                concat!("Size of: ", stringify!(tV56aTFy_Bi2))
            );
        }

        /// 定义 Hi3519AV100 的 3DNR 时域滤波参数。
        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aTFy {
            pub bi1: tV56aTFy_Bi1,
            pub TFS: [u8; 5usize],
            pub bi2: tV56aTFy_Bi2,
        }

        #[test]
        fn test_layout_tV56aTFy() {
            assert_eq!(
                ::core::mem::size_of::<tV56aTFy>(),
                10usize,
                concat!("Size of: ", stringify!(tV56aTFy))
            );
        }

        /// 定义 Hi3519AV100 的 3DNR 时域滤波参数位域。
        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aNRc_Bi(u8, u16, u16);

        impl tV56aNRc_Bi {
            bitfield_fields! {
                // 0
                pub rb, set_rb: u8 [1..0];
                pub TFC, set_TFC : u8 [7..2];
                // 1
                pub CSFS, set_CSFS: u16 [9..0] in 1;
                pub CSFR, set_CSFR: u16 [15..10] in 1;
                // 2
                pub CTFS, set_CTFS: u16 [3..0] in 2;
                pub CIIR, set_CIIR: u16 [4] in 2;
                pub CTFR, set_CTFR: u16 [15..5] in 2;
            }
        }

        #[test]
        fn test_layout_tV56aNRc_Bi() {
            assert_eq!(
                ::core::mem::size_of::<tV56aNRc_Bi>(),
                5usize,
                concat!("Size of: ", stringify!(tV56aNRc_Bi))
            );
        }

        /// 定义 Hi3519AV100 的 3DNR 去色噪参数。
        #[repr(packed(1))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct tV56aNRc {
            pub SFC: u8,
            pub bi: tV56aNRc_Bi,
        }

        #[test]
        fn test_layout_tV56aNRc() {
            assert_eq!(
                ::core::mem::size_of::<tV56aNRc>(),
                6usize,
                concat!("Size of: ", stringify!(tV56aNRc))
            );
        }

        /// 定义 3DNR X 接口 版本 V1 的参数。
        #[repr(packed(2))]
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
        pub struct VPSS_NRX_V1_S {
            pub IEy: [tV56aIEy; 2usize],
            pub SFy: [tV56aSFy; 4usize],
            pub MDy: [tV56aMDy; 2usize],
            pub TFy: [tV56aTFy; 2usize],
            pub NRc: tV56aNRc,
            pub SBSk2: [u16; 32usize],
            pub SDSk2: [u16; 32usize],
            pub SBSk3: [u16; 32usize],
            pub SDSk3: [u16; 32usize],
        }

        #[test]
        fn test_layout_VPSS_NRX_V1_S() {
            assert_eq!(
                ::core::mem::size_of::<VPSS_NRX_V1_S>(),
                366usize,
                concat!("Size of: ", stringify!(VPSS_NRX_V1_S))
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sys_init_exit() {
        unsafe {
            assert_eq!(0, HI_MPI_SYS_Init());
            assert_eq!(0, HI_MPI_SYS_Exit());
        }
    }
}
