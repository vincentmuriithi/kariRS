#[doc = "Register `PINA` reader"]
pub struct R(crate::R<PINA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINA` writer"]
pub struct W(crate::W<PINA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINA_SPEC>;
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
impl From<crate::W<PINA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA0` reader - Pin A0"]
pub type PA0_R = crate::BitReader<bool>;
#[doc = "Field `PA0` writer - Pin A0"]
pub type PA0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINA_SPEC, bool, O>;
#[doc = "Field `PA1` reader - Pin A1"]
pub type PA1_R = crate::BitReader<bool>;
#[doc = "Field `PA1` writer - Pin A1"]
pub type PA1_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINA_SPEC, bool, O>;
#[doc = "Field `PA2` reader - Pin A2"]
pub type PA2_R = crate::BitReader<bool>;
#[doc = "Field `PA2` writer - Pin A2"]
pub type PA2_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINA_SPEC, bool, O>;
#[doc = "Field `PA3` reader - Pin A3"]
pub type PA3_R = crate::BitReader<bool>;
#[doc = "Field `PA3` writer - Pin A3"]
pub type PA3_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINA_SPEC, bool, O>;
#[doc = "Field `PA4` reader - Pin A4"]
pub type PA4_R = crate::BitReader<bool>;
#[doc = "Field `PA4` writer - Pin A4"]
pub type PA4_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINA_SPEC, bool, O>;
#[doc = "Field `PA5` reader - Pin A5"]
pub type PA5_R = crate::BitReader<bool>;
#[doc = "Field `PA5` writer - Pin A5"]
pub type PA5_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINA_SPEC, bool, O>;
#[doc = "Field `PA6` reader - Pin A6"]
pub type PA6_R = crate::BitReader<bool>;
#[doc = "Field `PA6` writer - Pin A6"]
pub type PA6_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINA_SPEC, bool, O>;
#[doc = "Field `PA7` reader - Pin A7"]
pub type PA7_R = crate::BitReader<bool>;
#[doc = "Field `PA7` writer - Pin A7"]
pub type PA7_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin A0"]
    #[inline(always)]
    pub fn pa0(&self) -> PA0_R {
        PA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin A1"]
    #[inline(always)]
    pub fn pa1(&self) -> PA1_R {
        PA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin A2"]
    #[inline(always)]
    pub fn pa2(&self) -> PA2_R {
        PA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin A3"]
    #[inline(always)]
    pub fn pa3(&self) -> PA3_R {
        PA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin A4"]
    #[inline(always)]
    pub fn pa4(&self) -> PA4_R {
        PA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin A5"]
    #[inline(always)]
    pub fn pa5(&self) -> PA5_R {
        PA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin A6"]
    #[inline(always)]
    pub fn pa6(&self) -> PA6_R {
        PA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin A7"]
    #[inline(always)]
    pub fn pa7(&self) -> PA7_R {
        PA7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin A0"]
    #[inline(always)]
    #[must_use]
    pub fn pa0(&mut self) -> PA0_W<0> {
        PA0_W::new(self)
    }
    #[doc = "Bit 1 - Pin A1"]
    #[inline(always)]
    #[must_use]
    pub fn pa1(&mut self) -> PA1_W<1> {
        PA1_W::new(self)
    }
    #[doc = "Bit 2 - Pin A2"]
    #[inline(always)]
    #[must_use]
    pub fn pa2(&mut self) -> PA2_W<2> {
        PA2_W::new(self)
    }
    #[doc = "Bit 3 - Pin A3"]
    #[inline(always)]
    #[must_use]
    pub fn pa3(&mut self) -> PA3_W<3> {
        PA3_W::new(self)
    }
    #[doc = "Bit 4 - Pin A4"]
    #[inline(always)]
    #[must_use]
    pub fn pa4(&mut self) -> PA4_W<4> {
        PA4_W::new(self)
    }
    #[doc = "Bit 5 - Pin A5"]
    #[inline(always)]
    #[must_use]
    pub fn pa5(&mut self) -> PA5_W<5> {
        PA5_W::new(self)
    }
    #[doc = "Bit 6 - Pin A6"]
    #[inline(always)]
    #[must_use]
    pub fn pa6(&mut self) -> PA6_W<6> {
        PA6_W::new(self)
    }
    #[doc = "Bit 7 - Pin A7"]
    #[inline(always)]
    #[must_use]
    pub fn pa7(&mut self) -> PA7_W<7> {
        PA7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port A Input Pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pina](index.html) module"]
pub struct PINA_SPEC;
impl crate::RegisterSpec for PINA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pina::R](R) reader structure"]
impl crate::Readable for PINA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pina::W](W) writer structure"]
impl crate::Writable for PINA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINA to value 0"]
impl crate::Resettable for PINA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
