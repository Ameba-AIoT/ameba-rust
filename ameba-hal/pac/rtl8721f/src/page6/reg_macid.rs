#[doc = "Register `REG_MACID` reader"]
pub type R = crate::R<RegMacidSpec>;
#[doc = "Register `REG_MACID` writer"]
pub type W = crate::W<RegMacidSpec>;
#[doc = "Field `BIT_MACID_V1` reader - MAC Identification Number Register BIT\\[31:0\\]"]
pub type BitMacidV1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID_V1` writer - MAC Identification Number Register BIT\\[31:0\\]"]
pub type BitMacidV1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Identification Number Register BIT\\[31:0\\]"]
    #[inline(always)]
    pub fn bit_macid_v1(&self) -> BitMacidV1R {
        BitMacidV1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Identification Number Register BIT\\[31:0\\]"]
    #[inline(always)]
    pub fn bit_macid_v1(&mut self) -> BitMacidV1W<'_, RegMacidSpec> {
        BitMacidV1W::new(self, 0)
    }
}
#[doc = "MAC ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidSpec;
impl crate::RegisterSpec for RegMacidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid::R`](R) reader structure"]
impl crate::Readable for RegMacidSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid::W`](W) writer structure"]
impl crate::Writable for RegMacidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID to value 0"]
impl crate::Resettable for RegMacidSpec {}
