#[doc = "Register `REG_RQPN_NPQ` reader"]
pub type R = crate::R<RegRqpnNpqSpec>;
#[doc = "Register `REG_RQPN_NPQ` writer"]
pub type W = crate::W<RegRqpnNpqSpec>;
#[doc = "Field `BIT_NPQ_R_PGNUM` reader - Normal Priority Queue Reserved Page Number"]
pub type BitNpqRPgnumR = crate::FieldReader;
#[doc = "Field `BIT_NPQ_R_PGNUM` writer - Normal Priority Queue Reserved Page Number"]
pub type BitNpqRPgnumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_NPQ_PGNUM` reader - Normal Priority Queue Page Number"]
pub type BitNpqPgnumR = crate::FieldReader;
#[doc = "Field `BIT_EPQ_R_PGNUM` reader - Extra Queue Reserved Page Number"]
pub type BitEpqRPgnumR = crate::FieldReader;
#[doc = "Field `BIT_EPQ_R_PGNUM` writer - Extra Queue Reserved Page Number"]
pub type BitEpqRPgnumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EPQ_PGNUM` reader - Extra Queue Page Number"]
pub type BitEpqPgnumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Normal Priority Queue Reserved Page Number"]
    #[inline(always)]
    pub fn bit_npq_r_pgnum(&self) -> BitNpqRPgnumR {
        BitNpqRPgnumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Normal Priority Queue Page Number"]
    #[inline(always)]
    pub fn bit_npq_pgnum(&self) -> BitNpqPgnumR {
        BitNpqPgnumR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Extra Queue Reserved Page Number"]
    #[inline(always)]
    pub fn bit_epq_r_pgnum(&self) -> BitEpqRPgnumR {
        BitEpqRPgnumR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Extra Queue Page Number"]
    #[inline(always)]
    pub fn bit_epq_pgnum(&self) -> BitEpqPgnumR {
        BitEpqPgnumR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Normal Priority Queue Reserved Page Number"]
    #[inline(always)]
    pub fn bit_npq_r_pgnum(&mut self) -> BitNpqRPgnumW<'_, RegRqpnNpqSpec> {
        BitNpqRPgnumW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Extra Queue Reserved Page Number"]
    #[inline(always)]
    pub fn bit_epq_r_pgnum(&mut self) -> BitEpqRPgnumW<'_, RegRqpnNpqSpec> {
        BitEpqRPgnumW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rqpn_npq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rqpn_npq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRqpnNpqSpec;
impl crate::RegisterSpec for RegRqpnNpqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rqpn_npq::R`](R) reader structure"]
impl crate::Readable for RegRqpnNpqSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rqpn_npq::W`](W) writer structure"]
impl crate::Writable for RegRqpnNpqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RQPN_NPQ to value 0"]
impl crate::Resettable for RegRqpnNpqSpec {}
