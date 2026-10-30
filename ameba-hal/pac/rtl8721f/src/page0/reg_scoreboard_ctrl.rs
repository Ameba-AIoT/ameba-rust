#[doc = "Register `REG_SCOREBOARD_CTRL` reader"]
pub type R = crate::R<RegScoreboardCtrlSpec>;
#[doc = "Register `REG_SCOREBOARD_CTRL` writer"]
pub type W = crate::W<RegScoreboardCtrlSpec>;
#[doc = "Field `BIT_WL2BT_INFO` reader - WLON Status Report TO BTON"]
pub type BitWl2btInfoR = crate::FieldReader;
#[doc = "Field `BIT_WL2BT_INFO` writer - WLON Status Report TO BTON"]
pub type BitWl2btInfoW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_BT_INT_EN` writer - For WL, write 1 to issue interrupt 1 : Issue one INT to BT 0 : Don't care"]
pub type BitBtIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 24:30 - WLON Status Report TO BTON"]
    #[inline(always)]
    pub fn bit_wl2bt_info(&self) -> BitWl2btInfoR {
        BitWl2btInfoR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30 - WLON Status Report TO BTON"]
    #[inline(always)]
    pub fn bit_wl2bt_info(&mut self) -> BitWl2btInfoW<'_, RegScoreboardCtrlSpec> {
        BitWl2btInfoW::new(self, 24)
    }
    #[doc = "Bit 31 - For WL, write 1 to issue interrupt 1 : Issue one INT to BT 0 : Don't care"]
    #[inline(always)]
    pub fn bit_bt_int_en(&mut self) -> BitBtIntEnW<'_, RegScoreboardCtrlSpec> {
        BitBtIntEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_scoreboard_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_scoreboard_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegScoreboardCtrlSpec;
impl crate::RegisterSpec for RegScoreboardCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_scoreboard_ctrl::R`](R) reader structure"]
impl crate::Readable for RegScoreboardCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_scoreboard_ctrl::W`](W) writer structure"]
impl crate::Writable for RegScoreboardCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SCOREBOARD_CTRL to value 0"]
impl crate::Resettable for RegScoreboardCtrlSpec {}
