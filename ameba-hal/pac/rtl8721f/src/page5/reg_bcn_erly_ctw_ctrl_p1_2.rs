#[doc = "Register `REG_BCN_ERLY_CTW_CTRL_P1_2` reader"]
pub type R = crate::R<RegBcnErlyCtwCtrlP1_2Spec>;
#[doc = "Register `REG_BCN_ERLY_CTW_CTRL_P1_2` writer"]
pub type W = crate::W<RegBcnErlyCtwCtrlP1_2Spec>;
#[doc = "Field `BIT_CTWND_P1` reader - CTWindow width in units of TU. PORT1"]
pub type BitCtwndP1R = crate::FieldReader;
#[doc = "Field `BIT_CTWND_P1` writer - CTWindow width in units of TU. PORT1"]
pub type BitCtwndP1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BCNIVLCUNT_P1` reader - Be used in multiple BCNIVL early INT. When this counter =1, multiple BCNIVL early INT will be triggered."]
pub type BitBcnivlcuntP1R = crate::FieldReader;
#[doc = "Field `BIT_CTWND_P2` reader - CTWindow width in units of TU. PORT2"]
pub type BitCtwndP2R = crate::FieldReader;
#[doc = "Field `BIT_CTWND_P2` writer - CTWindow width in units of TU. PORT2"]
pub type BitCtwndP2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BCNIVLCUNT_P2` reader - Be used in multiple BCNIVL early INT. When this counter =1, multiple BCNIVL early INT will be triggered."]
pub type BitBcnivlcuntP2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CTWindow width in units of TU. PORT1"]
    #[inline(always)]
    pub fn bit_ctwnd_p1(&self) -> BitCtwndP1R {
        BitCtwndP1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Be used in multiple BCNIVL early INT. When this counter =1, multiple BCNIVL early INT will be triggered."]
    #[inline(always)]
    pub fn bit_bcnivlcunt_p1(&self) -> BitBcnivlcuntP1R {
        BitBcnivlcuntP1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - CTWindow width in units of TU. PORT2"]
    #[inline(always)]
    pub fn bit_ctwnd_p2(&self) -> BitCtwndP2R {
        BitCtwndP2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Be used in multiple BCNIVL early INT. When this counter =1, multiple BCNIVL early INT will be triggered."]
    #[inline(always)]
    pub fn bit_bcnivlcunt_p2(&self) -> BitBcnivlcuntP2R {
        BitBcnivlcuntP2R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTWindow width in units of TU. PORT1"]
    #[inline(always)]
    pub fn bit_ctwnd_p1(&mut self) -> BitCtwndP1W<'_, RegBcnErlyCtwCtrlP1_2Spec> {
        BitCtwndP1W::new(self, 0)
    }
    #[doc = "Bits 16:23 - CTWindow width in units of TU. PORT2"]
    #[inline(always)]
    pub fn bit_ctwnd_p2(&mut self) -> BitCtwndP2W<'_, RegBcnErlyCtwCtrlP1_2Spec> {
        BitCtwndP2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_erly_ctw_ctrl_p1_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_erly_ctw_ctrl_p1_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnErlyCtwCtrlP1_2Spec;
impl crate::RegisterSpec for RegBcnErlyCtwCtrlP1_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_erly_ctw_ctrl_p1_2::R`](R) reader structure"]
impl crate::Readable for RegBcnErlyCtwCtrlP1_2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_erly_ctw_ctrl_p1_2::W`](W) writer structure"]
impl crate::Writable for RegBcnErlyCtwCtrlP1_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_ERLY_CTW_CTRL_P1_2 to value 0x7f0a_7f0a"]
impl crate::Resettable for RegBcnErlyCtwCtrlP1_2Spec {
    const RESET_VALUE: u32 = 0x7f0a_7f0a;
}
