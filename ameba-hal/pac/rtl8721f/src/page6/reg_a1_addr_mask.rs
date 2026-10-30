#[doc = "Register `REG_A1_ADDR_MASK` reader"]
pub type R = crate::R<RegA1AddrMaskSpec>;
#[doc = "Register `REG_A1_ADDR_MASK` writer"]
pub type W = crate::W<RegA1AddrMaskSpec>;
#[doc = "Field `BIT_A1_ADDR_MASK` reader - mask rx A1 addr \\[39:8\\] for test"]
pub type BitA1AddrMaskR = crate::FieldReader<u32>;
#[doc = "Field `BIT_A1_ADDR_MASK` writer - mask rx A1 addr \\[39:8\\] for test"]
pub type BitA1AddrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - mask rx A1 addr \\[39:8\\] for test"]
    #[inline(always)]
    pub fn bit_a1_addr_mask(&self) -> BitA1AddrMaskR {
        BitA1AddrMaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - mask rx A1 addr \\[39:8\\] for test"]
    #[inline(always)]
    pub fn bit_a1_addr_mask(&mut self) -> BitA1AddrMaskW<'_, RegA1AddrMaskSpec> {
        BitA1AddrMaskW::new(self, 0)
    }
}
#[doc = "A1 Addr Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_a1_addr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_a1_addr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegA1AddrMaskSpec;
impl crate::RegisterSpec for RegA1AddrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_a1_addr_mask::R`](R) reader structure"]
impl crate::Readable for RegA1AddrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_a1_addr_mask::W`](W) writer structure"]
impl crate::Writable for RegA1AddrMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_A1_ADDR_MASK to value 0xffff_ffff"]
impl crate::Resettable for RegA1AddrMaskSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
