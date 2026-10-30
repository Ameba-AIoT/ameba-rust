#[doc = "Register `REG_ZCD_ITUNE` reader"]
pub type R = crate::R<RegZcdItuneSpec>;
#[doc = "Register `REG_ZCD_ITUNE` writer"]
pub type W = crate::W<RegZcdItuneSpec>;
#[doc = "Field `zcd_itune_out_l` reader - SWR output ZCD itune code"]
pub type ZcdItuneOutLR = crate::FieldReader;
#[doc = "Field `sw_zcd_itune` reader - Software config zcd itune code"]
pub type SwZcdItuneR = crate::FieldReader;
#[doc = "Field `sw_zcd_itune` writer - Software config zcd itune code"]
pub type SwZcdItuneW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `zcd_itune_sel` reader - Select itune code source 1: from software config 0: auto by regu self"]
pub type ZcdItuneSelR = crate::BitReader;
#[doc = "Field `zcd_itune_sel` writer - Select itune code source 1: from software config 0: auto by regu self"]
pub type ZcdItuneSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - SWR output ZCD itune code"]
    #[inline(always)]
    pub fn zcd_itune_out_l(&self) -> ZcdItuneOutLR {
        ZcdItuneOutLR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Software config zcd itune code"]
    #[inline(always)]
    pub fn sw_zcd_itune(&self) -> SwZcdItuneR {
        SwZcdItuneR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Select itune code source 1: from software config 0: auto by regu self"]
    #[inline(always)]
    pub fn zcd_itune_sel(&self) -> ZcdItuneSelR {
        ZcdItuneSelR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Software config zcd itune code"]
    #[inline(always)]
    pub fn sw_zcd_itune(&mut self) -> SwZcdItuneW<'_, RegZcdItuneSpec> {
        SwZcdItuneW::new(self, 4)
    }
    #[doc = "Bit 8 - Select itune code source 1: from software config 0: auto by regu self"]
    #[inline(always)]
    pub fn zcd_itune_sel(&mut self) -> ZcdItuneSelW<'_, RegZcdItuneSpec> {
        ZcdItuneSelW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_zcd_itune::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_zcd_itune::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegZcdItuneSpec;
impl crate::RegisterSpec for RegZcdItuneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_zcd_itune::R`](R) reader structure"]
impl crate::Readable for RegZcdItuneSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_zcd_itune::W`](W) writer structure"]
impl crate::Writable for RegZcdItuneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ZCD_ITUNE to value 0"]
impl crate::Resettable for RegZcdItuneSpec {}
