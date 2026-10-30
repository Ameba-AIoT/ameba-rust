#[doc = "Register `REG_OFDM_CCK_LEN_MASK` reader"]
pub type R = crate::R<RegOfdmCckLenMaskSpec>;
#[doc = "Register `REG_OFDM_CCK_LEN_MASK` writer"]
pub type W = crate::W<RegOfdmCckLenMaskSpec>;
#[doc = "Field `BIT_RX_CCK_LEN_TH` reader - rx cck len over this length to drop pkt"]
pub type BitRxCckLenThR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RX_CCK_LEN_TH` writer - rx cck len over this length to drop pkt"]
pub type BitRxCckLenThW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_RX_OFDM_LEN_TH` reader - rx ofdm len over this len to drop pkt(not for ht/vht)"]
pub type BitRxOfdmLenThR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RX_OFDM_LEN_TH` writer - rx ofdm len over this len to drop pkt(not for ht/vht)"]
pub type BitRxOfdmLenThW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - rx cck len over this length to drop pkt"]
    #[inline(always)]
    pub fn bit_rx_cck_len_th(&self) -> BitRxCckLenThR {
        BitRxCckLenThR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - rx ofdm len over this len to drop pkt(not for ht/vht)"]
    #[inline(always)]
    pub fn bit_rx_ofdm_len_th(&self) -> BitRxOfdmLenThR {
        BitRxOfdmLenThR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - rx cck len over this length to drop pkt"]
    #[inline(always)]
    pub fn bit_rx_cck_len_th(&mut self) -> BitRxCckLenThW<'_, RegOfdmCckLenMaskSpec> {
        BitRxCckLenThW::new(self, 0)
    }
    #[doc = "Bits 16:31 - rx ofdm len over this len to drop pkt(not for ht/vht)"]
    #[inline(always)]
    pub fn bit_rx_ofdm_len_th(&mut self) -> BitRxOfdmLenThW<'_, RegOfdmCckLenMaskSpec> {
        BitRxOfdmLenThW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ofdm_cck_len_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ofdm_cck_len_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegOfdmCckLenMaskSpec;
impl crate::RegisterSpec for RegOfdmCckLenMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ofdm_cck_len_mask::R`](R) reader structure"]
impl crate::Readable for RegOfdmCckLenMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ofdm_cck_len_mask::W`](W) writer structure"]
impl crate::Writable for RegOfdmCckLenMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_OFDM_CCK_LEN_MASK to value 0x0fff_0fff"]
impl crate::Resettable for RegOfdmCckLenMaskSpec {
    const RESET_VALUE: u32 = 0x0fff_0fff;
}
