#[doc = "Register `REG_POF_BCNELY_AGG_TBTT` reader"]
pub type R = crate::R<RegPofBcnelyAggTbttSpec>;
#[doc = "Register `REG_POF_BCNELY_AGG_TBTT` writer"]
pub type W = crate::W<RegPofBcnelyAggTbttSpec>;
#[doc = "Field `BIT_INI_POF_ELY_AGG_CNT` reader - bcn_ely_agg initial value of P0. Valid when BIT_EN_POF_ELY_AGG = 1"]
pub type BitIniPofElyAggCntR = crate::FieldReader;
#[doc = "Field `BIT_INI_POF_ELY_AGG_CNT` writer - bcn_ely_agg initial value of P0. Valid when BIT_EN_POF_ELY_AGG = 1"]
pub type BitIniPofElyAggCntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_EN_POF_ELY_AGG` reader - update bcn_ely_agg initial value of P0 to BIT_INI_POF_ELY_AGG_CNT 1: enable 0: disable"]
pub type BitEnPofElyAggR = crate::BitReader;
#[doc = "Field `BIT_EN_POF_ELY_AGG` writer - update bcn_ely_agg initial value of P0 to BIT_INI_POF_ELY_AGG_CNT 1: enable 0: disable"]
pub type BitEnPofElyAggW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P0_TBTT_CNT_THRS` reader - P0's TBTT counter threshold . Valid when BIT_EN_P0_TBTT_CNT = 1. when tbtt_cnt >= BIT_P0_TBTT_CNT_THRS, assert p0_tbtt_agg_int."]
pub type BitP0TbttCntThrsR = crate::FieldReader<u16>;
#[doc = "Field `BIT_P0_TBTT_CNT_THRS` writer - P0's TBTT counter threshold . Valid when BIT_EN_P0_TBTT_CNT = 1. when tbtt_cnt >= BIT_P0_TBTT_CNT_THRS, assert p0_tbtt_agg_int."]
pub type BitP0TbttCntThrsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_EN_P0_TBTT_CNT` reader - enable the tbtt counter function."]
pub type BitEnP0TbttCntR = crate::BitReader;
#[doc = "Field `BIT_EN_P0_TBTT_CNT` writer - enable the tbtt counter function."]
pub type BitEnP0TbttCntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - bcn_ely_agg initial value of P0. Valid when BIT_EN_POF_ELY_AGG = 1"]
    #[inline(always)]
    pub fn bit_ini_pof_ely_agg_cnt(&self) -> BitIniPofElyAggCntR {
        BitIniPofElyAggCntR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - update bcn_ely_agg initial value of P0 to BIT_INI_POF_ELY_AGG_CNT 1: enable 0: disable"]
    #[inline(always)]
    pub fn bit_en_pof_ely_agg(&self) -> BitEnPofElyAggR {
        BitEnPofElyAggR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:23 - P0's TBTT counter threshold . Valid when BIT_EN_P0_TBTT_CNT = 1. when tbtt_cnt >= BIT_P0_TBTT_CNT_THRS, assert p0_tbtt_agg_int."]
    #[inline(always)]
    pub fn bit_p0_tbtt_cnt_thrs(&self) -> BitP0TbttCntThrsR {
        BitP0TbttCntThrsR::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bit 24 - enable the tbtt counter function."]
    #[inline(always)]
    pub fn bit_en_p0_tbtt_cnt(&self) -> BitEnP0TbttCntR {
        BitEnP0TbttCntR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - bcn_ely_agg initial value of P0. Valid when BIT_EN_POF_ELY_AGG = 1"]
    #[inline(always)]
    pub fn bit_ini_pof_ely_agg_cnt(&mut self) -> BitIniPofElyAggCntW<'_, RegPofBcnelyAggTbttSpec> {
        BitIniPofElyAggCntW::new(self, 0)
    }
    #[doc = "Bit 7 - update bcn_ely_agg initial value of P0 to BIT_INI_POF_ELY_AGG_CNT 1: enable 0: disable"]
    #[inline(always)]
    pub fn bit_en_pof_ely_agg(&mut self) -> BitEnPofElyAggW<'_, RegPofBcnelyAggTbttSpec> {
        BitEnPofElyAggW::new(self, 7)
    }
    #[doc = "Bits 8:23 - P0's TBTT counter threshold . Valid when BIT_EN_P0_TBTT_CNT = 1. when tbtt_cnt >= BIT_P0_TBTT_CNT_THRS, assert p0_tbtt_agg_int."]
    #[inline(always)]
    pub fn bit_p0_tbtt_cnt_thrs(&mut self) -> BitP0TbttCntThrsW<'_, RegPofBcnelyAggTbttSpec> {
        BitP0TbttCntThrsW::new(self, 8)
    }
    #[doc = "Bit 24 - enable the tbtt counter function."]
    #[inline(always)]
    pub fn bit_en_p0_tbtt_cnt(&mut self) -> BitEnP0TbttCntW<'_, RegPofBcnelyAggTbttSpec> {
        BitEnP0TbttCntW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pof_bcnely_agg_tbtt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pof_bcnely_agg_tbtt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPofBcnelyAggTbttSpec;
impl crate::RegisterSpec for RegPofBcnelyAggTbttSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pof_bcnely_agg_tbtt::R`](R) reader structure"]
impl crate::Readable for RegPofBcnelyAggTbttSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pof_bcnely_agg_tbtt::W`](W) writer structure"]
impl crate::Writable for RegPofBcnelyAggTbttSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_POF_BCNELY_AGG_TBTT to value 0x03"]
impl crate::Resettable for RegPofBcnelyAggTbttSpec {
    const RESET_VALUE: u32 = 0x03;
}
