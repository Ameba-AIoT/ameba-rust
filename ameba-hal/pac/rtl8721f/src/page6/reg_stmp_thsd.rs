#[doc = "Register `REG_STMP_THSD` reader"]
pub type R = crate::R<RegStmpThsdSpec>;
#[doc = "Register `REG_STMP_THSD` writer"]
pub type W = crate::W<RegStmpThsdSpec>;
#[doc = "Field `BIT_STMP_THSD` reader - when PHY_rdptr is greater than this value, then latch STMP value from TSFT"]
pub type BitStmpThsdR = crate::FieldReader;
#[doc = "Field `BIT_STMP_THSD` writer - when PHY_rdptr is greater than this value, then latch STMP value from TSFT"]
pub type BitStmpThsdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_WMAC_TCR_E_TSFT_OFS` reader - BIT_WMAC_TCR_TSFT_OFS\\[7:0\\] is for ht/vht/he stamp time offset;"]
pub type BitWmacTcrETsftOfsR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_TCR_E_TSFT_OFS` writer - BIT_WMAC_TCR_TSFT_OFS\\[7:0\\] is for ht/vht/he stamp time offset;"]
pub type BitWmacTcrETsftOfsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - when PHY_rdptr is greater than this value, then latch STMP value from TSFT"]
    #[inline(always)]
    pub fn bit_stmp_thsd(&self) -> BitStmpThsdR {
        BitStmpThsdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - BIT_WMAC_TCR_TSFT_OFS\\[7:0\\] is for ht/vht/he stamp time offset;"]
    #[inline(always)]
    pub fn bit_wmac_tcr_e_tsft_ofs(&self) -> BitWmacTcrETsftOfsR {
        BitWmacTcrETsftOfsR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - when PHY_rdptr is greater than this value, then latch STMP value from TSFT"]
    #[inline(always)]
    pub fn bit_stmp_thsd(&mut self) -> BitStmpThsdW<'_, RegStmpThsdSpec> {
        BitStmpThsdW::new(self, 0)
    }
    #[doc = "Bits 8:15 - BIT_WMAC_TCR_TSFT_OFS\\[7:0\\] is for ht/vht/he stamp time offset;"]
    #[inline(always)]
    pub fn bit_wmac_tcr_e_tsft_ofs(&mut self) -> BitWmacTcrETsftOfsW<'_, RegStmpThsdSpec> {
        BitWmacTcrETsftOfsW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_stmp_thsd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_stmp_thsd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegStmpThsdSpec;
impl crate::RegisterSpec for RegStmpThsdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_stmp_thsd::R`](R) reader structure"]
impl crate::Readable for RegStmpThsdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_stmp_thsd::W`](W) writer structure"]
impl crate::Writable for RegStmpThsdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_STMP_THSD to value 0x0802"]
impl crate::Resettable for RegStmpThsdSpec {
    const RESET_VALUE: u32 = 0x0802;
}
