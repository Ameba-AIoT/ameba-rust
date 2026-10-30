#[doc = "Register `REG_AX_RESP_RRSR` reader"]
pub type R = crate::R<RegAxRespRrsrSpec>;
#[doc = "Register `REG_AX_RESP_RRSR` writer"]
pub type W = crate::W<RegAxRespRrsrSpec>;
#[doc = "Field `BIT_rate_sel` reader - New ARFR (REG 0x1484) rate group selection: 0: CCK1M - OFDM18M 1: OFDM24MRSVDOFDM54 , other no use 2: HT MCS0~MCS7 3: HT others, not use 4: VHT1SS/ MCS0~MCS7 5: VHT1SS/ MCS8~MCS9,other no use 6: HE 1SS / MCS0~MCS7 7: HE 1SS / MCS8~MCS9 ,other no use"]
pub type BitRateSelR = crate::FieldReader;
#[doc = "Field `BIT_rate_sel` writer - New ARFR (REG 0x1484) rate group selection: 0: CCK1M - OFDM18M 1: OFDM24MRSVDOFDM54 , other no use 2: HT MCS0~MCS7 3: HT others, not use 4: VHT1SS/ MCS0~MCS7 5: VHT1SS/ MCS8~MCS9,other no use 6: HE 1SS / MCS0~MCS7 7: HE 1SS / MCS8~MCS9 ,other no use"]
pub type BitRateSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_RRSR_RATE_EN` reader - Enabled response rate mode BIT\\[2:0\\] = {(msb) HE rate, VHT rate, HT rate, CCK_OFDM rate}"]
pub type BitRrsrRateEnR = crate::FieldReader;
#[doc = "Field `BIT_RRSR_RATE_EN` writer - Enabled response rate mode BIT\\[2:0\\] = {(msb) HE rate, VHT rate, HT rate, CCK_OFDM rate}"]
pub type BitRrsrRateEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_RRSR_CCK` reader - Response rate BIT\\[3:0\\] = {(msb) CCK11, ..., CCK1}"]
pub type BitRrsrCckR = crate::FieldReader;
#[doc = "Field `BIT_RRSR_CCK` writer - Response rate BIT\\[3:0\\] = {(msb) CCK11, ..., CCK1}"]
pub type BitRrsrCckW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - New ARFR (REG 0x1484) rate group selection: 0: CCK1M - OFDM18M 1: OFDM24MRSVDOFDM54 , other no use 2: HT MCS0~MCS7 3: HT others, not use 4: VHT1SS/ MCS0~MCS7 5: VHT1SS/ MCS8~MCS9,other no use 6: HE 1SS / MCS0~MCS7 7: HE 1SS / MCS8~MCS9 ,other no use"]
    #[inline(always)]
    pub fn bit_rate_sel(&self) -> BitRateSelR {
        BitRateSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - Enabled response rate mode BIT\\[2:0\\] = {(msb) HE rate, VHT rate, HT rate, CCK_OFDM rate}"]
    #[inline(always)]
    pub fn bit_rrsr_rate_en(&self) -> BitRrsrRateEnR {
        BitRrsrRateEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Response rate BIT\\[3:0\\] = {(msb) CCK11, ..., CCK1}"]
    #[inline(always)]
    pub fn bit_rrsr_cck(&self) -> BitRrsrCckR {
        BitRrsrCckR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - New ARFR (REG 0x1484) rate group selection: 0: CCK1M - OFDM18M 1: OFDM24MRSVDOFDM54 , other no use 2: HT MCS0~MCS7 3: HT others, not use 4: VHT1SS/ MCS0~MCS7 5: VHT1SS/ MCS8~MCS9,other no use 6: HE 1SS / MCS0~MCS7 7: HE 1SS / MCS8~MCS9 ,other no use"]
    #[inline(always)]
    pub fn bit_rate_sel(&mut self) -> BitRateSelW<'_, RegAxRespRrsrSpec> {
        BitRateSelW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Enabled response rate mode BIT\\[2:0\\] = {(msb) HE rate, VHT rate, HT rate, CCK_OFDM rate}"]
    #[inline(always)]
    pub fn bit_rrsr_rate_en(&mut self) -> BitRrsrRateEnW<'_, RegAxRespRrsrSpec> {
        BitRrsrRateEnW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Response rate BIT\\[3:0\\] = {(msb) CCK11, ..., CCK1}"]
    #[inline(always)]
    pub fn bit_rrsr_cck(&mut self) -> BitRrsrCckW<'_, RegAxRespRrsrSpec> {
        BitRrsrCckW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_resp_rrsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_resp_rrsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxRespRrsrSpec;
impl crate::RegisterSpec for RegAxRespRrsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_resp_rrsr::R`](R) reader structure"]
impl crate::Readable for RegAxRespRrsrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_resp_rrsr::W`](W) writer structure"]
impl crate::Writable for RegAxRespRrsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_RESP_RRSR to value 0x0005_0100"]
impl crate::Resettable for RegAxRespRrsrSpec {
    const RESET_VALUE: u32 = 0x0005_0100;
}
