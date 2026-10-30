#[doc = "Register `UART_TH_RETI` reader"]
pub type R = crate::R<UartThRetiSpec>;
#[doc = "Register `UART_TH_RETI` writer"]
pub type W = crate::W<UartThRetiSpec>;
#[doc = "Field `TH_RETI` reader - Set rxfifo empty timeout interrupt, when rxfifo is empty for longer than TH_RETI. Timeout counter works in uart_rclk."]
pub type ThRetiR = crate::FieldReader<u32>;
#[doc = "Field `TH_RETI` writer - Set rxfifo empty timeout interrupt, when rxfifo is empty for longer than TH_RETI. Timeout counter works in uart_rclk."]
pub type ThRetiW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Set rxfifo empty timeout interrupt, when rxfifo is empty for longer than TH_RETI. Timeout counter works in uart_rclk."]
    #[inline(always)]
    pub fn th_reti(&self) -> ThRetiR {
        ThRetiR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Set rxfifo empty timeout interrupt, when rxfifo is empty for longer than TH_RETI. Timeout counter works in uart_rclk."]
    #[inline(always)]
    pub fn th_reti(&mut self) -> ThRetiW<'_, UartThRetiSpec> {
        ThRetiW::new(self, 0)
    }
}
#[doc = "UART rxfifo empty interrupt timeout threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_th_reti::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_th_reti::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartThRetiSpec;
impl crate::RegisterSpec for UartThRetiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_th_reti::R`](R) reader structure"]
impl crate::Readable for UartThRetiSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_th_reti::W`](W) writer structure"]
impl crate::Writable for UartThRetiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_TH_RETI to value 0"]
impl crate::Resettable for UartThRetiSpec {}
