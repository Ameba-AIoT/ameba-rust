#[doc = "Register `REG_CH0_CMD2_CTRL` reader"]
pub type R = crate::R<RegCh0Cmd2CtrlSpec>;
#[doc = "Register `REG_CH0_CMD2_CTRL` writer"]
pub type W = crate::W<RegCh0Cmd2CtrlSpec>;
#[doc = "Field `ch0_cmd2_ofst` reader - "]
pub type Ch0Cmd2OfstR = crate::FieldReader<u16>;
#[doc = "Field `ch0_cmd2_ofst` writer - "]
pub type Ch0Cmd2OfstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch0_cmd2_type` reader - "]
pub type Ch0Cmd2TypeR = crate::FieldReader;
#[doc = "Field `ch0_cmd2_type` writer - "]
pub type Ch0Cmd2TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ch0_cmd2_en` reader - "]
pub type Ch0Cmd2EnR = crate::BitReader;
#[doc = "Field `ch0_cmd2_en` writer - "]
pub type Ch0Cmd2EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn ch0_cmd2_ofst(&self) -> Ch0Cmd2OfstR {
        Ch0Cmd2OfstR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn ch0_cmd2_type(&self) -> Ch0Cmd2TypeR {
        Ch0Cmd2TypeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ch0_cmd2_en(&self) -> Ch0Cmd2EnR {
        Ch0Cmd2EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn ch0_cmd2_ofst(&mut self) -> Ch0Cmd2OfstW<'_, RegCh0Cmd2CtrlSpec> {
        Ch0Cmd2OfstW::new(self, 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn ch0_cmd2_type(&mut self) -> Ch0Cmd2TypeW<'_, RegCh0Cmd2CtrlSpec> {
        Ch0Cmd2TypeW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ch0_cmd2_en(&mut self) -> Ch0Cmd2EnW<'_, RegCh0Cmd2CtrlSpec> {
        Ch0Cmd2EnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_cmd2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch0_cmd2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Cmd2CtrlSpec;
impl crate::RegisterSpec for RegCh0Cmd2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_cmd2_ctrl::R`](R) reader structure"]
impl crate::Readable for RegCh0Cmd2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch0_cmd2_ctrl::W`](W) writer structure"]
impl crate::Writable for RegCh0Cmd2CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH0_CMD2_CTRL to value 0"]
impl crate::Resettable for RegCh0Cmd2CtrlSpec {}
