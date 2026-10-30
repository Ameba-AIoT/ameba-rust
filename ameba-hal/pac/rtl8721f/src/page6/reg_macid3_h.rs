#[doc = "Register `REG_MACID3_H` reader"]
pub type R = crate::R<RegMacid3HSpec>;
#[doc = "Register `REG_MACID3_H` writer"]
pub type W = crate::W<RegMacid3HSpec>;
#[doc = "Field `BIT_MACID3_H_V1` reader - MAC Identification Number 3 Register"]
pub type BitMacid3HV1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_MACID3_H_V1` writer - MAC Identification Number 3 Register"]
pub type BitMacid3HV1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MAC Identification Number 3 Register"]
    #[inline(always)]
    pub fn bit_macid3_h_v1(&self) -> BitMacid3HV1R {
        BitMacid3HV1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Identification Number 3 Register"]
    #[inline(always)]
    pub fn bit_macid3_h_v1(&mut self) -> BitMacid3HV1W<'_, RegMacid3HSpec> {
        BitMacid3HV1W::new(self, 0)
    }
}
#[doc = "MAC ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid3_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid3_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacid3HSpec;
impl crate::RegisterSpec for RegMacid3HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid3_h::R`](R) reader structure"]
impl crate::Readable for RegMacid3HSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid3_h::W`](W) writer structure"]
impl crate::Writable for RegMacid3HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID3_H to value 0"]
impl crate::Resettable for RegMacid3HSpec {}
