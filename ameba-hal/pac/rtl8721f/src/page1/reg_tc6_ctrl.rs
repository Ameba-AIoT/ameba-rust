#[doc = "Register `REG_TC6_CTRL` reader"]
pub type R = crate::R<RegTc6CtrlSpec>;
#[doc = "Register `REG_TC6_CTRL` writer"]
pub type W = crate::W<RegTc6CtrlSpec>;
#[doc = "Field `BIT_TC6DATA` reader - Timer/Counter 6 data register. It specifies the time-out duration."]
pub type BitTc6dataR = crate::FieldReader<u32>;
#[doc = "Field `BIT_TC6DATA` writer - Timer/Counter 6 data register. It specifies the time-out duration."]
pub type BitTc6dataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BIT_TC6EN` reader - Timer/Counter6 enable"]
pub type BitTc6enR = crate::BitReader;
#[doc = "Field `BIT_TC6EN` writer - Timer/Counter6 enable"]
pub type BitTc6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC6MODE` reader - Timer/Counter 6 mode 0=counter mode 1=timer mode"]
pub type BitTc6modeR = crate::BitReader;
#[doc = "Field `BIT_TC6MODE` writer - Timer/Counter 6 mode 0=counter mode 1=timer mode"]
pub type BitTc6modeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TC60INT_EN` reader - When write \"1\" to this bit, Counter 6 starts counting down. This is a one-bit pulse."]
pub type BitTc60intEnR = crate::BitReader;
#[doc = "Field `BIT_TC60INT_EN` writer - When write \"1\" to this bit, Counter 6 starts counting down. This is a one-bit pulse."]
pub type BitTc60intEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:23 - Timer/Counter 6 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc6data(&self) -> BitTc6dataR {
        BitTc6dataR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Timer/Counter6 enable"]
    #[inline(always)]
    pub fn bit_tc6en(&self) -> BitTc6enR {
        BitTc6enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer/Counter 6 mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc6mode(&self) -> BitTc6modeR {
        BitTc6modeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 6 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc60int_en(&self) -> BitTc60intEnR {
        BitTc60intEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Timer/Counter 6 data register. It specifies the time-out duration."]
    #[inline(always)]
    pub fn bit_tc6data(&mut self) -> BitTc6dataW<'_, RegTc6CtrlSpec> {
        BitTc6dataW::new(self, 0)
    }
    #[doc = "Bit 24 - Timer/Counter6 enable"]
    #[inline(always)]
    pub fn bit_tc6en(&mut self) -> BitTc6enW<'_, RegTc6CtrlSpec> {
        BitTc6enW::new(self, 24)
    }
    #[doc = "Bit 25 - Timer/Counter 6 mode 0=counter mode 1=timer mode"]
    #[inline(always)]
    pub fn bit_tc6mode(&mut self) -> BitTc6modeW<'_, RegTc6CtrlSpec> {
        BitTc6modeW::new(self, 25)
    }
    #[doc = "Bit 26 - When write \"1\" to this bit, Counter 6 starts counting down. This is a one-bit pulse."]
    #[inline(always)]
    pub fn bit_tc60int_en(&mut self) -> BitTc60intEnW<'_, RegTc6CtrlSpec> {
        BitTc60intEnW::new(self, 26)
    }
    #[doc = "Bits 27:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTc6CtrlSpec> {
        RsvdW::new(self, 27)
    }
}
#[doc = "Counter 6 data\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tc6_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tc6_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTc6CtrlSpec;
impl crate::RegisterSpec for RegTc6CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tc6_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTc6CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tc6_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTc6CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TC6_CTRL to value 0"]
impl crate::Resettable for RegTc6CtrlSpec {}
