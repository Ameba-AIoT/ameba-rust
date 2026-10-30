#[doc = "Register `REG_SW_PATCH` reader"]
pub type R = crate::R<RegSwPatchSpec>;
#[doc = "Register `REG_SW_PATCH` writer"]
pub type W = crate::W<RegSwPatchSpec>;
#[doc = "Field `sw_patch_saddr` reader - SW patch code start addr. OTP physical addr 0x500-0x6FF will autoload to this addr ."]
pub type SwPatchSaddrR = crate::FieldReader<u32>;
#[doc = "Field `sw_patch_saddr` writer - SW patch code start addr. OTP physical addr 0x500-0x6FF will autoload to this addr ."]
pub type SwPatchSaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SW patch code start addr. OTP physical addr 0x500-0x6FF will autoload to this addr ."]
    #[inline(always)]
    pub fn sw_patch_saddr(&self) -> SwPatchSaddrR {
        SwPatchSaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SW patch code start addr. OTP physical addr 0x500-0x6FF will autoload to this addr ."]
    #[inline(always)]
    pub fn sw_patch_saddr(&mut self) -> SwPatchSaddrW<'_, RegSwPatchSpec> {
        SwPatchSaddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sw_patch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sw_patch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwPatchSpec;
impl crate::RegisterSpec for RegSwPatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sw_patch::R`](R) reader structure"]
impl crate::Readable for RegSwPatchSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sw_patch::W`](W) writer structure"]
impl crate::Writable for RegSwPatchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SW_PATCH to value 0"]
impl crate::Resettable for RegSwPatchSpec {}
