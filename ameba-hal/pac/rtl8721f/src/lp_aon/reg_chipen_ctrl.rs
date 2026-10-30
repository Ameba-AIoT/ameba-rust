#[doc = "Register `REG_CHIPEN_CTRL` reader"]
pub type R = crate::R<RegChipenCtrlSpec>;
#[doc = "Register `REG_CHIPEN_CTRL` writer"]
pub type W = crate::W<RegChipenCtrlSpec>;
#[doc = "Field `chipen_tdbc` reader - 000: 0us 001: 100us 010: 500us 011: 1000us 100: 2000us 101: 4000us 110: 8000us 111: 16000us"]
pub type ChipenTdbcR = crate::FieldReader;
#[doc = "Field `chipen_tdbc` writer - 000: 0us 001: 100us 010: 500us 011: 1000us 100: 2000us 101: 4000us 110: 8000us 111: 16000us"]
pub type ChipenTdbcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `chipen_tsp` reader - 000: 0ms 001: 50ms 010: 100ms 011: 150ms 100: 200ms 101: 250ms 110: 300ms 111: 350ms"]
pub type ChipenTspR = crate::FieldReader;
#[doc = "Field `chipen_tsp` writer - 000: 0ms 001: 50ms 010: 100ms 011: 150ms 100: 200ms 101: 250ms 110: 300ms 111: 350ms"]
pub type ChipenTspW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `chipen_tlp` reader - 000: 1000ms 001: 1500ms 010: 2000ms 011: 2500ms 100: 3000ms 101: 3500ms 110: 4000ms 111: 4500ms"]
pub type ChipenTlpR = crate::FieldReader;
#[doc = "Field `chipen_tlp` writer - 000: 1000ms 001: 1500ms 010: 2000ms 011: 2500ms 100: 3000ms 101: 3500ms 110: 4000ms 111: 4500ms"]
pub type ChipenTlpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `chipen_tack` reader - 00: 50ms 01: 100ms 10: 200ms 11: 400ms"]
pub type ChipenTackR = crate::FieldReader;
#[doc = "Field `chipen_tack` writer - 00: 50ms 01: 100ms 10: 200ms 11: 400ms"]
pub type ChipenTackW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `chipen_intr_mode` reader - Chipen operation mode. 00: reset mode. When chipen pad low, hw will reset all chip 01: interrup mode. It will enable short/long press function. 1X: pulse reset mode. The transation from other value to 2'd2 will gen a posedge to set RST_PULSE_EN in HV domain. Once in pulse reset mode , this field can't be modified again , and always read out 2'b11"]
pub type ChipenIntrModeR = crate::FieldReader;
#[doc = "Field `chipen_intr_mode` writer - Chipen operation mode. 00: reset mode. When chipen pad low, hw will reset all chip 01: interrup mode. It will enable short/long press function. 1X: pulse reset mode. The transation from other value to 2'd2 will gen a posedge to set RST_PULSE_EN in HV domain. Once in pulse reset mode , this field can't be modified again , and always read out 2'b11"]
pub type ChipenIntrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `chipen_st` reader - CHIP_EN pin status 0: logic '0' on CHIP_EN pin 1: logic '1' on CHIP_EN pin"]
pub type ChipenStR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - 000: 0us 001: 100us 010: 500us 011: 1000us 100: 2000us 101: 4000us 110: 8000us 111: 16000us"]
    #[inline(always)]
    pub fn chipen_tdbc(&self) -> ChipenTdbcR {
        ChipenTdbcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 000: 0ms 001: 50ms 010: 100ms 011: 150ms 100: 200ms 101: 250ms 110: 300ms 111: 350ms"]
    #[inline(always)]
    pub fn chipen_tsp(&self) -> ChipenTspR {
        ChipenTspR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 000: 1000ms 001: 1500ms 010: 2000ms 011: 2500ms 100: 3000ms 101: 3500ms 110: 4000ms 111: 4500ms"]
    #[inline(always)]
    pub fn chipen_tlp(&self) -> ChipenTlpR {
        ChipenTlpR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - 00: 50ms 01: 100ms 10: 200ms 11: 400ms"]
    #[inline(always)]
    pub fn chipen_tack(&self) -> ChipenTackR {
        ChipenTackR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Chipen operation mode. 00: reset mode. When chipen pad low, hw will reset all chip 01: interrup mode. It will enable short/long press function. 1X: pulse reset mode. The transation from other value to 2'd2 will gen a posedge to set RST_PULSE_EN in HV domain. Once in pulse reset mode , this field can't be modified again , and always read out 2'b11"]
    #[inline(always)]
    pub fn chipen_intr_mode(&self) -> ChipenIntrModeR {
        ChipenIntrModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - CHIP_EN pin status 0: logic '0' on CHIP_EN pin 1: logic '1' on CHIP_EN pin"]
    #[inline(always)]
    pub fn chipen_st(&self) -> ChipenStR {
        ChipenStR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 000: 0us 001: 100us 010: 500us 011: 1000us 100: 2000us 101: 4000us 110: 8000us 111: 16000us"]
    #[inline(always)]
    pub fn chipen_tdbc(&mut self) -> ChipenTdbcW<'_, RegChipenCtrlSpec> {
        ChipenTdbcW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 000: 0ms 001: 50ms 010: 100ms 011: 150ms 100: 200ms 101: 250ms 110: 300ms 111: 350ms"]
    #[inline(always)]
    pub fn chipen_tsp(&mut self) -> ChipenTspW<'_, RegChipenCtrlSpec> {
        ChipenTspW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 000: 1000ms 001: 1500ms 010: 2000ms 011: 2500ms 100: 3000ms 101: 3500ms 110: 4000ms 111: 4500ms"]
    #[inline(always)]
    pub fn chipen_tlp(&mut self) -> ChipenTlpW<'_, RegChipenCtrlSpec> {
        ChipenTlpW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 00: 50ms 01: 100ms 10: 200ms 11: 400ms"]
    #[inline(always)]
    pub fn chipen_tack(&mut self) -> ChipenTackW<'_, RegChipenCtrlSpec> {
        ChipenTackW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Chipen operation mode. 00: reset mode. When chipen pad low, hw will reset all chip 01: interrup mode. It will enable short/long press function. 1X: pulse reset mode. The transation from other value to 2'd2 will gen a posedge to set RST_PULSE_EN in HV domain. Once in pulse reset mode , this field can't be modified again , and always read out 2'b11"]
    #[inline(always)]
    pub fn chipen_intr_mode(&mut self) -> ChipenIntrModeW<'_, RegChipenCtrlSpec> {
        ChipenIntrModeW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_chipen_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_chipen_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegChipenCtrlSpec;
impl crate::RegisterSpec for RegChipenCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_chipen_ctrl::R`](R) reader structure"]
impl crate::Readable for RegChipenCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_chipen_ctrl::W`](W) writer structure"]
impl crate::Writable for RegChipenCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CHIPEN_CTRL to value 0x01"]
impl crate::Resettable for RegChipenCtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
