#[doc = "Register `REG_TC1_CTRL` reader"]
pub type R = crate::R<RegTc1CtrlSpec>;
#[doc = "Register `REG_TC1_CTRL` writer"]
pub type W = crate::W<RegTc1CtrlSpec>;
#[doc = "Field `BIT_TC1DATA` reader - Timer/Counter 1 data register. It specifies the time-out duration."]
pub type BitTc1dataR = crate::FieldReader<u32>;
#[doc = "Field `BIT_TC1DATA` writer - Timer/Counter 1 data register. It specifies the time-out duration."]
pub type BitTc1dataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BIT_TC1EN` reader - Timer/Counter 1 enable"]
pub type BitTc1enR = crate::BitReader;
#[doc = "Field `BIT_TC1EN` writer - Timer/Counter 1 enable"]
pub type BitTc1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC1MODE` reader - Timer/Counter 1 mode 0=counter mode 1=timer mode"]
pub type BitTc1modeR = crate::BitReader;
#[doc = "Field `BIT_TC1MODE` writer - Timer/Counter 1 mode 0=counter mode 1=timer mode"]
pub type BitTc1modeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC1INT_EN` reader - When write \"1\" to this bit, Counter 1 starts counting down. This is a one-bit pulse."]
pub type BitTc1intEnR = crate::BitReader;
#[doc = "Field `BIT_TC1INT_EN` writer - When write \"1\" to this bit, Counter 1 starts counting down. This is a one-bit pulse."]
pub type BitTc1intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - Rsvd"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - Rsvd"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:23 - Timer/Counter 1 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc1data(&self) -> BitTc1dataR {
        BitTc1dataR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Timer/Counter 1 enable"]
    #[inline(always)]
    pub fn bit_tc1en(&self) -> BitTc1enR {
        BitTc1enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer/Counter 1 mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc1mode(&self) -> BitTc1modeR {
        BitTc1modeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 1 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc1int_en(&self) -> BitTc1intEnR {
        BitTc1intEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - Rsvd"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Timer/Counter 1 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc1data(&mut self) -> BitTc1dataW<'_, RegTc1CtrlSpec> {
        BitTc1dataW::new(self, 0)
    }
    #[doc = "Bit 24 - Timer/Counter 1 enable"]
    #[inline(always)]
    pub fn bit_tc1en(&mut self) -> BitTc1enW<'_, RegTc1CtrlSpec> {
        BitTc1enW::new(self, 24)
    }
    #[doc = "Bit 25 - Timer/Counter 1 mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc1mode(&mut self) -> BitTc1modeW<'_, RegTc1CtrlSpec> {
        BitTc1modeW::new(self, 25)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 1 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc1int_en(&mut self) -> BitTc1intEnW<'_, RegTc1CtrlSpec> {
        BitTc1intEnW::new(self, 26)
    }
    #[doc = "Bits 27:31 - Rsvd"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTc1CtrlSpec> {
        RsvdW::new(self, 27)
    }
}
#[doc = "Counter 1 data\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tc1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tc1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTc1CtrlSpec;
impl crate::RegisterSpec for RegTc1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tc1_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTc1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tc1_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTc1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TC1_CTRL to value 0"]
impl crate::Resettable for RegTc1CtrlSpec {}
