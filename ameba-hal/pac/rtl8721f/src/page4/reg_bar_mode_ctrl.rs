#[doc = "Register `REG_BAR_MODE_CTRL` reader"]
pub type R = crate::R<RegBarModeCtrlSpec>;
#[doc = "Register `REG_BAR_MODE_CTRL` writer"]
pub type W = crate::W<RegBarModeCtrlSpec>;
#[doc = "Field `BIT_BAR_PKTNUM_TH_V1` reader - BAR Pktnum Thrs"]
pub type BitBarPktnumThV1R = crate::FieldReader;
#[doc = "Field `BIT_BAR_PKTNUM_TH_V1` writer - BAR Pktnum Thrs"]
pub type BitBarPktnumThV1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_BAR_PKTTIME_THRS` reader - BAR packet time thrs"]
pub type BitBarPkttimeThrsR = crate::FieldReader;
#[doc = "Field `BIT_BAR_PKTTIME_THRS` writer - BAR packet time thrs"]
pub type BitBarPkttimeThrsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BAR_RTY_LMT` reader - BAR retry limit"]
pub type BitBarRtyLmtR = crate::FieldReader;
#[doc = "Field `BIT_BAR_RTY_LMT` writer - BAR retry limit"]
pub type BitBarRtyLmtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RTY_PKT_LMT` reader - retry packet limit"]
pub type BitRtyPktLmtR = crate::FieldReader;
#[doc = "Field `BIT_RTY_PKT_LMT` writer - retry packet limit"]
pub type BitRtyPktLmtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:6 - BAR Pktnum Thrs"]
    #[inline(always)]
    pub fn bit_bar_pktnum_th_v1(&self) -> BitBarPktnumThV1R {
        BitBarPktnumThV1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - BAR packet time thrs"]
    #[inline(always)]
    pub fn bit_bar_pkttime_thrs(&self) -> BitBarPkttimeThrsR {
        BitBarPkttimeThrsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - BAR retry limit"]
    #[inline(always)]
    pub fn bit_bar_rty_lmt(&self) -> BitBarRtyLmtR {
        BitBarRtyLmtR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:29 - retry packet limit"]
    #[inline(always)]
    pub fn bit_rty_pkt_lmt(&self) -> BitRtyPktLmtR {
        BitRtyPktLmtR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - BAR Pktnum Thrs"]
    #[inline(always)]
    pub fn bit_bar_pktnum_th_v1(&mut self) -> BitBarPktnumThV1W<'_, RegBarModeCtrlSpec> {
        BitBarPktnumThV1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - BAR packet time thrs"]
    #[inline(always)]
    pub fn bit_bar_pkttime_thrs(&mut self) -> BitBarPkttimeThrsW<'_, RegBarModeCtrlSpec> {
        BitBarPkttimeThrsW::new(self, 8)
    }
    #[doc = "Bits 16:17 - BAR retry limit"]
    #[inline(always)]
    pub fn bit_bar_rty_lmt(&mut self) -> BitBarRtyLmtW<'_, RegBarModeCtrlSpec> {
        BitBarRtyLmtW::new(self, 16)
    }
    #[doc = "Bits 24:29 - retry packet limit"]
    #[inline(always)]
    pub fn bit_rty_pkt_lmt(&mut self) -> BitRtyPktLmtW<'_, RegBarModeCtrlSpec> {
        BitRtyPktLmtW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bar_mode_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bar_mode_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBarModeCtrlSpec;
impl crate::RegisterSpec for RegBarModeCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bar_mode_ctrl::R`](R) reader structure"]
impl crate::Readable for RegBarModeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bar_mode_ctrl::W`](W) writer structure"]
impl crate::Writable for RegBarModeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BAR_MODE_CTRL to value 0x0102_ff7f"]
impl crate::Resettable for RegBarModeCtrlSpec {
    const RESET_VALUE: u32 = 0x0102_ff7f;
}
