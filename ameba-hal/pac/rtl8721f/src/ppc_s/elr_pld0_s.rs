#[doc = "Register `ELR_PLD0_S` reader"]
pub type R = crate::R<ElrPld0SSpec>;
#[doc = "Field `ELR_SRC_S` reader - Error source of the error request. 0x10: S1 securedefault slave 0x20: S2 securedefault slave 0x30: S3-0~1,5,7(AHB) group secure default slave 0x31: S3-2~4,6,13~22(AHB) group secure default slave 0x32: S3-8(AXI) secure default slave 0x33:S3-9~10(SPIC)group secure default slave 0x34: S3-11~12(PSRAM)group secure default slave 0x60: S4-0~6 secure default slave 0x61: S4-7~15 secure default slave 0x70: S5 secure default slave 0x71: MPC/PPC secure default slave 0x80: S8 secure default slave"]
pub type ElrSrcSR = crate::FieldReader;
#[doc = "Field `ELR_CMD_S` reader - CMD of the error request. 0x1: Write command 0x2: Read command"]
pub type ElrCmdSR = crate::FieldReader;
#[doc = "Field `ELR_BSTTYPE_S` reader - BurstSeq of the error request."]
pub type ElrBsttypeSR = crate::FieldReader;
#[doc = "Field `ELR_BSTLEN_S` reader - Burst Length of the error request."]
pub type ElrBstlenSR = crate::FieldReader;
#[doc = "Field `ELR_BSTINDEX_S` reader - Burst index of the error request."]
pub type ElrBstindexSR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Error source of the error request. 0x10: S1 securedefault slave 0x20: S2 securedefault slave 0x30: S3-0~1,5,7(AHB) group secure default slave 0x31: S3-2~4,6,13~22(AHB) group secure default slave 0x32: S3-8(AXI) secure default slave 0x33:S3-9~10(SPIC)group secure default slave 0x34: S3-11~12(PSRAM)group secure default slave 0x60: S4-0~6 secure default slave 0x61: S4-7~15 secure default slave 0x70: S5 secure default slave 0x71: MPC/PPC secure default slave 0x80: S8 secure default slave"]
    #[inline(always)]
    pub fn elr_src_s(&self) -> ElrSrcSR {
        ElrSrcSR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - CMD of the error request. 0x1: Write command 0x2: Read command"]
    #[inline(always)]
    pub fn elr_cmd_s(&self) -> ElrCmdSR {
        ElrCmdSR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - BurstSeq of the error request."]
    #[inline(always)]
    pub fn elr_bsttype_s(&self) -> ElrBsttypeSR {
        ElrBsttypeSR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Burst Length of the error request."]
    #[inline(always)]
    pub fn elr_bstlen_s(&self) -> ElrBstlenSR {
        ElrBstlenSR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Burst index of the error request."]
    #[inline(always)]
    pub fn elr_bstindex_s(&self) -> ElrBstindexSR {
        ElrBstindexSR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Error log payload register0\n\nYou can [`read`](crate::Reg::read) this register and get [`elr_pld0_s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrPld0SSpec;
impl crate::RegisterSpec for ElrPld0SSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elr_pld0_s::R`](R) reader structure"]
impl crate::Readable for ElrPld0SSpec {}
#[doc = "`reset()` method sets ELR_PLD0_S to value 0"]
impl crate::Resettable for ElrPld0SSpec {}
