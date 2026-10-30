#[doc = "Register `STATUSINT_L` reader"]
pub type R = crate::R<StatusintLSpec>;
#[doc = "Field `STATUSINT_L_Tfr` reader - OR of the contents of StatusTfr register."]
pub type StatusintLTfrR = crate::BitReader;
#[doc = "Field `STATUSINT_L_Block` reader - OR of the contents of StatusBlock register."]
pub type StatusintLBlockR = crate::BitReader;
#[doc = "Field `STATUSINT_L_Error` reader - OR of the contents of StatusErr register."]
pub type StatusintLErrorR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - OR of the contents of StatusTfr register."]
    #[inline(always)]
    pub fn statusint_l_tfr(&self) -> StatusintLTfrR {
        StatusintLTfrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OR of the contents of StatusBlock register."]
    #[inline(always)]
    pub fn statusint_l_block(&self) -> StatusintLBlockR {
        StatusintLBlockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - OR of the contents of StatusErr register."]
    #[inline(always)]
    pub fn statusint_l_error(&self) -> StatusintLErrorR {
        StatusintLErrorR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Combined Interrupt Sataus Register\n\nYou can [`read`](crate::Reg::read) this register and get [`statusint_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusintLSpec;
impl crate::RegisterSpec for StatusintLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusint_l::R`](R) reader structure"]
impl crate::Readable for StatusintLSpec {}
#[doc = "`reset()` method sets STATUSINT_L to value 0"]
impl crate::Resettable for StatusintLSpec {}
