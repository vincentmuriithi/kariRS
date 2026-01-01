#[doc = "Register `OSC32KCTRLA` reader"]
pub struct R(crate::R<OSC32KCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC32KCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC32KCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC32KCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC32KCTRLA` writer"]
pub struct W(crate::W<OSC32KCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC32KCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OSC32KCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC32KCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, OSC32KCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Run standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Run standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<1> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC32K Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc32kctrla](index.html) module"]
pub struct OSC32KCTRLA_SPEC;
impl crate::RegisterSpec for OSC32KCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osc32kctrla::R](R) reader structure"]
impl crate::Readable for OSC32KCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc32kctrla::W](W) writer structure"]
impl crate::Writable for OSC32KCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC32KCTRLA to value 0"]
impl crate::Resettable for OSC32KCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
