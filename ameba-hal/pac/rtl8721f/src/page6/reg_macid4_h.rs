#[doc = "Register `REG_MACID4_H` reader"]
pub type R = crate::R<RegMacid4HSpec>;
#[doc = "Register `REG_MACID4_H` writer"]
pub type W = crate::W<RegMacid4HSpec>;
#[doc = "Field `BIT_MACID4_H_V1` reader - MAC Identification Number 4 Register"]
pub type BitMacid4HV1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_MACID4_H_V1` writer - MAC Identification Number 4 Register"]
pub type BitMacid4HV1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MAC Identification Number 4 Register"]
    #[inline(always)]
    pub fn bit_macid4_h_v1(&self) -> BitMacid4HV1R {
        BitMacid4HV1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Identification Number 4 Register"]
    #[inline(always)]
    pub fn bit_macid4_h_v1(&mut self) -> BitMacid4HV1W<'_, RegMacid4HSpec> {
        BitMacid4HV1W::new(self, 0)
    }
}
#[doc = "MAC ID4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid4_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid4_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacid4HSpec;
impl crate::RegisterSpec for RegMacid4HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid4_h::R`](R) reader structure"]
impl crate::Readable for RegMacid4HSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid4_h::W`](W) writer structure"]
impl crate::Writable for RegMacid4HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID4_H to value 0"]
impl crate::Resettable for RegMacid4HSpec {}
