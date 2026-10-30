#[doc = "Register `REG_REMAP0OR_CPU1` reader"]
pub type R = crate::R<RegRemap0orCpu1Spec>;
#[doc = "Register `REG_REMAP0OR_CPU1` writer"]
pub type W = crate::W<RegRemap0orCpu1Spec>;
#[doc = "Field `remap_en0_cpu1` reader - cpu1 mmu enable for entry0"]
pub type RemapEn0Cpu1R = crate::BitReader;
#[doc = "Field `remap_en0_cpu1` writer - cpu1 mmu enable for entry0"]
pub type RemapEn0Cpu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `remap_ofs0_cpu1` reader - cpu1 mmu offset addr when hit entry0. MMU unit in 1KB, so \\[9:0\\] is no used. HW always add this value to acture addr when hit , if need to sub, set this value to 2-comp minus value. Note:\\[31:20\\] not used for mmu only for sram and rom"]
pub type RemapOfs0Cpu1R = crate::FieldReader<u32>;
#[doc = "Field `remap_ofs0_cpu1` writer - cpu1 mmu offset addr when hit entry0. MMU unit in 1KB, so \\[9:0\\] is no used. HW always add this value to acture addr when hit , if need to sub, set this value to 2-comp minus value. Note:\\[31:20\\] not used for mmu only for sram and rom"]
pub type RemapOfs0Cpu1W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - cpu1 mmu enable for entry0"]
    #[inline(always)]
    pub fn remap_en0_cpu1(&self) -> RemapEn0Cpu1R {
        RemapEn0Cpu1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 10:31 - cpu1 mmu offset addr when hit entry0. MMU unit in 1KB, so \\[9:0\\] is no used. HW always add this value to acture addr when hit , if need to sub, set this value to 2-comp minus value. Note:\\[31:20\\] not used for mmu only for sram and rom"]
    #[inline(always)]
    pub fn remap_ofs0_cpu1(&self) -> RemapOfs0Cpu1R {
        RemapOfs0Cpu1R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - cpu1 mmu enable for entry0"]
    #[inline(always)]
    pub fn remap_en0_cpu1(&mut self) -> RemapEn0Cpu1W<'_, RegRemap0orCpu1Spec> {
        RemapEn0Cpu1W::new(self, 0)
    }
    #[doc = "Bits 10:31 - cpu1 mmu offset addr when hit entry0. MMU unit in 1KB, so \\[9:0\\] is no used. HW always add this value to acture addr when hit , if need to sub, set this value to 2-comp minus value. Note:\\[31:20\\] not used for mmu only for sram and rom"]
    #[inline(always)]
    pub fn remap_ofs0_cpu1(&mut self) -> RemapOfs0Cpu1W<'_, RegRemap0orCpu1Spec> {
        RemapOfs0Cpu1W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_remap0or_cpu1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_remap0or_cpu1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRemap0orCpu1Spec;
impl crate::RegisterSpec for RegRemap0orCpu1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_remap0or_cpu1::R`](R) reader structure"]
impl crate::Readable for RegRemap0orCpu1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_remap0or_cpu1::W`](W) writer structure"]
impl crate::Writable for RegRemap0orCpu1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_REMAP0OR_CPU1 to value 0"]
impl crate::Resettable for RegRemap0orCpu1Spec {}
