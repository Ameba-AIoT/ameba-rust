#[doc = "Register `REG_DROP_PKT_NUM` reader"]
pub type R = crate::R<RegDropPktNumSpec>;
#[doc = "Register `REG_DROP_PKT_NUM` writer"]
pub type W = crate::W<RegDropPktNumSpec>;
#[doc = "Field `BIT_DROP_PKT_NUM` reader - Statistic the number of dropped packets"]
pub type BitDropPktNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_DROP_PKT_NUM` writer - Statistic the number of dropped packets"]
pub type BitDropPktNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_LIFE_DROP_NUM` reader - TIE 0 in 8723F"]
pub type BitLifeDropNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_LIFE_DROP_NUM` writer - TIE 0 in 8723F"]
pub type BitLifeDropNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Statistic the number of dropped packets"]
    #[inline(always)]
    pub fn bit_drop_pkt_num(&self) -> BitDropPktNumR {
        BitDropPktNumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TIE 0 in 8723F"]
    #[inline(always)]
    pub fn bit_life_drop_num(&self) -> BitLifeDropNumR {
        BitLifeDropNumR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Statistic the number of dropped packets"]
    #[inline(always)]
    pub fn bit_drop_pkt_num(&mut self) -> BitDropPktNumW<'_, RegDropPktNumSpec> {
        BitDropPktNumW::new(self, 0)
    }
    #[doc = "Bits 16:31 - TIE 0 in 8723F"]
    #[inline(always)]
    pub fn bit_life_drop_num(&mut self) -> BitLifeDropNumW<'_, RegDropPktNumSpec> {
        BitLifeDropNumW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_drop_pkt_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_drop_pkt_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDropPktNumSpec;
impl crate::RegisterSpec for RegDropPktNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_drop_pkt_num::R`](R) reader structure"]
impl crate::Readable for RegDropPktNumSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_drop_pkt_num::W`](W) writer structure"]
impl crate::Writable for RegDropPktNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DROP_PKT_NUM to value 0"]
impl crate::Resettable for RegDropPktNumSpec {}
