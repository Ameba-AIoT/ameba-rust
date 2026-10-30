#[doc = "Register `REG_BACAMCMD` reader"]
pub type R = crate::R<RegBacamcmdSpec>;
#[doc = "Register `REG_BACAMCMD` writer"]
pub type W = crate::W<RegBacamcmdSpec>;
#[doc = "Field `BIT_BACAM_AD` reader - BA CAM address. Memory is double-word access."]
pub type BitBacamAdR = crate::FieldReader;
#[doc = "Field `BIT_BACAM_AD` writer - BA CAM address. Memory is double-word access."]
pub type BitBacamAdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_RXBACAM_DA_EN` reader - CAM Direct Acess enable"]
pub type BitRxbacamDaEnR = crate::BitReader;
#[doc = "Field `BIT_RXBACAM_DA_EN` writer - CAM Direct Acess enable"]
pub type BitRxbacamDaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXSBMPMOD` reader - Bit\\[15\\]: if macrx_dma idle, grant txsbmp_req Bit\\[14\\]: if macrx is idle, grant txsbmp_req 00: If FSM is idle, grant txsbmp_req 01: If FSM and mac rx is idle, grant txsbmp_req 10: If FSM and macrx_dma is idle, grant txsbmp_req 11: if FSM, macrx and macrx_dma is idle, grant txsbmp_req"]
pub type BitTxsbmpmodR = crate::FieldReader;
#[doc = "Field `BIT_TXSBMPMOD` writer - Bit\\[15\\]: if macrx_dma idle, grant txsbmp_req Bit\\[14\\]: if macrx is idle, grant txsbmp_req 00: If FSM is idle, grant txsbmp_req 01: If FSM and mac rx is idle, grant txsbmp_req 10: If FSM and macrx_dma is idle, grant txsbmp_req 11: if FSM, macrx and macrx_dma is idle, grant txsbmp_req"]
pub type BitTxsbmpmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_BACAM_RW` reader - Set 1 = Write, set 0 = Read"]
pub type BitBacamRwR = crate::BitReader;
#[doc = "Field `BIT_BACAM_RW` writer - Set 1 = Write, set 0 = Read"]
pub type BitBacamRwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BACAM_RST` reader - Set 1 to reset BA CAM. Hardware will set the invalid bit for all entries. After reset is finished this bit is reset to 0."]
pub type BitBacamRstR = crate::BitReader;
#[doc = "Field `BIT_BACAM_POLL` writer - Polling bit BA CAM"]
pub type BitBacamPollW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - BA CAM address. Memory is double-word access."]
    #[inline(always)]
    pub fn bit_bacam_ad(&self) -> BitBacamAdR {
        BitBacamAdR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 13 - CAM Direct Acess enable"]
    #[inline(always)]
    pub fn bit_rxbacam_da_en(&self) -> BitRxbacamDaEnR {
        BitRxbacamDaEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Bit\\[15\\]: if macrx_dma idle, grant txsbmp_req Bit\\[14\\]: if macrx is idle, grant txsbmp_req 00: If FSM is idle, grant txsbmp_req 01: If FSM and mac rx is idle, grant txsbmp_req 10: If FSM and macrx_dma is idle, grant txsbmp_req 11: if FSM, macrx and macrx_dma is idle, grant txsbmp_req"]
    #[inline(always)]
    pub fn bit_txsbmpmod(&self) -> BitTxsbmpmodR {
        BitTxsbmpmodR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Set 1 = Write, set 0 = Read"]
    #[inline(always)]
    pub fn bit_bacam_rw(&self) -> BitBacamRwR {
        BitBacamRwR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set 1 to reset BA CAM. Hardware will set the invalid bit for all entries. After reset is finished this bit is reset to 0."]
    #[inline(always)]
    pub fn bit_bacam_rst(&self) -> BitBacamRstR {
        BitBacamRstR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - BA CAM address. Memory is double-word access."]
    #[inline(always)]
    pub fn bit_bacam_ad(&mut self) -> BitBacamAdW<'_, RegBacamcmdSpec> {
        BitBacamAdW::new(self, 0)
    }
    #[doc = "Bit 13 - CAM Direct Acess enable"]
    #[inline(always)]
    pub fn bit_rxbacam_da_en(&mut self) -> BitRxbacamDaEnW<'_, RegBacamcmdSpec> {
        BitRxbacamDaEnW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Bit\\[15\\]: if macrx_dma idle, grant txsbmp_req Bit\\[14\\]: if macrx is idle, grant txsbmp_req 00: If FSM is idle, grant txsbmp_req 01: If FSM and mac rx is idle, grant txsbmp_req 10: If FSM and macrx_dma is idle, grant txsbmp_req 11: if FSM, macrx and macrx_dma is idle, grant txsbmp_req"]
    #[inline(always)]
    pub fn bit_txsbmpmod(&mut self) -> BitTxsbmpmodW<'_, RegBacamcmdSpec> {
        BitTxsbmpmodW::new(self, 14)
    }
    #[doc = "Bit 16 - Set 1 = Write, set 0 = Read"]
    #[inline(always)]
    pub fn bit_bacam_rw(&mut self) -> BitBacamRwW<'_, RegBacamcmdSpec> {
        BitBacamRwW::new(self, 16)
    }
    #[doc = "Bit 31 - Polling bit BA CAM"]
    #[inline(always)]
    pub fn bit_bacam_poll(&mut self) -> BitBacamPollW<'_, RegBacamcmdSpec> {
        BitBacamPollW::new(self, 31)
    }
}
#[doc = "Block ACK CAM Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bacamcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bacamcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBacamcmdSpec;
impl crate::RegisterSpec for RegBacamcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bacamcmd::R`](R) reader structure"]
impl crate::Readable for RegBacamcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bacamcmd::W`](W) writer structure"]
impl crate::Writable for RegBacamcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BACAMCMD to value 0x2000"]
impl crate::Resettable for RegBacamcmdSpec {
    const RESET_VALUE: u32 = 0x2000;
}
