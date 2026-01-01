#[doc = "Register `USBSTA` reader"]
pub struct R(crate::R<USBSTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBSTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBSTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBSTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBSTA` writer"]
pub struct W(crate::W<USBSTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBSTA_SPEC>;
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
impl From<crate::W<USBSTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBSTA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUS` reader - No Description."]
pub type VBUS_R = crate::BitReader<bool>;
#[doc = "Field `VBUS` writer - No Description."]
pub type VBUS_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBSTA_SPEC, bool, O>;
#[doc = "Field `ID` reader - No Description."]
pub type ID_R = crate::BitReader<bool>;
#[doc = "Field `ID` writer - No Description."]
pub type ID_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBSTA_SPEC, bool, O>;
#[doc = "Field `SPEED` reader - No Description."]
pub type SPEED_R = crate::BitReader<bool>;
#[doc = "Field `SPEED` writer - No Description."]
pub type SPEED_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBSTA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn vbus(&mut self) -> VBUS_W<0> {
        VBUS_W::new(self)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<1> {
        ID_W::new(self)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<3> {
        SPEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsta](index.html) module"]
pub struct USBSTA_SPEC;
impl crate::RegisterSpec for USBSTA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbsta::R](R) reader structure"]
impl crate::Readable for USBSTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbsta::W](W) writer structure"]
impl crate::Writable for USBSTA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBSTA to value 0"]
impl crate::Resettable for USBSTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
