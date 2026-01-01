#[doc = "Register `TIMSK2` reader"]
pub struct R(crate::R<TIMSK2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSK2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSK2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSK2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSK2` writer"]
pub struct W(crate::W<TIMSK2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSK2_SPEC>;
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
impl From<crate::W<TIMSK2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSK2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOIE2` reader - Timer/Counter2 Overflow Interrupt Enable"]
pub type TOIE2_R = crate::BitReader<bool>;
#[doc = "Field `TOIE2` writer - Timer/Counter2 Overflow Interrupt Enable"]
pub type TOIE2_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK2_SPEC, bool, O>;
#[doc = "Field `OCIE2A` reader - Timer/Counter2 Output Compare Match A Interrupt Enable"]
pub type OCIE2A_R = crate::BitReader<bool>;
#[doc = "Field `OCIE2A` writer - Timer/Counter2 Output Compare Match A Interrupt Enable"]
pub type OCIE2A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK2_SPEC, bool, O>;
#[doc = "Field `OCIE2B` reader - Timer/Counter2 Output Compare Match B Interrupt Enable"]
pub type OCIE2B_R = crate::BitReader<bool>;
#[doc = "Field `OCIE2B` writer - Timer/Counter2 Output Compare Match B Interrupt Enable"]
pub type OCIE2B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMSK2_SPEC, bool, O>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TIMSK2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter2 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie2(&self) -> TOIE2_R {
        TOIE2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter2 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    pub fn ocie2a(&self) -> OCIE2A_R {
        OCIE2A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter2 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    pub fn ocie2b(&self) -> OCIE2B_R {
        OCIE2B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 3) & 0x1f)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter2 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie2(&mut self) -> TOIE2_W<0> {
        TOIE2_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter2 Output Compare Match A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie2a(&mut self) -> OCIE2A_W<1> {
        OCIE2A_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter2 Output Compare Match B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie2b(&mut self) -> OCIE2B_W<2> {
        OCIE2B_W::new(self)
    }
    #[doc = "Bits 3:7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<3> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter Interrupt Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timsk2](index.html) module"]
pub struct TIMSK2_SPEC;
impl crate::RegisterSpec for TIMSK2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [timsk2::R](R) reader structure"]
impl crate::Readable for TIMSK2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timsk2::W](W) writer structure"]
impl crate::Writable for TIMSK2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMSK2 to value 0"]
impl crate::Resettable for TIMSK2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
