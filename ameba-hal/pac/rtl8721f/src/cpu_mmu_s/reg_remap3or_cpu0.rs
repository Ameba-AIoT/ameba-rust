#[doc = "Register `REG_REMAP3OR_CPU0` reader"]
pub type R = crate::R<RegRemap3orCpu0Spec>;
#[doc = "Register `REG_REMAP3OR_CPU0` writer"]
pub type W = crate::W<RegRemap3orCpu0Spec>;
#[doc = "Field `remap_en3_cpu0` reader - cpu0 mmu enable for entry3"]
pub type RemapEn3Cpu0R = crate::BitReader;
#[doc = "Field `remap_en3_cpu0` writer - cpu0 mmu enable for entry3"]
pub type RemapEn3Cpu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `remap_ofs3_cpu0` reader - cpu0 mmu offset addr when hit entry3. MMU unit in 1KB, so \\[9:0\\] is no used. HW always add this value to acture addr when hit , if need to sub, set this value to 2-comp minus value. Note:\\[31:20\\] not used for mmu only for sram and rom"]
pub type RemapOfs3Cpu0R = crate::FieldReader<u32>;
#[doc = "Field `remap_ofs3_cpu0` writer - cpu0 mmu offset addr when hit entry3. MMU unit in 1KB, so \\[9:0\\] is no used. HW always add this value to acture addr when hit , if need to sub, set this value to 2-comp minus value. Note:\\[31:20\\] not used for mmu only for sram and rom"]
pub type RemapOfs3Cpu0W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - cpu0 mmu enable for entry3"]
    #[inline(always)]
    pub fn remap_en3_cpu0(&self) -> RemapEn3Cpu0R {
        RemapEn3Cpu0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 10:31 - cpu0 mmu offset addr when hit entry3. MMU unit in 1KB, so \\[9:0\\] is no used. HW always add this value to acture addr when hit , if need to sub, set this value to 2-comp minus value. Note:\\[31:20\\] not used for mmu only for sram and rom"]
    #[inline(always)]
    pub fn remap_ofs3_cpu0(&self) -> RemapOfs3Cpu0R {
        RemapOfs3Cpu0R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - cpu0 mmu enable for entry3"]
    #[inline(always)]
    pub fn remap_en3_cpu0(&mut self) -> RemapEn3Cpu0W<'_, RegRemap3orCpu0Spec> {
        RemapEn3Cpu0W::new(self, 0)
    }
    #[doc = "Bits 10:31 - cpu0 mmu offset addr when hit entry3. MMU unit in 1KB, so \\[9:0\\] is no used. HW always add this value to acture addr when hit , if need to sub, set this value to 2-comp minus value. Note:\\[31:20\\] not used for mmu only for sram and rom"]
    #[inline(always)]
    pub fn remap_ofs3_cpu0(&mut self) -> RemapOfs3Cpu0W<'_, RegRemap3orCpu0Spec> {
        RemapOfs3Cpu0W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_remap3or_cpu0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_remap3or_cpu0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRemap3orCpu0Spec;
impl crate::RegisterSpec for RegRemap3orCpu0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_remap3or_cpu0::R`](R) reader structure"]
impl crate::Readable for RegRemap3orCpu0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_remap3or_cpu0::W`](W) writer structure"]
impl crate::Writable for RegRemap3orCpu0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_REMAP3OR_CPU0 to value 0"]
impl crate::Resettable for RegRemap3orCpu0Spec {}
