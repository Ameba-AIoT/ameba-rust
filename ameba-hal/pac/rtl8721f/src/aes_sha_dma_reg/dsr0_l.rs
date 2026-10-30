#[doc = "Register `DSR0_L` reader"]
pub type R = crate::R<Dsr0LSpec>;
#[doc = "Register `DSR0_L` writer"]
pub type W = crate::W<Dsr0LSpec>;
#[doc = "Field `DSR0_L_DSI` reader - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr0LDsiR = crate::FieldReader<u32>;
#[doc = "Field `DSR0_L_DSI` writer - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr0LDsiW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DSR0_L_DSC` reader - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr0LDscR = crate::FieldReader<u16>;
#[doc = "Field `DSR0_L_DSC` writer - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr0LDscW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr0_l_dsi(&self) -> Dsr0LDsiR {
        Dsr0LDsiR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr0_l_dsc(&self) -> Dsr0LDscR {
        Dsr0LDscR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr0_l_dsi(&mut self) -> Dsr0LDsiW<'_, Dsr0LSpec> {
        Dsr0LDsiW::new(self, 0)
    }
    #[doc = "Bits 20:31 - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr0_l_dsc(&mut self) -> Dsr0LDscW<'_, Dsr0LSpec> {
        Dsr0LDscW::new(self, 20)
    }
}
#[doc = "Channel 0 low-32 bits Destination Scatter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr0_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr0_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsr0LSpec;
impl crate::RegisterSpec for Dsr0LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr0_l::R`](R) reader structure"]
impl crate::Readable for Dsr0LSpec {}
#[doc = "`write(|w| ..)` method takes [`dsr0_l::W`](W) writer structure"]
impl crate::Writable for Dsr0LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSR0_L to value 0"]
impl crate::Resettable for Dsr0LSpec {}
