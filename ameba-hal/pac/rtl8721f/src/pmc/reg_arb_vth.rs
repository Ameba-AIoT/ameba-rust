#[doc = "Register `REG_ARB_VTH` reader"]
pub type R = crate::R<RegArbVthSpec>;
#[doc = "Register `REG_ARB_VTH` writer"]
pub type W = crate::W<RegArbVthSpec>;
#[doc = "Field `iagt_vthd_regu_sts` reader - This field used for regu sts arb control."]
pub type IagtVthdReguStsR = crate::FieldReader<u16>;
#[doc = "Field `iagt_vthd_regu_sts` writer - This field used for regu sts arb control."]
pub type IagtVthdReguStsW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `iagt_vthd_mldo` reader - This field used for mldo arb control."]
pub type IagtVthdMldoR = crate::FieldReader;
#[doc = "Field `iagt_vthd_mldo` writer - This field used for mldo arb control."]
pub type IagtVthdMldoW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `iagt_vthd_xmd` reader - This field used for xmd arb control."]
pub type IagtVthdXmdR = crate::FieldReader<u16>;
#[doc = "Field `iagt_vthd_xmd` writer - This field used for xmd arb control."]
pub type IagtVthdXmdW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:8 - This field used for regu sts arb control."]
    #[inline(always)]
    pub fn iagt_vthd_regu_sts(&self) -> IagtVthdReguStsR {
        IagtVthdReguStsR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:14 - This field used for mldo arb control."]
    #[inline(always)]
    pub fn iagt_vthd_mldo(&self) -> IagtVthdMldoR {
        IagtVthdMldoR::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bits 16:30 - This field used for xmd arb control."]
    #[inline(always)]
    pub fn iagt_vthd_xmd(&self) -> IagtVthdXmdR {
        IagtVthdXmdR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - This field used for regu sts arb control."]
    #[inline(always)]
    pub fn iagt_vthd_regu_sts(&mut self) -> IagtVthdReguStsW<'_, RegArbVthSpec> {
        IagtVthdReguStsW::new(self, 0)
    }
    #[doc = "Bits 9:14 - This field used for mldo arb control."]
    #[inline(always)]
    pub fn iagt_vthd_mldo(&mut self) -> IagtVthdMldoW<'_, RegArbVthSpec> {
        IagtVthdMldoW::new(self, 9)
    }
    #[doc = "Bits 16:30 - This field used for xmd arb control."]
    #[inline(always)]
    pub fn iagt_vthd_xmd(&mut self) -> IagtVthdXmdW<'_, RegArbVthSpec> {
        IagtVthdXmdW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_arb_vth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_arb_vth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegArbVthSpec;
impl crate::RegisterSpec for RegArbVthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_arb_vth::R`](R) reader structure"]
impl crate::Readable for RegArbVthSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_arb_vth::W`](W) writer structure"]
impl crate::Writable for RegArbVthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ARB_VTH to value 0"]
impl crate::Resettable for RegArbVthSpec {}
