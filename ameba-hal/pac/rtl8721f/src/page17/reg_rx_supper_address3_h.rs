#[doc = "Register `REG_RX_SUPPER_ADDRESS3_H` reader"]
pub type R = crate::R<RegRxSupperAddress3HSpec>;
#[doc = "Register `REG_RX_SUPPER_ADDRESS3_H` writer"]
pub type W = crate::W<RegRxSupperAddress3HSpec>;
#[doc = "Field `r_sup_addr3_h` reader - Address3 for data packet check. When address3 is match, RX will receive the mpdu to RXFF without considering other filter condition except crc32"]
pub type RSupAddr3HR = crate::FieldReader<u16>;
#[doc = "Field `r_sup_addr3_h` writer - Address3 for data packet check. When address3 is match, RX will receive the mpdu to RXFF without considering other filter condition except crc32"]
pub type RSupAddr3HW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_sup_addr3_en` reader - when set, enable check supper Address3"]
pub type RSupAddr3EnR = crate::BitReader;
#[doc = "Field `r_sup_addr3_en` writer - when set, enable check supper Address3"]
pub type RSupAddr3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy` reader - dummy"]
pub type DummyR = crate::BitReader;
#[doc = "Field `dummy` writer - dummy"]
pub type DummyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Address3 for data packet check. When address3 is match, RX will receive the mpdu to RXFF without considering other filter condition except crc32"]
    #[inline(always)]
    pub fn r_sup_addr3_h(&self) -> RSupAddr3HR {
        RSupAddr3HR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - when set, enable check supper Address3"]
    #[inline(always)]
    pub fn r_sup_addr3_en(&self) -> RSupAddr3EnR {
        RSupAddr3EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Address3 for data packet check. When address3 is match, RX will receive the mpdu to RXFF without considering other filter condition except crc32"]
    #[inline(always)]
    pub fn r_sup_addr3_h(&mut self) -> RSupAddr3HW<'_, RegRxSupperAddress3HSpec> {
        RSupAddr3HW::new(self, 0)
    }
    #[doc = "Bit 16 - when set, enable check supper Address3"]
    #[inline(always)]
    pub fn r_sup_addr3_en(&mut self) -> RSupAddr3EnW<'_, RegRxSupperAddress3HSpec> {
        RSupAddr3EnW::new(self, 16)
    }
    #[doc = "Bit 17 - dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegRxSupperAddress3HSpec> {
        DummyW::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_supper_address3_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_supper_address3_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxSupperAddress3HSpec;
impl crate::RegisterSpec for RegRxSupperAddress3HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_supper_address3_h::R`](R) reader structure"]
impl crate::Readable for RegRxSupperAddress3HSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_supper_address3_h::W`](W) writer structure"]
impl crate::Writable for RegRxSupperAddress3HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_SUPPER_ADDRESS3_H to value 0"]
impl crate::Resettable for RegRxSupperAddress3HSpec {}
