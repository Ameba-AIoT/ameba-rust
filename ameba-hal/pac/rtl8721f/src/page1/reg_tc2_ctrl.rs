#[doc = "Register `REG_TC2_CTRL` reader"]
pub type R = crate::R<RegTc2CtrlSpec>;
#[doc = "Register `REG_TC2_CTRL` writer"]
pub type W = crate::W<RegTc2CtrlSpec>;
#[doc = "Field `BIT_TC2DATA` reader - Timer/Counter 2 data register. It specifies the time-out duration."]
pub type BitTc2dataR = crate::FieldReader<u32>;
#[doc = "Field `BIT_TC2DATA` writer - Timer/Counter 2 data register. It specifies the time-out duration."]
pub type BitTc2dataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BIT_TC2EN` reader - Timer/Counter 2 enable"]
pub type BitTc2enR = crate::BitReader;
#[doc = "Field `BIT_TC2EN` writer - Timer/Counter 2 enable"]
pub type BitTc2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC2MODE` reader - Timer/Counter 2 mode 0=counter mode 1=timer mode"]
pub type BitTc2modeR = crate::BitReader;
#[doc = "Field `BIT_TC2MODE` writer - Timer/Counter 2 mode 0=counter mode 1=timer mode"]
pub type BitTc2modeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC2INT_EN` reader - When write \"1\" to this bit, Counter 2 starts counting down. This is a one-bit pulse."]
pub type BitTc2intEnR = crate::BitReader;
#[doc = "Field `BIT_TC2INT_EN` writer - When write \"1\" to this bit, Counter 2 starts counting down. This is a one-bit pulse."]
pub type BitTc2intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - Reserved"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:23 - Timer/Counter 2 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc2data(&self) -> BitTc2dataR {
        BitTc2dataR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Timer/Counter 2 enable"]
    #[inline(always)]
    pub fn bit_tc2en(&self) -> BitTc2enR {
        BitTc2enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer/Counter 2 mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc2mode(&self) -> BitTc2modeR {
        BitTc2modeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 2 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc2int_en(&self) -> BitTc2intEnR {
        BitTc2intEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Timer/Counter 2 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc2data(&mut self) -> BitTc2dataW<'_, RegTc2CtrlSpec> {
        BitTc2dataW::new(self, 0)
    }
    #[doc = "Bit 24 - Timer/Counter 2 enable"]
    #[inline(always)]
    pub fn bit_tc2en(&mut self) -> BitTc2enW<'_, RegTc2CtrlSpec> {
        BitTc2enW::new(self, 24)
    }
    #[doc = "Bit 25 - Timer/Counter 2 mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc2mode(&mut self) -> BitTc2modeW<'_, RegTc2CtrlSpec> {
        BitTc2modeW::new(self, 25)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 2 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc2int_en(&mut self) -> BitTc2intEnW<'_, RegTc2CtrlSpec> {
        BitTc2intEnW::new(self, 26)
    }
    #[doc = "Bits 27:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTc2CtrlSpec> {
        RsvdW::new(self, 27)
    }
}
#[doc = "Counter 2 data\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tc2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tc2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTc2CtrlSpec;
impl crate::RegisterSpec for RegTc2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tc2_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTc2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tc2_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTc2CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TC2_CTRL to value 0"]
impl crate::Resettable for RegTc2CtrlSpec {}
