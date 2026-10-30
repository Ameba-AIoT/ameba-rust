#[doc = "Register `REG_BB_ACCESS_DATA` reader"]
pub type R = crate::R<RegBbAccessDataSpec>;
#[doc = "Register `REG_BB_ACCESS_DATA` writer"]
pub type W = crate::W<RegBbAccessDataSpec>;
#[doc = "Field `BIT_BB_DATA` reader - Baseband Access Write/Read Data (in DW unit)"]
pub type BitBbDataR = crate::FieldReader<u32>;
#[doc = "Field `BIT_BB_DATA` writer - Baseband Access Write/Read Data (in DW unit)"]
pub type BitBbDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Baseband Access Write/Read Data (in DW unit)"]
    #[inline(always)]
    pub fn bit_bb_data(&self) -> BitBbDataR {
        BitBbDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Baseband Access Write/Read Data (in DW unit)"]
    #[inline(always)]
    pub fn bit_bb_data(&mut self) -> BitBbDataW<'_, RegBbAccessDataSpec> {
        BitBbDataW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bb_access_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bb_access_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBbAccessDataSpec;
impl crate::RegisterSpec for RegBbAccessDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bb_access_data::R`](R) reader structure"]
impl crate::Readable for RegBbAccessDataSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bb_access_data::W`](W) writer structure"]
impl crate::Writable for RegBbAccessDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BB_ACCESS_DATA to value 0"]
impl crate::Resettable for RegBbAccessDataSpec {}
