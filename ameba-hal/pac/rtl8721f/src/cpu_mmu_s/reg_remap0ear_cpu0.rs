#[doc = "Register `REG_REMAP0EAR_CPU0` reader"]
pub type R = crate::R<RegRemap0earCpu0Spec>;
#[doc = "Register `REG_REMAP0EAR_CPU0` writer"]
pub type W = crate::W<RegRemap0earCpu0Spec>;
#[doc = "Field `remap_ear0_cpu0` reader - cpu0 mmu end addr for entry0 Note:\\[31:20\\] not used for mmu only for sram and rom"]
pub type RemapEar0Cpu0R = crate::FieldReader<u32>;
#[doc = "Field `remap_ear0_cpu0` writer - cpu0 mmu end addr for entry0 Note:\\[31:20\\] not used for mmu only for sram and rom"]
pub type RemapEar0Cpu0W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - cpu0 mmu end addr for entry0 Note:\\[31:20\\] not used for mmu only for sram and rom"]
    #[inline(always)]
    pub fn remap_ear0_cpu0(&self) -> RemapEar0Cpu0R {
        RemapEar0Cpu0R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 10:31 - cpu0 mmu end addr for entry0 Note:\\[31:20\\] not used for mmu only for sram and rom"]
    #[inline(always)]
    pub fn remap_ear0_cpu0(&mut self) -> RemapEar0Cpu0W<'_, RegRemap0earCpu0Spec> {
        RemapEar0Cpu0W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_remap0ear_cpu0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_remap0ear_cpu0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRemap0earCpu0Spec;
impl crate::RegisterSpec for RegRemap0earCpu0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_remap0ear_cpu0::R`](R) reader structure"]
impl crate::Readable for RegRemap0earCpu0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_remap0ear_cpu0::W`](W) writer structure"]
impl crate::Writable for RegRemap0earCpu0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_REMAP0EAR_CPU0 to value 0"]
impl crate::Resettable for RegRemap0earCpu0Spec {}
