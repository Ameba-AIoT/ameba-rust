#[doc = "Register `REG_TC5_CTRL` reader"]
pub type R = crate::R<RegTc5CtrlSpec>;
#[doc = "Register `REG_TC5_CTRL` writer"]
pub type W = crate::W<RegTc5CtrlSpec>;
#[doc = "Field `BIT_TC5DATA` reader - Timer/Counter 5 data register. It specifies the time-out duration."]
pub type BitTc5dataR = crate::FieldReader<u32>;
#[doc = "Field `BIT_TC5DATA` writer - Timer/Counter 5 data register. It specifies the time-out duration."]
pub type BitTc5dataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BIT_TC5EN` reader - Timer/Counter5 enable"]
pub type BitTc5enR = crate::BitReader;
#[doc = "Field `BIT_TC5EN` writer - Timer/Counter5 enable"]
pub type BitTc5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC5MODE` reader - Timer/Counter 5 mode 0=counter mode 1=timer mode"]
pub type BitTc5modeR = crate::BitReader;
#[doc = "Field `BIT_TC5MODE` writer - Timer/Counter 5 mode 0=counter mode 1=timer mode"]
pub type BitTc5modeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC50INT_EN` reader - When write \"1\" to this bit, Counter 5 starts counting down. This is a one-bit pulse."]
pub type BitTc50intEnR = crate::BitReader;
#[doc = "Field `BIT_TC50INT_EN` writer - When write \"1\" to this bit, Counter 5 starts counting down. This is a one-bit pulse."]
pub type BitTc50intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:23 - Timer/Counter 5 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc5data(&self) -> BitTc5dataR {
        BitTc5dataR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Timer/Counter5 enable"]
    #[inline(always)]
    pub fn bit_tc5en(&self) -> BitTc5enR {
        BitTc5enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer/Counter 5 mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc5mode(&self) -> BitTc5modeR {
        BitTc5modeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 5 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc50int_en(&self) -> BitTc50intEnR {
        BitTc50intEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Timer/Counter 5 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc5data(&mut self) -> BitTc5dataW<'_, RegTc5CtrlSpec> {
        BitTc5dataW::new(self, 0)
    }
    #[doc = "Bit 24 - Timer/Counter5 enable"]
    #[inline(always)]
    pub fn bit_tc5en(&mut self) -> BitTc5enW<'_, RegTc5CtrlSpec> {
        BitTc5enW::new(self, 24)
    }
    #[doc = "Bit 25 - Timer/Counter 5 mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc5mode(&mut self) -> BitTc5modeW<'_, RegTc5CtrlSpec> {
        BitTc5modeW::new(self, 25)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 5 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc50int_en(&mut self) -> BitTc50intEnW<'_, RegTc5CtrlSpec> {
        BitTc50intEnW::new(self, 26)
    }
    #[doc = "Bits 27:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTc5CtrlSpec> {
        RsvdW::new(self, 27)
    }
}
#[doc = "Counter 5 data\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tc5_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tc5_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTc5CtrlSpec;
impl crate::RegisterSpec for RegTc5CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tc5_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTc5CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tc5_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTc5CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TC5_CTRL to value 0"]
impl crate::Resettable for RegTc5CtrlSpec {}
