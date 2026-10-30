#[doc = "Register `REG_PG_SIZE` reader"]
pub type R = crate::R<RegPgSizeSpec>;
#[doc = "Register `REG_PG_SIZE` writer"]
pub type W = crate::W<RegPgSizeSpec>;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::FieldReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 9:15 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 9:15 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegPgSizeSpec> {
        Rsvd2W::new(self, 9)
    }
    #[doc = "Bits 24:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegPgSizeSpec> {
        RsvdW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pg_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pg_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPgSizeSpec;
impl crate::RegisterSpec for RegPgSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pg_size::R`](R) reader structure"]
impl crate::Readable for RegPgSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pg_size::W`](W) writer structure"]
impl crate::Writable for RegPgSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PG_SIZE to value 0"]
impl crate::Resettable for RegPgSizeSpec {}
