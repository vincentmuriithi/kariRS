#[doc = "Register `UCSR1C` reader"]
pub struct R(crate::R<UCSR1C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSR1C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSR1C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSR1C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSR1C` writer"]
pub struct W(crate::W<UCSR1C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSR1C_SPEC>;
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
impl From<crate::W<UCSR1C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSR1C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCPOL1` reader - Clock Polarity"]
pub type UCPOL1_R = crate::BitReader<bool>;
#[doc = "Field `UCPOL1` writer - Clock Polarity"]
pub type UCPOL1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR1C_SPEC, bool, O>;
#[doc = "Field `UCPHA1` reader - Clock Phase"]
pub type UCPHA1_R = crate::BitReader<bool>;
#[doc = "Field `UCPHA1` writer - Clock Phase"]
pub type UCPHA1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR1C_SPEC, bool, O>;
#[doc = "Field `UDORD1` reader - Data Order"]
pub type UDORD1_R = crate::BitReader<bool>;
#[doc = "Field `UDORD1` writer - Data Order"]
pub type UDORD1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR1C_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn ucpol1(&self) -> UCPOL1_R {
        UCPOL1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ucpha1(&self) -> UCPHA1_R {
        UCPHA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Order"]
    #[inline(always)]
    pub fn udord1(&self) -> UDORD1_R {
        UDORD1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ucpol1(&mut self) -> UCPOL1_W<0> {
        UCPOL1_W::new(self)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ucpha1(&mut self) -> UCPHA1_W<1> {
        UCPHA1_W::new(self)
    }
    #[doc = "Bit 2 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn udord1(&mut self) -> UDORD1_W<2> {
        UDORD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART1 MSPIM Control and Status Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsr1c](index.html) module"]
pub struct UCSR1C_SPEC;
impl crate::RegisterSpec for UCSR1C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsr1c::R](R) reader structure"]
impl crate::Readable for UCSR1C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsr1c::W](W) writer structure"]
impl crate::Writable for UCSR1C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR1C to value 0"]
impl crate::Resettable for UCSR1C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
