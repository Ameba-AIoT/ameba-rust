#[doc = "Register `REG_ANTCD_INFO` reader"]
pub type R = crate::R<RegAntcdInfoSpec>;
#[doc = "Register `REG_ANTCD_INFO` writer"]
pub type W = crate::W<RegAntcdInfoSpec>;
#[doc = "Field `BIT_WMAC_RESP_ANTA` reader - used for antenna select a value BIT_WMAC_RESP_ANTCD\\[3:5\\], used for antsel_d value;"]
pub type BitWmacRespAntaR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_RESP_ANTA` writer - used for antenna select a value BIT_WMAC_RESP_ANTCD\\[3:5\\], used for antsel_d value;"]
pub type BitWmacRespAntaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_WMAC_RESP_ANTB` reader - used for antenna select b value BIT_WMAC_RESP_ANTCD\\[3:4\\], used for antsel_d value;"]
pub type BitWmacRespAntbR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_RESP_ANTB` writer - used for antenna select b value BIT_WMAC_RESP_ANTCD\\[3:4\\], used for antsel_d value;"]
pub type BitWmacRespAntbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_WMAC_RESP_ANTC` reader - used for antenna select c value BIT_WMAC_RESP_ANTCD\\[3:3\\], used for antsel_d value;"]
pub type BitWmacRespAntcR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_RESP_ANTC` writer - used for antenna select c value BIT_WMAC_RESP_ANTCD\\[3:3\\], used for antsel_d value;"]
pub type BitWmacRespAntcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_WMAC_RESP_ANTD` reader - used for antenna select d value BIT_WMAC_RESP_ANTCD\\[3:2\\], used for antsel_d value;"]
pub type BitWmacRespAntdR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_RESP_ANTD` writer - used for antenna select d value BIT_WMAC_RESP_ANTCD\\[3:2\\], used for antsel_d value;"]
pub type BitWmacRespAntdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - used for antenna select a value BIT_WMAC_RESP_ANTCD\\[3:5\\], used for antsel_d value;"]
    #[inline(always)]
    pub fn bit_wmac_resp_anta(&self) -> BitWmacRespAntaR {
        BitWmacRespAntaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - used for antenna select b value BIT_WMAC_RESP_ANTCD\\[3:4\\], used for antsel_d value;"]
    #[inline(always)]
    pub fn bit_wmac_resp_antb(&self) -> BitWmacRespAntbR {
        BitWmacRespAntbR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - used for antenna select c value BIT_WMAC_RESP_ANTCD\\[3:3\\], used for antsel_d value;"]
    #[inline(always)]
    pub fn bit_wmac_resp_antc(&self) -> BitWmacRespAntcR {
        BitWmacRespAntcR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - used for antenna select d value BIT_WMAC_RESP_ANTCD\\[3:2\\], used for antsel_d value;"]
    #[inline(always)]
    pub fn bit_wmac_resp_antd(&self) -> BitWmacRespAntdR {
        BitWmacRespAntdR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - used for antenna select a value BIT_WMAC_RESP_ANTCD\\[3:5\\], used for antsel_d value;"]
    #[inline(always)]
    pub fn bit_wmac_resp_anta(&mut self) -> BitWmacRespAntaW<'_, RegAntcdInfoSpec> {
        BitWmacRespAntaW::new(self, 0)
    }
    #[doc = "Bits 4:7 - used for antenna select b value BIT_WMAC_RESP_ANTCD\\[3:4\\], used for antsel_d value;"]
    #[inline(always)]
    pub fn bit_wmac_resp_antb(&mut self) -> BitWmacRespAntbW<'_, RegAntcdInfoSpec> {
        BitWmacRespAntbW::new(self, 4)
    }
    #[doc = "Bits 8:11 - used for antenna select c value BIT_WMAC_RESP_ANTCD\\[3:3\\], used for antsel_d value;"]
    #[inline(always)]
    pub fn bit_wmac_resp_antc(&mut self) -> BitWmacRespAntcW<'_, RegAntcdInfoSpec> {
        BitWmacRespAntcW::new(self, 8)
    }
    #[doc = "Bits 12:15 - used for antenna select d value BIT_WMAC_RESP_ANTCD\\[3:2\\], used for antsel_d value;"]
    #[inline(always)]
    pub fn bit_wmac_resp_antd(&mut self) -> BitWmacRespAntdW<'_, RegAntcdInfoSpec> {
        BitWmacRespAntdW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_antcd_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_antcd_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAntcdInfoSpec;
impl crate::RegisterSpec for RegAntcdInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_antcd_info::R`](R) reader structure"]
impl crate::Readable for RegAntcdInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_antcd_info::W`](W) writer structure"]
impl crate::Writable for RegAntcdInfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANTCD_INFO to value 0"]
impl crate::Resettable for RegAntcdInfoSpec {}
