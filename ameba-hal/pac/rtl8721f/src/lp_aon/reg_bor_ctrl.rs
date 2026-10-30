#[doc = "Register `REG_BOR_CTRL` reader"]
pub type R = crate::R<RegBorCtrlSpec>;
#[doc = "Register `REG_BOR_CTRL` writer"]
pub type W = crate::W<RegBorCtrlSpec>;
#[doc = "Field `bor_tdbc` reader - 000: 0us 001: 100us 010: 500us 011: 1000us 100: 2000us 101: 4000us 110: 8000us 111: 16000us"]
pub type BorTdbcR = crate::FieldReader;
#[doc = "Field `bor_tdbc` writer - 000: 0us 001: 100us 010: 500us 011: 1000us 100: 2000us 101: 4000us 110: 8000us 111: 16000us"]
pub type BorTdbcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `bor_intr_mode` reader - BOR operation mode. 0: reset mode. When analog detect bor, hw will trigger a bor reset. 1: interrup mode. It will raise an interrup first. If this bit set to 0, then trigger a bor reset."]
pub type BorIntrModeR = crate::BitReader;
#[doc = "Field `bor_intr_mode` writer - BOR operation mode. 0: reset mode. When analog detect bor, hw will trigger a bor reset. 1: interrup mode. It will raise an interrup first. If this bit set to 0, then trigger a bor reset."]
pub type BorIntrModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 000: 0us 001: 100us 010: 500us 011: 1000us 100: 2000us 101: 4000us 110: 8000us 111: 16000us"]
    #[inline(always)]
    pub fn bor_tdbc(&self) -> BorTdbcR {
        BorTdbcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 16 - BOR operation mode. 0: reset mode. When analog detect bor, hw will trigger a bor reset. 1: interrup mode. It will raise an interrup first. If this bit set to 0, then trigger a bor reset."]
    #[inline(always)]
    pub fn bor_intr_mode(&self) -> BorIntrModeR {
        BorIntrModeR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 000: 0us 001: 100us 010: 500us 011: 1000us 100: 2000us 101: 4000us 110: 8000us 111: 16000us"]
    #[inline(always)]
    pub fn bor_tdbc(&mut self) -> BorTdbcW<'_, RegBorCtrlSpec> {
        BorTdbcW::new(self, 0)
    }
    #[doc = "Bit 16 - BOR operation mode. 0: reset mode. When analog detect bor, hw will trigger a bor reset. 1: interrup mode. It will raise an interrup first. If this bit set to 0, then trigger a bor reset."]
    #[inline(always)]
    pub fn bor_intr_mode(&mut self) -> BorIntrModeW<'_, RegBorCtrlSpec> {
        BorIntrModeW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bor_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bor_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBorCtrlSpec;
impl crate::RegisterSpec for RegBorCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bor_ctrl::R`](R) reader structure"]
impl crate::Readable for RegBorCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bor_ctrl::W`](W) writer structure"]
impl crate::Writable for RegBorCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BOR_CTRL to value 0x01"]
impl crate::Resettable for RegBorCtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
