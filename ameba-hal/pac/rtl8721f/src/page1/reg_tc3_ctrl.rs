#[doc = "Register `REG_TC3_CTRL` reader"]
pub type R = crate::R<RegTc3CtrlSpec>;
#[doc = "Register `REG_TC3_CTRL` writer"]
pub type W = crate::W<RegTc3CtrlSpec>;
#[doc = "Field `BIT_TC3Data` reader - Timer/Counter 3 data register. It specifies the time-out duration."]
pub type BitTc3dataR = crate::FieldReader<u32>;
#[doc = "Field `BIT_TC3Data` writer - Timer/Counter 3 data register. It specifies the time-out duration."]
pub type BitTc3dataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BIT_TC3EN` reader - Timer/Counter 3 enable"]
pub type BitTc3enR = crate::BitReader;
#[doc = "Field `BIT_TC3EN` writer - Timer/Counter 3 enable"]
pub type BitTc3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC3MODE` reader - Timer/Counter 3 mode 0=counter mode 1=timer mode"]
pub type BitTc3modeR = crate::BitReader;
#[doc = "Field `BIT_TC3MODE` writer - Timer/Counter 3 mode 0=counter mode 1=timer mode"]
pub type BitTc3modeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC3INT_EN` reader - When write \"1\" to this bit, Counter 3 starts counting down. This is a one-bit pulse."]
pub type BitTc3intEnR = crate::BitReader;
#[doc = "Field `BIT_TC3INT_EN` writer - When write \"1\" to this bit, Counter 3 starts counting down. This is a one-bit pulse."]
pub type BitTc3intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - Reserved"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:23 - Timer/Counter 3 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc3data(&self) -> BitTc3dataR {
        BitTc3dataR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Timer/Counter 3 enable"]
    #[inline(always)]
    pub fn bit_tc3en(&self) -> BitTc3enR {
        BitTc3enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer/Counter 3 mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc3mode(&self) -> BitTc3modeR {
        BitTc3modeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 3 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc3int_en(&self) -> BitTc3intEnR {
        BitTc3intEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Timer/Counter 3 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc3data(&mut self) -> BitTc3dataW<'_, RegTc3CtrlSpec> {
        BitTc3dataW::new(self, 0)
    }
    #[doc = "Bit 24 - Timer/Counter 3 enable"]
    #[inline(always)]
    pub fn bit_tc3en(&mut self) -> BitTc3enW<'_, RegTc3CtrlSpec> {
        BitTc3enW::new(self, 24)
    }
    #[doc = "Bit 25 - Timer/Counter 3 mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc3mode(&mut self) -> BitTc3modeW<'_, RegTc3CtrlSpec> {
        BitTc3modeW::new(self, 25)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 3 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc3int_en(&mut self) -> BitTc3intEnW<'_, RegTc3CtrlSpec> {
        BitTc3intEnW::new(self, 26)
    }
    #[doc = "Bits 27:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTc3CtrlSpec> {
        RsvdW::new(self, 27)
    }
}
#[doc = "Counter 3 data\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tc3_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tc3_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTc3CtrlSpec;
impl crate::RegisterSpec for RegTc3CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tc3_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTc3CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tc3_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTc3CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TC3_CTRL to value 0"]
impl crate::Resettable for RegTc3CtrlSpec {}
