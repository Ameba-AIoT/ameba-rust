#[doc = "Register `REG_I2S_CTRL` reader"]
pub type R = crate::R<RegI2sCtrlSpec>;
#[doc = "Register `REG_I2S_CTRL` writer"]
pub type W = crate::W<RegI2sCtrlSpec>;
#[doc = "Field `SP0_DIO0_MUXSEL` reader - SPORT0 DIO0 function mux 0: DIN_0 func 1: DOUT_3 func"]
pub type Sp0Dio0MuxselR = crate::BitReader;
#[doc = "Field `SP0_DIO0_MUXSEL` writer - SPORT0 DIO0 function mux 0: DIN_0 func 1: DOUT_3 func"]
pub type Sp0Dio0MuxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP0_DIO1_MUXSEL` reader - SPORT0 DIO1 function mux 0: DIN_1 func 1: DOUT_2 func"]
pub type Sp0Dio1MuxselR = crate::BitReader;
#[doc = "Field `SP0_DIO1_MUXSEL` writer - SPORT0 DIO1 function mux 0: DIN_1 func 1: DOUT_2 func"]
pub type Sp0Dio1MuxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP0_DIO2_MUXSEL` reader - SPORT0 DIO2 function mux 0: DIN_2 func 1: DOUT_1 func"]
pub type Sp0Dio2MuxselR = crate::BitReader;
#[doc = "Field `SP0_DIO2_MUXSEL` writer - SPORT0 DIO2 function mux 0: DIN_2 func 1: DOUT_1 func"]
pub type Sp0Dio2MuxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP0_DIO3_MUXSEL` reader - SPORT0 DIO3 function mux 0: DIN_3 func 1: DOUT_0 func"]
pub type Sp0Dio3MuxselR = crate::BitReader;
#[doc = "Field `SP0_DIO3_MUXSEL` writer - SPORT0 DIO3 function mux 0: DIN_3 func 1: DOUT_0 func"]
pub type Sp0Dio3MuxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP1_DIO0_MUXSEL` reader - SPORT1 DIO0 function mux 0: DIN_0 func 1: DOUT_3 func"]
pub type Sp1Dio0MuxselR = crate::BitReader;
#[doc = "Field `SP1_DIO0_MUXSEL` writer - SPORT1 DIO0 function mux 0: DIN_0 func 1: DOUT_3 func"]
pub type Sp1Dio0MuxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP1_DIO1_MUXSEL` reader - SPORT1 DIO1 function mux 0: DIN_1 func 1: DOUT_2 func"]
pub type Sp1Dio1MuxselR = crate::BitReader;
#[doc = "Field `SP1_DIO1_MUXSEL` writer - SPORT1 DIO1 function mux 0: DIN_1 func 1: DOUT_2 func"]
pub type Sp1Dio1MuxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP1_DIO2_MUXSEL` reader - SPORT1 DIO2 function mux 0: DIN_2 func 1: DOUT_1 func"]
pub type Sp1Dio2MuxselR = crate::BitReader;
#[doc = "Field `SP1_DIO2_MUXSEL` writer - SPORT1 DIO2 function mux 0: DIN_2 func 1: DOUT_1 func"]
pub type Sp1Dio2MuxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP1_DIO3_MUXSEL` reader - SPORT1 DIO3 function mux 0: DIN_3 func 1: DOUT_0 func"]
pub type Sp1Dio3MuxselR = crate::BitReader;
#[doc = "Field `SP1_DIO3_MUXSEL` writer - SPORT1 DIO3 function mux 0: DIN_3 func 1: DOUT_0 func"]
pub type Sp1Dio3MuxselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPORT0 DIO0 function mux 0: DIN_0 func 1: DOUT_3 func"]
    #[inline(always)]
    pub fn sp0_dio0_muxsel(&self) -> Sp0Dio0MuxselR {
        Sp0Dio0MuxselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPORT0 DIO1 function mux 0: DIN_1 func 1: DOUT_2 func"]
    #[inline(always)]
    pub fn sp0_dio1_muxsel(&self) -> Sp0Dio1MuxselR {
        Sp0Dio1MuxselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPORT0 DIO2 function mux 0: DIN_2 func 1: DOUT_1 func"]
    #[inline(always)]
    pub fn sp0_dio2_muxsel(&self) -> Sp0Dio2MuxselR {
        Sp0Dio2MuxselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPORT0 DIO3 function mux 0: DIN_3 func 1: DOUT_0 func"]
    #[inline(always)]
    pub fn sp0_dio3_muxsel(&self) -> Sp0Dio3MuxselR {
        Sp0Dio3MuxselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPORT1 DIO0 function mux 0: DIN_0 func 1: DOUT_3 func"]
    #[inline(always)]
    pub fn sp1_dio0_muxsel(&self) -> Sp1Dio0MuxselR {
        Sp1Dio0MuxselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPORT1 DIO1 function mux 0: DIN_1 func 1: DOUT_2 func"]
    #[inline(always)]
    pub fn sp1_dio1_muxsel(&self) -> Sp1Dio1MuxselR {
        Sp1Dio1MuxselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPORT1 DIO2 function mux 0: DIN_2 func 1: DOUT_1 func"]
    #[inline(always)]
    pub fn sp1_dio2_muxsel(&self) -> Sp1Dio2MuxselR {
        Sp1Dio2MuxselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPORT1 DIO3 function mux 0: DIN_3 func 1: DOUT_0 func"]
    #[inline(always)]
    pub fn sp1_dio3_muxsel(&self) -> Sp1Dio3MuxselR {
        Sp1Dio3MuxselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPORT0 DIO0 function mux 0: DIN_0 func 1: DOUT_3 func"]
    #[inline(always)]
    pub fn sp0_dio0_muxsel(&mut self) -> Sp0Dio0MuxselW<'_, RegI2sCtrlSpec> {
        Sp0Dio0MuxselW::new(self, 0)
    }
    #[doc = "Bit 1 - SPORT0 DIO1 function mux 0: DIN_1 func 1: DOUT_2 func"]
    #[inline(always)]
    pub fn sp0_dio1_muxsel(&mut self) -> Sp0Dio1MuxselW<'_, RegI2sCtrlSpec> {
        Sp0Dio1MuxselW::new(self, 1)
    }
    #[doc = "Bit 2 - SPORT0 DIO2 function mux 0: DIN_2 func 1: DOUT_1 func"]
    #[inline(always)]
    pub fn sp0_dio2_muxsel(&mut self) -> Sp0Dio2MuxselW<'_, RegI2sCtrlSpec> {
        Sp0Dio2MuxselW::new(self, 2)
    }
    #[doc = "Bit 3 - SPORT0 DIO3 function mux 0: DIN_3 func 1: DOUT_0 func"]
    #[inline(always)]
    pub fn sp0_dio3_muxsel(&mut self) -> Sp0Dio3MuxselW<'_, RegI2sCtrlSpec> {
        Sp0Dio3MuxselW::new(self, 3)
    }
    #[doc = "Bit 4 - SPORT1 DIO0 function mux 0: DIN_0 func 1: DOUT_3 func"]
    #[inline(always)]
    pub fn sp1_dio0_muxsel(&mut self) -> Sp1Dio0MuxselW<'_, RegI2sCtrlSpec> {
        Sp1Dio0MuxselW::new(self, 4)
    }
    #[doc = "Bit 5 - SPORT1 DIO1 function mux 0: DIN_1 func 1: DOUT_2 func"]
    #[inline(always)]
    pub fn sp1_dio1_muxsel(&mut self) -> Sp1Dio1MuxselW<'_, RegI2sCtrlSpec> {
        Sp1Dio1MuxselW::new(self, 5)
    }
    #[doc = "Bit 6 - SPORT1 DIO2 function mux 0: DIN_2 func 1: DOUT_1 func"]
    #[inline(always)]
    pub fn sp1_dio2_muxsel(&mut self) -> Sp1Dio2MuxselW<'_, RegI2sCtrlSpec> {
        Sp1Dio2MuxselW::new(self, 6)
    }
    #[doc = "Bit 7 - SPORT1 DIO3 function mux 0: DIN_3 func 1: DOUT_0 func"]
    #[inline(always)]
    pub fn sp1_dio3_muxsel(&mut self) -> Sp1Dio3MuxselW<'_, RegI2sCtrlSpec> {
        Sp1Dio3MuxselW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_i2s_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_i2s_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegI2sCtrlSpec;
impl crate::RegisterSpec for RegI2sCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_i2s_ctrl::R`](R) reader structure"]
impl crate::Readable for RegI2sCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_i2s_ctrl::W`](W) writer structure"]
impl crate::Writable for RegI2sCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_I2S_CTRL to value 0"]
impl crate::Resettable for RegI2sCtrlSpec {}
