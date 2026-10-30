#[doc = "Register `REG_TC0_CTRL` reader"]
pub type R = crate::R<RegTc0CtrlSpec>;
#[doc = "Register `REG_TC0_CTRL` writer"]
pub type W = crate::W<RegTc0CtrlSpec>;
#[doc = "Field `BIT_TC0DATA` reader - Timer/Counter 0 data register. It specifies the time-out duration."]
pub type BitTc0dataR = crate::FieldReader<u32>;
#[doc = "Field `BIT_TC0DATA` writer - Timer/Counter 0 data register. It specifies the time-out duration."]
pub type BitTc0dataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BIT_TC0EN` reader - Timer/Counter 0 enable"]
pub type BitTc0enR = crate::BitReader;
#[doc = "Field `BIT_TC0EN` writer - Timer/Counter 0 enable"]
pub type BitTc0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC0MODE` reader - Timer/Counter 0 mode 0=counter mode 1=timer mode"]
pub type BitTc0modeR = crate::BitReader;
#[doc = "Field `BIT_TC0MODE` writer - Timer/Counter 0 mode 0=counter mode 1=timer mode"]
pub type BitTc0modeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC0INT_EN` reader - When write \"1\" to this bit, Counter 0 starts counting down. This is a one-bit pulse."]
pub type BitTc0intEnR = crate::BitReader;
#[doc = "Field `BIT_TC0INT_EN` writer - When write \"1\" to this bit, Counter 0 starts counting down. This is a one-bit pulse."]
pub type BitTc0intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - Rsvd"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - Rsvd"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:23 - Timer/Counter 0 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc0data(&self) -> BitTc0dataR {
        BitTc0dataR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Timer/Counter 0 enable"]
    #[inline(always)]
    pub fn bit_tc0en(&self) -> BitTc0enR {
        BitTc0enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer/Counter 0 mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc0mode(&self) -> BitTc0modeR {
        BitTc0modeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 0 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc0int_en(&self) -> BitTc0intEnR {
        BitTc0intEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - Rsvd"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Timer/Counter 0 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc0data(&mut self) -> BitTc0dataW<'_, RegTc0CtrlSpec> {
        BitTc0dataW::new(self, 0)
    }
    #[doc = "Bit 24 - Timer/Counter 0 enable"]
    #[inline(always)]
    pub fn bit_tc0en(&mut self) -> BitTc0enW<'_, RegTc0CtrlSpec> {
        BitTc0enW::new(self, 24)
    }
    #[doc = "Bit 25 - Timer/Counter 0 mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc0mode(&mut self) -> BitTc0modeW<'_, RegTc0CtrlSpec> {
        BitTc0modeW::new(self, 25)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 0 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc0int_en(&mut self) -> BitTc0intEnW<'_, RegTc0CtrlSpec> {
        BitTc0intEnW::new(self, 26)
    }
    #[doc = "Bits 27:31 - Rsvd"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTc0CtrlSpec> {
        RsvdW::new(self, 27)
    }
}
#[doc = "Counter 0 data\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tc0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tc0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTc0CtrlSpec;
impl crate::RegisterSpec for RegTc0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tc0_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTc0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tc0_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTc0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TC0_CTRL to value 0"]
impl crate::Resettable for RegTc0CtrlSpec {}
