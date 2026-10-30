#[doc = "Register `REG_RQPN_CTRL_HLPQ` reader"]
pub type R = crate::R<RegRqpnCtrlHlpqSpec>;
#[doc = "Register `REG_RQPN_CTRL_HLPQ` writer"]
pub type W = crate::W<RegRqpnCtrlHlpqSpec>;
#[doc = "Field `BIT_HPQ` reader - High Priority Queue Reserved Page Number"]
pub type BitHpqR = crate::FieldReader;
#[doc = "Field `BIT_HPQ` writer - High Priority Queue Reserved Page Number"]
pub type BitHpqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_LPQ` reader - Low Priority Queue Reserved Page Number"]
pub type BitLpqR = crate::FieldReader;
#[doc = "Field `BIT_LPQ` writer - Low Priority Queue Reserved Page Number"]
pub type BitLpqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_PUBQ` reader - Public Reserved Page Number"]
pub type BitPubqR = crate::FieldReader;
#[doc = "Field `BIT_PUBQ` writer - Public Reserved Page Number"]
pub type BitPubqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_HPQ_PUBLIC_DIS` reader - When this bit is set, High queue available Tx page size excludes the public queue. Otherwise, High queue available Tx page size includes public queue"]
pub type BitHpqPublicDisR = crate::BitReader;
#[doc = "Field `BIT_HPQ_PUBLIC_DIS` writer - When this bit is set, High queue available Tx page size excludes the public queue. Otherwise, High queue available Tx page size includes public queue"]
pub type BitHpqPublicDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LPQ_PUBLIC_DIS` reader - When this bit is set, Low queue available Tx page size excludes the public queue. Otherwise, Low queue available Tx page size includes public queue"]
pub type BitLpqPublicDisR = crate::BitReader;
#[doc = "Field `BIT_LPQ_PUBLIC_DIS` writer - When this bit is set, Low queue available Tx page size excludes the public queue. Otherwise, Low queue available Tx page size includes public queue"]
pub type BitLpqPublicDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NPQ_PUBLIC_DIS` reader - When this bit is set, Normal queue available Tx page size excludes the public queue. Otherwise, Normal queue available Tx page size includes public queue"]
pub type BitNpqPublicDisR = crate::BitReader;
#[doc = "Field `BIT_NPQ_PUBLIC_DIS` writer - When this bit is set, Normal queue available Tx page size excludes the public queue. Otherwise, Normal queue available Tx page size includes public queue"]
pub type BitNpqPublicDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EPQ_PUBLIC_DIS` reader - When this bit is set, Extra queue available Tx page size excludes the public queue. Otherwise, Extra queue available Tx page size includes public queue"]
pub type BitEpqPublicDisR = crate::BitReader;
#[doc = "Field `BIT_EPQ_PUBLIC_DIS` writer - When this bit is set, Extra queue available Tx page size excludes the public queue. Otherwise, Extra queue available Tx page size includes public queue"]
pub type BitEpqPublicDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_LOAD_R_PGNUM` writer - Write 1 to set RQPN bit\\[79:0\\] value to page numbers for initialization"]
pub type BitLoadRPgnumW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - High Priority Queue Reserved Page Number"]
    #[inline(always)]
    pub fn bit_hpq(&self) -> BitHpqR {
        BitHpqR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Low Priority Queue Reserved Page Number"]
    #[inline(always)]
    pub fn bit_lpq(&self) -> BitLpqR {
        BitLpqR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Public Reserved Page Number"]
    #[inline(always)]
    pub fn bit_pubq(&self) -> BitPubqR {
        BitPubqR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - When this bit is set, High queue available Tx page size excludes the public queue. Otherwise, High queue available Tx page size includes public queue"]
    #[inline(always)]
    pub fn bit_hpq_public_dis(&self) -> BitHpqPublicDisR {
        BitHpqPublicDisR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When this bit is set, Low queue available Tx page size excludes the public queue. Otherwise, Low queue available Tx page size includes public queue"]
    #[inline(always)]
    pub fn bit_lpq_public_dis(&self) -> BitLpqPublicDisR {
        BitLpqPublicDisR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When this bit is set, Normal queue available Tx page size excludes the public queue. Otherwise, Normal queue available Tx page size includes public queue"]
    #[inline(always)]
    pub fn bit_npq_public_dis(&self) -> BitNpqPublicDisR {
        BitNpqPublicDisR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When this bit is set, Extra queue available Tx page size excludes the public queue. Otherwise, Extra queue available Tx page size includes public queue"]
    #[inline(always)]
    pub fn bit_epq_public_dis(&self) -> BitEpqPublicDisR {
        BitEpqPublicDisR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High Priority Queue Reserved Page Number"]
    #[inline(always)]
    pub fn bit_hpq(&mut self) -> BitHpqW<'_, RegRqpnCtrlHlpqSpec> {
        BitHpqW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Low Priority Queue Reserved Page Number"]
    #[inline(always)]
    pub fn bit_lpq(&mut self) -> BitLpqW<'_, RegRqpnCtrlHlpqSpec> {
        BitLpqW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Public Reserved Page Number"]
    #[inline(always)]
    pub fn bit_pubq(&mut self) -> BitPubqW<'_, RegRqpnCtrlHlpqSpec> {
        BitPubqW::new(self, 16)
    }
    #[doc = "Bit 24 - When this bit is set, High queue available Tx page size excludes the public queue. Otherwise, High queue available Tx page size includes public queue"]
    #[inline(always)]
    pub fn bit_hpq_public_dis(&mut self) -> BitHpqPublicDisW<'_, RegRqpnCtrlHlpqSpec> {
        BitHpqPublicDisW::new(self, 24)
    }
    #[doc = "Bit 25 - When this bit is set, Low queue available Tx page size excludes the public queue. Otherwise, Low queue available Tx page size includes public queue"]
    #[inline(always)]
    pub fn bit_lpq_public_dis(&mut self) -> BitLpqPublicDisW<'_, RegRqpnCtrlHlpqSpec> {
        BitLpqPublicDisW::new(self, 25)
    }
    #[doc = "Bit 26 - When this bit is set, Normal queue available Tx page size excludes the public queue. Otherwise, Normal queue available Tx page size includes public queue"]
    #[inline(always)]
    pub fn bit_npq_public_dis(&mut self) -> BitNpqPublicDisW<'_, RegRqpnCtrlHlpqSpec> {
        BitNpqPublicDisW::new(self, 26)
    }
    #[doc = "Bit 27 - When this bit is set, Extra queue available Tx page size excludes the public queue. Otherwise, Extra queue available Tx page size includes public queue"]
    #[inline(always)]
    pub fn bit_epq_public_dis(&mut self) -> BitEpqPublicDisW<'_, RegRqpnCtrlHlpqSpec> {
        BitEpqPublicDisW::new(self, 27)
    }
    #[doc = "Bits 28:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegRqpnCtrlHlpqSpec> {
        RsvdW::new(self, 28)
    }
    #[doc = "Bit 31 - Write 1 to set RQPN bit\\[79:0\\] value to page numbers for initialization"]
    #[inline(always)]
    pub fn bit_load_r_pgnum(&mut self) -> BitLoadRPgnumW<'_, RegRqpnCtrlHlpqSpec> {
        BitLoadRPgnumW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rqpn_ctrl_hlpq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rqpn_ctrl_hlpq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRqpnCtrlHlpqSpec;
impl crate::RegisterSpec for RegRqpnCtrlHlpqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rqpn_ctrl_hlpq::R`](R) reader structure"]
impl crate::Readable for RegRqpnCtrlHlpqSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rqpn_ctrl_hlpq::W`](W) writer structure"]
impl crate::Writable for RegRqpnCtrlHlpqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RQPN_CTRL_HLPQ to value 0"]
impl crate::Resettable for RegRqpnCtrlHlpqSpec {}
