#[doc = "Register `REG_BT_KR4_CTRL` reader"]
pub type R = crate::R<RegBtKr4CtrlSpec>;
#[doc = "Register `REG_BT_KR4_CTRL` writer"]
pub type W = crate::W<RegBtKr4CtrlSpec>;
#[doc = "Field `bt_en_pwm_load` reader - 1:when bt enter normal , it will enable pwm high loading option 0:pwm loading change effect by bt"]
pub type BtEnPwmLoadR = crate::BitReader;
#[doc = "Field `bt_en_pwm_load` writer - 1:when bt enter normal , it will enable pwm high loading option 0:pwm loading change effect by bt"]
pub type BtEnPwmLoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_en_pon_req_regu` reader - 1: enable to change BT regu_sts request during BT active"]
pub type BtEnPonReqReguR = crate::BitReader;
#[doc = "Field `bt_en_pon_req_regu` writer - 1: enable to change BT regu_sts request during BT active"]
pub type BtEnPonReqReguW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ckd_bt_pll_40m` reader - pll divider for BT 0: Not support 1: div2 ... DIV_MIN:1"]
pub type CkdBtPll40mR = crate::FieldReader;
#[doc = "Field `ckd_bt_pll_40m` writer - pll divider for BT 0: Not support 1: div2 ... DIV_MIN:1"]
pub type CkdBtPll40mW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 24 - 1:when bt enter normal , it will enable pwm high loading option 0:pwm loading change effect by bt"]
    #[inline(always)]
    pub fn bt_en_pwm_load(&self) -> BtEnPwmLoadR {
        BtEnPwmLoadR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: enable to change BT regu_sts request during BT active"]
    #[inline(always)]
    pub fn bt_en_pon_req_regu(&self) -> BtEnPonReqReguR {
        BtEnPonReqReguR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31 - pll divider for BT 0: Not support 1: div2 ... DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_bt_pll_40m(&self) -> CkdBtPll40mR {
        CkdBtPll40mR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - 1:when bt enter normal , it will enable pwm high loading option 0:pwm loading change effect by bt"]
    #[inline(always)]
    pub fn bt_en_pwm_load(&mut self) -> BtEnPwmLoadW<'_, RegBtKr4CtrlSpec> {
        BtEnPwmLoadW::new(self, 24)
    }
    #[doc = "Bit 26 - 1: enable to change BT regu_sts request during BT active"]
    #[inline(always)]
    pub fn bt_en_pon_req_regu(&mut self) -> BtEnPonReqReguW<'_, RegBtKr4CtrlSpec> {
        BtEnPonReqReguW::new(self, 26)
    }
    #[doc = "Bits 28:31 - pll divider for BT 0: Not support 1: div2 ... DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_bt_pll_40m(&mut self) -> CkdBtPll40mW<'_, RegBtKr4CtrlSpec> {
        CkdBtPll40mW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_kr4_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_kr4_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtKr4CtrlSpec;
impl crate::RegisterSpec for RegBtKr4CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_kr4_ctrl::R`](R) reader structure"]
impl crate::Readable for RegBtKr4CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_kr4_ctrl::W`](W) writer structure"]
impl crate::Writable for RegBtKr4CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_KR4_CTRL to value 0x7100_0000"]
impl crate::Resettable for RegBtKr4CtrlSpec {
    const RESET_VALUE: u32 = 0x7100_0000;
}
