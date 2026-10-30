#[doc = "Register `REG_BCN_IFS_TBTT_CTN` reader"]
pub type R = crate::R<RegBcnIfsTbttCtnSpec>;
#[doc = "Register `REG_BCN_IFS_TBTT_CTN` writer"]
pub type W = crate::W<RegBcnIfsTbttCtnSpec>;
#[doc = "Field `BIT_TBTT_CTN_AREA` reader - TBTT contention area,Time unit is 32us,for PORT 0 / 1 / 2"]
pub type BitTbttCtnAreaR = crate::FieldReader;
#[doc = "Field `BIT_TBTT_CTN_AREA` writer - TBTT contention area,Time unit is 32us,for PORT 0 / 1 / 2"]
pub type BitTbttCtnAreaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_FORCE_BCN_IFS` reader - when 0x528 Bit18 set to 0 and TBTT contention area end,force tx beacon use this IFS"]
pub type BitForceBcnIfsR = crate::FieldReader;
#[doc = "Field `BIT_FORCE_BCN_IFS` writer - when 0x528 Bit18 set to 0 and TBTT contention area end,force tx beacon use this IFS"]
pub type BitForceBcnIfsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_PRE_BCN_DMATIM` reader - when 0x552\\[3\\] is set, hw will enable pre_download beacon. this register Indicates the time before TBTT to perform beacon queue DMA."]
pub type BitPreBcnDmatimR = crate::FieldReader;
#[doc = "Field `BIT_PRE_BCN_DMATIM` writer - when 0x552\\[3\\] is set, hw will enable pre_download beacon. this register Indicates the time before TBTT to perform beacon queue DMA."]
pub type BitPreBcnDmatimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TBTT contention area,Time unit is 32us,for PORT 0 / 1 / 2"]
    #[inline(always)]
    pub fn bit_tbtt_ctn_area(&self) -> BitTbttCtnAreaR {
        BitTbttCtnAreaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - when 0x528 Bit18 set to 0 and TBTT contention area end,force tx beacon use this IFS"]
    #[inline(always)]
    pub fn bit_force_bcn_ifs(&self) -> BitForceBcnIfsR {
        BitForceBcnIfsR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - when 0x552\\[3\\] is set, hw will enable pre_download beacon. this register Indicates the time before TBTT to perform beacon queue DMA."]
    #[inline(always)]
    pub fn bit_pre_bcn_dmatim(&self) -> BitPreBcnDmatimR {
        BitPreBcnDmatimR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TBTT contention area,Time unit is 32us,for PORT 0 / 1 / 2"]
    #[inline(always)]
    pub fn bit_tbtt_ctn_area(&mut self) -> BitTbttCtnAreaW<'_, RegBcnIfsTbttCtnSpec> {
        BitTbttCtnAreaW::new(self, 0)
    }
    #[doc = "Bits 16:23 - when 0x528 Bit18 set to 0 and TBTT contention area end,force tx beacon use this IFS"]
    #[inline(always)]
    pub fn bit_force_bcn_ifs(&mut self) -> BitForceBcnIfsW<'_, RegBcnIfsTbttCtnSpec> {
        BitForceBcnIfsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - when 0x552\\[3\\] is set, hw will enable pre_download beacon. this register Indicates the time before TBTT to perform beacon queue DMA."]
    #[inline(always)]
    pub fn bit_pre_bcn_dmatim(&mut self) -> BitPreBcnDmatimW<'_, RegBcnIfsTbttCtnSpec> {
        BitPreBcnDmatimW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_ifs_tbtt_ctn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_ifs_tbtt_ctn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnIfsTbttCtnSpec;
impl crate::RegisterSpec for RegBcnIfsTbttCtnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_ifs_tbtt_ctn::R`](R) reader structure"]
impl crate::Readable for RegBcnIfsTbttCtnSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_ifs_tbtt_ctn::W`](W) writer structure"]
impl crate::Writable for RegBcnIfsTbttCtnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_IFS_TBTT_CTN to value 0x0620_0060"]
impl crate::Resettable for RegBcnIfsTbttCtnSpec {
    const RESET_VALUE: u32 = 0x0620_0060;
}
