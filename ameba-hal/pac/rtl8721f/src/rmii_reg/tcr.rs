#[doc = "Register `TCR` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Field `LBK` reader - The loopback function must be independent of the link state. 2'b00 : normal operation 2'b01 : Loopback mode_R2T(lbkmode_r2t) : Lbk methld: no SW requirement. 1. receive ingress data. 2. assign rxd to txd (txd). 3. egress pkt. b10 : Reserved 2'b11 : Loopback mode(lbk_mode). Lbk method: 1. rx fet descriptor. 2. tx fet descriptor and data 3. put data in txfifo 4. assign txd to rxd.(lbk) 5. gmac mv data to lxb. In this mode(lbk_mode), the recommended value of RC_REG(0x44) is 0xf."]
pub type LbkR = crate::FieldReader;
#[doc = "Field `LBK` writer - The loopback function must be independent of the link state. 2'b00 : normal operation 2'b01 : Loopback mode_R2T(lbkmode_r2t) : Lbk methld: no SW requirement. 1. receive ingress data. 2. assign rxd to txd (txd). 3. egress pkt. b10 : Reserved 2'b11 : Loopback mode(lbk_mode). Lbk method: 1. rx fet descriptor. 2. tx fet descriptor and data 3. put data in txfifo 4. assign txd to rxd.(lbk) 5. gmac mv data to lxb. In this mode(lbk_mode), the recommended value of RC_REG(0x44) is 0xf."]
pub type LbkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IFG2_0` reader - InterFrameGap Time: This field allows the user to adjust the interframe gap time longer than the standard: 9.6 us for 10Mbps, 960 ns for 100Mbps. The time can be programmed from 9.6 us to 14.4 us (10Mbps) and 960ns to 1440ns (100Mbps). The formula for the inter frame gap is: IFG\\[2:0\\] IFG@1000MHz (nS) IFG@100MHz (nS) IFG@10MHz (uS) 0 1 1 96 960 9.6 1 0 0 96 +8 * 1 960 +8 * 10 9.6 +8 * 0.1 1 0 1 96 + 16 * 1 960 + 16 * 10 9.6 + 16 * 0.1 1 1 0 96 + 24 * 1 960 + 24 * 10 9.6 + 24 * 0.1 1 1 1 96 + 32 * 1 960 + 32 * 10 9.6 + 32 * 0.1 0 0 0 96 + 40 * 1 960 + 40 * 10 9.6 + 40 * 0.1 0 0 1 96 + 48 * 1 960 + 48 * 10 9.6 + 48 * 0.1 0 1 0 96 + 96 * 1 960 + 96 * 01 9.6 + 96 * 0.1"]
pub type Ifg2_0R = crate::FieldReader;
#[doc = "Field `IFG2_0` writer - InterFrameGap Time: This field allows the user to adjust the interframe gap time longer than the standard: 9.6 us for 10Mbps, 960 ns for 100Mbps. The time can be programmed from 9.6 us to 14.4 us (10Mbps) and 960ns to 1440ns (100Mbps). The formula for the inter frame gap is: IFG\\[2:0\\] IFG@1000MHz (nS) IFG@100MHz (nS) IFG@10MHz (uS) 0 1 1 96 960 9.6 1 0 0 96 +8 * 1 960 +8 * 10 9.6 +8 * 0.1 1 0 1 96 + 16 * 1 960 + 16 * 10 9.6 + 16 * 0.1 1 1 0 96 + 24 * 1 960 + 24 * 10 9.6 + 24 * 0.1 1 1 1 96 + 32 * 1 960 + 32 * 10 9.6 + 32 * 0.1 0 0 0 96 + 40 * 1 960 + 40 * 10 9.6 + 40 * 0.1 0 0 1 96 + 48 * 1 960 + 48 * 10 9.6 + 48 * 0.1 0 1 0 96 + 96 * 1 960 + 96 * 01 9.6 + 96 * 0.1"]
pub type Ifg2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 8:9 - The loopback function must be independent of the link state. 2'b00 : normal operation 2'b01 : Loopback mode_R2T(lbkmode_r2t) : Lbk methld: no SW requirement. 1. receive ingress data. 2. assign rxd to txd (txd). 3. egress pkt. b10 : Reserved 2'b11 : Loopback mode(lbk_mode). Lbk method: 1. rx fet descriptor. 2. tx fet descriptor and data 3. put data in txfifo 4. assign txd to rxd.(lbk) 5. gmac mv data to lxb. In this mode(lbk_mode), the recommended value of RC_REG(0x44) is 0xf."]
    #[inline(always)]
    pub fn lbk(&self) -> LbkR {
        LbkR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - InterFrameGap Time: This field allows the user to adjust the interframe gap time longer than the standard: 9.6 us for 10Mbps, 960 ns for 100Mbps. The time can be programmed from 9.6 us to 14.4 us (10Mbps) and 960ns to 1440ns (100Mbps). The formula for the inter frame gap is: IFG\\[2:0\\] IFG@1000MHz (nS) IFG@100MHz (nS) IFG@10MHz (uS) 0 1 1 96 960 9.6 1 0 0 96 +8 * 1 960 +8 * 10 9.6 +8 * 0.1 1 0 1 96 + 16 * 1 960 + 16 * 10 9.6 + 16 * 0.1 1 1 0 96 + 24 * 1 960 + 24 * 10 9.6 + 24 * 0.1 1 1 1 96 + 32 * 1 960 + 32 * 10 9.6 + 32 * 0.1 0 0 0 96 + 40 * 1 960 + 40 * 10 9.6 + 40 * 0.1 0 0 1 96 + 48 * 1 960 + 48 * 10 9.6 + 48 * 0.1 0 1 0 96 + 96 * 1 960 + 96 * 01 9.6 + 96 * 0.1"]
    #[inline(always)]
    pub fn ifg2_0(&self) -> Ifg2_0R {
        Ifg2_0R::new(((self.bits >> 10) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - The loopback function must be independent of the link state. 2'b00 : normal operation 2'b01 : Loopback mode_R2T(lbkmode_r2t) : Lbk methld: no SW requirement. 1. receive ingress data. 2. assign rxd to txd (txd). 3. egress pkt. b10 : Reserved 2'b11 : Loopback mode(lbk_mode). Lbk method: 1. rx fet descriptor. 2. tx fet descriptor and data 3. put data in txfifo 4. assign txd to rxd.(lbk) 5. gmac mv data to lxb. In this mode(lbk_mode), the recommended value of RC_REG(0x44) is 0xf."]
    #[inline(always)]
    pub fn lbk(&mut self) -> LbkW<'_, TcrSpec> {
        LbkW::new(self, 8)
    }
    #[doc = "Bits 10:12 - InterFrameGap Time: This field allows the user to adjust the interframe gap time longer than the standard: 9.6 us for 10Mbps, 960 ns for 100Mbps. The time can be programmed from 9.6 us to 14.4 us (10Mbps) and 960ns to 1440ns (100Mbps). The formula for the inter frame gap is: IFG\\[2:0\\] IFG@1000MHz (nS) IFG@100MHz (nS) IFG@10MHz (uS) 0 1 1 96 960 9.6 1 0 0 96 +8 * 1 960 +8 * 10 9.6 +8 * 0.1 1 0 1 96 + 16 * 1 960 + 16 * 10 9.6 + 16 * 0.1 1 1 0 96 + 24 * 1 960 + 24 * 10 9.6 + 24 * 0.1 1 1 1 96 + 32 * 1 960 + 32 * 10 9.6 + 32 * 0.1 0 0 0 96 + 40 * 1 960 + 40 * 10 9.6 + 40 * 0.1 0 0 1 96 + 48 * 1 960 + 48 * 10 9.6 + 48 * 0.1 0 1 0 96 + 96 * 1 960 + 96 * 01 9.6 + 96 * 0.1"]
    #[inline(always)]
    pub fn ifg2_0(&mut self) -> Ifg2_0W<'_, TcrSpec> {
        Ifg2_0W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TcrSpec {}
