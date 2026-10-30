#[doc = "Register `PKE_Status` reader"]
pub type R = crate::R<PkeStatusSpec>;
#[doc = "Register `PKE_Status` writer"]
pub type W = crate::W<PkeStatusSpec>;
#[doc = "Field `PKE_Status_FINISH` reader - PKE operation finish"]
pub type PkeStatusFinishR = crate::BitReader;
#[doc = "Field `PKE_Status_FINISH` writer - PKE operation finish"]
pub type PkeStatusFinishW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_STATUS` reader - 7'h1: prime check error(for modular number) 7'h2: R^2 mod N failed 7'h4: ECC odd point (Y = 0) (Montgomery curve will not check the Y value) 7'h5: RSA not supported 7'h6: check version error 7'h7: no such function 7'h8: ECC Z = 0 7'h10: Modular Inverse failed 7'h20: N < input message for RSA 7'h40: No valid exponent"]
pub type PkeStatusR = crate::FieldReader;
#[doc = "Field `PKE_STATUS` writer - 7'h1: prime check error(for modular number) 7'h2: R^2 mod N failed 7'h4: ECC odd point (Y = 0) (Montgomery curve will not check the Y value) 7'h5: RSA not supported 7'h6: check version error 7'h7: no such function 7'h8: ECC Z = 0 7'h10: Modular Inverse failed 7'h20: N < input message for RSA 7'h40: No valid exponent"]
pub type PkeStatusW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKE_Status_IDLE` reader - PKE idle"]
pub type PkeStatusIdleR = crate::BitReader;
#[doc = "Field `PKE_Status_HASH_NEED` reader - SIMPLIFY_ECDSA/EdDSA needs hash value"]
pub type PkeStatusHashNeedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PKE operation finish"]
    #[inline(always)]
    pub fn pke_status_finish(&self) -> PkeStatusFinishR {
        PkeStatusFinishR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7'h1: prime check error(for modular number) 7'h2: R^2 mod N failed 7'h4: ECC odd point (Y = 0) (Montgomery curve will not check the Y value) 7'h5: RSA not supported 7'h6: check version error 7'h7: no such function 7'h8: ECC Z = 0 7'h10: Modular Inverse failed 7'h20: N < input message for RSA 7'h40: No valid exponent"]
    #[inline(always)]
    pub fn pke_status(&self) -> PkeStatusR {
        PkeStatusR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - PKE idle"]
    #[inline(always)]
    pub fn pke_status_idle(&self) -> PkeStatusIdleR {
        PkeStatusIdleR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SIMPLIFY_ECDSA/EdDSA needs hash value"]
    #[inline(always)]
    pub fn pke_status_hash_need(&self) -> PkeStatusHashNeedR {
        PkeStatusHashNeedR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PKE operation finish"]
    #[inline(always)]
    pub fn pke_status_finish(&mut self) -> PkeStatusFinishW<'_, PkeStatusSpec> {
        PkeStatusFinishW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7'h1: prime check error(for modular number) 7'h2: R^2 mod N failed 7'h4: ECC odd point (Y = 0) (Montgomery curve will not check the Y value) 7'h5: RSA not supported 7'h6: check version error 7'h7: no such function 7'h8: ECC Z = 0 7'h10: Modular Inverse failed 7'h20: N < input message for RSA 7'h40: No valid exponent"]
    #[inline(always)]
    pub fn pke_status(&mut self) -> PkeStatusW<'_, PkeStatusSpec> {
        PkeStatusW::new(self, 1)
    }
}
#[doc = "PKE Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pke_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pke_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkeStatusSpec;
impl crate::RegisterSpec for PkeStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pke_status::R`](R) reader structure"]
impl crate::Readable for PkeStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pke_status::W`](W) writer structure"]
impl crate::Writable for PkeStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKE_Status to value 0x0100"]
impl crate::Resettable for PkeStatusSpec {
    const RESET_VALUE: u32 = 0x0100;
}
