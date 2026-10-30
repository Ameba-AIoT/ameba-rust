#[doc = "Register `REG_REMAP3EAR_CPU1` reader"]
pub type R = crate::R<RegRemap3earCpu1Spec>;
#[doc = "Register `REG_REMAP3EAR_CPU1` writer"]
pub type W = crate::W<RegRemap3earCpu1Spec>;
#[doc = "Field `remap_ear3_cpu1` reader - cpu1 mmu end addr for entry3 Note:\\[31:20\\] not used for mmu only for sram and rom"]
pub type RemapEar3Cpu1R = crate::FieldReader<u32>;
#[doc = "Field `remap_ear3_cpu1` writer - cpu1 mmu end addr for entry3 Note:\\[31:20\\] not used for mmu only for sram and rom"]
pub type RemapEar3Cpu1W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - cpu1 mmu end addr for entry3 Note:\\[31:20\\] not used for mmu only for sram and rom"]
    #[inline(always)]
    pub fn remap_ear3_cpu1(&self) -> RemapEar3Cpu1R {
        RemapEar3Cpu1R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 10:31 - cpu1 mmu end addr for entry3 Note:\\[31:20\\] not used for mmu only for sram and rom"]
    #[inline(always)]
    pub fn remap_ear3_cpu1(&mut self) -> RemapEar3Cpu1W<'_, RegRemap3earCpu1Spec> {
        RemapEar3Cpu1W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_remap3ear_cpu1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_remap3ear_cpu1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRemap3earCpu1Spec;
impl crate::RegisterSpec for RegRemap3earCpu1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_remap3ear_cpu1::R`](R) reader structure"]
impl crate::Readable for RegRemap3earCpu1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_remap3ear_cpu1::W`](W) writer structure"]
impl crate::Writable for RegRemap3earCpu1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_REMAP3EAR_CPU1 to value 0"]
impl crate::Resettable for RegRemap3earCpu1Spec {}
