#[doc = "Register `REG_REMAP2BAR_CPU0` reader"]
pub type R = crate::R<RegRemap2barCpu0Spec>;
#[doc = "Register `REG_REMAP2BAR_CPU0` writer"]
pub type W = crate::W<RegRemap2barCpu0Spec>;
#[doc = "Field `remap_bar2_cpu0` reader - cpu0 mmu base addr for entry2 Note:\\[31:20\\] not used for mmu only for sram and rom"]
pub type RemapBar2Cpu0R = crate::FieldReader<u32>;
#[doc = "Field `remap_bar2_cpu0` writer - cpu0 mmu base addr for entry2 Note:\\[31:20\\] not used for mmu only for sram and rom"]
pub type RemapBar2Cpu0W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - cpu0 mmu base addr for entry2 Note:\\[31:20\\] not used for mmu only for sram and rom"]
    #[inline(always)]
    pub fn remap_bar2_cpu0(&self) -> RemapBar2Cpu0R {
        RemapBar2Cpu0R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 10:31 - cpu0 mmu base addr for entry2 Note:\\[31:20\\] not used for mmu only for sram and rom"]
    #[inline(always)]
    pub fn remap_bar2_cpu0(&mut self) -> RemapBar2Cpu0W<'_, RegRemap2barCpu0Spec> {
        RemapBar2Cpu0W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_remap2bar_cpu0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_remap2bar_cpu0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRemap2barCpu0Spec;
impl crate::RegisterSpec for RegRemap2barCpu0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_remap2bar_cpu0::R`](R) reader structure"]
impl crate::Readable for RegRemap2barCpu0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_remap2bar_cpu0::W`](W) writer structure"]
impl crate::Writable for RegRemap2barCpu0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_REMAP2BAR_CPU0 to value 0"]
impl crate::Resettable for RegRemap2barCpu0Spec {}
