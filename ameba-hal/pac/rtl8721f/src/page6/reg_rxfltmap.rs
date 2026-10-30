#[doc = "Register `REG_RXFLTMAP` reader"]
pub type R = crate::R<RegRxfltmapSpec>;
#[doc = "Register `REG_RXFLTMAP` writer"]
pub type W = crate::W<RegRxfltmapSpec>;
#[doc = "Field `BIT_DATAFLT0En` reader - When set, frames with type=10 and subtype=0000 are received into RXFF."]
pub type BitDataflt0enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT0En` writer - When set, frames with type=10 and subtype=0000 are received into RXFF."]
pub type BitDataflt0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT1En` reader - When set, frames with type=10 and subtype=0001 are received into RXFF."]
pub type BitDataflt1enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT1En` writer - When set, frames with type=10 and subtype=0001 are received into RXFF."]
pub type BitDataflt1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT2En` reader - When set, frames with type=10 and subtype=0010 are received into RXFF."]
pub type BitDataflt2enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT2En` writer - When set, frames with type=10 and subtype=0010 are received into RXFF."]
pub type BitDataflt2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT3En` reader - When set, frames with type=10 and subtype=0011 are received into RXFF"]
pub type BitDataflt3enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT3En` writer - When set, frames with type=10 and subtype=0011 are received into RXFF"]
pub type BitDataflt3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT4En` reader - When set, frames with type=10 and subtype=0100 are received into RXFF."]
pub type BitDataflt4enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT4En` writer - When set, frames with type=10 and subtype=0100 are received into RXFF."]
pub type BitDataflt4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT5En` reader - When set, frames with type=10 and subtype=0101 are received into RXFF"]
pub type BitDataflt5enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT5En` writer - When set, frames with type=10 and subtype=0101 are received into RXFF"]
pub type BitDataflt5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT6En` reader - When set, frames with type=10 and subtype=0110 are received into RXFF"]
pub type BitDataflt6enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT6En` writer - When set, frames with type=10 and subtype=0110 are received into RXFF"]
pub type BitDataflt6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT7En` reader - When set, frames with type=10 and subtype=0111 are received into RXFF."]
pub type BitDataflt7enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT7En` writer - When set, frames with type=10 and subtype=0111 are received into RXFF."]
pub type BitDataflt7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT8En` reader - When set, frames with type=10 and subtype=1000 are received into RXFF"]
pub type BitDataflt8enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT8En` writer - When set, frames with type=10 and subtype=1000 are received into RXFF"]
pub type BitDataflt8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT9En` reader - When set, frames with type=10 and subtype=1001 are received into RXFF"]
pub type BitDataflt9enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT9En` writer - When set, frames with type=10 and subtype=1001 are received into RXFF"]
pub type BitDataflt9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT10En` reader - When set, frames with type=10 and subtype=1010 are received into RXFF."]
pub type BitDataflt10enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT10En` writer - When set, frames with type=10 and subtype=1010 are received into RXFF."]
pub type BitDataflt10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT11En` reader - When set, frames with type=10 and subtype=1011 are received into RXFF."]
pub type BitDataflt11enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT11En` writer - When set, frames with type=10 and subtype=1011 are received into RXFF."]
pub type BitDataflt11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT12En` reader - When set, frames with type=10 and subtype=1100 are received into RXFF."]
pub type BitDataflt12enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT12En` writer - When set, frames with type=10 and subtype=1100 are received into RXFF."]
pub type BitDataflt12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT13En` reader - When set, frames with type=10 and subtype=1101 are received into RXFF."]
pub type BitDataflt13enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT13En` writer - When set, frames with type=10 and subtype=1101 are received into RXFF."]
pub type BitDataflt13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT14En` reader - When set, frames with type=10 and subtype=1110 are received into RXFF"]
pub type BitDataflt14enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT14En` writer - When set, frames with type=10 and subtype=1110 are received into RXFF"]
pub type BitDataflt14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DATAFLT15En` reader - When set, frames with type=10 and subtype=1111 are received into RXFF"]
pub type BitDataflt15enR = crate::BitReader;
#[doc = "Field `BIT_DATAFLT15En` writer - When set, frames with type=10 and subtype=1111 are received into RXFF"]
pub type BitDataflt15enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set, frames with type=10 and subtype=0000 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt0en(&self) -> BitDataflt0enR {
        BitDataflt0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set, frames with type=10 and subtype=0001 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt1en(&self) -> BitDataflt1enR {
        BitDataflt1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set, frames with type=10 and subtype=0010 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt2en(&self) -> BitDataflt2enR {
        BitDataflt2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set, frames with type=10 and subtype=0011 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt3en(&self) -> BitDataflt3enR {
        BitDataflt3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set, frames with type=10 and subtype=0100 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt4en(&self) -> BitDataflt4enR {
        BitDataflt4enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set, frames with type=10 and subtype=0101 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt5en(&self) -> BitDataflt5enR {
        BitDataflt5enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set, frames with type=10 and subtype=0110 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt6en(&self) -> BitDataflt6enR {
        BitDataflt6enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set, frames with type=10 and subtype=0111 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt7en(&self) -> BitDataflt7enR {
        BitDataflt7enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When set, frames with type=10 and subtype=1000 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt8en(&self) -> BitDataflt8enR {
        BitDataflt8enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When set, frames with type=10 and subtype=1001 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt9en(&self) -> BitDataflt9enR {
        BitDataflt9enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When set, frames with type=10 and subtype=1010 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt10en(&self) -> BitDataflt10enR {
        BitDataflt10enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When set, frames with type=10 and subtype=1011 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt11en(&self) -> BitDataflt11enR {
        BitDataflt11enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When set, frames with type=10 and subtype=1100 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt12en(&self) -> BitDataflt12enR {
        BitDataflt12enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When set, frames with type=10 and subtype=1101 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt13en(&self) -> BitDataflt13enR {
        BitDataflt13enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When set, frames with type=10 and subtype=1110 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt14en(&self) -> BitDataflt14enR {
        BitDataflt14enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When set, frames with type=10 and subtype=1111 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt15en(&self) -> BitDataflt15enR {
        BitDataflt15enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set, frames with type=10 and subtype=0000 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt0en(&mut self) -> BitDataflt0enW<'_, RegRxfltmapSpec> {
        BitDataflt0enW::new(self, 0)
    }
    #[doc = "Bit 1 - When set, frames with type=10 and subtype=0001 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt1en(&mut self) -> BitDataflt1enW<'_, RegRxfltmapSpec> {
        BitDataflt1enW::new(self, 1)
    }
    #[doc = "Bit 2 - When set, frames with type=10 and subtype=0010 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt2en(&mut self) -> BitDataflt2enW<'_, RegRxfltmapSpec> {
        BitDataflt2enW::new(self, 2)
    }
    #[doc = "Bit 3 - When set, frames with type=10 and subtype=0011 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt3en(&mut self) -> BitDataflt3enW<'_, RegRxfltmapSpec> {
        BitDataflt3enW::new(self, 3)
    }
    #[doc = "Bit 4 - When set, frames with type=10 and subtype=0100 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt4en(&mut self) -> BitDataflt4enW<'_, RegRxfltmapSpec> {
        BitDataflt4enW::new(self, 4)
    }
    #[doc = "Bit 5 - When set, frames with type=10 and subtype=0101 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt5en(&mut self) -> BitDataflt5enW<'_, RegRxfltmapSpec> {
        BitDataflt5enW::new(self, 5)
    }
    #[doc = "Bit 6 - When set, frames with type=10 and subtype=0110 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt6en(&mut self) -> BitDataflt6enW<'_, RegRxfltmapSpec> {
        BitDataflt6enW::new(self, 6)
    }
    #[doc = "Bit 7 - When set, frames with type=10 and subtype=0111 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt7en(&mut self) -> BitDataflt7enW<'_, RegRxfltmapSpec> {
        BitDataflt7enW::new(self, 7)
    }
    #[doc = "Bit 8 - When set, frames with type=10 and subtype=1000 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt8en(&mut self) -> BitDataflt8enW<'_, RegRxfltmapSpec> {
        BitDataflt8enW::new(self, 8)
    }
    #[doc = "Bit 9 - When set, frames with type=10 and subtype=1001 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt9en(&mut self) -> BitDataflt9enW<'_, RegRxfltmapSpec> {
        BitDataflt9enW::new(self, 9)
    }
    #[doc = "Bit 10 - When set, frames with type=10 and subtype=1010 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt10en(&mut self) -> BitDataflt10enW<'_, RegRxfltmapSpec> {
        BitDataflt10enW::new(self, 10)
    }
    #[doc = "Bit 11 - When set, frames with type=10 and subtype=1011 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt11en(&mut self) -> BitDataflt11enW<'_, RegRxfltmapSpec> {
        BitDataflt11enW::new(self, 11)
    }
    #[doc = "Bit 12 - When set, frames with type=10 and subtype=1100 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt12en(&mut self) -> BitDataflt12enW<'_, RegRxfltmapSpec> {
        BitDataflt12enW::new(self, 12)
    }
    #[doc = "Bit 13 - When set, frames with type=10 and subtype=1101 are received into RXFF."]
    #[inline(always)]
    pub fn bit_dataflt13en(&mut self) -> BitDataflt13enW<'_, RegRxfltmapSpec> {
        BitDataflt13enW::new(self, 13)
    }
    #[doc = "Bit 14 - When set, frames with type=10 and subtype=1110 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt14en(&mut self) -> BitDataflt14enW<'_, RegRxfltmapSpec> {
        BitDataflt14enW::new(self, 14)
    }
    #[doc = "Bit 15 - When set, frames with type=10 and subtype=1111 are received into RXFF"]
    #[inline(always)]
    pub fn bit_dataflt15en(&mut self) -> BitDataflt15enW<'_, RegRxfltmapSpec> {
        BitDataflt15enW::new(self, 15)
    }
}
#[doc = "RX Filter Map Group 1\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxfltmap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxfltmap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxfltmapSpec;
impl crate::RegisterSpec for RegRxfltmapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxfltmap::R`](R) reader structure"]
impl crate::Readable for RegRxfltmapSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxfltmap::W`](W) writer structure"]
impl crate::Writable for RegRxfltmapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXFLTMAP to value 0"]
impl crate::Resettable for RegRxfltmapSpec {}
