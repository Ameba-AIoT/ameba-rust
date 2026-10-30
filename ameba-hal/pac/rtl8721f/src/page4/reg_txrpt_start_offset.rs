#[doc = "Register `REG_TXRPT_START_OFFSET` reader"]
pub type R = crate::R<RegTxrptStartOffsetSpec>;
#[doc = "Register `REG_TXRPT_START_OFFSET` writer"]
pub type W = crate::W<RegTxrptStartOffsetSpec>;
#[doc = "Field `BIT_AMPDU_TXRPT_OFFSET` reader - offset of AMPDU txrpt"]
pub type BitAmpduTxrptOffsetR = crate::FieldReader;
#[doc = "Field `BIT_AMPDU_TXRPT_OFFSET` writer - offset of AMPDU txrpt"]
pub type BitAmpduTxrptOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_MACID_CTRL_OFFSET` reader - offset of macid control: ctrl_info in the start of txrptbuf"]
pub type BitMacidCtrlOffsetR = crate::FieldReader;
#[doc = "Field `BIT_MACID_CTRL_OFFSET` writer - offset of macid control: ctrl_info in the start of txrptbuf"]
pub type BitMacidCtrlOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_rptfifo_1k` reader - select the size of rptfifo"]
pub type BitRptfifo1kR = crate::BitReader;
#[doc = "Field `BIT_rptfifo_1k` writer - select the size of rptfifo"]
pub type BitRptfifo1kW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - offset of AMPDU txrpt"]
    #[inline(always)]
    pub fn bit_ampdu_txrpt_offset(&self) -> BitAmpduTxrptOffsetR {
        BitAmpduTxrptOffsetR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - offset of macid control: ctrl_info in the start of txrptbuf"]
    #[inline(always)]
    pub fn bit_macid_ctrl_offset(&self) -> BitMacidCtrlOffsetR {
        BitMacidCtrlOffsetR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - select the size of rptfifo"]
    #[inline(always)]
    pub fn bit_rptfifo_1k(&self) -> BitRptfifo1kR {
        BitRptfifo1kR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - offset of AMPDU txrpt"]
    #[inline(always)]
    pub fn bit_ampdu_txrpt_offset(&mut self) -> BitAmpduTxrptOffsetW<'_, RegTxrptStartOffsetSpec> {
        BitAmpduTxrptOffsetW::new(self, 0)
    }
    #[doc = "Bits 8:15 - offset of macid control: ctrl_info in the start of txrptbuf"]
    #[inline(always)]
    pub fn bit_macid_ctrl_offset(&mut self) -> BitMacidCtrlOffsetW<'_, RegTxrptStartOffsetSpec> {
        BitMacidCtrlOffsetW::new(self, 8)
    }
    #[doc = "Bit 16 - select the size of rptfifo"]
    #[inline(always)]
    pub fn bit_rptfifo_1k(&mut self) -> BitRptfifo1kW<'_, RegTxrptStartOffsetSpec> {
        BitRptfifo1kW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txrpt_start_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_txrpt_start_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxrptStartOffsetSpec;
impl crate::RegisterSpec for RegTxrptStartOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txrpt_start_offset::R`](R) reader structure"]
impl crate::Readable for RegTxrptStartOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_txrpt_start_offset::W`](W) writer structure"]
impl crate::Writable for RegTxrptStartOffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TXRPT_START_OFFSET to value 0x05"]
impl crate::Resettable for RegTxrptStartOffsetSpec {
    const RESET_VALUE: u32 = 0x05;
}
