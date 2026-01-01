#[doc = "Register `HIGH` reader"]
pub struct R(crate::R<HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIGH` writer"]
pub struct W(crate::W<HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIGH_SPEC>;
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
impl From<crate::W<HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BODLEVEL` reader - Brown-out Detector trigger level"]
pub type BODLEVEL_R = crate::FieldReader<u8, BODLEVEL_A>;
#[doc = "Brown-out Detector trigger level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BODLEVEL_A {
    #[doc = "4: Brown-out detection at VCC=4.3 V"]
    _4V3 = 4,
    #[doc = "5: Brown-out detection at VCC=2.7 V"]
    _2V7 = 5,
    #[doc = "6: Brown-out detection at VCC=1.8 V"]
    _1V8 = 6,
}
impl From<BODLEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BODLEVEL_A) -> Self {
        variant as _
    }
}
impl BODLEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BODLEVEL_A> {
        match self.bits {
            4 => Some(BODLEVEL_A::_4V3),
            5 => Some(BODLEVEL_A::_2V7),
            6 => Some(BODLEVEL_A::_1V8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4V3`"]
    #[inline(always)]
    pub fn is_4v3(&self) -> bool {
        *self == BODLEVEL_A::_4V3
    }
    #[doc = "Checks if the value of the field is `_2V7`"]
    #[inline(always)]
    pub fn is_2v7(&self) -> bool {
        *self == BODLEVEL_A::_2V7
    }
    #[doc = "Checks if the value of the field is `_1V8`"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == BODLEVEL_A::_1V8
    }
}
#[doc = "Field `BODLEVEL` writer - Brown-out Detector trigger level"]
pub type BODLEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, HIGH_SPEC, u8, BODLEVEL_A, 3, O>;
impl<'a, const O: u8> BODLEVEL_W<'a, O> {
    #[doc = "Brown-out detection at VCC=4.3 V"]
    #[inline(always)]
    pub fn _4v3(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_4V3)
    }
    #[doc = "Brown-out detection at VCC=2.7 V"]
    #[inline(always)]
    pub fn _2v7(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_2V7)
    }
    #[doc = "Brown-out detection at VCC=1.8 V"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut W {
        self.variant(BODLEVEL_A::_1V8)
    }
}
#[doc = "Field `EESAVE` reader - Preserve EEPROM through the Chip Erase cycle"]
pub type EESAVE_R = crate::BitReader<bool>;
#[doc = "Field `EESAVE` writer - Preserve EEPROM through the Chip Erase cycle"]
pub type EESAVE_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
#[doc = "Field `WDTON` reader - Watch-dog Timer always on"]
pub type WDTON_R = crate::BitReader<bool>;
#[doc = "Field `WDTON` writer - Watch-dog Timer always on"]
pub type WDTON_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
#[doc = "Field `SPIEN` reader - Serial program downloading (SPI) enabled"]
pub type SPIEN_R = crate::BitReader<bool>;
#[doc = "Field `SPIEN` writer - Serial program downloading (SPI) enabled"]
pub type SPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
#[doc = "Field `DWEN` reader - Debug Wire enable"]
pub type DWEN_R = crate::BitReader<bool>;
#[doc = "Field `DWEN` writer - Debug Wire enable"]
pub type DWEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
#[doc = "Field `RSTDISBL` reader - Reset Disabled (Enable PC2 as i/o pin)"]
pub type RSTDISBL_R = crate::BitReader<bool>;
#[doc = "Field `RSTDISBL` writer - Reset Disabled (Enable PC2 as i/o pin)"]
pub type RSTDISBL_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Brown-out Detector trigger level"]
    #[inline(always)]
    pub fn bodlevel(&self) -> BODLEVEL_R {
        BODLEVEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Preserve EEPROM through the Chip Erase cycle"]
    #[inline(always)]
    pub fn eesave(&self) -> EESAVE_R {
        EESAVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watch-dog Timer always on"]
    #[inline(always)]
    pub fn wdton(&self) -> WDTON_R {
        WDTON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Serial program downloading (SPI) enabled"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug Wire enable"]
    #[inline(always)]
    pub fn dwen(&self) -> DWEN_R {
        DWEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset Disabled (Enable PC2 as i/o pin)"]
    #[inline(always)]
    pub fn rstdisbl(&self) -> RSTDISBL_R {
        RSTDISBL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Brown-out Detector trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn bodlevel(&mut self) -> BODLEVEL_W<0> {
        BODLEVEL_W::new(self)
    }
    #[doc = "Bit 3 - Preserve EEPROM through the Chip Erase cycle"]
    #[inline(always)]
    #[must_use]
    pub fn eesave(&mut self) -> EESAVE_W<3> {
        EESAVE_W::new(self)
    }
    #[doc = "Bit 4 - Watch-dog Timer always on"]
    #[inline(always)]
    #[must_use]
    pub fn wdton(&mut self) -> WDTON_W<4> {
        WDTON_W::new(self)
    }
    #[doc = "Bit 5 - Serial program downloading (SPI) enabled"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<5> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 6 - Debug Wire enable"]
    #[inline(always)]
    #[must_use]
    pub fn dwen(&mut self) -> DWEN_W<6> {
        DWEN_W::new(self)
    }
    #[doc = "Bit 7 - Reset Disabled (Enable PC2 as i/o pin)"]
    #[inline(always)]
    #[must_use]
    pub fn rstdisbl(&mut self) -> RSTDISBL_W<7> {
        RSTDISBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [high](index.html) module"]
pub struct HIGH_SPEC;
impl crate::RegisterSpec for HIGH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [high::R](R) reader structure"]
impl crate::Readable for HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [high::W](W) writer structure"]
impl crate::Writable for HIGH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIGH to value 0"]
impl crate::Resettable for HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
