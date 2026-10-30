#[doc = "Register `REG_SCOREBOARD_RD_BT2WL` reader"]
pub type R = crate::R<RegScoreboardRdBt2wlSpec>;
#[doc = "Field `BIT_RD_BT2WL_INFO` reader - Read Data : WL TO BT INFO, WLON Status Report"]
pub type BitRdBt2wlInfoR = crate::FieldReader;
impl R {
    #[doc = "Bits 24:31 - Read Data : WL TO BT INFO, WLON Status Report"]
    #[inline(always)]
    pub fn bit_rd_bt2wl_info(&self) -> BitRdBt2wlInfoR {
        BitRdBt2wlInfoR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_scoreboard_rd_bt2wl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegScoreboardRdBt2wlSpec;
impl crate::RegisterSpec for RegScoreboardRdBt2wlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_scoreboard_rd_bt2wl::R`](R) reader structure"]
impl crate::Readable for RegScoreboardRdBt2wlSpec {}
#[doc = "`reset()` method sets REG_SCOREBOARD_RD_BT2WL to value 0"]
impl crate::Resettable for RegScoreboardRdBt2wlSpec {}
