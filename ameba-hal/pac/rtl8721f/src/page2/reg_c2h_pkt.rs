#[doc = "Register `REG_C2H_PKT` reader"]
pub type R = crate::R<RegC2hPktSpec>;
#[doc = "Register `REG_C2H_PKT` writer"]
pub type W = crate::W<RegC2hPktSpec>;
#[doc = "Field `BIT_R_C2H_STR_ADDR` reader - C2h packet start address"]
pub type BitRC2hStrAddrR = crate::FieldReader<u16>;
#[doc = "Field `BIT_R_C2H_STR_ADDR` writer - C2h packet start address"]
pub type BitRC2hStrAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_R_C2H_PKT_REQ` reader - C2h packet dma request"]
pub type BitRC2hPktReqR = crate::BitReader;
#[doc = "Field `BIT_R_C2H_PKT_REQ` writer - C2h packet dma request"]
pub type BitRC2hPktReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:15 - C2h packet start address"]
    #[inline(always)]
    pub fn bit_r_c2h_str_addr(&self) -> BitRC2hStrAddrR {
        BitRC2hStrAddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - C2h packet dma request"]
    #[inline(always)]
    pub fn bit_r_c2h_pkt_req(&self) -> BitRC2hPktReqR {
        BitRC2hPktReqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - C2h packet start address"]
    #[inline(always)]
    pub fn bit_r_c2h_str_addr(&mut self) -> BitRC2hStrAddrW<'_, RegC2hPktSpec> {
        BitRC2hStrAddrW::new(self, 0)
    }
    #[doc = "Bit 16 - C2h packet dma request"]
    #[inline(always)]
    pub fn bit_r_c2h_pkt_req(&mut self) -> BitRC2hPktReqW<'_, RegC2hPktSpec> {
        BitRC2hPktReqW::new(self, 16)
    }
    #[doc = "Bits 17:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegC2hPktSpec> {
        RsvdW::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_c2h_pkt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_c2h_pkt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegC2hPktSpec;
impl crate::RegisterSpec for RegC2hPktSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_c2h_pkt::R`](R) reader structure"]
impl crate::Readable for RegC2hPktSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_c2h_pkt::W`](W) writer structure"]
impl crate::Writable for RegC2hPktSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_C2H_PKT to value 0"]
impl crate::Resettable for RegC2hPktSpec {}
