#[doc = "Register `PRR1` reader"]
pub struct R(crate::R<PRR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRR1` writer"]
pub struct W(crate::W<PRR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRR1_SPEC>;
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
impl From<crate::W<PRR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRTIM3` reader - Power Reduction Timer/Counter3"]
pub type PRTIM3_R = crate::BitReader<bool>;
#[doc = "Field `PRTIM3` writer - Power Reduction Timer/Counter3"]
pub type PRTIM3_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power Reduction Timer/Counter3"]
    #[inline(always)]
    pub fn prtim3(&self) -> PRTIM3_R {
        PRTIM3_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Reduction Timer/Counter3"]
    #[inline(always)]
    #[must_use]
    pub fn prtim3(&mut self) -> PRTIM3_W<0> {
        PRTIM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Reduction Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prr1](index.html) module"]
pub struct PRR1_SPEC;
impl crate::RegisterSpec for PRR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [prr1::R](R) reader structure"]
impl crate::Readable for PRR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prr1::W](W) writer structure"]
impl crate::Writable for PRR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRR1 to value 0"]
impl crate::Resettable for PRR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
