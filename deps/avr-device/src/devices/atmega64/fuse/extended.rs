#[doc = "Register `EXTENDED` reader"]
pub struct R(crate::R<EXTENDED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTENDED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTENDED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTENDED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTENDED` writer"]
pub struct W(crate::W<EXTENDED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTENDED_SPEC>;
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
impl From<crate::W<EXTENDED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTENDED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTON` reader - Watchdog Timer always on"]
pub type WDTON_R = crate::BitReader<bool>;
#[doc = "Field `WDTON` writer - Watchdog Timer always on"]
pub type WDTON_W<'a, const O: u8> = crate::BitWriter<'a, u8, EXTENDED_SPEC, bool, O>;
#[doc = "Field `M103C` reader - ATmega103 Compatibility Mode"]
pub type M103C_R = crate::BitReader<bool>;
#[doc = "Field `M103C` writer - ATmega103 Compatibility Mode"]
pub type M103C_W<'a, const O: u8> = crate::BitWriter<'a, u8, EXTENDED_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Timer always on"]
    #[inline(always)]
    pub fn wdton(&self) -> WDTON_R {
        WDTON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ATmega103 Compatibility Mode"]
    #[inline(always)]
    pub fn m103c(&self) -> M103C_R {
        M103C_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer always on"]
    #[inline(always)]
    #[must_use]
    pub fn wdton(&mut self) -> WDTON_W<0> {
        WDTON_W::new(self)
    }
    #[doc = "Bit 1 - ATmega103 Compatibility Mode"]
    #[inline(always)]
    #[must_use]
    pub fn m103c(&mut self) -> M103C_W<1> {
        M103C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extended](index.html) module"]
pub struct EXTENDED_SPEC;
impl crate::RegisterSpec for EXTENDED_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [extended::R](R) reader structure"]
impl crate::Readable for EXTENDED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extended::W](W) writer structure"]
impl crate::Writable for EXTENDED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTENDED to value 0"]
impl crate::Resettable for EXTENDED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
