#[doc = "Register `Probe_selexct` reader"]
pub type R = crate::R<ProbeSelexctSpec>;
#[doc = "Register `Probe_selexct` writer"]
pub type W = crate::W<ProbeSelexctSpec>;
#[doc = "Field `Probe_signal_select` reader - For debug port signals use Value = 4: Tx/Rx related signals"]
pub type ProbeSignalSelectR = crate::FieldReader;
#[doc = "Field `Probe_signal_select` writer - For debug port signals use Value = 4: Tx/Rx related signals"]
pub type ProbeSignalSelectW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - For debug port signals use Value = 4: Tx/Rx related signals"]
    #[inline(always)]
    pub fn probe_signal_select(&self) -> ProbeSignalSelectR {
        ProbeSignalSelectR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - For debug port signals use Value = 4: Tx/Rx related signals"]
    #[inline(always)]
    pub fn probe_signal_select(&mut self) -> ProbeSignalSelectW<'_, ProbeSelexctSpec> {
        ProbeSignalSelectW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`probe_selexct::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`probe_selexct::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProbeSelexctSpec;
impl crate::RegisterSpec for ProbeSelexctSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probe_selexct::R`](R) reader structure"]
impl crate::Readable for ProbeSelexctSpec {}
#[doc = "`write(|w| ..)` method takes [`probe_selexct::W`](W) writer structure"]
impl crate::Writable for ProbeSelexctSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Probe_selexct to value 0"]
impl crate::Resettable for ProbeSelexctSpec {}
