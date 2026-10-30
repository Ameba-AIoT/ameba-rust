#[doc = "Register `REG_CH0_CMD0_CTRL` reader"]
pub type R = crate::R<RegCh0Cmd0CtrlSpec>;
#[doc = "Register `REG_CH0_CMD0_CTRL` writer"]
pub type W = crate::W<RegCh0Cmd0CtrlSpec>;
#[doc = "Field `ch0_cmd0_ofst` reader - "]
pub type Ch0Cmd0OfstR = crate::FieldReader<u16>;
#[doc = "Field `ch0_cmd0_ofst` writer - "]
pub type Ch0Cmd0OfstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch0_cmd0_type` reader - "]
pub type Ch0Cmd0TypeR = crate::FieldReader;
#[doc = "Field `ch0_cmd0_type` writer - "]
pub type Ch0Cmd0TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ch0_cmd0_en` reader - "]
pub type Ch0Cmd0EnR = crate::BitReader;
#[doc = "Field `ch0_cmd0_en` writer - "]
pub type Ch0Cmd0EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn ch0_cmd0_ofst(&self) -> Ch0Cmd0OfstR {
        Ch0Cmd0OfstR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn ch0_cmd0_type(&self) -> Ch0Cmd0TypeR {
        Ch0Cmd0TypeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ch0_cmd0_en(&self) -> Ch0Cmd0EnR {
        Ch0Cmd0EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn ch0_cmd0_ofst(&mut self) -> Ch0Cmd0OfstW<'_, RegCh0Cmd0CtrlSpec> {
        Ch0Cmd0OfstW::new(self, 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn ch0_cmd0_type(&mut self) -> Ch0Cmd0TypeW<'_, RegCh0Cmd0CtrlSpec> {
        Ch0Cmd0TypeW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ch0_cmd0_en(&mut self) -> Ch0Cmd0EnW<'_, RegCh0Cmd0CtrlSpec> {
        Ch0Cmd0EnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_cmd0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch0_cmd0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Cmd0CtrlSpec;
impl crate::RegisterSpec for RegCh0Cmd0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_cmd0_ctrl::R`](R) reader structure"]
impl crate::Readable for RegCh0Cmd0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch0_cmd0_ctrl::W`](W) writer structure"]
impl crate::Writable for RegCh0Cmd0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH0_CMD0_CTRL to value 0"]
impl crate::Resettable for RegCh0Cmd0CtrlSpec {}
