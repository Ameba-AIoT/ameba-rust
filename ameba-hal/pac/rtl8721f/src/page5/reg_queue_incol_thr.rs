#[doc = "Register `REG_QUEUE_INCOL_THR` reader"]
pub type R = crate::R<RegQueueIncolThrSpec>;
#[doc = "Register `REG_QUEUE_INCOL_THR` writer"]
pub type W = crate::W<RegQueueIncolThrSpec>;
#[doc = "Field `BIT_VO_QUEUE_THR` reader - 0: Disable this VO queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
pub type BitVoQueueThrR = crate::FieldReader;
#[doc = "Field `BIT_VO_QUEUE_THR` writer - 0: Disable this VO queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
pub type BitVoQueueThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_VI_QUEUE_THR` reader - 0: Disable this VI queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
pub type BitViQueueThrR = crate::FieldReader;
#[doc = "Field `BIT_VI_QUEUE_THR` writer - 0: Disable this VI queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
pub type BitViQueueThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BE_QUEUE_THR` reader - 0: Disable this BE queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
pub type BitBeQueueThrR = crate::FieldReader;
#[doc = "Field `BIT_BE_QUEUE_THR` writer - 0: Disable this BE queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
pub type BitBeQueueThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BK_QUEUE_THR` reader - 0: Disable this BK queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
pub type BitBkQueueThrR = crate::FieldReader;
#[doc = "Field `BIT_BK_QUEUE_THR` writer - 0: Disable this BK queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
pub type BitBkQueueThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 0: Disable this VO queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
    #[inline(always)]
    pub fn bit_vo_queue_thr(&self) -> BitVoQueueThrR {
        BitVoQueueThrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 0: Disable this VI queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
    #[inline(always)]
    pub fn bit_vi_queue_thr(&self) -> BitViQueueThrR {
        BitViQueueThrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 0: Disable this BE queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
    #[inline(always)]
    pub fn bit_be_queue_thr(&self) -> BitBeQueueThrR {
        BitBeQueueThrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 0: Disable this BK queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
    #[inline(always)]
    pub fn bit_bk_queue_thr(&self) -> BitBkQueueThrR {
        BitBkQueueThrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 0: Disable this VO queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
    #[inline(always)]
    pub fn bit_vo_queue_thr(&mut self) -> BitVoQueueThrW<'_, RegQueueIncolThrSpec> {
        BitVoQueueThrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 0: Disable this VI queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
    #[inline(always)]
    pub fn bit_vi_queue_thr(&mut self) -> BitViQueueThrW<'_, RegQueueIncolThrSpec> {
        BitViQueueThrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 0: Disable this BE queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
    #[inline(always)]
    pub fn bit_be_queue_thr(&mut self) -> BitBeQueueThrW<'_, RegQueueIncolThrSpec> {
        BitBeQueueThrW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 0: Disable this BK queue condition. Otherwise: When internal collision number of queue equal threshold, the priority of queue enhance. But have more 2 queues which satisfy each queue threshold, the priority will follow VO>VI>BE>BK."]
    #[inline(always)]
    pub fn bit_bk_queue_thr(&mut self) -> BitBkQueueThrW<'_, RegQueueIncolThrSpec> {
        BitBkQueueThrW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_queue_incol_thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_queue_incol_thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQueueIncolThrSpec;
impl crate::RegisterSpec for RegQueueIncolThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_queue_incol_thr::R`](R) reader structure"]
impl crate::Readable for RegQueueIncolThrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_queue_incol_thr::W`](W) writer structure"]
impl crate::Writable for RegQueueIncolThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_QUEUE_INCOL_THR to value 0"]
impl crate::Resettable for RegQueueIncolThrSpec {}
