#[doc = "Register `REG_ANTTRN_CTR_1_V1` reader"]
pub type R = crate::R<RegAnttrnCtr1V1Spec>;
#[doc = "Register `REG_ANTTRN_CTR_1_V1` writer"]
pub type W = crate::W<RegAnttrnCtr1V1Spec>;
#[doc = "Field `BIT_TRAIN_STA_ADDR2` reader - The Address of the trained station. Only available when ADDR2_MATCH_EN = 1"]
pub type BitTrainStaAddr2R = crate::FieldReader<u16>;
#[doc = "Field `BIT_TRAIN_STA_ADDR2` writer - The Address of the trained station. Only available when ADDR2_MATCH_EN = 1"]
pub type BitTrainStaAddr2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_ANTTRN_EN` reader - Enable the antenna training function"]
pub type BitAnttrnEnR = crate::BitReader;
#[doc = "Field `BIT_ANTTRN_EN` writer - Enable the antenna training function"]
pub type BitAnttrnEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ADDR2_MATCH_EN` reader - Enable the comparison of ADDR2 for the rx packet.."]
pub type BitAddr2MatchEnR = crate::BitReader;
#[doc = "Field `BIT_ADDR2_MATCH_EN` writer - Enable the comparison of ADDR2 for the rx packet.."]
pub type BitAddr2MatchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APPEND_MACID_IN_RESP_EN` reader - Whether to append MACID in the rx descriptor of rx response packet"]
pub type BitAppendMacidInRespEnR = crate::BitReader;
#[doc = "Field `BIT_APPEND_MACID_IN_RESP_EN` writer - Whether to append MACID in the rx descriptor of rx response packet"]
pub type BitAppendMacidInRespEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ANTTRN_SWITCH` reader - Smart antenna use address cam or port register to search A2. 0:Use address cam 1:Use port register"]
pub type BitAnttrnSwitchR = crate::BitReader;
#[doc = "Field `BIT_ANTTRN_SWITCH` writer - Smart antenna use address cam or port register to search A2. 0:Use address cam 1:Use port register"]
pub type BitAnttrnSwitchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - The Address of the trained station. Only available when ADDR2_MATCH_EN = 1"]
    #[inline(always)]
    pub fn bit_train_sta_addr2(&self) -> BitTrainStaAddr2R {
        BitTrainStaAddr2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable the antenna training function"]
    #[inline(always)]
    pub fn bit_anttrn_en(&self) -> BitAnttrnEnR {
        BitAnttrnEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable the comparison of ADDR2 for the rx packet.."]
    #[inline(always)]
    pub fn bit_addr2_match_en(&self) -> BitAddr2MatchEnR {
        BitAddr2MatchEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Whether to append MACID in the rx descriptor of rx response packet"]
    #[inline(always)]
    pub fn bit_append_macid_in_resp_en(&self) -> BitAppendMacidInRespEnR {
        BitAppendMacidInRespEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Smart antenna use address cam or port register to search A2. 0:Use address cam 1:Use port register"]
    #[inline(always)]
    pub fn bit_anttrn_switch(&self) -> BitAnttrnSwitchR {
        BitAnttrnSwitchR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - The Address of the trained station. Only available when ADDR2_MATCH_EN = 1"]
    #[inline(always)]
    pub fn bit_train_sta_addr2(&mut self) -> BitTrainStaAddr2W<'_, RegAnttrnCtr1V1Spec> {
        BitTrainStaAddr2W::new(self, 0)
    }
    #[doc = "Bit 16 - Enable the antenna training function"]
    #[inline(always)]
    pub fn bit_anttrn_en(&mut self) -> BitAnttrnEnW<'_, RegAnttrnCtr1V1Spec> {
        BitAnttrnEnW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable the comparison of ADDR2 for the rx packet.."]
    #[inline(always)]
    pub fn bit_addr2_match_en(&mut self) -> BitAddr2MatchEnW<'_, RegAnttrnCtr1V1Spec> {
        BitAddr2MatchEnW::new(self, 17)
    }
    #[doc = "Bit 18 - Whether to append MACID in the rx descriptor of rx response packet"]
    #[inline(always)]
    pub fn bit_append_macid_in_resp_en(
        &mut self,
    ) -> BitAppendMacidInRespEnW<'_, RegAnttrnCtr1V1Spec> {
        BitAppendMacidInRespEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Smart antenna use address cam or port register to search A2. 0:Use address cam 1:Use port register"]
    #[inline(always)]
    pub fn bit_anttrn_switch(&mut self) -> BitAnttrnSwitchW<'_, RegAnttrnCtr1V1Spec> {
        BitAnttrnSwitchW::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anttrn_ctr_1_v1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anttrn_ctr_1_v1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnttrnCtr1V1Spec;
impl crate::RegisterSpec for RegAnttrnCtr1V1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anttrn_ctr_1_v1::R`](R) reader structure"]
impl crate::Readable for RegAnttrnCtr1V1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anttrn_ctr_1_v1::W`](W) writer structure"]
impl crate::Writable for RegAnttrnCtr1V1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANTTRN_CTR_1_V1 to value 0"]
impl crate::Resettable for RegAnttrnCtr1V1Spec {}
