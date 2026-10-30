#[doc = "Register `PGLBK` reader"]
pub type R = crate::R<PglbkSpec>;
#[doc = "Register `PGLBK` writer"]
pub type W = crate::W<PglbkSpec>;
#[doc = "Field `pglbk_data_pat` reader - Pattern inside tx packet when pglbk_data_pat_sel is high. For RLE0516, only pglbk_data_pat\\[3:0\\] are used."]
pub type PglbkDataPatR = crate::FieldReader;
#[doc = "Field `pglbk_data_pat` writer - Pattern inside tx packet when pglbk_data_pat_sel is high. For RLE0516, only pglbk_data_pat\\[3:0\\] are used."]
pub type PglbkDataPatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R_disable_txcrc` reader - 1: not append CRC for egress packet. 0: append CRC for egress packet."]
pub type RDisableTxcrcR = crate::BitReader;
#[doc = "Field `R_disable_txcrc` writer - 1: not append CRC for egress packet. 0: append CRC for egress packet."]
pub type RDisableTxcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pglbk_pkt_num_sel` reader - 2'b00: 100 pkts, 2'b10: 5 pkts. 2'b01: 1 pkt 2'b11: reserved."]
pub type PglbkPktNumSelR = crate::FieldReader;
#[doc = "Field `pglbk_pkt_num_sel` writer - 2'b00: 100 pkts, 2'b10: 5 pkts. 2'b01: 1 pkt 2'b11: reserved."]
pub type PglbkPktNumSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pblbk_pkt_length_sel` reader - 0: random (range: 64-1504B), 1: 64B"]
pub type PblbkPktLengthSelR = crate::BitReader;
#[doc = "Field `pblbk_pkt_length_sel` writer - 0: random (range: 64-1504B), 1: 64B"]
pub type PblbkPktLengthSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pglbk_fail` reader - packet generator in wire lbk mode status. 1: fail"]
pub type PglbkFailR = crate::BitReader;
#[doc = "Field `pglbk_done` reader - packet generator in wire lbk mode status. 1: done"]
pub type PglbkDoneR = crate::BitReader;
#[doc = "Field `pglbk_data_pat_sel` reader - 1: fixed pattern, 0: pseudo random"]
pub type PglbkDataPatSelR = crate::BitReader;
#[doc = "Field `pglbk_data_pat_sel` writer - 1: fixed pattern, 0: pseudo random"]
pub type PglbkDataPatSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_pglbk` reader - Enable packet generator mode"]
pub type EnPglbkR = crate::BitReader;
#[doc = "Field `en_pglbk` writer - Enable packet generator mode"]
pub type EnPglbkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Pattern inside tx packet when pglbk_data_pat_sel is high. For RLE0516, only pglbk_data_pat\\[3:0\\] are used."]
    #[inline(always)]
    pub fn pglbk_data_pat(&self) -> PglbkDataPatR {
        PglbkDataPatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1: not append CRC for egress packet. 0: append CRC for egress packet."]
    #[inline(always)]
    pub fn r_disable_txcrc(&self) -> RDisableTxcrcR {
        RDisableTxcrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - 2'b00: 100 pkts, 2'b10: 5 pkts. 2'b01: 1 pkt 2'b11: reserved."]
    #[inline(always)]
    pub fn pglbk_pkt_num_sel(&self) -> PglbkPktNumSelR {
        PglbkPktNumSelR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - 0: random (range: 64-1504B), 1: 64B"]
    #[inline(always)]
    pub fn pblbk_pkt_length_sel(&self) -> PblbkPktLengthSelR {
        PblbkPktLengthSelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - packet generator in wire lbk mode status. 1: fail"]
    #[inline(always)]
    pub fn pglbk_fail(&self) -> PglbkFailR {
        PglbkFailR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - packet generator in wire lbk mode status. 1: done"]
    #[inline(always)]
    pub fn pglbk_done(&self) -> PglbkDoneR {
        PglbkDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: fixed pattern, 0: pseudo random"]
    #[inline(always)]
    pub fn pglbk_data_pat_sel(&self) -> PglbkDataPatSelR {
        PglbkDataPatSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable packet generator mode"]
    #[inline(always)]
    pub fn en_pglbk(&self) -> EnPglbkR {
        EnPglbkR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pattern inside tx packet when pglbk_data_pat_sel is high. For RLE0516, only pglbk_data_pat\\[3:0\\] are used."]
    #[inline(always)]
    pub fn pglbk_data_pat(&mut self) -> PglbkDataPatW<'_, PglbkSpec> {
        PglbkDataPatW::new(self, 0)
    }
    #[doc = "Bit 8 - 1: not append CRC for egress packet. 0: append CRC for egress packet."]
    #[inline(always)]
    pub fn r_disable_txcrc(&mut self) -> RDisableTxcrcW<'_, PglbkSpec> {
        RDisableTxcrcW::new(self, 8)
    }
    #[doc = "Bits 9:10 - 2'b00: 100 pkts, 2'b10: 5 pkts. 2'b01: 1 pkt 2'b11: reserved."]
    #[inline(always)]
    pub fn pglbk_pkt_num_sel(&mut self) -> PglbkPktNumSelW<'_, PglbkSpec> {
        PglbkPktNumSelW::new(self, 9)
    }
    #[doc = "Bit 11 - 0: random (range: 64-1504B), 1: 64B"]
    #[inline(always)]
    pub fn pblbk_pkt_length_sel(&mut self) -> PblbkPktLengthSelW<'_, PglbkSpec> {
        PblbkPktLengthSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 1: fixed pattern, 0: pseudo random"]
    #[inline(always)]
    pub fn pglbk_data_pat_sel(&mut self) -> PglbkDataPatSelW<'_, PglbkSpec> {
        PglbkDataPatSelW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable packet generator mode"]
    #[inline(always)]
    pub fn en_pglbk(&mut self) -> EnPglbkW<'_, PglbkSpec> {
        EnPglbkW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pglbk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pglbk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PglbkSpec;
impl crate::RegisterSpec for PglbkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pglbk::R`](R) reader structure"]
impl crate::Readable for PglbkSpec {}
#[doc = "`write(|w| ..)` method takes [`pglbk::W`](W) writer structure"]
impl crate::Writable for PglbkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PGLBK to value 0"]
impl crate::Resettable for PglbkSpec {}
