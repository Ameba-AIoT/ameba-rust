#[doc = "Register `REG_TC4_CTRL` reader"]
pub type R = crate::R<RegTc4CtrlSpec>;
#[doc = "Register `REG_TC4_CTRL` writer"]
pub type W = crate::W<RegTc4CtrlSpec>;
#[doc = "Field `BIT_TC4Data` reader - Timer/Counter 4 data register. It specifies the time-out duration."]
pub type BitTc4dataR = crate::FieldReader<u32>;
#[doc = "Field `BIT_TC4Data` writer - Timer/Counter 4 data register. It specifies the time-out duration."]
pub type BitTc4dataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BIT_TC4EN` reader - Timer/Counter 4 enable"]
pub type BitTc4enR = crate::BitReader;
#[doc = "Field `BIT_TC4EN` writer - Timer/Counter 4 enable"]
pub type BitTc4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC4MODE` reader - Timer/Counter 4mode 0=counter mode 1=timer mode"]
pub type BitTc4modeR = crate::BitReader;
#[doc = "Field `BIT_TC4MODE` writer - Timer/Counter 4mode 0=counter mode 1=timer mode"]
pub type BitTc4modeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC4INT_EN` reader - When write \"1\" to this bit, Counter 4 starts counting down. This is a one-bit pulse."]
pub type BitTc4intEnR = crate::BitReader;
#[doc = "Field `BIT_TC4INT_EN` writer - When write \"1\" to this bit, Counter 4 starts counting down. This is a one-bit pulse."]
pub type BitTc4intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - Reserved"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:23 - Timer/Counter 4 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc4data(&self) -> BitTc4dataR {
        BitTc4dataR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Timer/Counter 4 enable"]
    #[inline(always)]
    pub fn bit_tc4en(&self) -> BitTc4enR {
        BitTc4enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer/Counter 4mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc4mode(&self) -> BitTc4modeR {
        BitTc4modeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 4 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc4int_en(&self) -> BitTc4intEnR {
        BitTc4intEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Timer/Counter 4 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc4data(&mut self) -> BitTc4dataW<'_, RegTc4CtrlSpec> {
        BitTc4dataW::new(self, 0)
    }
    #[doc = "Bit 24 - Timer/Counter 4 enable"]
    #[inline(always)]
    pub fn bit_tc4en(&mut self) -> BitTc4enW<'_, RegTc4CtrlSpec> {
        BitTc4enW::new(self, 24)
    }
    #[doc = "Bit 25 - Timer/Counter 4mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc4mode(&mut self) -> BitTc4modeW<'_, RegTc4CtrlSpec> {
        BitTc4modeW::new(self, 25)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 4 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc4int_en(&mut self) -> BitTc4intEnW<'_, RegTc4CtrlSpec> {
        BitTc4intEnW::new(self, 26)
    }
    #[doc = "Bits 27:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTc4CtrlSpec> {
        RsvdW::new(self, 27)
    }
}
#[doc = "Counter 4 data\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tc4_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tc4_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTc4CtrlSpec;
impl crate::RegisterSpec for RegTc4CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tc4_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTc4CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tc4_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTc4CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TC4_CTRL to value 0"]
impl crate::Resettable for RegTc4CtrlSpec {}
