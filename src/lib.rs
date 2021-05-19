//! Hi35XXX 多媒体应用接口安全绑定。
//!
//! 本项目当前基于 HiMPP V4.0 媒体处理软件接口实现。
//!
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::op_ref)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::transmute_ptr_to_ptr)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::useless_transmute)]

use bitfield_derive::BitFields;
use pavo_traits::{impl_as_bundle_many, AsPtr, AsPtrMut};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Fix missing types.
pub type HI_VOID = ::std::os::raw::c_void;

/// Make HI_BOOL can convert to bool.
impl std::convert::Into<bool> for HI_BOOL {
    fn into(self) -> bool {
        self == HI_BOOL::HI_TRUE
    }
}

/// Make bool can convert to HI_BOOL.
impl std::convert::Into<HI_BOOL> for bool {
    fn into(self) -> HI_BOOL {
        if self {
            HI_BOOL::HI_TRUE
        } else {
            HI_BOOL::HI_FALSE
        }
    }
}

impl_as_bundle_many!(POINT_S, RECT_S, SIZE_S,);

#[cfg(feature = "mpi-audio")]
impl_as_bundle_many!(
    ADEC_ATTR_ADPCM_S,
    ADEC_CHN_ATTR_S,
    ADEC_DECODER_S,
    AEC_FRAME_S,
    AIO_ATTR_S,
    AI_CHN_PARAM_S,
    AO_CHN_STATE_S,
    AO_VQE_CONFIG_S,
    AUDIO_FADE_S,
    AUDIO_FILE_STATUS_S,
    AUDIO_FRAME_S,
    AUDIO_SAVE_FILE_INFO_S,
    AUDIO_STREAM_S,
);

#[cfg(feature = "mpi-audio")]
impl Default for AIO_ATTR_S {
    fn default() -> Self {
        Self {
            enSamplerate: AUDIO_SAMPLE_RATE_E::AUDIO_SAMPLE_RATE_8000,
            enBitwidth: AUDIO_BIT_WIDTH_E::AUDIO_BIT_WIDTH_8,
            enWorkmode: AIO_MODE_E::AIO_MODE_I2S_MASTER,
            enSoundmode: AUDIO_SOUND_MODE_E::AUDIO_SOUND_MODE_MONO,
            u32EXFlag: 0,
            u32FrmNum: 0,
            u32PtNumPerFrm: 0,
            u32ChnCnt: 0,
            u32ClkSel: 0,
            enI2sType: AIO_I2STYPE_E::AIO_I2STYPE_INNERCODEC,
        }
    }
}

#[cfg(feature = "mpi-audio")]
impl Default for AUDIO_TRACK_MODE_E {
    fn default() -> Self {
        Self::AUDIO_TRACK_NORMAL
    }
}

#[cfg(feature = "mpi-avs")]
impl_as_bundle_many!(
    AVS_CHN_ATTR_S,
    AVS_CUBE_MAP_ATTR_S,
    AVS_FOV_S,
    AVS_GAIN_ATTR_S,
    AVS_GRP_ATTR_S,
    AVS_MOD_PARAM_S,
    AVS_OUTPUT_ATTR_S,
    AVS_ROTATION_S,
    AVS_SPLIT_ATTR_S,
);

#[cfg(feature = "mpi-gdc")]
impl_as_bundle_many!(
    FISHEYE_ATTR_S,
    FISHEYE_CONFIG_S,
    FISHEYE_JOB_CONFIG_S,
    FISHEYE_REGION_ATTR_S,
    GDC_PMF_ATTR_S,
    GDC_TASK_ATTR_S,
    SPREAD_ATTR_S,
);

#[cfg(feature = "mpi-hdmi")]
impl_as_bundle_many!(
    HI_CEC_RAWDATA_S,
    HI_HDMI_ATTR_S,
    HI_HDMI_AUD_INFOFRAME_VER1_S,
    // HI_HDMI_AUDIO_INFO_S,
    HI_HDMI_AVI_INFOFRAME_VER2_S,
    // HI_HDMI_CALLBACK_FUNC_S,
    HI_HDMI_CEC_CMD_S,
    HI_HDMI_CEC_STATUS_S,
    // HI_HDMI_CECCALLBACK_FUNC_S,
    // HI_HDMI_DET_TIMING_S,
    HI_HDMI_EDID_S,
    // HI_HDMI_EOTF_S,
    // HI_HDMI_HDR_CAP_S,
    // HI_HDMI_HDR_METADATA_TYPE_S,
    HI_HDMI_INFOFRAME_S,
    // HI_HDMI_MOD_PARAM_S,
    HI_HDMI_MPEGSOURCE_INFOFRAME_S,
    HI_HDMI_SINK_CAPABILITY_S,
    HI_HDMI_SPD_INFOFRAME_S,
    // HI_HDMI_TIMING_INFO_S,
    HI_HDMI_VENDORSPEC_INFOFRAME_S
);

#[cfg(all(feature = "mpi-hdmi", not(feature = "hi3531v100")))]
impl_as_bundle_many!(
    HI_HDMI_AUDIO_INFO_S,
    HI_HDMI_CALLBACK_FUNC_S,
    HI_HDMI_CECCALLBACK_FUNC_S,
    HI_HDMI_DET_TIMING_S,
    HI_HDMI_EOTF_S,
    HI_HDMI_HDR_CAP_S,
    HI_HDMI_HDR_METADATA_TYPE_S,
    HI_HDMI_MOD_PARAM_S,
    HI_HDMI_TIMING_INFO_S,
);

#[cfg(feature = "mpi-isp")]
impl_as_bundle_many!(
    ALG_LIB_S,
    ISP_AE_EXP_FUNC_S,
    ISP_AE_INFO_S,
    ISP_AE_PARAM_S,
    ISP_AE_REGISTER_S,
    ISP_AE_RESULT_S,
    ISP_AE_STAT_ATTR_S,
    ISP_AWB_Calibration_Gain_S,
    ISP_AWB_EXP_FUNC_S,
    ISP_AWB_INFO_S,
    ISP_AWB_PARAM_S,
    ISP_AWB_RAW_STAT_ATTR_S,
    ISP_AWB_REGISTER_S,
    ISP_AWB_RESULT_S,
    ISP_AWB_STAT_1_S,
    ISP_CTRL_PARAM_S,
    ISP_CTRL_PROC_WRITE_S,
    ISP_CMOS_ANTIFALSECOLOR_S,
    ISP_CMOS_BAYERNR_S,
    ISP_CMOS_BLACK_LEVEL_S,
    ISP_CMOS_CA_S,
    // ISP_CMOS_CLUT_S,
    // ISP_CMOS_DEHAZE_S,
    ISP_CMOS_DEMOSAIC_S,
    ISP_CMOS_DEFAULT_S,
    // ISP_CMOS_DETAIL_S,
    ISP_CMOS_DNG_COLORPARAM_S,
    ISP_CMOS_DPC_S,
    ISP_CMOS_DRC_S,
    // ISP_CMOS_EDGEMARK_S,
    // ISP_CMOS_EXPANDER_POINT_S,
    // ISP_CMOS_EXPANDER_S,
    ISP_CMOS_GAMMA_S,
    ISP_CMOS_GE_S,
    // ISP_CMOS_HLC_S,
    ISP_CMOS_LDCI_S,
    // ISP_CMOS_LOGLUT_S,
    ISP_CMOS_LSC_S,
    ISP_CMOS_NOISE_CALIBRATION_S,
    ISP_CMOS_PREGAMMA_S,
    // ISP_CMOS_PRELOGLUT_S,
    // ISP_CMOS_RGBIR_S,
    // ISP_CMOS_RLSC_S,
    ISP_CMOS_SENSOR_IMAGE_MODE_S,
    ISP_CMOS_SENSOR_MAX_RESOLUTION_S,
    ISP_CMOS_SENSOR_MODE_S,
    // ISP_CMOS_SHARPEN_AUTO_S,
    // ISP_CMOS_SHARPEN_MANUAL_S,
    ISP_CMOS_SHARPEN_S,
    // ISP_CMOS_SPLIT_POINT_S,
    // ISP_CMOS_SPLIT_S,
    ISP_CMOS_WDR_S,
    ISP_CMOS_WDR_SWITCH_ATTR_S,
    ISP_CONFIG_INFO_S,
    ISP_DCF_CONST_INFO_S,
    ISP_DCF_INFO_S,
    ISP_DCF_UPDATE_INFO_S,
    ISP_INIT_ATTR_S,
    ISP_LSC_CABLI_TABLE_S,
    ISP_MOD_PARAM_S,
    ISP_PIPE_DIFF_ATTR_S,
    ISP_PUB_ATTR_S,
    // ISP_RLSC_CABLI_TABLE_S,
    ISP_SENSOR_EXP_FUNC_S,
    ISP_SENSOR_REGISTER_S,
    ISP_SLAVE_SNS_SYNC_S,
    ISP_SNS_ATTR_INFO_S,
    ISP_SNS_OBJ_S,
    ISP_SNS_REGS_INFO_S,
    ISP_SNS_STATE_S,
    ISP_SPECAWB_ATTR_S,
    ISP_SPECAWB_BBL_TBL_S,
    ISP_SPECAWB_CAA_CONTROl_S,
    ISP_SPECAWB_CAA_CONVERSION_S,
    ISP_SPECAWB_CAA_TBL_S,
    ISP_SPECAWB_FACTTBL_ELEMENT_S,
    ISP_SPECAWB_KELVIN_DBB_MAP_S,
    ISP_SPECKCWB_S,
    ISP_SPECKCWBS16_S,
    ISP_STITCH_ATTR_S,
    ISP_WDR_MODE_S,
);

#[cfg(all(
    feature = "mpi-isp",
    not(any(feature = "hi3516ev200", feature = "hi3516ev300"))
))]
impl_as_bundle_many!(
    ISP_CMOS_CLUT_S,
    ISP_CMOS_EDGEMARK_S,
    ISP_CMOS_LOGLUT_S,
    ISP_CMOS_PRELOGLUT_S,
    ISP_CMOS_RLSC_S,
    ISP_CMOS_SPLIT_POINT_S,
    ISP_CMOS_SPLIT_S,
    ISP_RLSC_CABLI_TABLE_S,
);

#[cfg(feature = "mpi-ive")]
impl_as_bundle_many!(
    // IVE_16BIT_TO_8BIT_CTRL_S,
    // IVE_ADD_CTRL_S,
    // IVE_CANDI_BG_PIX_S,
    // IVE_CANNY_HYS_EDGE_CTRL_S,
    // IVE_CANNY_STACK_SIZE_S,
    // IVE_CCBLOB_S,
    // IVE_CCL_CTRL_S,
    IVE_CSC_CTRL_S,
    // IVE_DATA_S,
    IVE_DILATE_CTRL_S,
    // IVE_DMA_CTRL_S,
    // IVE_EQUALIZE_HIST_CTRL_S,
    // IVE_EQUALIZE_HIST_CTRL_MEM_S,
    IVE_ERODE_CTRL_S,
    IVE_FILTER_AND_CSC_CTRL_S,
    IVE_FILTER_CTRL_S,
    // IVE_GMM_CTRL_S,
    // IVE_GMM2_CTRL_S,
    // IVE_GRAD_FG_CTRL_S,
    // IVE_INTEG_CTRL_S,
    // IVE_IMAGE_S,
    // IVE_LBP_CTRL_S,
    // IVE_MAG_AND_ANG_CTRL_S,
    // IVE_MAP_CTRL_S,
    // IVE_MAP_S16BIT_LUT_MEM_S,
    // IVE_MAP_U8BIT_LUT_MEM_S,
    // IVE_MAP_U16BIT_LUT_MEM_S,
    // IVE_MEM_INFO_S,
    // IVE_NCC_DST_MEM_S,
    // IVE_NORM_GRAD_CTRL_S,
    // IVE_ORD_STAT_FILTER_CTRL_S,
    // IVE_REGION_S,
    // IVE_RESIZE_CTRL_S,
    // IVE_SAD_CTRL_S,
    IVE_SOBEL_CTRL_S,
    // IVE_ST_CANDI_CORNER_CTRL_S,
    // IVE_ST_CORNER_CTRL_S,
    // IVE_ST_CORNER_INFO_S,
    // IVE_ST_MAX_EIG_S,
    // IVE_SUB_CTRL_S,
    IVE_THRESH_CTRL_S,
    // IVE_THRESH_S16_CTRL_S,
    // IVE_THRESH_U16_CTRL_S,
    // IVE_WORK_BG_PIX_S,
);

#[cfg(all(feature = "mpi-ive", not(feature = "hi3531v100")))]
impl_as_bundle_many!(
    IVE_16BIT_TO_8BIT_CTRL_S,
    IVE_ADD_CTRL_S,
    IVE_CANDI_BG_PIX_S,
    IVE_CANNY_HYS_EDGE_CTRL_S,
    IVE_CANNY_STACK_SIZE_S,
    IVE_CCBLOB_S,
    IVE_CCL_CTRL_S,
    IVE_DATA_S,
    IVE_DMA_CTRL_S,
    IVE_EQUALIZE_HIST_CTRL_S,
    IVE_EQUALIZE_HIST_CTRL_MEM_S,
    IVE_GMM_CTRL_S,
    IVE_GMM2_CTRL_S,
    IVE_GRAD_FG_CTRL_S,
    IVE_INTEG_CTRL_S,
    IVE_IMAGE_S,
    IVE_LBP_CTRL_S,
    IVE_MAG_AND_ANG_CTRL_S,
    IVE_MAP_CTRL_S,
    IVE_MAP_S16BIT_LUT_MEM_S,
    IVE_MAP_U8BIT_LUT_MEM_S,
    IVE_MAP_U16BIT_LUT_MEM_S,
    IVE_MEM_INFO_S,
    IVE_NCC_DST_MEM_S,
    IVE_NORM_GRAD_CTRL_S,
    IVE_ORD_STAT_FILTER_CTRL_S,
    IVE_REGION_S,
    IVE_RESIZE_CTRL_S,
    IVE_SAD_CTRL_S,
    IVE_ST_CANDI_CORNER_CTRL_S,
    IVE_ST_CORNER_CTRL_S,
    IVE_ST_CORNER_INFO_S,
    IVE_ST_MAX_EIG_S,
    IVE_SUB_CTRL_S,
    IVE_THRESH_S16_CTRL_S,
    IVE_THRESH_U16_CTRL_S,
    IVE_WORK_BG_PIX_S,
);

#[cfg(all(
    feature = "mpi-ive",
    any(
        feature = "hi3536v100",
        feature = "hi3521av100",
        feature = "hi3518ev200",
        feature = "hi3531av100",
        feature = "hi3536cv100",
        feature = "hi3531dv100",
        feature = "hi3521dv100"
    )
))]
impl_as_bundle_many!(
    IVE_LK_OPTICAL_FLOW_CTRL_S,
    IVE_LK_OPTICAL_FLOW_PYR_CTRL_S,
    IVE_MAP_LUT_MEM_S,
    IVE_MV_S9Q7_S,
);

#[cfg(all(feature = "mpi-ive", any(feature = "hi3516cv300")))]
impl_as_bundle_many!(IVE_RESIZE2_CTRL_S,);

#[cfg(all(feature = "mpi-ive", any(feature = "hi3516dv300")))]
impl_as_bundle_many!(IVE_RECT_U16_S, IVE_RECT_U32_S,);

#[cfg(feature = "mpi-nnie")]
impl_as_bundle_many!(
    SVP_BLOB_S,
    SVP_NNIE_FORWARD_CTRL_S,
    SVP_NNIE_FORWARD_WITHBBOX_CTRL_S,
    SVP_NNIE_MODEL_S,
    SVP_NNIE_NODE_S,
    SVP_NNIE_SEG_S,
);

// Fix incomplete Debug trait for FISHEYE_CONFIG_S
#[cfg(feature = "mpi-gdc")]
impl std::fmt::Debug for FISHEYE_CONFIG_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FISHEYE_CONFIG_S")
            .field("au16LMFCoef", &self.au16LMFCoef.to_vec())
            .finish()
    }
}

// Fix incomplete Eq trait for FISHEYE_CONFIG_S
#[cfg(feature = "mpi-gdc")]
impl Eq for FISHEYE_CONFIG_S {}

// Fix incomplete Debug trait for SVP_BLOB_S
#[cfg(feature = "mpi-nnie")]
impl std::fmt::Debug for SVP_BLOB_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            f.debug_struct("SVP_BLOB_S")
                .field("enType", &self.enType)
                .field("u32Stride", &self.u32Stride)
                .field("u64VirAddr", &self.u64VirAddr)
                .field("u64PhyAddr", &self.u64PhyAddr)
                .field("u32Num", &self.u32Num)
                .field("stWhc", &self.unShape.stWhc)
                .field("stSeq", &self.unShape.stSeq)
                .finish()
        }
    }
}

// Fix incomplete PartialEq trait for SVP_BLOB_S
#[cfg(feature = "mpi-nnie")]
impl PartialEq for SVP_BLOB_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            self.enType == other.enType
                && self.u32Stride == other.u32Stride
                && self.u64VirAddr == other.u64VirAddr
                && self.u64PhyAddr == other.u64PhyAddr
                && self.u32Num == other.u32Num
                && self.unShape.stWhc == other.unShape.stWhc
        }
    }
}

// Fix incomplete Eq trait for SVP_BLOB_S
#[cfg(feature = "mpi-nnie")]
impl Eq for SVP_BLOB_S {}

// Fix incomplete Debug trait for SVP_NNIE_NODE_S
#[cfg(feature = "mpi-nnie")]
impl std::fmt::Debug for SVP_NNIE_NODE_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            f.debug_struct("SVP_NNIE_NODE_S")
                .field("enType", &self.enType)
                .field("stWhc", &self.unShape.stWhc)
                .field("u32Dim", &self.unShape.u32Dim)
                .field("u32NodeId", &self.u32NodeId)
                .field("szName", &std::ffi::CStr::from_ptr(self.szName.as_ptr()))
                .finish()
        }
    }
}

// Fix incomplete PartialEq trait for SVP_NNIE_NODE_S
#[cfg(feature = "mpi-nnie")]
impl PartialEq for SVP_NNIE_NODE_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            self.enType == other.enType
                && self.unShape.stWhc == other.unShape.stWhc
                && self.unShape.u32Dim == other.unShape.u32Dim
                && self.u32NodeId == other.u32NodeId
                && self.szName == other.szName
        }
    }
}

// Fix incomplete Eq trait for SVP_NNIE_NODE_S
#[cfg(feature = "mpi-nnie")]
impl Eq for SVP_NNIE_NODE_S {}

// Fix incomplete Debug trait for SVP_NNIE_MODEL_S
#[cfg(feature = "mpi-nnie")]
impl std::fmt::Debug for SVP_NNIE_MODEL_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SVP_NNIE_MODEL_S")
            .field("enRunMode", &self.enRunMode)
            .field("u32TmpBufSize", &self.u32TmpBufSize)
            .field("u32NetSegNum", &self.u32NetSegNum)
            .field("astSeg", &self.astSeg)
            .field("astRoiInfo", &self.astRoiInfo)
            .field("stBase", &self.stBase)
            .finish()
    }
}

// Fix incomplete PartialEq trait for SVP_NNIE_MODEL_S
#[cfg(feature = "mpi-nnie")]
impl PartialEq for SVP_NNIE_MODEL_S {
    fn eq(&self, other: &Self) -> bool {
        self.enRunMode == other.enRunMode
            && self.u32TmpBufSize == other.u32TmpBufSize
            && self.u32NetSegNum == other.u32NetSegNum
            && self.astSeg == other.astSeg
            && self.astRoiInfo == other.astRoiInfo
            && self.stBase == other.stBase
    }
}

// Fix incomplete Eq trait for SVP_NNIE_MODEL_S
#[cfg(feature = "mpi-nnie")]
impl Eq for SVP_NNIE_MODEL_S {}

// Fix incomplete Debug trait for SVP_NNIE_SEG_S
#[cfg(feature = "mpi-nnie")]
impl std::fmt::Debug for SVP_NNIE_SEG_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SVP_NNIE_SEG_S")
            .field("enNetType", &self.enNetType)
            .field("u16SrcNum", &self.u16SrcNum)
            .field("u16DstNum", &self.u16DstNum)
            .field("u16RoiPoolNum", &self.u16RoiPoolNum)
            .field("u16MaxStep", &self.u16MaxStep)
            .field("u32InstOffset", &self.u32InstOffset)
            .field("u32InstLen", &self.u32InstLen)
            .field("astSrcNode", &self.astSrcNode)
            .field("astDstNode", &self.astDstNode)
            .field("au32RoiIdx", &self.au32RoiIdx)
            .finish()
    }
}

// Fix incomplete PartialEq trait for SVP_NNIE_SEG_S
#[cfg(feature = "mpi-nnie")]
impl PartialEq for SVP_NNIE_SEG_S {
    fn eq(&self, other: &Self) -> bool {
        self.enNetType == other.enNetType
            && self.u16SrcNum == other.u16SrcNum
            && self.u16DstNum == other.u16DstNum
            && self.u16RoiPoolNum == other.u16RoiPoolNum
            && self.u16MaxStep == other.u16MaxStep
            && self.u32InstOffset == other.u32InstOffset
            && self.u32InstLen == other.u32InstLen
            && self.astSrcNode == other.astSrcNode
            && self.astDstNode == other.astDstNode
            && self.au32RoiIdx == other.au32RoiIdx
    }
}

// Fix incomplete Eq trait for SVP_NNIE_SEG_S
#[cfg(feature = "mpi-nnie")]
impl Eq for SVP_NNIE_SEG_S {}

#[cfg(feature = "mpi-region")]
impl_as_bundle_many!(
    COVER_CHN_ATTR_S,
    COVEREX_CHN_ATTR_S,
    // MOSAIC_CHN_ATTR_S,
    OVERLAY_QP_INFO_S,
    OVERLAY_ATTR_S,
    OVERLAY_CHN_ATTR_S,
    OVERLAY_INVERT_COLOR_S,
    OVERLAYEX_ATTR_S,
    OVERLAYEX_CHN_ATTR_S,
    RGN_ATTR_S,
    // RGN_CANVAS_INFO_S,
    RGN_CHN_ATTR_S,
    // RGN_QUADRANGLE_S,
);

#[cfg(all(feature = "mpi-region", not(feature = "hi3531v100")))]
impl_as_bundle_many!(MOSAIC_CHN_ATTR_S, RGN_CANVAS_INFO_S, RGN_QUADRANGLE_S,);

// Fix incomplete Debug trait for COVER_CHN_ATTR_S
#[cfg(feature = "mpi-region")]
impl std::fmt::Debug for COVER_CHN_ATTR_S {
    #[cfg(feature = "hi3531v100")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("COVER_CHN_ATTR_S")
            .field("stRect", &self.stRect)
            .field("u32Color", &self.u32Color)
            .field("u32Layer", &self.u32Layer)
            .finish()
    }

    #[cfg(not(feature = "hi3531v100"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("COVER_CHN_ATTR_S");
        ds.field("enCoverType", &self.enCoverType);
        unsafe {
            use RGN_AREA_TYPE_E::*;
            match self.enCoverType {
                AREA_RECT => {
                    ds.field("stRect", &self.un1.stRect);
                }
                AREA_QUAD_RANGLE => {
                    ds.field("stQuadRangle", &self.un1.stQuadRangle);
                }
                _ => unreachable!(),
            }
        }
        ds.field("u32Color", &self.u32Color)
            .field("u32Layer", &self.u32Layer)
            .field("enCoordinate", &self.enCoordinate)
            .finish()
    }
}

// Fix incomplete PartialEq trait for COVER_CHN_ATTR_S
#[cfg(feature = "mpi-region")]
impl PartialEq for COVER_CHN_ATTR_S {
    #[cfg(feature = "hi3531v100")]
    fn eq(&self, other: &Self) -> bool {
        self.stRect == other.stRect
            && self.u32Color == other.u32Color
            && self.u32Layer == other.u32Layer
    }

    #[cfg(not(feature = "hi3531v100"))]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let r: bool = self.enCoverType == other.enCoverType
                && self.u32Color == other.u32Color
                && self.u32Layer == other.u32Layer
                && self.enCoordinate == other.enCoordinate;
            use RGN_AREA_TYPE_E::*;
            match self.enCoverType {
                AREA_RECT => r && (self.un1.stRect == other.un1.stRect),
                AREA_QUAD_RANGLE => r && (self.un1.stQuadRangle == other.un1.stQuadRangle),
                _ => unreachable!(),
            }
        }
    }
}

// Fix incomplete Eq trait for COVER_CHN_ATTR_S.
#[cfg(feature = "mpi-region")]
impl std::cmp::Eq for COVER_CHN_ATTR_S {}

// Fix incomplete Debug trait for COVEREX_CHN_ATTR_S
#[cfg(feature = "mpi-region")]
impl std::fmt::Debug for COVEREX_CHN_ATTR_S {
    #[cfg(feature = "hi3531v100")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("COVEREX_CHN_ATTR_S")
            .field("stRect", &self.stRect)
            .field("u32Color", &self.u32Color)
            .field("u32Layer", &self.u32Layer)
            .finish()
    }

    #[cfg(not(feature = "hi3531v100"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("COVEREX_CHN_ATTR_S");
        ds.field("enCoverType", &self.enCoverType);
        unsafe {
            use RGN_AREA_TYPE_E::*;
            match self.enCoverType {
                AREA_RECT => {
                    ds.field("stRect", &self.un1.stRect);
                }
                AREA_QUAD_RANGLE => {
                    ds.field("stQuadRangle", &self.un1.stQuadRangle);
                }
                _ => unreachable!(),
            }
        }
        ds.field("u32Color", &self.u32Color)
            .field("u32Layer", &self.u32Layer)
            .finish()
    }
}

// Fix incomplete PartialEq trait for COVEREX_CHN_ATTR_S
#[cfg(feature = "mpi-region")]
impl PartialEq for COVEREX_CHN_ATTR_S {
    #[cfg(feature = "hi3531v100")]
    fn eq(&self, other: &Self) -> bool {
        self.stRect == other.stRect
            && self.u32Color == other.u32Color
            && self.u32Layer == other.u32Layer
    }

    #[cfg(not(feature = "hi3531v100"))]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let r: bool = self.enCoverType == other.enCoverType
                && self.u32Color == other.u32Color
                && self.u32Layer == other.u32Layer;
            use RGN_AREA_TYPE_E::*;
            match self.enCoverType {
                AREA_RECT => r && (self.un1.stRect == other.un1.stRect),
                AREA_QUAD_RANGLE => r && (self.un1.stQuadRangle == other.un1.stQuadRangle),
                _ => unreachable!(),
            }
        }
    }
}

// Fix incomplete Eq trait for COVEREX_CHN_ATTR_S.
#[cfg(feature = "mpi-region")]
impl std::cmp::Eq for COVEREX_CHN_ATTR_S {}

// Fix incomplete Debug trait for RGN_ATTR_S
#[cfg(feature = "mpi-region")]
impl std::fmt::Debug for RGN_ATTR_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("RGN_ATTR_S");
        ds.field("enType", &self.enType);
        unsafe {
            use RGN_TYPE_E::*;
            match self.enType {
                OVERLAY_RGN => {
                    ds.field("stOverlay", &self.unAttr.stOverlay);
                }
                OVERLAYEX_RGN => {
                    ds.field("stOverlayEx", &self.unAttr.stOverlayEx);
                }
                _ => unreachable!(),
            }
        }
        ds.finish()
    }
}

// Fix incomplete PartialEq trait for RGN_ATTR_S
#[cfg(feature = "mpi-region")]
impl PartialEq for RGN_ATTR_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let r: bool = self.enType == other.enType;
            use RGN_TYPE_E::*;
            match self.enType {
                OVERLAY_RGN => r && (self.unAttr.stOverlay == other.unAttr.stOverlay),
                OVERLAYEX_RGN => r && (self.unAttr.stOverlayEx == other.unAttr.stOverlayEx),
                _ => unreachable!(),
            }
        }
    }
}

// Fix incomplete Eq trait for RGN_ATTR_S.
#[cfg(feature = "mpi-region")]
impl std::cmp::Eq for RGN_ATTR_S {}

// Fix incomplete Debug trait for RGN_CHN_ATTR_S
#[cfg(feature = "mpi-region")]
impl std::fmt::Debug for RGN_CHN_ATTR_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("RGN_CHN_ATTR_S");
        ds.field("bShow", &self.bShow).field("enType", &self.enType);
        unsafe {
            use RGN_TYPE_E::*;
            match self.enType {
                OVERLAY_RGN => {
                    ds.field("stOverlayChn", &self.unChnAttr.stOverlayChn);
                }
                COVER_RGN => {
                    ds.field("stCoverChn", &self.unChnAttr.stCoverChn);
                }
                COVEREX_RGN => {
                    ds.field("stCoverExChn", &self.unChnAttr.stCoverExChn);
                }
                OVERLAYEX_RGN => {
                    ds.field("stOverlayExChn", &self.unChnAttr.stOverlayExChn);
                }
                MOSAIC_RGN => {
                    ds.field("stMosaicChn", &self.unChnAttr.stMosaicChn);
                }
                _ => unreachable!(),
            }
        }
        ds.finish()
    }
}

// Fix incomplete PartialEq trait for RGN_CHN_ATTR_S
#[cfg(feature = "mpi-region")]
impl PartialEq for RGN_CHN_ATTR_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let r: bool = self.bShow == other.bShow && self.enType == other.enType;
            use RGN_TYPE_E::*;
            match self.enType {
                OVERLAY_RGN => r && (self.unChnAttr.stOverlayChn == other.unChnAttr.stOverlayChn),
                COVER_RGN => r && (self.unChnAttr.stCoverChn == other.unChnAttr.stCoverChn),
                COVEREX_RGN => r && (self.unChnAttr.stCoverExChn == other.unChnAttr.stCoverExChn),
                OVERLAYEX_RGN => {
                    r && (self.unChnAttr.stOverlayExChn == other.unChnAttr.stOverlayExChn)
                }
                MOSAIC_RGN => r && (self.unChnAttr.stMosaicChn == other.unChnAttr.stMosaicChn),
                _ => unreachable!(),
            }
        }
    }
}

// Fix incomplete Eq trait for RGN_CHN_ATTR_S.
#[cfg(feature = "mpi-region")]
impl std::cmp::Eq for RGN_CHN_ATTR_S {}

#[cfg(feature = "mpi-sys")]
impl_as_bundle_many!(
    // GPS_INFO_S,
    // MPP_BIND_DEST_S,
    MPP_CHN_S,
    // MPP_SYS_CONFIG_S,
    MPP_VERSION_S,
    // SCALE_COEFF_LEVEL_S,
    // SYS_VIRMEM_INFO_S,
    // VI_VPSS_MODE_S,
);

#[cfg(all(feature = "mpi-sys", feature = "hi3531v100"))]
impl_as_bundle_many!(MPP_SYS_CONF_S,);

#[cfg(all(feature = "mpi-sys", not(feature = "hi3531v100")))]
impl_as_bundle_many!(
    GPS_INFO_S,
    MPP_BIND_DEST_S,
    MPP_SYS_CONFIG_S,
    SCALE_COEFF_LEVEL_S,
    SYS_VIRMEM_INFO_S,
    VI_VPSS_MODE_S,
);

#[cfg(all(
    feature = "mpi-sys",
    any(feature = "hi3559av100", feature = "hi3519av100")
))]
impl_as_bundle_many!(RAW_FRAME_COMPRESS_PARAM_S,);

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

// Fix incomplete Eq trait for MPP_BIND_DEST_S.
#[cfg(not(feature = "hi3531v100"))]
impl std::cmp::Eq for MPP_BIND_DEST_S {}

// Fix incomplete Debug trait for MPP_VERSION_S
impl std::fmt::Debug for MPP_VERSION_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::ffi::CStr::from_bytes_with_nul(&self.aVersion).unwrap_or_default();
        write!(f, "MPP_VERSION_S {{ aVersion: {} }}", s.to_string_lossy())
    }
}

// Fix incomplete Eq trait for MPP_VERSION_S.
impl std::cmp::Eq for MPP_VERSION_S {}

// Impl AsMut/AsRef/AsPtr/AsPtrMut of VB_CAL_CONFIG_S
#[cfg(all(feature = "mpi-vb", feature = "hi3531v100"))]
impl_as_bundle_many!(VB_CONF_S, VB_CPOOL_S,);

#[cfg(all(feature = "mpi-vb", not(feature = "hi3531v100")))]
impl_as_bundle_many!(VB_CAL_CONFIG_S, VB_CONFIG_S, VB_POOL_CONFIG_S,);

#[cfg(feature = "mpi-vdec")]
impl_as_bundle_many!(
    // H264_PRTCL_PARAM_S,
    // H265_PRTCL_PARAM_S,
    VDEC_ATTR_VIDEO_S,
    VDEC_CHN_ATTR_S,
    VDEC_CHN_PARAM_S,
    // VDEC_CHN_POOL_S,
    // VDEC_CHN_STATUS_S,
    // VDEC_MOD_PARAM_S,
    // VDEC_PARAM_VIDEO_S,
    // VDEC_PARAM_PICTURE_S,
    VDEC_PRTCL_PARAM_S,
    VDEC_STREAM_S,
    VDEC_USERDATA_S,
    // VDEC_VIDEO_MOD_PARAM_S,
);

#[cfg(all(feature = "mpi-vdec", feature = "hi3531v100"))]
impl_as_bundle_many!(VDEC_CHN_STAT_S,);

#[cfg(all(feature = "mpi-vdec", not(feature = "hi3531v100")))]
impl_as_bundle_many!(
    H264_PRTCL_PARAM_S,
    H265_PRTCL_PARAM_S,
    VDEC_CHN_POOL_S,
    VDEC_CHN_STATUS_S,
    VDEC_MOD_PARAM_S,
    VDEC_PARAM_VIDEO_S,
    VDEC_PARAM_PICTURE_S,
    VDEC_VIDEO_MOD_PARAM_S,
);

// Fix incomplete Debug trait for VDEC_CHN_ATTR_S
#[cfg(feature = "mpi-vdec")]
impl std::fmt::Debug for VDEC_CHN_ATTR_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            f.debug_struct("VDEC_CHN_ATTR_S")
                .field("enType", &self.enType)
                .field("enMode", &self.enMode)
                .field("u32PicWidth", &self.u32PicWidth)
                .field("u32PicHeight", &self.u32PicHeight)
                .field("u32StreamBufSize", &self.u32StreamBufSize)
                .field("u32FrameBufSize", &self.u32FrameBufSize)
                .field("u32FrameBufCnt", &self.u32FrameBufCnt)
                .field("stVdecVideoAttr", &self.un1.stVdecVideoAttr)
                .finish()
        }
    }
}

// Fix incomplete PartialEq trait for VDEC_CHN_ATTR_S
#[cfg(feature = "mpi-vdec")]
impl PartialEq for VDEC_CHN_ATTR_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            self.enType == other.enType
                && self.enMode == other.enMode
                && self.u32PicWidth == other.u32PicWidth
                && self.u32PicHeight == other.u32PicHeight
                && self.u32StreamBufSize == other.u32StreamBufSize
                && self.u32FrameBufSize == other.u32FrameBufSize
                && self.u32FrameBufCnt == other.u32FrameBufCnt
                && self.un1.stVdecVideoAttr == other.un1.stVdecVideoAttr
        }
    }
}

// Fix incomplete Eq trait for VDEC_CHN_ATTR_S
#[cfg(feature = "mpi-vdec")]
impl Eq for VDEC_CHN_ATTR_S {}

// Fix incomplete Debug trait for VDEC_CHN_PARAM_S
#[cfg(feature = "mpi-vdec")]
impl std::fmt::Debug for VDEC_CHN_PARAM_S {
    #[cfg(feature = "hi3531v100")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VDEC_CHN_PARAM_S")
            .field("s32ChanErrThr", &self.s32ChanErrThr)
            .field("s32ChanStrmOFThr", &self.s32ChanStrmOFThr)
            .field("s32DecMode", &self.s32DecMode)
            .field("s32DecOrderOutput", &self.s32DecOrderOutput)
            .field("s32DnrTfEnable", &self.s32DnrTfEnable)
            .field("s32DnrDispOutEnable", &self.s32DnrDispOutEnable)
    }

    #[cfg(not(feature = "hi3531v100"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fds = f.debug_struct("VDEC_CHN_PARAM_S");
        fds.field("enType", &self.enType)
            .field("u32DisplayFrameNum", &self.u32DisplayFrameNum);
        use PAYLOAD_TYPE_E::*;
        match self.enType {
            PT_H264 | PT_H265 => unsafe {
                fds.field("stVdecVideoParam", &self.un1.stVdecVideoParam);
            },
            PT_JPEG | PT_MJPEG => unsafe {
                fds.field("stVdecPictureParam", &self.un1.stVdecPictureParam);
            },
            _ => {}
        }
        fds.finish()
    }
}

// Fix incomplete PartialEq trait for VDEC_CHN_PARAM_S
#[cfg(feature = "mpi-vdec")]
impl PartialEq for VDEC_CHN_PARAM_S {
    #[cfg(feature = "hi3531v100")]
    fn eq(&self, other: &Self) -> bool {
        self.s32ChanErrThr == other.s32ChanErrThr
            && self.s32ChanStrmOFThr == other.s32ChanStrmOFThr
            && self.s32DecMode == other.s32DecMode
            && self.s32DecOrderOutput == other.s32DecOrderOutput
            && self.s32DnrTfEnable == other.s32DnrTfEnable
            && self.s32DnrDispOutEnable == other.s32DnrDispOutEnable
    }

    #[cfg(not(feature = "hi3531v100"))]
    fn eq(&self, other: &Self) -> bool {
        let b = self.enType == other.enType && self.u32DisplayFrameNum == other.u32DisplayFrameNum;
        use PAYLOAD_TYPE_E::*;
        match self.enType {
            PT_H264 | PT_H265 => unsafe {
                b && self.un1.stVdecVideoParam == other.un1.stVdecVideoParam
            },
            PT_JPEG | PT_MJPEG => unsafe {
                b && self.un1.stVdecPictureParam == other.un1.stVdecPictureParam
            },
            _ => b,
        }
    }
}

// Fix incomplete Eq trait for VDEC_CHN_PARAM_S
#[cfg(feature = "mpi-vdec")]
impl Eq for VDEC_CHN_PARAM_S {}

// Fix incomplete Debug trait for VDEC_PRTCL_PARAM_S
#[cfg(feature = "mpi-vdec")]
impl std::fmt::Debug for VDEC_PRTCL_PARAM_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fds = f.debug_struct("VDEC_PRTCL_PARAM_S");
        fds.field("enType", &self.enType);
        use PAYLOAD_TYPE_E::*;
        match self.enType {
            PT_H264 => unsafe {
                fds.field("stH264PrtclParam", &self.un1.stH264PrtclParam);
            },
            PT_H265 => unsafe {
                fds.field("stH265PrtclParam", &self.un1.stH265PrtclParam);
            },
            _ => {}
        }
        fds.finish()
    }
}

// Fix incomplete PartialEq trait for VDEC_PRTCL_PARAM_S
#[cfg(feature = "mpi-vdec")]
impl PartialEq for VDEC_PRTCL_PARAM_S {
    fn eq(&self, other: &Self) -> bool {
        let b = self.enType == other.enType;
        use PAYLOAD_TYPE_E::*;
        match self.enType {
            PT_H264 => unsafe { b && self.un1.stH264PrtclParam == other.un1.stH264PrtclParam },
            PT_H265 => unsafe { b && self.un1.stH265PrtclParam == other.un1.stH265PrtclParam },
            _ => b,
        }
    }
}

// Fix incomplete Eq trait for VDEC_PRTCL_PARAM_S
#[cfg(feature = "mpi-vdec")]
impl Eq for VDEC_PRTCL_PARAM_S {}

#[cfg(feature = "mpi-venc")]
impl_as_bundle_many!(
    VENC_ATTR_H264_S,
    // VENC_ATTR_H265_S,
    VENC_ATTR_JPEG_S,
    VENC_ATTR_MJPEG_S,
    // VENC_ATTR_PRORES_S,
    VENC_ATTR_S,
    VENC_CHN_ATTR_S,
    VENC_CHN_STATUS_S,
    // VENC_GOP_ATTR_S,
    // VENC_GOP_ADVSMARTP_S,
    // VENC_GOP_BIPREDB_S,
    // VENC_GOP_DUALP_S,
    // VENC_GOP_NORMALP_S,
    // VENC_GOP_SMARTP_S,
    VENC_H264_DBLK_S,
    VENC_H264_ENTROPY_S,
    VENC_H264_INTRA_PRED_S,
    VENC_H264_SLICE_SPLIT_S,
    VENC_H264_TRANS_S,
    VENC_H264_VUI_S,
    // VENC_H265_VUI_S,
    VENC_JPEG_PARAM_S,
    VENC_MJPEG_PARAM_S,
    VENC_MPF_CFG_S,
    VENC_PACK_INFO_S,
    VENC_PACK_S,
    VENC_RC_ATTR_S,
    VENC_RECV_PIC_PARAM_S,
    VENC_REF_PARAM_S,
    VENC_ROI_ATTR_S,
    VENC_ROI_ATTR_EX_S,
    VENC_ROIBG_FRAME_RATE_S,
    VENC_SSE_INFO_S,
    VENC_STREAM_ADVANCE_INFO_H264_S,
    VENC_STREAM_ADVANCE_INFO_H265_S,
    VENC_STREAM_ADVANCE_INFO_JPEG_S,
    VENC_STREAM_ADVANCE_INFO_PRORES_S,
    VENC_STREAM_BUF_INFO_S,
    VENC_STREAM_INFO_H264_S,
    VENC_STREAM_INFO_H265_S,
    VENC_STREAM_INFO_JPEG_S,
    VENC_STREAM_INFO_PRORES_S,
    VENC_STREAM_INFO_S,
    VENC_STREAM_S,
    VENC_VUI_ASPECT_RATIO_S,
    VENC_VUI_BITSTREAM_RESTRIC_S,
    VENC_VUI_H264_TIME_INFO_S,
    VENC_VUI_H265_TIME_INFO_S,
    VENC_VUI_VIDEO_SIGNAL_S,
);

#[cfg(all(feature = "mpi-venc", not(feature = "hi3531v100")))]
impl_as_bundle_many!(
    VENC_ATTR_H265_S,
    VENC_ATTR_PRORES_S,
    VENC_GOP_ATTR_S,
    VENC_GOP_ADVSMARTP_S,
    VENC_GOP_BIPREDB_S,
    VENC_GOP_DUALP_S,
    VENC_GOP_NORMALP_S,
    VENC_GOP_SMARTP_S,
    VENC_H265_VUI_S,
);

// Fix incomplete Debug trait for VENC_ATTR_S
#[cfg(feature = "mpi-venc")]
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
#[cfg(feature = "mpi-venc")]
impl PartialEq for VENC_ATTR_S {
    fn eq(&self, other: &Self) -> bool {
        self.enType == other.enType
            && self.u32MaxPicWidth == other.u32MaxPicWidth
            && self.u32MaxPicHeight == other.u32MaxPicHeight
            && self.u32BufSize == other.u32BufSize
            && self.u32Profile == other.u32Profile
            && self.bByFrame == other.bByFrame
            && self.u32PicWidth == other.u32PicWidth
            && self.u32PicHeight == other.u32PicHeight
    }
}

// Fix incomplete Eq trait for VENC_ATTR_S
#[cfg(feature = "mpi-venc")]
impl Eq for VENC_ATTR_S {}

// Fix incomplete Debug trait for VENC_CHN_ATTR_S
#[cfg(feature = "mpi-venc")]
impl std::fmt::Debug for VENC_CHN_ATTR_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VENC_CHN_ATTR_S")
            .field("stVencAttr", &self.stVencAttr)
            .field("stRcAttr", &self.stRcAttr)
            .field("stGopAttr", &self.stGopAttr)
            .finish()
    }
}

// Fix incomplete PartialEq trait for VENC_CHN_ATTR_S
#[cfg(feature = "mpi-venc")]
impl PartialEq for VENC_CHN_ATTR_S {
    fn eq(&self, other: &Self) -> bool {
        self.stVencAttr == other.stVencAttr
            && self.stRcAttr == other.stRcAttr
            && self.stGopAttr == other.stGopAttr
    }
}

// Fix incomplete Eq trait for VENC_CHN_ATTR_S
#[cfg(feature = "mpi-venc")]
impl Eq for VENC_CHN_ATTR_S {}

// Fix missing Eq trait for VENC_CHN_STATUS_S
#[cfg(feature = "mpi-venc")]
impl Eq for VENC_CHN_STATUS_S {}

// Cast u32 -> VENC_DATA_TYPE_U
#[cfg(feature = "mpi-venc")]
impl From<u32> for VENC_DATA_TYPE_U {
    fn from(val: u32) -> VENC_DATA_TYPE_U {
        VENC_DATA_TYPE_U {
            _bindgen_union_align: val,
        }
    }
}

// Cast VENC_DATA_TYPE_U -> u32
#[cfg(feature = "mpi-venc")]
impl Into<u32> for VENC_DATA_TYPE_U {
    fn into(self) -> u32 {
        unsafe { self._bindgen_union_align }
    }
}

// Fix incomplete Debug trait for VENC_DATA_TYPE_U
#[cfg(feature = "mpi-venc")]
impl std::fmt::Debug for VENC_DATA_TYPE_U {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { write!(f, "{:?}", self._bindgen_union_align) }
    }
}

// Fix incomplete PartialEq trait for VENC_DATA_TYPE_U
#[cfg(feature = "mpi-venc")]
impl PartialEq for VENC_DATA_TYPE_U {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.enH264EType == other.enH264EType }
    }
}

// Fix incomplete Eq trait for VENC_GOP_ATTR_S
#[cfg(feature = "mpi-venc")]
impl Eq for VENC_DATA_TYPE_U {}

// Fix incomplete Debug trait for VENC_GOP_ATTR_S
#[cfg(all(feature = "mpi-venc", not(feature = "hi3531v100")))]
impl std::fmt::Debug for VENC_GOP_ATTR_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fds = f.debug_struct("VENC_GOP_ATTR_S");
        fds.field("enGopMode", &self.enGopMode);
        unsafe {
            use VENC_GOP_MODE_E::*;
            match self.enGopMode {
                VENC_GOPMODE_NORMALP => fds.field("stNormalP", &self.un1.stNormalP).finish(),
                VENC_GOPMODE_DUALP => fds.field("stDualP", &self.un1.stDualP).finish(),
                VENC_GOPMODE_SMARTP => fds.field("stSmartP", &self.un1.stSmartP).finish(),
                VENC_GOPMODE_ADVSMARTP => fds.field("stAdvSmartP", &self.un1.stAdvSmartP).finish(),
                VENC_GOPMODE_BIPREDB => fds.field("stBipredB", &self.un1.stBipredB).finish(),
                VENC_GOPMODE_LOWDELAYB => fds.field("stBipredB", &self.un1.stBipredB).finish(),
                _ => fds.finish(),
            }
        }
    }
}

// Fix incomplete PartialEq trait for VENC_GOP_ATTR_S
#[cfg(all(feature = "mpi-venc", not(feature = "hi3531v100")))]
impl PartialEq for VENC_GOP_ATTR_S {
    fn eq(&self, other: &Self) -> bool {
        if self.enGopMode != other.enGopMode {
            return false;
        }
        unsafe {
            use VENC_GOP_MODE_E::*;
            match self.enGopMode {
                VENC_GOPMODE_NORMALP => self.un1.stNormalP == other.un1.stNormalP,
                VENC_GOPMODE_DUALP => self.un1.stDualP == other.un1.stDualP,
                VENC_GOPMODE_SMARTP => self.un1.stSmartP == other.un1.stSmartP,
                VENC_GOPMODE_ADVSMARTP => self.un1.stAdvSmartP == other.un1.stAdvSmartP,
                VENC_GOPMODE_BIPREDB => self.un1.stBipredB == other.un1.stBipredB,
                VENC_GOPMODE_LOWDELAYB => self.un1.stBipredB == other.un1.stBipredB,
                _ => false,
            }
        }
    }
}

// Fix incomplete Eq trait for VENC_GOP_ATTR_S
#[cfg(all(feature = "mpi-venc", not(feature = "hi3531v100")))]
impl Eq for VENC_GOP_ATTR_S {}

// Force impl Send for VENC_PACK_S
#[cfg(feature = "mpi-venc")]
unsafe impl Send for VENC_PACK_S {}

// Force impl Sync for VENC_PACK_S
#[cfg(feature = "mpi-venc")]
unsafe impl Sync for VENC_PACK_S {}

// Fix incomplete Debug trait for VENC_PARAM_MOD_S
#[cfg(all(feature = "mpi-venc", not(feature = "hi3531v100")))]
impl std::fmt::Debug for VENC_PARAM_MOD_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fds = f.debug_struct("VENC_PARAM_MOD_S");
        fds.field("enVencModType", &self.enVencModType);
        unsafe {
            use VENC_MODTYPE_E::*;
            match self.enVencModType {
                MODTYPE_VENC => fds
                    .field("stVencModParam", &self.un1.stVencModParam)
                    .finish(),
                MODTYPE_H264E => fds
                    .field("stH264eModParam", &self.un1.stH264eModParam)
                    .finish(),
                MODTYPE_H265E => fds
                    .field("stH265eModParam", &self.un1.stH265eModParam)
                    .finish(),
                MODTYPE_JPEGE => fds
                    .field("stJpegeModParam", &self.un1.stJpegeModParam)
                    .finish(),
                MODTYPE_RC => fds.field("stRcModParam", &self.un1.stRcModParam).finish(),
                _ => fds.finish(),
            }
        }
    }
}

// Fix incomplete PartialEq trait for VENC_PARAM_MOD_S
#[cfg(all(feature = "mpi-venc", not(feature = "hi3531v100")))]
impl PartialEq for VENC_PARAM_MOD_S {
    fn eq(&self, other: &Self) -> bool {
        if self.enVencModType != other.enVencModType {
            return false;
        }
        unsafe {
            use VENC_MODTYPE_E::*;
            match self.enVencModType {
                MODTYPE_VENC => self.un1.stVencModParam == other.un1.stVencModParam,
                MODTYPE_H264E => self.un1.stH264eModParam == other.un1.stH264eModParam,
                MODTYPE_H265E => self.un1.stH265eModParam == other.un1.stH265eModParam,
                MODTYPE_JPEGE => self.un1.stJpegeModParam == other.un1.stJpegeModParam,
                MODTYPE_RC => self.un1.stRcModParam == other.un1.stRcModParam,
                _ => false,
            }
        }
    }
}

// Fix incomplete Eq trait for VENC_PARAM_MOD_S
#[cfg(all(feature = "mpi-venc", not(feature = "hi3531v100")))]
impl Eq for VENC_PARAM_MOD_S {}

// Fix incomplete Debug trait for VENC_PACK_INFO_S
#[cfg(feature = "mpi-venc")]
impl std::fmt::Debug for VENC_PACK_INFO_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VENC_PACK_INFO_S")
            .field("u32PackType", &self.u32PackType)
            .field("u32PackOffset", &self.u32PackOffset)
            .field("u32PackLength", &self.u32PackLength)
            .finish()
    }
}

// Fix incomplete PartialEq trait for VENC_PACK_INFO_S
#[cfg(feature = "mpi-venc")]
impl PartialEq for VENC_PACK_INFO_S {
    fn eq(&self, other: &Self) -> bool {
        self.u32PackType == other.u32PackType
            && self.u32PackOffset == other.u32PackOffset
            && self.u32PackLength == other.u32PackLength
    }
}

// Fix incomplete Eq trait for VENC_PACK_INFO_S
#[cfg(feature = "mpi-venc")]
impl Eq for VENC_PACK_INFO_S {}

// Fix incomplete Debug trait for VENC_PACK_S
#[cfg(feature = "mpi-venc")]
impl std::fmt::Debug for VENC_PACK_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VENC_PACK_S")
            .field("u64PhyAddr", &self.u64PhyAddr)
            .field("pu8Addr", &self.pu8Addr)
            .field("u32Len", &self.u32Len)
            .field("u64PTS", &self.u64PTS)
            .field("bFrameEnd", &self.bFrameEnd)
            .field("DataType", &self.DataType)
            .field("u32Offset", &self.u32Offset)
            .field("u32DataNum", &self.u32DataNum)
            .field("stPackInfo", &&self.stPackInfo[..self.u32DataNum as usize])
            .finish()
    }
}

// Fix incomplete PartialEq trait for VENC_PACK_S
#[cfg(feature = "mpi-venc")]
impl PartialEq for VENC_PACK_S {
    fn eq(&self, other: &Self) -> bool {
        self.u64PhyAddr == other.u64PhyAddr
            && self.u32Len == other.u32Len
            && self.u64PTS == other.u64PTS
            && self.bFrameEnd == other.bFrameEnd
            && self.DataType == other.DataType
            && self.u32Offset == other.u32Offset
            && self.u32DataNum == other.u32DataNum
            && self.stPackInfo == other.stPackInfo
    }
}

// Fix incomplete Eq trait for VENC_PACK_S
#[cfg(feature = "mpi-venc")]
impl Eq for VENC_PACK_S {}

// Fix Default trait for VENC_RC_MODE_E
#[cfg(feature = "mpi-venc")]
impl Default for VENC_RC_MODE_E {
    fn default() -> Self {
        VENC_RC_MODE_E::VENC_RC_MODE_H264CBR
    }
}

// Fix incomplete Debug trait for VENC_RC_ATTR_S
#[cfg(feature = "mpi-venc")]
impl std::fmt::Debug for VENC_RC_ATTR_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fds = f.debug_struct("VENC_RC_ATTR_S");
        // Prevent crash if `enRcMode` not in range.
        let num = self.enRcMode as u32;
        if num < VENC_RC_MODE_E::VENC_RC_MODE_H264CBR as u32
            || num >= VENC_RC_MODE_E::VENC_RC_MODE_BUTT as u32
        {
            return fds.field("enRcMode", &"Invalid").finish();
        }
        fds.field("enRcMode", &self.enRcMode);
        unsafe {
            use VENC_RC_MODE_E::*;
            match self.enRcMode {
                VENC_RC_MODE_H264CBR => fds.field("stH264Cbr", &self.un1.stH264Cbr).finish(),
                VENC_RC_MODE_H264VBR => fds.field("stH264Vbr", &self.un1.stH264Vbr).finish(),
                VENC_RC_MODE_H264AVBR => fds.field("stH264AVbr", &self.un1.stH264AVbr).finish(),
                VENC_RC_MODE_H264QVBR => fds.field("stH264QVbr", &self.un1.stH264QVbr).finish(),
                VENC_RC_MODE_H264CVBR => fds.field("stH264CVbr", &self.un1.stH264CVbr).finish(),
                VENC_RC_MODE_H264FIXQP => fds.field("stH264FixQp", &self.un1.stH264FixQp).finish(),
                VENC_RC_MODE_H264QPMAP => fds.field("stH264QpMap", &self.un1.stH264QpMap).finish(),
                VENC_RC_MODE_MJPEGCBR => fds.field("stMjpegCbr", &self.un1.stMjpegCbr).finish(),
                VENC_RC_MODE_MJPEGVBR => fds.field("stMjpegVbr", &self.un1.stMjpegVbr).finish(),
                VENC_RC_MODE_MJPEGFIXQP => {
                    fds.field("stMjpegFixQp", &self.un1.stMjpegFixQp).finish()
                }
                VENC_RC_MODE_H265CBR => fds.field("stH265Cbr", &self.un1.stH265Cbr).finish(),
                VENC_RC_MODE_H265VBR => fds.field("stH265Vbr", &self.un1.stH265Vbr).finish(),
                VENC_RC_MODE_H265AVBR => fds.field("stH265AVbr", &self.un1.stH265AVbr).finish(),
                VENC_RC_MODE_H265QVBR => fds.field("stH265QVbr", &self.un1.stH265QVbr).finish(),
                VENC_RC_MODE_H265CVBR => fds.field("stH265CVbr", &self.un1.stH265CVbr).finish(),
                VENC_RC_MODE_H265FIXQP => fds.field("stH265FixQp", &self.un1.stH265FixQp).finish(),
                VENC_RC_MODE_H265QPMAP => fds.field("stH265QpMap", &self.un1.stH265QpMap).finish(),
                _ => fds.finish(),
            }
        }
    }
}

// Fix incomplete PartialEq trait for VENC_RC_ATTR_S
#[cfg(feature = "mpi-venc")]
impl PartialEq for VENC_RC_ATTR_S {
    fn eq(&self, other: &Self) -> bool {
        if self.enRcMode != other.enRcMode {
            return false;
        }
        unsafe {
            use VENC_RC_MODE_E::*;
            match self.enRcMode {
                VENC_RC_MODE_H264CBR => self.un1.stH264Cbr == other.un1.stH264Cbr,
                VENC_RC_MODE_H264VBR => self.un1.stH264Vbr == other.un1.stH264Vbr,
                VENC_RC_MODE_H264AVBR => self.un1.stH264AVbr == other.un1.stH264AVbr,
                VENC_RC_MODE_H264QVBR => self.un1.stH264QVbr == other.un1.stH264QVbr,
                VENC_RC_MODE_H264CVBR => self.un1.stH264CVbr == other.un1.stH264CVbr,
                VENC_RC_MODE_H264FIXQP => self.un1.stH264FixQp == other.un1.stH264FixQp,
                VENC_RC_MODE_H264QPMAP => self.un1.stH264QpMap == other.un1.stH264QpMap,
                VENC_RC_MODE_MJPEGCBR => self.un1.stMjpegCbr == other.un1.stMjpegCbr,
                VENC_RC_MODE_MJPEGVBR => self.un1.stMjpegVbr == other.un1.stMjpegVbr,
                VENC_RC_MODE_MJPEGFIXQP => self.un1.stMjpegFixQp == other.un1.stMjpegFixQp,
                VENC_RC_MODE_H265CBR => self.un1.stH265Cbr == other.un1.stH265Cbr,
                VENC_RC_MODE_H265VBR => self.un1.stH265Vbr == other.un1.stH265Vbr,
                VENC_RC_MODE_H265AVBR => self.un1.stH265AVbr == other.un1.stH265AVbr,
                VENC_RC_MODE_H265QVBR => self.un1.stH265QVbr == other.un1.stH265QVbr,
                VENC_RC_MODE_H265CVBR => self.un1.stH265CVbr == other.un1.stH265CVbr,
                VENC_RC_MODE_H265FIXQP => self.un1.stH265FixQp == other.un1.stH265FixQp,
                VENC_RC_MODE_H265QPMAP => self.un1.stH265QpMap == other.un1.stH265QpMap,
                _ => false,
            }
        }
    }
}

// Fix incomplete Eq trait for VENC_RC_ATTR_S
#[cfg(feature = "mpi-venc")]
impl Eq for VENC_RC_ATTR_S {}

// Fix incomplete Debug trait for VENC_STREAM_S
#[cfg(feature = "mpi-venc")]
impl std::fmt::Debug for VENC_STREAM_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fds = f.debug_struct("VENC_STREAM_S");
        fds.field("pstPack", &self.pstPack)
            .field("u32PackCount", &self.u32PackCount)
            .field("u32Seq", &self.u32Seq);
        fds.finish()
    }
}

// Fix incomplete PartialEq trait for VENC_STREAM_S
#[cfg(feature = "mpi-venc")]
impl PartialEq for VENC_STREAM_S {
    fn eq(&self, other: &Self) -> bool {
        self.pstPack == other.pstPack
            && self.u32PackCount == other.u32PackCount
            && self.u32Seq == other.u32Seq
    }
}

// Fix incomplete Eq trait for VENC_STREAM_S
#[cfg(feature = "mpi-venc")]
impl Eq for VENC_STREAM_S {}

// Force impl Send for VENC_STREAM_S
#[cfg(feature = "mpi-venc")]
unsafe impl Send for VENC_STREAM_S {}

// Force impl Sync for VENC_STREAM_S
#[cfg(feature = "mpi-venc")]
unsafe impl Sync for VENC_STREAM_S {}

// Fix incomplete Eq trait for VENC_STREAM_INFO_S
#[cfg(feature = "mpi-venc")]
impl Eq for VENC_STREAM_INFO_S {}

#[cfg(feature = "mpi-vgs")]
impl_as_bundle_many!(
    VGS_ADD_COVER_S,
    VGS_OSD_REVERT_S,
    // VGS_MODULE_PARAMS_S,
    VGS_QUADRANGLE_COVER_S,
    VGS_TASK_ATTR_S,
);

// Fix incomplete Debug trait for VGS_ADD_COVER_S
#[cfg(feature = "mpi-vgs")]
impl std::fmt::Debug for VGS_ADD_COVER_S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("VGS_ADD_COVER_S");
        ds.field("enCoverType", &self.enCoverType);
        match self.enCoverType {
            VGS_COVER_TYPE_E::COVER_RECT => unsafe {
                ds.field("stDstRect", &self.un1.stDstRect);
            },
            VGS_COVER_TYPE_E::COVER_QUAD_RANGLE => unsafe {
                ds.field("stQuadRangle", &self.un1.stQuadRangle);
            },
            _ => {}
        }
        ds.field("u32Color", &self.u32Color).finish()
    }
}

// Fix incomplete PartialEq trait for VGS_ADD_COVER_S
#[cfg(feature = "mpi-vgs")]
impl PartialEq for VGS_ADD_COVER_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            self.enCoverType == other.enCoverType
                && self.un1.stDstRect == other.un1.stDstRect
                && self.un1.stQuadRangle == other.un1.stQuadRangle
                && self.u32Color == other.u32Color
        }
    }
}

// Fix incomplete Eq trait for VGS_ADD_COVER_S
#[cfg(feature = "mpi-vgs")]
impl Eq for VGS_ADD_COVER_S {}

#[cfg(feature = "mpi-vi")]
impl_as_bundle_many!(
    BNR_DUMP_ATTR_S,
    DIS_ATTR_S,
    DIS_CONFIG_S,
    NRX_PARAM_AUTO_V1_S,
    NRX_PARAM_MANUAL_V1_S,
    NRX_PARAM_V1_S,
    NRX_PARAM_AUTO_V2_S,
    NRX_PARAM_MANUAL_V2_S,
    NRX_PARAM_V2_S,
    VI_BAS_ATTR_S,
    VI_BAS_REPHASE_ATTR_S,
    VI_BAS_SCALE_ATTR_S,
    VI_BT656_SYNC_CFG_S,
    VI_CHN_ATTR_S,
    VI_CHN_STATUS_S,
    VI_CMP_PARAM_S,
    VI_CROP_INFO_S,
    VI_DEV_ATTR_S,
    VI_DEV_ATTR_EX_S,
    VI_DEV_BIND_PIPE_S,
    VI_DEV_TIMING_ATTR_S,
    VI_DUMP_ATTR_S,
    VI_EARLY_INTERRUPT_S,
    VI_EXT_CHN_ATTR_S,
    VI_LDC_ATTR_S,
    VI_LDCV2_ATTR_S,
    VI_LDCV3_ATTR_S,
    VI_LOW_DELAY_INFO_S,
    VI_MOD_PARAM_S,
    VI_NR_ATTR_S,
    VI_PIPE_ATTR_S,
    VI_PIPE_NRX_PARAM_V1_S,
    VI_PIPE_NRX_PARAM_V2_S,
    VI_PIPE_NRX_PARAM_S,
    VI_PIPE_STATUS_S,
    VI_ROTATION_EX_ATTR_S,
    VI_STITCH_GRP_ATTR_S,
    VI_SYNC_CFG_S,
    VI_TIMING_BLANK_S,
    VI_USERPIC_ATTR_S,
    VI_USERPIC_BGC_S,
    VI_VS_SIGNAL_ATTR_S,
    VI_WDR_ATTR_S,
);

#[cfg(feature = "mpi-video")]
impl_as_bundle_many!(
    ASPECT_RATIO_S,
    BITMAP_S,
    VIDEO_FRAME_S,
    VIDEO_FRAME_INFO_S,
    VIDEO_REGION_INFO_S,
    VIDEO_SUPPLEMENT_S,
);

#[cfg(feature = "mpi-video")]
impl Default for COLOR_GAMUT_E {
    fn default() -> Self {
        Self::COLOR_GAMUT_BT601
    }
}

#[cfg(feature = "mpi-video")]
impl Default for COMPRESS_MODE_E {
    fn default() -> Self {
        Self::COMPRESS_MODE_NONE
    }
}

#[cfg(feature = "mpi-video")]
impl Default for DYNAMIC_RANGE_E {
    fn default() -> Self {
        Self::DYNAMIC_RANGE_SDR8
    }
}

#[cfg(feature = "mpi-video")]
impl Default for PIXEL_FORMAT_E {
    fn default() -> Self {
        Self::PIXEL_FORMAT_YUV_SEMIPLANAR_420
    }
}

#[cfg(feature = "mpi-video")]
impl Default for VIDEO_FIELD_E {
    fn default() -> Self {
        Self::VIDEO_FIELD_FRAME
    }
}

#[cfg(feature = "mpi-video")]
impl Default for VIDEO_FORMAT_E {
    fn default() -> Self {
        Self::VIDEO_FORMAT_LINEAR
    }
}

#[cfg(feature = "mpi-video")]
impl Default for VIDEO_FRAME_S {
    fn default() -> Self {
        Self {
            u32Width: 0,
            u32Height: 0,
            enField: Default::default(),
            enPixelFormat: Default::default(),
            enVideoFormat: Default::default(),
            enCompressMode: Default::default(),
            enDynamicRange: Default::default(),
            enColorGamut: Default::default(),
            u32HeaderStride: Default::default(),
            u32Stride: Default::default(),
            u32ExtStride: Default::default(),
            u64HeaderPhyAddr: Default::default(),
            u64HeaderVirAddr: Default::default(),
            u64PhyAddr: Default::default(),
            u64VirAddr: Default::default(),
            u64ExtPhyAddr: Default::default(),
            u64ExtVirAddr: Default::default(),
            s16OffsetTop: 0,
            s16OffsetBottom: 0,
            s16OffsetLeft: 0,
            s16OffsetRight: 0,
            u32MaxLuminance: 0,
            u32MinLuminance: 0,
            u32TimeRef: 0,
            u64PTS: 0,
            u64PrivateData: 0,
            u32FrameFlag: 0,
            stSupplement: Default::default(),
        }
    }
}

#[cfg(feature = "mpi-video")]
impl Default for VIDEO_FRAME_INFO_S {
    fn default() -> Self {
        Self {
            stVFrame: Default::default(),
            u32PoolId: 0,
            enModId: MOD_ID_E::HI_ID_VB,
        }
    }
}

#[cfg(feature = "mpi-video")]
impl Default for VIDEO_SUPPLEMENT_S {
    fn default() -> Self {
        Self {
            u64JpegDCFPhyAddr: 0,
            u64IspInfoPhyAddr: 0,
            u64LowDelayPhyAddr: 0,
            u64MotionDataPhyAddr: 0,
            u64FrameDNGPhyAddr: 0,
            pJpegDCFVirAddr: std::ptr::null_mut(),
            pIspInfoVirAddr: std::ptr::null_mut(),
            pLowDelayVirAddr: std::ptr::null_mut(),
            pMotionDataVirAddr: std::ptr::null_mut(),
            pFrameDNGVirAddr: std::ptr::null_mut(),
        }
    }
}

#[cfg(feature = "mpi-vo")]
impl_as_bundle_many!(
    VO_BORDER_S,
    VO_CHN_ATTR_S,
    VO_CHN_BOUNDARY_S,
    VO_CHN_PARAM_S,
    VO_CSC_S,
    VO_LAYER_BOUNDARY_S,
    VO_LAYER_PARAM_S,
    VO_MOD_PARAM_S,
    VO_PUB_ATTR_S,
    VO_QUERY_STATUS_S,
    VO_REGION_INFO_S,
    VO_USER_INTFSYNC_ATTR_S,
    VO_USER_INTFSYNC_INFO_S,
    VO_USER_INTFSYNC_PLL_S,
    VO_VIDEO_LAYER_ATTR_S,
    VO_WBC_ATTR_S,
    VO_WBC_SOURCE_S,
    VO_ZOOM_RATIO_S,
);

#[cfg(feature = "mpi-vpss")]
impl_as_bundle_many!(
    VPSS_CHN_ATTR_S,
    VPSS_CROP_INFO_S,
    VPSS_EXT_CHN_ATTR_S,
    VPSS_GRP_ATTR_S,
    VPSS_GRP_NRX_PARAM_S,
    VPSS_GRP_SHARPEN_ATTR_S,
    VPSS_GRP_SHARPEN_AUTO_ATTR_S,
    VPSS_GRP_SHARPEN_MANUAL_ATTR_S,
    VPSS_LDC_ATTR_S,
    VPSS_LOW_DELAY_INFO_S,
    VPSS_MOD_PARAM_S,
    VPSS_NR_ATTR_S,
    VPSS_NRX_PARAM_AUTO_V1_S,
    VPSS_NRX_PARAM_MANUAL_V1_S,
    VPSS_NRX_PARAM_V1_S,
    VPSS_NRX_V1_S,
    VPSS_ROTATION_EX_ATTR_S,
);

#[cfg(all(
    feature = "mpi-vpss",
    any(
        feature = "hi3516cv500",
        feature = "hi3516dv300",
        feature = "hi3556v200",
        feature = "hi3559v200"
    )
))]
impl_as_bundle!(
    VPSS_NRX_PARAM_AUTO_V2_S,
    VPSS_NRX_PARAM_MANUAL_V2_S,
    VPSS_NRX_PARAM_V2_S,
    VPSS_NRX_V2_S,
);

#[cfg(all(feature = "mpi-vpss", any(feature = "hi3516ev200")))]
impl_as_bundle_many!(
    VPSS_CHN_BUF_WRAP_S,
    VPSS_LDCV3_ATTR_S,
    VPSS_NRX_PARAM_AUTO_V3_S,
    VPSS_NRX_PARAM_MANUAL_V3_S,
    VPSS_NRX_PARAM_V3_S,
    VPSS_NRX_V3_S,
);

#[cfg(feature = "mpi-vpss")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, BitFields)]
pub struct tV56aMDy {
    /// The bitfields for `MADZ`, `MAI0`, `MAI1` and `MAI2`.
    /// * MADZ - Range: `[0, 999]`
    /// * MAI0 - Range: `[0,   3]`
    /// * MAI1 - Range: `[0,   3]`
    /// * MAI2 - Range: `[0,   3]`
    #[bitfield(MADZ : [9:0] "The blending ratio between MAI2 and MAI1 based on image statistics.")]
    #[bitfield(MAI0 : [11:10] "The three blending results between spatial and temporal filtering.")]
    #[bitfield(MAI1 : [13:12] "The three blending results between spatial and temporal filtering.")]
    #[bitfield(MAI2 : [15:14] "The three blending results between spatial and temporal filtering.")]
    pub _bitfield1: u16,
    /// The blending ratio between MAI2 and MAI1 based on brightness. (low limit).
    /// * MADK - Range: `[0, 255]`
    pub MADK: u8,
    /// The blending ratio between MAI2 and MAI1 based on brightness. (high limit).
    /// * MABR - Range: `[0, 255]`
    pub MABR: u8,
    /// The bitfields for `MATH`, `MATE` and `MATW`.
    /// * MATH - Range: `[0, 999]`
    /// * MATE - Range: `[0,   8]`
    /// * MATW - Range: `[0,   3]`
    #[bitfield(MATH : [9:0] "The theshold for motion detection.")]
    #[bitfield(MATE : [13:10] "The motion index for smooth image area.")]
    #[bitfield(MATW : [15:14] "The motion index for prevention of motion ghost.")]
    pub _bitfield2: u16,
    /// The bitfields for `MATH`, `MATE` and `MATW`.
    /// * MASW - Range: `[0,  15]`
    /// * MABW - Range: `[0,   4]`
    /// * MAXN - Range: `[0,   1]`
    #[bitfield(MASW : [3:0] "The motion index for low-frequency noises.")]
    #[bitfield(MABW : [6:4] "The window size for motion detection.")]
    #[bitfield(MAXN : [7] "Not for tunning.")]
    pub _bitfield3: u8,
    pub _rB_: u8,
}

#[cfg(feature = "mpi-vpss")]
#[allow(clippy::all)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, BitFields)]
pub struct tV56aNRc {
    /// Spatial NR strength for the first level.
    /// * SFC - Range: `[0, 255]`
    pub SFC: u8,
    /// The bitfields for `_rb_` and `TFC`.
    /// * TFC - Range: `[0,  63]`
    #[bitfield(_rb_ : [1:0])]
    #[bitfield(TFC : [7:2] "Temporal NR strength.")]
    pub _bitfield1: u8,
    /// The bitfields for `CSFS` and `CSFR`.
    /// * CSFS - Range: `[0, 999]`
    /// * CSFR - Range: `[0,  32]`
    #[bitfield(CSFS : [9:0] "Spatial NR strength for the second level.")]
    #[bitfield(CSFR : [15:10] "Spatial NR strength control.")]
    pub _bitfield2: u16,
    /// The bitfield for `CTFR`, `CTFS` and `CIIR`.
    /// * CTFR - Range: `[0, 999]`
    /// * CTFS - Range: `[0,  15]`
    /// * CIIR - Range: `[0,   1]`
    #[bitfield(CTFR : [10:0] "Temporal NR strength control.")]
    #[bitfield(CTFS : [14:1] "Temporal NR filtering strength.")]
    #[bitfield(CIIR : [15] "Spatial NR mode for the first level.")]
    pub _bitfield3: u16,
    /// The bitfields for `MODE` and `_rb1_`.
    /// * MODE - Range: `[0,  1]`
    #[bitfield(MODE : [0] "The switch for new chroma denoise mode.")]
    #[bitfield(_rb1_ : [7:1])]
    pub _bitfield4: u8,
    /// The bitfields for `PRESFC` and `_rb2_`.
    /// * PRESFC - Range: `[0,  32]`
    #[bitfield(PRESFC : [5:0] "The strength for chroma pre spatial filter.")]
    #[bitfield(_rb2_ : [7:6])]
    pub _bitfield5: u8,
}

#[cfg(feature = "mpi-vpss")]
#[allow(clippy::all)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, BitFields)]
pub struct tV56aTFy {
    /// The bitfields for `TFS`, `TDZ` and `TDX`.
    /// * TFS - Range: `[0,  15]`
    /// * TDZ - Range: `[0, 999]`
    /// * TDX - Range: `[0,   3]`
    #[bitfield(TFS : [3:0] "The NR strength for temporal filtering.")]
    #[bitfield(TDZ : [13:4] "Protection of the weak texture area from temporal filtering.")]
    #[bitfield(TDX : [15:14] "Not for tuning.")]
    pub _bitfield1: u16,
    /// The temoproal NR strength control for background (static) area.
    /// * TFR - Range: `[0,  31]`
    pub TFR: [u8; 5],
    /// The bitfields for `TSS`, `TSI` and `_rb_`.
    /// * TSS - Range: `[0,  15]`
    /// * TSI - Range: `[0,   1]`
    #[bitfield(TSS : [3:0] "The ratio for blending spatial NR with the temproal NR results.")]
    #[bitfield(TSI : [4] "The selection of blending filter for TSS.")]
    #[bitfield(_rb_ : [6:5])]
    pub _bitfield2: u8,
    /// The bitfields for `SDZ`, `STR` and `bRef`.
    /// * SDZ  - Range: `[0, 999]`
    /// * STR  - Range: `[0,  31]`
    /// * bRef - Range: `[0,   1]`
    #[bitfield(SDZ : [9:0] "The threshold of NR control for result MAI1.")]
    #[bitfield(STR : [14:10] "The strength of NR control for result MAI1.")]
    #[bitfield(bRef : [15] "The switch for temproal filtering.")]
    pub _bitfield3: u16,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "hi3531v100")] {
        pub use HI_MPI_SYS_Mmap as HI_MPI_SYS_MmapCache;
        pub use HI_MPI_SYS_GetCurPts as HI_MPI_SYS_GetCurPTS;
        pub use HI_MPI_SYS_InitPtsBase as HI_MPI_SYS_InitPTSBase;
        pub use HI_MPI_SYS_SyncPts as HI_MPI_SYS_SyncPTS;
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
