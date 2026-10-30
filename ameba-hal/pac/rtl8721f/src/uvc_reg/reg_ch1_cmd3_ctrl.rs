#[doc = "Register `REG_CH1_CMD3_CTRL` reader"]
pub type R = crate::R<RegCh1Cmd3CtrlSpec>;
#[doc = "Register `REG_CH1_CMD3_CTRL` writer"]
pub type W = crate::W<RegCh1Cmd3CtrlSpec>;
#[doc = "Field `ch1_cmd3_ofst` reader - "]
pub type Ch1Cmd3OfstR = crate::FieldReader<u16>;
#[doc = "Field `ch1_cmd3_ofst` writer - "]
pub type Ch1Cmd3OfstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch1_cmd3_type` reader - "]
pub type Ch1Cmd3TypeR = crate::FieldReader;
#[doc = "Field `ch1_cmd3_type` writer - "]
pub type Ch1Cmd3TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ch1_cmd3_en` reader - "]
pub type Ch1Cmd3EnR = crate::BitReader;
#[doc = "Field `ch1_cmd3_en` writer - "]
pub type Ch1Cmd3EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn ch1_cmd3_ofst(&self) -> Ch1Cmd3OfstR {
        Ch1Cmd3OfstR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn ch1_cmd3_type(&self) -> Ch1Cmd3TypeR {
        Ch1Cmd3TypeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ch1_cmd3_en(&self) -> Ch1Cmd3EnR {
        Ch1Cmd3EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn ch1_cmd3_ofst(&mut self) -> Ch1Cmd3OfstW<'_, RegCh1Cmd3CtrlSpec> {
        Ch1Cmd3OfstW::new(self, 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn ch1_cmd3_type(&mut self) -> Ch1Cmd3TypeW<'_, RegCh1Cmd3CtrlSpec> {
        Ch1Cmd3TypeW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ch1_cmd3_en(&mut self) -> Ch1Cmd3EnW<'_, RegCh1Cmd3CtrlSpec> {
        Ch1Cmd3EnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch1_cmd3_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch1_cmd3_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh1Cmd3CtrlSpec;
impl crate::RegisterSpec for RegCh1Cmd3CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch1_cmd3_ctrl::R`](R) reader structure"]
impl crate::Readable for RegCh1Cmd3CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch1_cmd3_ctrl::W`](W) writer structure"]
impl crate::Writable for RegCh1Cmd3CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH1_CMD3_CTRL to value 0"]
impl crate::Resettable for RegCh1Cmd3CtrlSpec {}
