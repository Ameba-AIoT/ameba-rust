#[doc = "Register `REG_CH0_CMD1_CTRL` reader"]
pub type R = crate::R<RegCh0Cmd1CtrlSpec>;
#[doc = "Register `REG_CH0_CMD1_CTRL` writer"]
pub type W = crate::W<RegCh0Cmd1CtrlSpec>;
#[doc = "Field `ch0_cmd1_ofst` reader - "]
pub type Ch0Cmd1OfstR = crate::FieldReader<u16>;
#[doc = "Field `ch0_cmd1_ofst` writer - "]
pub type Ch0Cmd1OfstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch0_cmd1_type` reader - "]
pub type Ch0Cmd1TypeR = crate::FieldReader;
#[doc = "Field `ch0_cmd1_type` writer - "]
pub type Ch0Cmd1TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ch0_cmd1_en` reader - "]
pub type Ch0Cmd1EnR = crate::BitReader;
#[doc = "Field `ch0_cmd1_en` writer - "]
pub type Ch0Cmd1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn ch0_cmd1_ofst(&self) -> Ch0Cmd1OfstR {
        Ch0Cmd1OfstR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn ch0_cmd1_type(&self) -> Ch0Cmd1TypeR {
        Ch0Cmd1TypeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ch0_cmd1_en(&self) -> Ch0Cmd1EnR {
        Ch0Cmd1EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn ch0_cmd1_ofst(&mut self) -> Ch0Cmd1OfstW<'_, RegCh0Cmd1CtrlSpec> {
        Ch0Cmd1OfstW::new(self, 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn ch0_cmd1_type(&mut self) -> Ch0Cmd1TypeW<'_, RegCh0Cmd1CtrlSpec> {
        Ch0Cmd1TypeW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ch0_cmd1_en(&mut self) -> Ch0Cmd1EnW<'_, RegCh0Cmd1CtrlSpec> {
        Ch0Cmd1EnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_cmd1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch0_cmd1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Cmd1CtrlSpec;
impl crate::RegisterSpec for RegCh0Cmd1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_cmd1_ctrl::R`](R) reader structure"]
impl crate::Readable for RegCh0Cmd1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch0_cmd1_ctrl::W`](W) writer structure"]
impl crate::Writable for RegCh0Cmd1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH0_CMD1_CTRL to value 0"]
impl crate::Resettable for RegCh0Cmd1CtrlSpec {}
